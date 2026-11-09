use super::{
    clear_containers, finish_arrange, new_game, pass_turn, set_cash, set_cash_lot, set_factories,
    set_holding, set_loans, set_lots, set_scoring_area, set_ship, set_warehouses,
};
use crate::action::{Action, District};
use crate::auction::MAX_CONTAINER_BID;
use crate::color::{Color, ColorCounts};
use crate::config::GameLength;
use crate::game::Game;
use crate::phase::Phase;
use crate::state::{
    Auction, AuctionKind, Auctions, BANK_LOTS, Bid, ContainerBid, PlayerId, PlayerState,
};
use AuctionKind::{CashLot, ContainerLot};
use Color::{Blue, Green, Red, White, Yellow};
use District::{Factory, Harbor};

/// A container of a district's lot, as picked for a bid.
type Pick = (District, usize, Color);

fn cash(g: &Game, p: PlayerId) -> u32 {
    g.state.players[p].cash
}

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

/// The active auction of a cash lot, which is bid on with containers.
fn tile(g: &Game) -> Option<Auction> {
    g.state.bank.auctions[CashLot]
}

fn offer(district: District, lot: usize, color: Color) -> Action {
    Action::BidContainer {
        district,
        lot,
        color,
    }
}

fn start(g: &mut Game, lot: usize) {
    g.apply(Action::StartAuction { kind: CashLot, lot })
        .unwrap();
}

fn can_start(g: &Game, lot: usize) -> bool {
    g.is_legal(Action::StartAuction { kind: CashLot, lot })
}

fn pick(g: &mut Game, (district, lot, color): Pick) {
    g.apply(offer(district, lot, color)).unwrap();
}

/// Picks the containers one by one, factory containers first as the bid requires, and puts them on the tile.
fn bid(g: &mut Game, picks: &[Pick]) {
    for &p in picks {
        pick(g, p);
    }
    g.apply(Action::DoneBidding).unwrap();
}

/// Outbids with these containers. A player who is beaten is left to lay theirs out again.
fn outbid(g: &mut Game, picks: &[Pick]) {
    g.apply(Action::Outbid(CashLot)).unwrap();
    bid(g, picks);
}

/// A harmless action that only moves containers between lots.
fn reprice(g: &mut Game) {
    g.apply(Action::Reprice(Factory)).unwrap();
    finish_arrange(g);
}

/// A new game in which the player to move has a factory limit and a harbor limit of 6, holding
///   factory: red in lot 0, blue in lot 1, yellow in lot 2
///   harbor: red in lot 0, blue in lot 3.
/// Everyone has $20 and the Bank's cash lots hold $1, $2 and $3.
fn stocked_game(players: usize) -> (Game, PlayerId) {
    let mut g = new_game(players, GameLength::Standard, 1);
    let me = g.current_player();
    set_factories(&mut g, me, &[Red, Blue, Yellow]);
    set_warehouses(&mut g, me, 6);
    set_lots(
        &mut g,
        me,
        Factory,
        &[(0, Red, 1), (1, Blue, 1), (2, Yellow, 1)],
    );
    set_lots(&mut g, me, Harbor, &[(0, Red, 1), (3, Blue, 1)]);
    (g, me)
}

/// Gives a player a factory limit and a harbor limit of 6 and fills them with 12 containers: 3 red in
/// factory lot 0, 3 blue in factory lot 1 and 6 white in harbor lot 0.
fn full_board(g: &mut Game, player: PlayerId) {
    set_factories(g, player, &[Red, Blue, Yellow]);
    set_warehouses(g, player, 6);
    set_lots(g, player, Factory, &[(0, Red, 3), (1, Blue, 3)]);
    set_lots(g, player, Harbor, &[(0, White, 6)]);
}

/// The most a bid can hold, from a full board: all 6 factory containers and 4 of the harbor's.
fn ten_picks() -> Vec<Pick> {
    let mut picks = vec![(Factory, 0, Red); 3];
    picks.extend([(Factory, 1, Blue); 3]);
    picks.extend([(Harbor, 0, White); 4]);
    picks
}

/// Gives a player exactly `n` white containers, all in a harbor that has room for them.
fn only_harbor(g: &mut Game, player: PlayerId, n: u8) {
    clear_containers(g, player);
    set_warehouses(g, player, n.max(1));
    set_lots(g, player, Harbor, &[(0, White, n)]);
}

/// Puts a container bid straight on the tile, with its reserve tokens on the holder's board.
fn put_on_tile(
    g: &mut Game,
    holder: PlayerId,
    lot: usize,
    factory: ColorCounts,
    harbor: ColorCounts,
) {
    g.state.supply.containers.sub_all(&factory);
    g.state.supply.containers.sub_all(&harbor);
    g.state.players[holder].reserve_factory = factory.total() as u8;
    g.state.players[holder].reserve_harbor = harbor.total() as u8;
    g.state.bank.auctions[CashLot] = Some(Auction {
        lot,
        holder,
        bid: Bid::Containers(ContainerBid { factory, harbor }),
    });
}

/// The first player puts these containers on cash lot `lot` and takes their other action, so that the
/// next player's turn begins.
fn held_bid(players: usize, lot: usize, picks: &[Pick]) -> (Game, PlayerId) {
    let (mut g, holder) = stocked_game(players);
    start(&mut g, lot);
    bid(&mut g, picks);
    reprice(&mut g);
    (g, holder)
}

/// Every other player passes, so the player who held the bid is up again.
fn pass_around(g: &mut Game, players: usize) {
    for _ in 1..players {
        pass_turn(g);
    }
}

/// Every cell of both districts, in the order its containers are picked: the factory district first,
/// then lot by lot and color by color.
fn picking_order() -> Vec<Pick> {
    let mut order = Vec::new();
    for district in [Factory, Harbor] {
        for lot in 0..district.lot_count() {
            for color in Color::ALL {
                order.push((district, lot, color));
            }
        }
    }
    order
}

// What a bid is made of

#[test]
fn a_container_bid_is_worth_its_containers_whatever_their_colors_or_sides() {
    let bid = ContainerBid {
        factory: counts(&[(Red, 2), (Blue, 1)]),
        harbor: counts(&[(Green, 3)]),
    };

    assert_eq!(bid.total(), 6);
    assert_eq!(bid.all(), counts(&[(Red, 2), (Blue, 1), (Green, 3)]));
    assert_eq!(Bid::Containers(bid).value(), 6);
    assert_eq!(Bid::Cash(4).value(), 4);
    assert_eq!(ContainerBid::default().total(), 0);
}

#[test]
fn only_a_cash_bid_puts_cash_on_a_tile_and_only_a_container_bid_puts_containers() {
    let mut auctions = Auctions::default();
    assert_eq!(auctions.cash_on_tiles(), 0);
    assert_eq!(auctions.containers_on_tiles(), ColorCounts::default());

    auctions[ContainerLot] = Some(Auction {
        lot: 0,
        holder: 0,
        bid: Bid::Cash(7),
    });
    auctions[CashLot] = Some(Auction {
        lot: 1,
        holder: 1,
        bid: Bid::Containers(ContainerBid {
            factory: counts(&[(Red, 2)]),
            harbor: counts(&[(Blue, 1)]),
        }),
    });

    assert_eq!(auctions.cash_on_tiles(), 7);
    assert_eq!(
        auctions.containers_on_tiles(),
        counts(&[(Red, 2), (Blue, 1)])
    );
}

#[test]
fn reserve_tokens_count_against_both_storage_limits() {
    let mut p = PlayerState::default();
    for color in [Red, Blue, Yellow] {
        p.factories[color.index()] = true;
    }
    p.warehouses = 5;
    p.factory_lots[0][Red] = 1;
    p.harbor_lots[0][Blue] = 1;
    assert_eq!((p.factory_room(), p.harbor_room()), (5, 4));

    p.reserve_factory = 1;
    p.reserve_harbor = 2;

    assert_eq!((p.factory_room(), p.harbor_room()), (4, 2));
}

// Starting an auction

#[test]
fn a_cash_lot_auction_opens_with_a_bid_of_at_least_one_container() {
    let (mut g, me) = stocked_game(3);

    start(&mut g, 0);

    assert!(matches!(g.phase, Phase::BankBid(_)));
    assert_eq!(g.current_player(), me);
    assert_eq!(
        g.legal_actions(),
        vec![
            offer(Factory, 0, Red),
            offer(Factory, 1, Blue),
            offer(Factory, 2, Yellow),
            offer(Harbor, 0, Red),
            offer(Harbor, 3, Blue),
        ],
        "one container to begin with, and nothing to finish before it"
    );
    assert_eq!(tile(&g), None, "the token goes down with the bid");
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn only_a_cash_lot_that_holds_cash_can_be_auctioned() {
    let (mut g, _) = stocked_game(3);
    set_cash_lot(&mut g, 1, 0);

    assert!(can_start(&g, 0));
    assert!(!can_start(&g, 1), "lot II is empty");
    assert!(can_start(&g, 2));
    assert!(!can_start(&g, BANK_LOTS), "there is no fourth lot");
}

#[test]
fn only_containers_of_the_factory_and_harbor_districts_can_be_bid() {
    let (mut g, me) = stocked_game(3);
    clear_containers(&mut g, me);
    set_ship(&mut g, me, counts(&[(Red, 1)]));
    set_holding(&mut g, me, counts(&[(Blue, 1)]));
    set_scoring_area(&mut g, me, counts(&[(Green, 1)]));

    for lot in 0..BANK_LOTS {
        assert!(!can_start(&g, lot), "lot {lot}");
    }

    set_lots(&mut g, me, Harbor, &[(4, White, 1)]);

    assert!(can_start(&g, 0));
}

#[test]
fn each_kind_of_auction_is_bid_on_with_its_own_kind_of_action() {
    let (mut g, _) = stocked_game(3);
    assert!(!g.is_legal(Action::DoneBidding), "no bid is being placed");
    assert!(
        !g.is_legal(offer(Factory, 0, Red)),
        "no bid is being placed"
    );

    start(&mut g, 0);
    assert!(
        !g.is_legal(Action::BidCash(1)),
        "containers are bid for cash"
    );

    let (mut g, _) = stocked_game(3);
    g.apply(Action::StartAuction {
        kind: ContainerLot,
        lot: 0,
    })
    .unwrap();
    assert!(
        !g.is_legal(offer(Factory, 0, Red)),
        "cash is bid for containers"
    );
    assert!(!g.is_legal(Action::DoneBidding));
    assert!(g.is_legal(Action::BidCash(1)));
}

#[test]
fn no_loan_can_be_taken_while_bidding_containers() {
    let (mut g, _) = stocked_game(3);
    assert!(g.is_legal(Action::RequestLoan));

    start(&mut g, 0);

    assert!(!g.is_legal(Action::RequestLoan));
}

// Picking the containers

#[test]
fn a_container_of_a_lot_that_does_not_exist_cannot_be_picked() {
    let (mut g, _) = stocked_game(3);
    start(&mut g, 0);

    for district in [Factory, Harbor] {
        assert!(
            !g.is_legal(offer(district, district.lot_count(), Red)),
            "{district:?}"
        );
    }
}

#[test]
fn a_picked_container_waits_in_hand_until_the_bid_is_done() {
    let (mut g, me) = stocked_game(3);
    start(&mut g, 0);
    let totals = g.container_totals();

    pick(&mut g, (Factory, 1, Blue));

    assert!(matches!(g.phase, Phase::BankBid(_)));
    assert_eq!(g.state.players[me].factory_lots[1], ColorCounts::default());
    assert_eq!(g.container_totals(), totals, "it is in hand, not lost");
    assert!(g.legal_actions().contains(&Action::DoneBidding));
    assert_eq!(tile(&g), None);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn containers_are_picked_in_one_fixed_order_so_a_bid_has_one_action_sequence() {
    let (mut g, _) = stocked_game(3);
    start(&mut g, 0);

    pick(&mut g, (Factory, 1, Blue));
    assert_eq!(
        g.legal_actions(),
        vec![
            offer(Factory, 2, Yellow),
            offer(Harbor, 0, Red),
            offer(Harbor, 3, Blue),
            Action::DoneBidding,
        ],
        "not the red container of lot 0, which comes before the blue one"
    );

    pick(&mut g, (Harbor, 0, Red));
    assert_eq!(
        g.legal_actions(),
        vec![offer(Harbor, 3, Blue), Action::DoneBidding],
        "no factory container after a harbor one"
    );
}

#[test]
fn after_a_container_only_the_ones_that_come_later_in_the_order_can_be_picked() {
    let order = picking_order();
    assert_eq!(order.len(), 45);
    for (i, &first) in order.iter().enumerate() {
        let mut g = new_game(5, GameLength::Extended, 1);
        let me = g.current_player();
        for district in [Factory, Harbor] {
            let cells: Vec<_> = order
                .iter()
                .filter(|cell| cell.0 == district)
                .map(|&(_, lot, color)| (lot, color, 1))
                .collect();
            set_lots(&mut g, me, district, &cells);
        }
        start(&mut g, 0);

        pick(&mut g, first);

        let mut later: Vec<Action> = order[i + 1..]
            .iter()
            .map(|&(district, lot, color)| offer(district, lot, color))
            .collect();
        later.push(Action::DoneBidding);
        assert_eq!(g.legal_actions(), later, "after picking {first:?}");
    }
}

#[test]
fn several_containers_of_one_lot_and_color_are_picked_one_after_another() {
    let (mut g, me) = stocked_game(3);
    set_lots(&mut g, me, Harbor, &[(2, White, 2)]);
    start(&mut g, 0);

    pick(&mut g, (Harbor, 2, White));
    assert_eq!(
        g.legal_actions(),
        vec![offer(Harbor, 2, White), Action::DoneBidding]
    );

    pick(&mut g, (Harbor, 2, White));
    assert_eq!(g.legal_actions(), vec![Action::DoneBidding]);
}

#[test]
fn a_bid_that_must_beat_another_only_offers_picks_that_can_still_get_there() {
    let (mut g, _) = held_bid(3, 0, &[(Factory, 0, Red), (Factory, 1, Blue)]);
    let second = g.current_player();
    clear_containers(&mut g, second);
    set_warehouses(&mut g, second, 2);
    set_lots(&mut g, second, Factory, &[(0, Yellow, 1)]);
    set_lots(&mut g, second, Harbor, &[(0, White, 1), (4, Green, 1)]);
    g.apply(Action::Outbid(CashLot)).unwrap();

    // Three containers are needed and there are exactly three: every pick is forced.
    assert_eq!(g.legal_actions(), vec![offer(Factory, 0, Yellow)]);
    pick(&mut g, (Factory, 0, Yellow));
    assert_eq!(g.legal_actions(), vec![offer(Harbor, 0, White)]);
    pick(&mut g, (Harbor, 0, White));
    assert_eq!(g.legal_actions(), vec![offer(Harbor, 4, Green)]);
    pick(&mut g, (Harbor, 4, Green));
    assert_eq!(g.legal_actions(), vec![Action::DoneBidding]);
}

#[test]
fn a_bid_is_at_most_ten_containers() {
    assert_eq!(MAX_CONTAINER_BID, 10, "there are 10 reserve tokens");
    let (mut g, me) = stocked_game(5);
    full_board(&mut g, me);
    start(&mut g, 0);

    let picks = ten_picks();
    for &p in &picks[..9] {
        pick(&mut g, p);
    }
    assert_eq!(
        g.legal_actions(),
        vec![offer(Harbor, 0, White), Action::DoneBidding]
    );

    pick(&mut g, picks[9]);

    assert_eq!(
        g.legal_actions(),
        vec![Action::DoneBidding],
        "two white containers are left, but there is no reserve token for them"
    );
}

#[test]
fn nothing_beats_a_bid_of_ten_containers_but_nine_can_be_beaten() {
    for (size, beatable) in [(9, true), (10, false)] {
        let (mut g, me) = stocked_game(5);
        full_board(&mut g, me);
        start(&mut g, 0);
        bid(&mut g, &ten_picks()[..size]);
        reprice(&mut g);
        let second = g.current_player();
        full_board(&mut g, second);

        assert_eq!(
            g.is_legal(Action::Outbid(CashLot)),
            beatable,
            "a bid of {size}, against 12 containers"
        );
    }
}

// Putting the bid down

#[test]
fn the_opening_bid_puts_containers_on_the_tile_and_reserve_tokens_on_the_board() {
    let (mut g, me) = stocked_game(3);
    let totals = g.container_totals();
    let cash_before = cash(&g, me);

    start(&mut g, 2);
    bid(&mut g, &[(Factory, 1, Blue), (Harbor, 0, Red)]);

    assert_eq!(
        tile(&g),
        Some(Auction {
            lot: 2,
            holder: me,
            bid: Bid::Containers(ContainerBid {
                factory: counts(&[(Blue, 1)]),
                harbor: counts(&[(Red, 1)]),
            }),
        })
    );
    let p = &g.state.players[me];
    assert_eq!((p.reserve_factory, p.reserve_harbor), (1, 1));
    assert_eq!((p.factory_stock(), p.harbor_stock()), (2, 1));
    assert_eq!(p.cash, cash_before, "no cash is bid");
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.turn.actions_left, 1, "calling the Bank is one action");
    assert!(g.turn.bank_called);
    assert_eq!(
        g.container_totals(),
        totals,
        "the tile's containers still count"
    );
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_reserve_token_takes_the_place_of_the_container_in_the_factory_for_the_rest_of_the_turn() {
    // The rulebook's example: a factory limit of 6, 4 containers and a reserve token leave room for 1.
    let (mut g, me) = stocked_game(3);
    set_lots(
        &mut g,
        me,
        Factory,
        &[(0, Red, 2), (1, Blue, 2), (2, Yellow, 1)],
    );
    assert_eq!(g.state.players[me].factory_room(), 1);

    start(&mut g, 0);
    bid(&mut g, &[(Factory, 0, Red)]);

    let p = &g.state.players[me];
    assert_eq!((p.factory_stock(), p.reserve_factory), (4, 1));
    assert_eq!(p.factory_room(), 1);

    g.apply(Action::Produce).unwrap();
    assert_eq!(
        g.legal_actions(),
        vec![
            Action::ProduceColor(Red),
            Action::ProduceColor(Blue),
            Action::ProduceColor(Yellow)
        ],
        "three factories, room for one"
    );
    g.apply(Action::ProduceColor(Blue)).unwrap();
    finish_arrange(&mut g);

    assert_eq!(g.state.players[me].factory_stock(), 5);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_reserve_token_in_the_harbor_leaves_less_room_for_a_factory_purchase() {
    let (mut g, me) = stocked_game(3);
    set_warehouses(&mut g, me, 3);
    set_lots(&mut g, me, Harbor, &[(0, Red, 2)]);
    let seller = (me + 1) % 3;
    set_lots(&mut g, seller, Factory, &[(0, White, 2)]);
    start(&mut g, 0);
    bid(&mut g, &[(Harbor, 0, Red)]);
    assert_eq!(
        g.state.players[me].harbor_room(),
        1,
        "1 container, 1 reserve token and a limit of 3"
    );

    g.apply(Action::FactoryPurchase(seller)).unwrap();
    g.apply(Action::Buy {
        lot: 0,
        color: White,
    })
    .unwrap();

    assert!(
        matches!(g.phase, Phase::Arrange(_)),
        "the harbor has no room for a second container"
    );
    finish_arrange(&mut g);
    assert_eq!(g.state.players[me].harbor_stock(), 2);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_containers_on_the_tile_are_not_laid_out_again_by_repricing() {
    let (mut g, _) = stocked_game(3);
    start(&mut g, 0);
    bid(&mut g, &[(Factory, 1, Blue)]);

    g.apply(Action::Reprice(Factory)).unwrap();

    let Phase::Arrange(a) = g.phase else {
        panic!("the factory district is laid out again");
    };
    assert_eq!(a.hand, counts(&[(Red, 1), (Yellow, 1)]));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_containers_on_the_tile_cannot_be_bought() {
    for on_tile in [false, true] {
        let (mut g, me) = stocked_game(3);
        set_lots(&mut g, me, Factory, &[(0, Red, 1)]);
        if on_tile {
            start(&mut g, 0);
            bid(&mut g, &[(Factory, 0, Red)]);
        } else {
            reprice(&mut g);
        }
        reprice(&mut g);

        assert_eq!(
            g.is_legal(Action::FactoryPurchase(me)),
            !on_tile,
            "on the tile: {on_tile}"
        );
    }
}

#[test]
fn opening_a_cash_lot_auction_is_the_one_call_of_the_turn() {
    let (mut g, _) = stocked_game(5);
    start(&mut g, 0);
    bid(&mut g, &[(Factory, 0, Red)]);

    assert!(g.turn.bank_called);
    for lot in 0..BANK_LOTS {
        assert!(
            !g.is_legal(Action::StartAuction {
                kind: ContainerLot,
                lot
            }),
            "lot {lot}"
        );
    }
}

// Outbidding

#[test]
fn an_outbid_needs_one_container_more_than_the_holder_has_on_the_tile() {
    let (mut g, _) = held_bid(3, 0, &[(Factory, 0, Red), (Factory, 1, Blue)]);
    let second = g.current_player();

    only_harbor(&mut g, second, 2);
    assert!(!g.is_legal(Action::Outbid(CashLot)), "two do not beat two");

    only_harbor(&mut g, second, 3);
    assert!(g.is_legal(Action::Outbid(CashLot)));

    g.apply(Action::Outbid(CashLot)).unwrap();
    assert!(matches!(g.phase, Phase::BankBid(_)));
    pick(&mut g, (Harbor, 0, White));
    pick(&mut g, (Harbor, 0, White));
    assert!(
        !g.legal_actions().contains(&Action::DoneBidding),
        "two containers are still not enough"
    );
    pick(&mut g, (Harbor, 0, White));
    assert!(g.legal_actions().contains(&Action::DoneBidding));
}

#[test]
fn nobody_can_outbid_their_own_bid() {
    let (mut g, me) = stocked_game(3);
    put_on_tile(&mut g, me, 0, counts(&[(Red, 1)]), ColorCounts::default());
    assert!(
        g.state.players[me].bid_stock() > 1,
        "they have enough to beat it"
    );

    assert!(!g.is_legal(Action::Outbid(CashLot)));
}

#[test]
fn an_outbid_puts_the_new_bid_on_the_tile_with_the_new_reserve_tokens() {
    let (mut g, holder) = held_bid(3, 1, &[(Factory, 0, Red)]);
    let challenger = g.current_player();
    only_harbor(&mut g, challenger, 3);

    outbid(&mut g, &[(Harbor, 0, White), (Harbor, 0, White)]);
    finish_arrange(&mut g);

    assert_eq!(
        tile(&g),
        Some(Auction {
            lot: 1,
            holder: challenger,
            bid: Bid::Containers(ContainerBid {
                factory: ColorCounts::default(),
                harbor: counts(&[(White, 2)]),
            }),
        }),
        "it is the same lot"
    );
    let (old, new) = (&g.state.players[holder], &g.state.players[challenger]);
    assert_eq!((old.reserve_factory, old.reserve_harbor), (0, 0));
    assert_eq!((new.reserve_factory, new.reserve_harbor), (0, 2));
    assert_eq!(new.harbor_stock(), 1);
    assert!(g.turn.bank_called, "an outbid is the call of the turn too");
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_beaten_bidder_gets_their_containers_back_and_prices_them_again() {
    let (mut g, holder) = held_bid(3, 0, &[(Factory, 1, Blue), (Harbor, 0, Red)]);
    let challenger = g.current_player();
    only_harbor(&mut g, challenger, 3);
    let before = g.state.players[holder];

    outbid(&mut g, &[(Harbor, 0, White); 3]);

    let Phase::Arrange(a) = g.phase else {
        panic!("the beaten bidder lays their containers out");
    };
    assert_eq!((a.district, a.hand), (Factory, counts(&[(Blue, 1)])));
    assert_eq!(
        g.current_player(),
        holder,
        "they decide, not the player who beat them"
    );
    assert_eq!(
        g.legal_actions(),
        (0..4).map(Action::Place).collect::<Vec<_>>()
    );
    assert_eq!(g.check_invariants(), Ok(()));
    g.apply(Action::Place(3)).unwrap();

    let Phase::Arrange(a) = g.phase else {
        panic!("then the harbor containers");
    };
    assert_eq!((a.district, a.hand), (Harbor, counts(&[(Red, 1)])));
    assert_eq!(g.current_player(), holder);
    assert_eq!(
        g.legal_actions(),
        (0..5).map(Action::Place).collect::<Vec<_>>()
    );
    assert_eq!(g.check_invariants(), Ok(()));
    g.apply(Action::Place(4)).unwrap();

    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.current_player(), challenger);
    assert_eq!(g.turn.actions_left, 1, "the outbid was one action");
    let p = &g.state.players[holder];
    assert_eq!((p.reserve_factory, p.reserve_harbor), (0, 0));
    assert_eq!(
        p.factory_lots[3],
        counts(&[(Blue, 1)]),
        "not in lot 1, where it was"
    );
    assert_eq!(p.harbor_lots[4], counts(&[(Red, 1)]));
    assert_eq!(
        p.factory_lots[0], before.factory_lots[0],
        "the rest is untouched"
    );
    assert_eq!(p.harbor_lots[3], before.harbor_lots[3]);
    assert_eq!(tile(&g).map(|a| a.holder), Some(challenger));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn only_the_district_a_beaten_bid_came_from_is_laid_out_again() {
    for (picks, district) in [
        (vec![(Factory, 1, Blue)], Factory),
        (vec![(Harbor, 0, Red)], Harbor),
    ] {
        let (mut g, holder) = held_bid(3, 0, &picks);
        let challenger = g.current_player();
        only_harbor(&mut g, challenger, 2);

        outbid(&mut g, &[(Harbor, 0, White); 2]);

        let Phase::Arrange(a) = g.phase else {
            panic!("the beaten bidder lays their container out");
        };
        assert_eq!(a.district, district);
        assert_eq!(a.hand.total(), 1);
        assert_eq!(g.current_player(), holder);
        g.apply(Action::Place(2)).unwrap();

        assert_eq!(g.phase, Phase::Main, "{district:?}: that was all");
        assert_eq!(g.current_player(), challenger);
        assert_eq!(g.check_invariants(), Ok(()));
    }
}

#[test]
fn an_outbid_as_the_second_action_ends_the_turn_once_the_beaten_bidder_has_laid_out() {
    let (mut g, holder) = held_bid(3, 0, &[(Factory, 0, Red)]);
    let challenger = g.current_player();
    let third = (challenger + 1) % 3;
    only_harbor(&mut g, challenger, 2);
    reprice(&mut g);
    assert_eq!(g.turn.actions_left, 1);

    outbid(&mut g, &[(Harbor, 0, White); 2]);

    assert!(matches!(g.phase, Phase::Arrange(_)));
    assert_eq!(g.current_player(), holder);
    assert_eq!(g.state.active_player, challenger, "it is still their turn");
    g.apply(Action::Place(0)).unwrap();

    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.current_player(), third);
    assert_eq!(g.turn.actions_left, 2, "a new turn has begun");
    assert_eq!(g.check_invariants(), Ok(()));
}

// Winning

#[test]
fn the_holder_wins_the_cash_lot_at_the_start_of_their_next_turn() {
    let (mut g, holder) = stocked_game(3);
    set_cash_lot(&mut g, 1, 6);
    start(&mut g, 1);
    bid(
        &mut g,
        &[(Factory, 0, Red), (Factory, 1, Blue), (Harbor, 0, Red)],
    );
    reprice(&mut g);
    pass_turn(&mut g);
    assert_eq!(tile(&g).map(|a| a.holder), Some(holder), "not yet");
    pass_turn(&mut g);

    assert_eq!(g.current_player(), holder);
    let Phase::Distribute(dealing) = g.phase else {
        panic!("the bid is paid to the Bank first");
    };
    assert_eq!(dealing.hand, counts(&[(Red, 2), (Blue, 1)]));
    assert_eq!(dealing.dealt, 0);
    assert_eq!(
        g.legal_actions(),
        vec![Action::Deal(Red), Action::Deal(Blue)],
        "the winner decides which color goes in which lot"
    );
    assert_eq!(g.check_invariants(), Ok(()));

    let lots = g.state.bank.container_lots;
    let cash_before = cash(&g, holder);
    g.apply(Action::Deal(Red)).unwrap();
    g.apply(Action::Deal(Blue)).unwrap();

    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.turn.actions_left, 2);
    assert!(g.turn.won_auction);
    assert_eq!(g.state.bank.container_lots[0], plus(lots[0], Red));
    assert_eq!(g.state.bank.container_lots[1], plus(lots[1], Blue));
    assert_eq!(
        g.state.bank.container_lots[2],
        plus(lots[2], Red),
        "the last red container needed no decision"
    );
    assert_eq!(cash(&g, holder), cash_before + 6, "the whole lot is theirs");
    assert_eq!(g.state.bank.cash_lots, [1, 0, 3]);
    assert_eq!(tile(&g), None, "the token and tile are back in the supply");
    let p = &g.state.players[holder];
    assert_eq!((p.reserve_factory, p.reserve_harbor), (0, 0));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_containers_won_with_are_dealt_one_lot_at_a_time_and_wrap_around() {
    let (mut g, holder) = stocked_game(3);
    start(&mut g, 0);
    bid(
        &mut g,
        &[
            (Factory, 0, Red),
            (Factory, 1, Blue),
            (Harbor, 0, Red),
            (Harbor, 3, Blue),
        ],
    );
    reprice(&mut g);
    pass_around(&mut g, 3);
    assert_eq!(g.current_player(), holder);
    let lots = g.state.bank.container_lots;

    g.apply(Action::Deal(Red)).unwrap();
    g.apply(Action::Deal(Blue)).unwrap();
    assert!(matches!(g.phase, Phase::Distribute(_)));
    g.apply(Action::Deal(Blue)).unwrap();

    assert_eq!(g.phase, Phase::Main);
    assert_eq!(
        g.state.bank.container_lots[0],
        plus(plus(lots[0], Red), Red),
        "I, then I again"
    );
    assert_eq!(g.state.bank.container_lots[1], plus(lots[1], Blue));
    assert_eq!(g.state.bank.container_lots[2], plus(lots[2], Blue));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_bid_of_one_color_is_paid_without_any_decision() {
    let (mut g, holder) = held_bid(3, 0, &[(Factory, 0, Red), (Harbor, 0, Red)]);
    let lots = g.state.bank.container_lots;
    pass_around(&mut g, 3);

    assert_eq!(g.current_player(), holder);
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.state.bank.container_lots[0], plus(lots[0], Red));
    assert_eq!(g.state.bank.container_lots[1], plus(lots[1], Red));
    assert_eq!(g.state.bank.container_lots[2], lots[2]);
    assert_eq!(cash(&g, holder), 21, "lot I's $1");
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_winner_cannot_call_the_bank_that_turn_but_the_token_is_back_for_the_next() {
    let (mut g, holder) = held_bid(3, 0, &[(Factory, 0, Red)]);
    pass_around(&mut g, 3);
    assert_eq!(g.current_player(), holder);
    assert!(g.turn.won_auction);
    assert_eq!(tile(&g), None);

    for lot in 0..BANK_LOTS {
        assert!(!can_start(&g, lot), "lot {lot}: an auction was just won");
    }

    pass_turn(&mut g);

    assert!(!g.turn.won_auction);
    assert!(can_start(&g, 1), "lot II still holds $2");
}

#[test]
fn a_cash_lot_is_won_after_the_interest_and_what_it_holds_cannot_pay_it() {
    let (mut g, holder) = stocked_game(3);
    set_cash_lot(&mut g, 0, 5);
    start(&mut g, 0);
    bid(&mut g, &[(Factory, 0, Red)]);
    set_cash(&mut g, holder, 0);
    set_loans(&mut g, holder, 1);
    reprice(&mut g);
    pass_around(&mut g, 3);

    assert_eq!(g.current_player(), holder);
    assert_eq!(
        g.state.players[holder].loans, 2,
        "a second loan paid the interest"
    );
    assert_eq!(
        cash(&g, holder),
        9 + 5,
        "$10 less $1 interest, then the lot"
    );
    assert_eq!(
        g.state.bank.cash_lots,
        [0, 3, 3],
        "the interest went to lot II, around the token on lot I"
    );
    assert_eq!(tile(&g), None);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_cash_lot_is_still_won_after_a_default() {
    let (mut g, holder) = stocked_game(3);
    start(&mut g, 0);
    bid(&mut g, &[(Factory, 0, Red), (Factory, 1, Blue)]);
    set_cash(&mut g, holder, 0);
    set_loans(&mut g, holder, 2);
    set_scoring_area(&mut g, holder, counts(&[(Green, 2)]));
    reprice(&mut g);
    let lots = g.state.bank.container_lots;
    pass_around(&mut g, 3);

    assert_eq!(g.current_player(), holder);
    let Phase::Distribute(dealing) = g.phase else {
        panic!("the bid is paid after the default");
    };
    assert_eq!(
        dealing.decider, holder,
        "the winner decides, not the player on their right"
    );
    assert_eq!(dealing.hand, counts(&[(Red, 1), (Blue, 1)]));
    assert_eq!(dealing.dealt, 0, "a new payment starts at lot I");
    assert_eq!(
        g.state.bank.container_lots[0],
        plus(lots[0], Green),
        "the seized containers are in the Bank already"
    );
    assert_eq!(g.check_invariants(), Ok(()));

    g.apply(Action::Deal(Blue)).unwrap();

    assert_eq!(g.phase, Phase::Main);
    assert_eq!(
        g.state.bank.container_lots[0],
        plus(plus(lots[0], Green), Blue)
    );
    assert_eq!(
        g.state.bank.container_lots[1],
        plus(plus(lots[1], Green), Red)
    );
    assert_eq!(cash(&g, holder), 1, "the lot's $1");
    assert_eq!(tile(&g), None);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_containers_on_the_tile_cannot_be_seized_so_a_default_with_nothing_else_is_forgiven() {
    let (mut g, holder) = stocked_game(3);
    clear_containers(&mut g, holder);
    set_lots(&mut g, holder, Harbor, &[(0, White, 2)]);
    start(&mut g, 0);
    bid(&mut g, &[(Harbor, 0, White), (Harbor, 0, White)]);
    set_cash(&mut g, holder, 0);
    set_loans(&mut g, holder, 2);
    reprice(&mut g);
    let lots = g.state.bank.container_lots;
    pass_around(&mut g, 3);

    assert_eq!(g.current_player(), holder);
    assert_eq!(
        g.phase,
        Phase::Main,
        "nothing to seize, so nothing to decide"
    );
    assert_eq!(g.state.players[holder].loans, 2);
    assert_eq!(g.state.bank.container_lots[0], plus(lots[0], White));
    assert_eq!(g.state.bank.container_lots[1], plus(lots[1], White));
    assert_eq!(g.state.bank.container_lots[2], lots[2]);
    assert_eq!(tile(&g), None);
    assert_eq!(g.check_invariants(), Ok(()));
}

// Paying the Bank around a token

#[test]
fn interest_is_paid_around_the_token_of_a_cash_lot_auction() {
    let (mut g, holder) = stocked_game(3);
    let debtor = (holder + 1) % 3;
    start(&mut g, 1);
    bid(&mut g, &[(Factory, 0, Red)]);
    set_loans(&mut g, debtor, 2);
    reprice(&mut g);

    assert_eq!(g.current_player(), debtor);
    assert_eq!(cash(&g, debtor), 18);
    assert_eq!(
        g.state.bank.cash_lots,
        [2, 2, 4],
        "$1 each to lots I and III, none to lot II where the token is"
    );
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_container_lot_is_paid_for_around_the_token_of_a_cash_lot_auction() {
    let (mut g, first) = stocked_game(5);
    g.apply(Action::StartAuction {
        kind: ContainerLot,
        lot: 0,
    })
    .unwrap();
    g.apply(Action::BidCash(5)).unwrap();
    reprice(&mut g);
    let second = g.current_player();
    only_harbor(&mut g, second, 1);
    start(&mut g, 1);
    bid(&mut g, &[(Harbor, 0, White)]);
    reprice(&mut g);
    // The three players after the second one pass, so the first is up again.
    for _ in 0..3 {
        pass_turn(&mut g);
    }

    assert_eq!(g.current_player(), first);
    assert_eq!(
        g.state.bank.cash_lots,
        [4, 2, 5],
        "$5 goes 3 and 2 to lots I and III, around the token on lot II"
    );
    assert_eq!(tile(&g).map(|a| a.holder), Some(second));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_cash_lot_is_paid_for_around_the_token_of_a_container_lot_auction() {
    let (mut g, first) = stocked_game(5);
    start(&mut g, 0);
    bid(
        &mut g,
        &[(Factory, 0, Red), (Factory, 1, Blue), (Harbor, 0, Red)],
    );
    reprice(&mut g);
    g.apply(Action::StartAuction {
        kind: ContainerLot,
        lot: 1,
    })
    .unwrap();
    g.apply(Action::BidCash(1)).unwrap();
    reprice(&mut g);
    for _ in 0..3 {
        pass_turn(&mut g);
    }

    assert_eq!(g.current_player(), first);
    let lots = g.state.bank.container_lots;
    g.apply(Action::Deal(Red)).unwrap();
    g.apply(Action::Deal(Blue)).unwrap();

    assert_eq!(
        g.state.bank.container_lots[0],
        plus(plus(lots[0], Red), Red)
    );
    assert_eq!(
        g.state.bank.container_lots[1], lots[1],
        "the token keeps lot II from receiving"
    );
    assert_eq!(g.state.bank.container_lots[2], plus(lots[2], Blue));
    assert_eq!(g.check_invariants(), Ok(()));
}

// How many auctions the Bank holds

#[test]
fn three_or_four_players_can_only_have_one_auction_at_a_time() {
    for players in [3, 4] {
        // A cash lot auction keeps the Bank closed to a container lot auction...
        let (mut g, _) = held_bid(players, 0, &[(Factory, 0, Red)]);
        let second = g.current_player();
        only_harbor(&mut g, second, 2);
        for kind in AuctionKind::ALL {
            for lot in 0..BANK_LOTS {
                assert!(
                    !g.is_legal(Action::StartAuction { kind, lot }),
                    "{players} players, {kind:?} {lot}"
                );
            }
        }
        assert!(g.is_legal(Action::Outbid(CashLot)));

        // ...and the other way round.
        let (mut g, _) = stocked_game(players);
        g.apply(Action::StartAuction {
            kind: ContainerLot,
            lot: 0,
        })
        .unwrap();
        g.apply(Action::BidCash(2)).unwrap();
        reprice(&mut g);
        for lot in 0..BANK_LOTS {
            assert!(!can_start(&g, lot), "{players} players, lot {lot}");
        }
    }
}

#[test]
fn five_players_can_have_one_auction_of_each_kind_at_once() {
    let (mut g, _) = held_bid(5, 0, &[(Factory, 0, Red)]);

    assert!(
        g.is_legal(Action::StartAuction {
            kind: ContainerLot,
            lot: 0
        }),
        "the next player may start the other kind"
    );
    assert!(!can_start(&g, 1), "but the cash lot token is on the board");
    g.apply(Action::StartAuction {
        kind: ContainerLot,
        lot: 0,
    })
    .unwrap();
    g.apply(Action::BidCash(3)).unwrap();
    reprice(&mut g);

    let third = g.current_player();
    only_harbor(&mut g, third, 3);
    for kind in AuctionKind::ALL {
        for lot in 0..BANK_LOTS {
            assert!(
                !g.is_legal(Action::StartAuction { kind, lot }),
                "both tokens are down: {kind:?} {lot}"
            );
        }
    }
    assert!(g.is_legal(Action::Outbid(ContainerLot)));
    assert!(g.is_legal(Action::Outbid(CashLot)));
    assert_eq!(g.check_invariants(), Ok(()));
}

// The invariants

#[test]
fn the_invariants_flag_reserve_tokens_that_belong_to_no_bid() {
    let (mut g, me) = stocked_game(3);
    assert_eq!(g.check_invariants(), Ok(()));

    g.state.players[me].reserve_harbor = 1;

    assert!(g.check_invariants().is_err());
}

#[test]
fn the_invariants_flag_reserve_tokens_that_do_not_match_the_bid_on_the_tile() {
    for (factory, harbor) in [(0, 1), (1, 0), (2, 1), (1, 2)] {
        let (mut g, me) = stocked_game(3);
        start(&mut g, 0);
        bid(&mut g, &[(Factory, 0, Red), (Harbor, 0, Red)]);
        assert_eq!(g.check_invariants(), Ok(()));

        g.state.players[me].reserve_factory = factory;
        g.state.players[me].reserve_harbor = harbor;

        assert!(
            g.check_invariants().is_err(),
            "{factory} and {harbor} reserve tokens for a bid of 1 and 1"
        );
    }

    let (mut g, me) = stocked_game(3);
    start(&mut g, 0);
    bid(&mut g, &[(Factory, 0, Red)]);
    g.state.players[(me + 1) % 3].reserve_factory = 1;
    assert!(
        g.check_invariants().is_err(),
        "a token of someone who has no bid"
    );
}

#[test]
fn the_invariants_count_reserve_tokens_against_the_storage_limits() {
    for (district, limit) in [
        (Factory, "factory district capacity"),
        (Harbor, "harbor capacity"),
    ] {
        let (mut g, me) = stocked_game(3);
        set_lots(
            &mut g,
            me,
            district,
            &[(0, Red, 2), (1, Blue, 2), (2, Yellow, 1)],
        );
        start(&mut g, 0);
        bid(&mut g, &[(district, 0, Red)]);
        assert_eq!(
            g.check_invariants(),
            Ok(()),
            "{district:?}: 4 containers and a token in a limit of 6"
        );

        g.state.supply.containers[White] -= 2;
        g.state.players[me].lots_mut(district)[3][White] += 2;

        let error = g.check_invariants().unwrap_err();
        assert!(error.contains(limit), "{district:?}: {error}");
    }
}

#[test]
fn the_invariants_flag_a_bid_of_more_than_ten_containers() {
    for (harbor, fine) in [(4, true), (5, false)] {
        let (mut g, me) = stocked_game(3);
        clear_containers(&mut g, me);
        put_on_tile(
            &mut g,
            me,
            0,
            counts(&[(Red, 6)]),
            counts(&[(White, harbor)]),
        );

        assert_eq!(
            g.check_invariants().is_ok(),
            fine,
            "{} containers on the tile",
            6 + harbor
        );
    }
}

#[test]
fn the_invariants_flag_a_bid_of_the_wrong_kind_on_a_tile() {
    let (g, me) = stocked_game(3);
    assert_eq!(g.check_invariants(), Ok(()));

    for (kind, wrong) in [
        (CashLot, Bid::Cash(0)),
        (ContainerLot, Bid::Containers(ContainerBid::default())),
    ] {
        let mut g = g.clone();
        g.state.bank.auctions[kind] = Some(Auction {
            lot: 0,
            holder: me,
            bid: wrong,
        });

        assert!(
            g.check_invariants().is_err(),
            "{wrong:?} on the {kind:?} tile"
        );
    }
}
