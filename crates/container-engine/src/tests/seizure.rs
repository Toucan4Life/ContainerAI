use super::{
    clear_containers, new_game, pass_turn, set_cash, set_factories, set_holding, set_loans,
    set_lots, set_scoring_area, set_ship, set_warehouses,
};
use crate::action::{Action, District};
use crate::color::{Color, ColorCounts};
use crate::config::GameLength;
use crate::game::Game;
use crate::phase::Phase;
use crate::state::{BANK_LOTS, FACTORY_LOTS, HARBOR_LOTS, PlayerId, PlayerState, ShipLocation};
use Color::{Blue, Green, Red, White, Yellow};
use std::collections::{HashMap, HashSet};

fn counts(items: &[(Color, u8)]) -> ColorCounts {
    let mut tally = ColorCounts::default();
    for &(color, n) in items {
        tally[color] += n;
    }
    tally
}

fn plus(mut lot: ColorCounts, color: Color) -> ColorCounts {
    lot[color] += 1;
    lot
}

fn seize(color: Color) -> Action {
    Action::Seize { lot: 0, color }
}

fn lot_seize(lot: usize, color: Color) -> Action {
    Action::Seize { lot, color }
}

fn seizing(g: &Game) -> bool {
    matches!(g.phase, Phase::Seize(_))
}

fn distributing(g: &Game) -> bool {
    matches!(g.phase, Phase::Distribute(_))
}

fn bank_lots(g: &Game) -> [ColorCounts; BANK_LOTS] {
    g.state.bank.container_lots
}

/// A game one turn before the debtor's. They owe `loans`, have `cash`, and own only what `stock` gives them.
/// The turn passes with `pass_turn`.
fn before_default(
    players: usize,
    loans: u8,
    cash: u32,
    stock: impl FnOnce(&mut Game, PlayerId),
) -> (Game, PlayerId) {
    let mut g = new_game(players, GameLength::Standard, 1);
    let debtor = (g.current_player() + 1) % players;
    set_loans(&mut g, debtor, loans);
    set_cash(&mut g, debtor, cash);
    clear_containers(&mut g, debtor);
    stock(&mut g, debtor);
    (g, debtor)
}

/// The places the Bank seizes from, in the order it looks: scoring area, ship, holding, harbor, factory.
/// Place `first` and every one after it holds one container of its own color.
fn places_from(first: usize) -> impl FnOnce(&mut Game, PlayerId) {
    move |g, d| {
        if first == 0 {
            set_scoring_area(g, d, counts(&[(Red, 1)]));
        }
        if first <= 1 {
            set_ship(g, d, counts(&[(Blue, 1)]));
        }
        if first <= 2 {
            set_holding(g, d, counts(&[(Yellow, 1)]));
        }
        if first <= 3 {
            set_lots(g, d, District::Harbor, &[(0, White, 1)]);
        }
        set_lots(g, d, District::Factory, &[(0, Green, 1)]);
    }
}

fn place_total(p: &PlayerState, place: usize) -> u32 {
    match place {
        0 => p.scoring_area.total(),
        1 => p.ship.total(),
        2 => p.holding.total(),
        3 => p.harbor_stock(),
        _ => p.factory_stock(),
    }
}

// What is seized

#[test]
fn each_unpaid_loan_costs_one_container() {
    for (held, seized) in [(0, 2), (1, 1), (2, 0)] {
        let (mut g, debtor) = before_default(3, 2, held, |g, d| {
            set_scoring_area(g, d, counts(&[(Red, 5)]));
        });

        pass_turn(&mut g);

        let left = g.state.players[debtor].scoring_area[Red];
        assert_eq!(left, 5 - seized, "cash {held}");
        assert_eq!(g.phase, Phase::Main, "cash {held}");
        assert_eq!(g.current_player(), debtor);
        assert_eq!(g.check_invariants(), Ok(()));
    }
}

#[test]
fn the_interest_the_player_can_afford_is_paid_before_anything_is_seized() {
    let (mut g, debtor) = before_default(3, 2, 1, |g, d| {
        set_scoring_area(g, d, counts(&[(Red, 5)]));
    });

    pass_turn(&mut g);

    assert_eq!(g.state.players[debtor].cash, 0);
    assert_eq!(g.state.bank.cash_lots, [2, 2, 3]);
    assert_eq!(g.state.players[debtor].scoring_area[Red], 4);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_loans_are_kept_after_a_default() {
    let (mut g, debtor) = before_default(3, 2, 0, |g, d| {
        set_scoring_area(g, d, counts(&[(Red, 5)]));
    });

    pass_turn(&mut g);

    assert_eq!(g.state.players[debtor].loans, 2);
}

#[test]
fn seized_containers_are_dealt_to_the_bank_lots_from_lot_one() {
    let (mut g, _) = before_default(3, 2, 0, |g, d| {
        set_scoring_area(g, d, counts(&[(Red, 5)]));
    });
    let before = bank_lots(&g);

    pass_turn(&mut g);

    let after = bank_lots(&g);
    assert_eq!(after[0], plus(before[0], Red));
    assert_eq!(after[1], plus(before[1], Red));
    assert_eq!(after[2], before[2]);
}

#[test]
fn a_single_seized_container_goes_to_lot_one() {
    let (mut g, _) = before_default(3, 2, 1, |g, d| {
        set_scoring_area(g, d, counts(&[(Red, 5)]));
    });
    let before = bank_lots(&g);

    pass_turn(&mut g);

    let after = bank_lots(&g);
    assert_eq!(after[0], plus(before[0], Red));
    assert_eq!(after[1], before[1]);
    assert_eq!(after[2], before[2]);
}

#[test]
fn the_interest_falls_due_again_next_turn_and_the_bank_seizes_again() {
    let (mut g, debtor) = before_default(3, 2, 0, |g, d| {
        set_scoring_area(g, d, counts(&[(Red, 5)]));
    });
    pass_turn(&mut g);
    assert_eq!(g.state.players[debtor].scoring_area[Red], 3);

    for _ in 0..3 {
        pass_turn(&mut g);
    }

    assert_eq!(g.current_player(), debtor);
    assert_eq!(g.state.players[debtor].scoring_area[Red], 1);
    assert_eq!(g.state.players[debtor].loans, 2);
    assert_eq!(g.check_invariants(), Ok(()));
}

// Where it is seized from

#[test]
fn the_bank_takes_from_the_first_place_where_the_player_has_a_container() {
    let colors = [Red, Blue, Yellow, White, Green];
    for (first, &color) in colors.iter().enumerate() {
        let (mut g, debtor) = before_default(3, 2, 1, places_from(first));
        let before = bank_lots(&g);

        pass_turn(&mut g);

        let p = &g.state.players[debtor];
        for place in 0..5 {
            let had = u32::from(place >= first);
            let taken = u32::from(place == first);
            assert_eq!(
                place_total(p, place),
                had - taken,
                "first {first}, place {place}"
            );
        }
        assert_eq!(bank_lots(&g)[0], plus(before[0], color), "first {first}");
        assert_eq!(g.check_invariants(), Ok(()));
    }
}

#[test]
fn a_second_unpaid_loan_looks_again_for_the_first_place_with_a_container() {
    let (mut g, debtor) = before_default(3, 2, 0, |g, d| {
        set_scoring_area(g, d, counts(&[(Red, 1)]));
        set_ship(g, d, counts(&[(Blue, 1)]));
        set_holding(g, d, counts(&[(Yellow, 1)]));
    });

    pass_turn(&mut g);

    let p = &g.state.players[debtor];
    assert!(p.scoring_area.is_empty());
    assert!(p.ship.is_empty());
    assert_eq!(p.holding, counts(&[(Yellow, 1)]));
    assert!(
        distributing(&g),
        "red and blue are seized, and need dealing out"
    );
}

#[test]
fn the_pick_order_within_one_place_does_not_restrict_the_next_place() {
    let (mut g, debtor) = before_default(3, 2, 0, |g, d| {
        set_scoring_area(g, d, counts(&[(Green, 1)]));
        set_ship(g, d, counts(&[(Red, 1)]));
    });

    pass_turn(&mut g);

    let p = &g.state.players[debtor];
    assert!(p.scoring_area.is_empty());
    assert!(
        p.ship.is_empty(),
        "red sorts before green, in a later place"
    );
    assert!(distributing(&g));
}

#[test]
fn a_player_with_fewer_containers_than_unpaid_loans_loses_what_they_have() {
    let (mut g, debtor) = before_default(3, 2, 0, |g, d| {
        set_ship(g, d, counts(&[(Blue, 1)]));
    });
    let before = bank_lots(&g);

    pass_turn(&mut g);

    assert!(g.state.players[debtor].ship.is_empty());
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(bank_lots(&g)[0], plus(before[0], Blue));
    assert_eq!(bank_lots(&g)[1], before[1], "the other loan is forgiven");
}

#[test]
fn a_player_with_nothing_to_seize_is_forgiven_for_this_turn() {
    let (mut g, debtor) = before_default(3, 2, 0, |_, _| {});
    let before = bank_lots(&g);

    pass_turn(&mut g);

    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.current_player(), debtor);
    assert_eq!(g.state.players[debtor].loans, 2);
    assert_eq!(bank_lots(&g), before);
    assert_eq!(g.check_invariants(), Ok(()));
}

// Who decides

#[test]
fn the_player_on_the_right_chooses_which_container_is_seized() {
    for players in 3..=5 {
        let (mut g, debtor) = before_default(players, 2, 1, |g, d| {
            set_ship(g, d, counts(&[(Red, 1), (Blue, 1), (Green, 2)]));
        });

        pass_turn(&mut g);

        assert!(seizing(&g), "{players} players");
        let right = (debtor + players - 1) % players;
        assert_eq!(g.current_player(), right, "{players} players");
        assert_eq!(
            g.legal_actions(),
            vec![seize(Red), seize(Blue), seize(Green)]
        );
        assert_eq!(
            g.check_invariants(),
            Ok(()),
            "the containers taken are still counted"
        );
    }
}

#[test]
fn the_choice_is_applied_to_the_debtors_containers() {
    let (mut g, debtor) = before_default(4, 2, 1, |g, d| {
        set_ship(g, d, counts(&[(Red, 1), (Blue, 1), (Green, 2)]));
    });
    let before = bank_lots(&g);
    pass_turn(&mut g);

    g.apply(seize(Blue)).unwrap();

    assert_eq!(
        g.state.players[debtor].ship,
        counts(&[(Red, 1), (Green, 2)])
    );
    assert_eq!(bank_lots(&g)[0], plus(before[0], Blue));
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.current_player(), debtor);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn nothing_but_a_seizure_is_legal_while_the_bank_seizes() {
    let (mut g, _) = before_default(3, 2, 1, |g, d| {
        set_ship(g, d, counts(&[(Red, 1), (Blue, 1)]));
    });
    pass_turn(&mut g);
    assert!(seizing(&g));

    for action in [
        Action::RequestLoan,
        Action::RepayLoan,
        Action::Produce,
        Action::Reprice(District::Factory),
        Action::Sail(ShipLocation::Ocean),
        Action::Place(0),
        Action::Bid(0),
        Action::BuyOut,
        Action::Accept(0),
        Action::Deal(Red),
    ] {
        assert!(!g.is_legal(action), "{action:?}");
    }
}

#[test]
fn places_without_lots_are_seized_by_color_alone() {
    let (mut g, _) = before_default(3, 2, 1, |g, d| {
        set_scoring_area(g, d, counts(&[(Red, 1), (Blue, 1)]));
    });
    pass_turn(&mut g);

    assert_eq!(g.legal_actions(), vec![seize(Red), seize(Blue)]);
    assert!(!g.is_legal(lot_seize(1, Red)));
    assert!(!g.is_legal(seize(Green)), "none of that color to take");
}

#[test]
fn a_lot_that_does_not_exist_is_refused_whatever_its_number() {
    let (mut g, _) = before_default(3, 2, 1, |g, d| {
        set_warehouses(g, d, 3);
        set_lots(g, d, District::Harbor, &[(0, Red, 1), (2, Blue, 1)]);
    });
    pass_turn(&mut g);
    assert!(seizing(&g));

    for lot in [HARBOR_LOTS, HARBOR_LOTS + 1, usize::MAX / 2, usize::MAX] {
        for color in Color::ALL {
            assert!(!g.is_legal(lot_seize(lot, color)), "lot {lot}, {color:?}");
        }
    }
}

#[test]
fn harbor_containers_are_seized_by_lot_and_color() {
    let (mut g, debtor) = before_default(3, 2, 1, |g, d| {
        set_warehouses(g, d, 3);
        set_lots(
            g,
            d,
            District::Harbor,
            &[(0, Red, 1), (2, Red, 1), (2, Blue, 1)],
        );
    });
    pass_turn(&mut g);

    assert_eq!(
        g.legal_actions(),
        vec![lot_seize(0, Red), lot_seize(2, Red), lot_seize(2, Blue)]
    );
    g.apply(lot_seize(2, Red)).unwrap();

    let p = &g.state.players[debtor];
    assert_eq!(p.harbor_lots[0], counts(&[(Red, 1)]));
    assert_eq!(p.harbor_lots[1], ColorCounts::default());
    assert_eq!(p.harbor_lots[2], counts(&[(Blue, 1)]));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn factory_containers_are_seized_by_lot_and_color() {
    let (mut g, debtor) = before_default(3, 2, 1, |g, d| {
        set_factories(g, d, &[Red, Blue]);
        set_lots(g, d, District::Factory, &[(1, Blue, 2), (3, Red, 1)]);
    });
    pass_turn(&mut g);

    assert_eq!(
        g.legal_actions(),
        vec![lot_seize(1, Blue), lot_seize(3, Red)]
    );
    g.apply(lot_seize(1, Blue)).unwrap();

    let p = &g.state.players[debtor];
    assert_eq!(p.factory_lots[1], counts(&[(Blue, 1)]));
    assert_eq!(p.factory_lots[3], counts(&[(Red, 1)]));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_choice_that_leaves_too_few_containers_for_the_unpaid_loans_is_not_offered() {
    let (mut g, _) = before_default(3, 2, 0, |g, d| {
        set_ship(g, d, counts(&[(Red, 1), (Blue, 1), (Green, 1)]));
    });

    pass_turn(&mut g);

    assert_eq!(
        g.legal_actions(),
        vec![seize(Red), seize(Blue)],
        "taking green first would leave nothing to follow it with"
    );
    g.apply(seize(Blue)).unwrap();
    assert!(
        distributing(&g),
        "green is the only container that can follow blue"
    );
    assert_eq!(g.check_invariants(), Ok(()));
}

// Dealing the seized containers out

#[test]
fn the_player_on_the_right_decides_which_color_goes_in_which_lot() {
    let (mut g, debtor) = before_default(3, 2, 0, |g, d| {
        set_scoring_area(g, d, counts(&[(Red, 1), (Blue, 1)]));
    });
    let before = bank_lots(&g);

    pass_turn(&mut g);

    assert!(distributing(&g));
    assert_eq!(g.current_player(), g.state.player_on_right(debtor));
    assert_eq!(
        g.legal_actions(),
        vec![Action::Deal(Red), Action::Deal(Blue)]
    );
    assert_eq!(
        g.check_invariants(),
        Ok(()),
        "the containers in hand are counted"
    );

    g.apply(Action::Deal(Blue)).unwrap();

    assert_eq!(g.phase, Phase::Main, "the last container needs no decision");
    assert_eq!(g.current_player(), debtor);
    assert_eq!(bank_lots(&g)[0], plus(before[0], Blue));
    assert_eq!(bank_lots(&g)[1], plus(before[1], Red));
    assert_eq!(bank_lots(&g)[2], before[2]);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn nothing_but_dealing_is_legal_while_the_seized_containers_are_dealt() {
    let (mut g, _) = before_default(3, 2, 0, |g, d| {
        set_scoring_area(g, d, counts(&[(Red, 1), (Blue, 1)]));
    });
    pass_turn(&mut g);
    assert!(distributing(&g));

    for action in [
        Action::RequestLoan,
        Action::RepayLoan,
        Action::Produce,
        Action::Reprice(District::Factory),
        Action::Bid(0),
        seize(Red),
        Action::Deal(Green),
    ] {
        assert!(!g.is_legal(action), "{action:?}");
    }
}

// Every way a default can go, against the rules written out independently

type Outcome = (
    [ColorCounts; 3],
    [ColorCounts; HARBOR_LOTS],
    [ColorCounts; FACTORY_LOTS],
    [ColorCounts; BANK_LOTS],
);

fn outcome(g: &Game, debtor: PlayerId) -> Outcome {
    let p = &g.state.players[debtor];
    (
        [p.scoring_area, p.ship, p.holding],
        p.harbor_lots,
        p.factory_lots,
        g.state.bank.container_lots,
    )
}

/// Plays every action sequence of a default to its end, counting the sequences for each outcome.
fn engine_outcomes(g: &Game, debtor: PlayerId, found: &mut HashMap<Outcome, usize>) {
    assert_eq!(g.check_invariants(), Ok(()));
    if g.phase == Phase::Main {
        *found.entry(outcome(g, debtor)).or_default() += 1;
        return;
    }
    assert!(seizing(g) || distributing(g), "{:?}", g.phase);
    let legal = g.legal_actions();
    assert!(!legal.is_empty());
    for action in legal {
        let mut next = g.clone();
        next.apply(action).unwrap();
        engine_outcomes(&next, debtor, found);
    }
}

/// The lots of each place the Bank looks in, in the order it looks.
fn places(p: &PlayerState) -> [Vec<ColorCounts>; 5] {
    [
        vec![p.scoring_area],
        vec![p.ship],
        vec![p.holding],
        p.harbor_lots.to_vec(),
        p.factory_lots.to_vec(),
    ]
}

fn take(p: &mut PlayerState, place: usize, lot: usize, color: Color) {
    let lots: &mut [ColorCounts] = match place {
        0 => std::slice::from_mut(&mut p.scoring_area),
        1 => std::slice::from_mut(&mut p.ship),
        2 => std::slice::from_mut(&mut p.holding),
        3 => &mut p.harbor_lots,
        _ => &mut p.factory_lots,
    };
    lots[lot][color] -= 1;
}

fn orderings(colors: &[Color]) -> HashSet<Vec<Color>> {
    if colors.len() <= 1 {
        return HashSet::from([colors.to_vec()]);
    }
    let mut all = HashSet::new();
    for (i, &first) in colors.iter().enumerate() {
        let mut rest = colors.to_vec();
        rest.remove(i);
        for mut order in orderings(&rest) {
            order.insert(0, first);
            all.insert(order);
        }
    }
    all
}

/// One container per unpaid loan, each from the first place that has any, whichever the player on the
/// right picks; then the seized colors in every order the lots can be dealt.
fn model_outcomes(
    start: &PlayerState,
    bank: [ColorCounts; BANK_LOTS],
    unpaid: u32,
) -> HashSet<Outcome> {
    fn go(
        p: &PlayerState,
        seized: &mut Vec<Color>,
        unpaid: u32,
        bank: [ColorCounts; BANK_LOTS],
        out: &mut HashSet<Outcome>,
    ) {
        let first = places(p)
            .iter()
            .position(|lots| lots.iter().any(|lot| !lot.is_empty()));
        match first {
            Some(place) if unpaid > 0 => {
                for (lot, tally) in places(p)[place].iter().enumerate() {
                    for (color, _) in tally.iter_nonzero() {
                        let mut next = *p;
                        take(&mut next, place, lot, color);
                        seized.push(color);
                        go(&next, seized, unpaid - 1, bank, out);
                        seized.pop();
                    }
                }
            }
            _ => {
                for order in orderings(seized) {
                    let mut lots = bank;
                    for (i, &color) in order.iter().enumerate() {
                        lots[i % BANK_LOTS][color] += 1;
                    }
                    out.insert((
                        [p.scoring_area, p.ship, p.holding],
                        p.harbor_lots,
                        p.factory_lots,
                        lots,
                    ));
                }
            }
        }
    }
    let mut out = HashSet::new();
    go(start, &mut Vec::new(), unpaid, bank, &mut out);
    out
}

type Stock = fn(&mut Game, PlayerId);

fn three_colors_on_the_ship(g: &mut Game, d: PlayerId) {
    set_ship(g, d, counts(&[(Red, 1), (Blue, 1), (Green, 1)]));
}

fn scoring_area_and_ship(g: &mut Game, d: PlayerId) {
    set_scoring_area(g, d, counts(&[(Red, 2), (Blue, 1)]));
    set_ship(g, d, counts(&[(Yellow, 1), (White, 1)]));
}

fn one_then_a_choice(g: &mut Game, d: PlayerId) {
    set_scoring_area(g, d, counts(&[(Green, 1)]));
    set_ship(g, d, counts(&[(Red, 1), (Blue, 1)]));
}

fn a_harbor_of_lots(g: &mut Game, d: PlayerId) {
    set_warehouses(g, d, 5);
    set_lots(
        g,
        d,
        District::Harbor,
        &[(0, Red, 1), (2, Red, 1), (2, Blue, 1), (4, Green, 1)],
    );
}

fn a_factory_of_lots(g: &mut Game, d: PlayerId) {
    set_factories(g, d, &[Red, Blue]);
    set_lots(
        g,
        d,
        District::Factory,
        &[(0, Red, 1), (1, Blue, 2), (3, Red, 1)],
    );
}

fn everywhere(g: &mut Game, d: PlayerId) {
    set_scoring_area(g, d, counts(&[(Blue, 1)]));
    set_ship(g, d, counts(&[(Red, 1)]));
    set_holding(g, d, counts(&[(Yellow, 1), (Green, 1)]));
    set_warehouses(g, d, 2);
    set_lots(g, d, District::Harbor, &[(1, White, 1)]);
    set_lots(g, d, District::Factory, &[(2, Green, 1)]);
}

fn nothing_at_all(_: &mut Game, _: PlayerId) {}

fn a_single_container(g: &mut Game, d: PlayerId) {
    set_holding(g, d, counts(&[(White, 1)]));
}

#[test]
fn every_way_a_default_can_go_is_reachable_by_exactly_one_action_sequence() {
    // The stock, the cash held (so two loans or one are unpaid), and how many outcomes there should be.
    let cases: [(&str, Stock, u32, usize); 14] = [
        ("three colors, two unpaid", three_colors_on_the_ship, 0, 6),
        ("three colors, one unpaid", three_colors_on_the_ship, 1, 3),
        ("area and ship, two unpaid", scoring_area_and_ship, 0, 3),
        ("area and ship, one unpaid", scoring_area_and_ship, 1, 2),
        ("one then a choice", one_then_a_choice, 0, 4),
        ("harbor lots, two unpaid", a_harbor_of_lots, 0, 11),
        ("harbor lots, one unpaid", a_harbor_of_lots, 1, 4),
        ("factory lots, two unpaid", a_factory_of_lots, 0, 6),
        ("factory lots, one unpaid", a_factory_of_lots, 1, 3),
        ("everywhere, two unpaid", everywhere, 0, 2),
        ("everywhere, one unpaid", everywhere, 1, 1),
        ("nothing, two unpaid", nothing_at_all, 0, 1),
        ("a single container, two unpaid", a_single_container, 0, 1),
        ("a single container, one unpaid", a_single_container, 1, 1),
    ];
    for (name, stock, held, expected) in cases {
        for players in [3, 5] {
            let (mut g, debtor) = before_default(players, 2, held, stock);
            let start = g.state.players[debtor];
            let bank = bank_lots(&g);
            pass_turn(&mut g);

            let mut found = HashMap::new();
            engine_outcomes(&g, debtor, &mut found);

            let unpaid = 2 - held;
            let model = model_outcomes(&start, bank, unpaid);
            let reached: HashSet<Outcome> = found.keys().copied().collect();
            assert_eq!(reached, model, "{name}, {players} players");
            assert_eq!(model.len(), expected, "{name}, {players} players");
            assert!(
                found.values().all(|&sequences| sequences == 1),
                "{name}, {players} players: some outcome has several action sequences"
            );
        }
    }
}
