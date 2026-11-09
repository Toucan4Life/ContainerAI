use super::{
    actions_while_playing, finish_arrange, new_game, pass_turn, set_cash, set_container_lot,
    set_loans, set_scoring_area,
};
use crate::action::{Action, District};
use crate::auction::max_active_auctions;
use crate::color::{Color, ColorCounts};
use crate::config::GameLength;
use crate::game::Game;
use crate::phase::Phase;
use crate::rng::Rng;
use crate::state::{Auction, AuctionKind, BANK_LOTS, Bid, ContainerBid, PlayerId};
use AuctionKind::ContainerLot;
use Color::{Blue, Green, Red};

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

/// The active auction of a container lot, which is bid on with cash.
fn tile(g: &Game) -> Option<Auction> {
    g.state.bank.auctions[ContainerLot]
}

fn start(g: &mut Game, lot: usize) {
    g.apply(Action::StartAuction {
        kind: ContainerLot,
        lot,
    })
    .unwrap();
}

fn bid(g: &mut Game, amount: u32) {
    g.apply(Action::BidCash(amount)).unwrap();
}

fn outbid(g: &mut Game, amount: u32) {
    g.apply(Action::Outbid(ContainerLot)).unwrap();
    bid(g, amount);
}

/// A harmless action that only moves containers between lots.
fn reprice(g: &mut Game) {
    g.apply(Action::Reprice(District::Factory)).unwrap();
    finish_arrange(g);
}

fn can_start(g: &Game, lot: usize) -> bool {
    g.is_legal(Action::StartAuction {
        kind: ContainerLot,
        lot,
    })
}

/// A new game in which every player has `held` cash. Bank lots I and II hold containers, lot III none.
fn game_with_cash(players: usize, held: u32) -> (Game, PlayerId) {
    let mut g = new_game(players, GameLength::Standard, 1);
    for p in 0..players {
        set_cash(&mut g, p, held);
    }
    let me = g.current_player();
    (g, me)
}

/// The first player puts `opening` on container lot `lot` and passes, so the next player's turn begins.
/// Everyone has $20 to begin with.
fn held_auction(players: usize, lot: usize, opening: u32) -> (Game, PlayerId) {
    let (mut g, holder) = game_with_cash(players, 20);
    start(&mut g, lot);
    bid(&mut g, opening);
    reprice(&mut g);
    (g, holder)
}

// Starting an auction

#[test]
fn a_container_lot_auction_opens_with_a_bid_of_at_least_one_dollar() {
    let (mut g, me) = game_with_cash(3, 7);

    g.apply(Action::StartAuction {
        kind: ContainerLot,
        lot: 0,
    })
    .unwrap();

    assert!(matches!(g.phase, Phase::BankBid(_)));
    assert_eq!(g.current_player(), me);
    let expected: Vec<Action> = (1..=7).map(Action::BidCash).collect();
    assert_eq!(g.legal_actions(), expected);
}

#[test]
fn the_opening_bid_leaves_the_hand_and_sits_on_the_tile() {
    let (mut g, me) = game_with_cash(3, 10);

    start(&mut g, 1);
    bid(&mut g, 4);

    assert_eq!(cash(&g, me), 6);
    assert_eq!(
        tile(&g),
        Some(Auction {
            lot: 1,
            holder: me,
            bid: Bid::Cash(4)
        })
    );
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn calling_the_bank_costs_one_of_the_two_actions() {
    let (mut g, me) = game_with_cash(3, 10);

    start(&mut g, 0);
    bid(&mut g, 1);

    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.current_player(), me);
    assert_eq!(g.turn.actions_left, 1);
}

#[test]
fn only_a_lot_with_containers_can_be_auctioned() {
    let (g, _) = game_with_cash(3, 10);
    let lots = g.state.bank.container_lots;
    assert!(
        lots[0].total() > 0 && lots[1].total() > 0 && lots[2].total() == 0,
        "the setup"
    );

    for (lot, held) in lots.iter().enumerate() {
        assert_eq!(can_start(&g, lot), held.total() > 0, "lot {lot}");
    }
    assert!(!can_start(&g, BANK_LOTS));
}

#[test]
fn a_player_without_cash_cannot_start_an_auction() {
    let (mut g, me) = game_with_cash(3, 10);

    set_cash(&mut g, me, 0);
    assert!(!can_start(&g, 0));

    set_cash(&mut g, me, 1);
    assert!(can_start(&g, 0));
}

#[test]
fn no_loan_is_offered_while_a_bank_bid_is_placed() {
    let (mut g, _) = game_with_cash(3, 10);
    start(&mut g, 0);

    assert!(!g.is_legal(Action::RequestLoan));
    assert!(!g.is_legal(Action::RepayLoan));
}

// Outbidding

#[test]
fn an_opponent_must_bid_at_least_a_dollar_more() {
    let (mut g, holder) = held_auction(3, 0, 5);
    let me = g.current_player();
    assert_ne!(me, holder);
    set_cash(&mut g, me, 9);

    g.apply(Action::Outbid(ContainerLot)).unwrap();

    let expected: Vec<Action> = (6..=9).map(Action::BidCash).collect();
    assert_eq!(g.legal_actions(), expected);
}

#[test]
fn outbidding_needs_the_cash_for_a_dollar_more() {
    let (mut g, _) = held_auction(3, 0, 5);
    let me = g.current_player();

    set_cash(&mut g, me, 5);
    assert!(!g.is_legal(Action::Outbid(ContainerLot)));

    set_cash(&mut g, me, 6);
    assert!(g.is_legal(Action::Outbid(ContainerLot)));
}

#[test]
fn an_outbid_player_gets_their_cash_back_and_the_new_bid_takes_the_tile() {
    let (mut g, holder) = held_auction(3, 0, 5);
    assert_eq!(cash(&g, holder), 15);
    let me = g.current_player();

    outbid(&mut g, 8);

    assert_eq!(cash(&g, holder), 20);
    assert_eq!(cash(&g, me), 12);
    assert_eq!(
        tile(&g),
        Some(Auction {
            lot: 0,
            holder: me,
            bid: Bid::Cash(8)
        })
    );
    assert_eq!(g.turn.actions_left, 1);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn there_is_nothing_to_outbid_without_an_auction() {
    let (g, _) = game_with_cash(3, 20);

    assert!(!g.is_legal(Action::Outbid(ContainerLot)));
}

#[test]
fn nobody_can_outbid_their_own_bid() {
    let (mut g, me) = game_with_cash(3, 20);
    g.state.bank.auctions[ContainerLot] = Some(Auction {
        lot: 0,
        holder: me,
        bid: Bid::Cash(3),
    });

    assert!(!g.is_legal(Action::Outbid(ContainerLot)));
}

#[test]
fn a_new_bid_restarts_the_clock() {
    let (mut g, first) = held_auction(3, 0, 5);
    let second = g.current_player();
    outbid(&mut g, 6);
    reprice(&mut g);
    pass_turn(&mut g);

    assert_eq!(g.current_player(), first);
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(tile(&g).map(|a| a.holder), Some(second), "outbid");
    assert!(g.state.players[first].holding.is_empty());

    pass_turn(&mut g);

    assert_eq!(g.current_player(), second);
    assert_eq!(
        tile(&g),
        None,
        "the new holder wins at the start of their turn"
    );
    assert!(!g.state.players[second].holding.is_empty());
}

#[test]
fn a_bid_outside_what_is_allowed_is_refused() {
    let (mut g, _) = held_auction(3, 0, 5);
    let me = g.current_player();
    set_cash(&mut g, me, 9);
    g.apply(Action::Outbid(ContainerLot)).unwrap();

    for amount in [0, 5, 10, 50] {
        assert!(!g.is_legal(Action::BidCash(amount)), "${amount}");
        assert!(g.apply(Action::BidCash(amount)).is_err(), "${amount}");
    }
    assert!(g.is_legal(Action::BidCash(6)));
    assert!(g.is_legal(Action::BidCash(9)));
}

#[test]
fn nothing_but_a_cash_bid_is_offered_while_bidding() {
    let (mut g, _) = game_with_cash(3, 10);
    start(&mut g, 0);

    for action in [
        Action::Produce,
        Action::Reprice(District::Factory),
        Action::HarborPurchase,
    ] {
        assert!(!g.is_legal(action), "{action:?}");
    }
    assert!(
        g.legal_actions()
            .iter()
            .all(|a| matches!(a, Action::BidCash(_)))
    );
}

#[test]
fn an_outbid_is_for_the_same_lot() {
    let (mut g, _) = held_auction(3, 1, 3);
    let lot = g.state.bank.container_lots[1];
    let second = g.current_player();
    outbid(&mut g, 4);

    assert_eq!(tile(&g).map(|a| a.lot), Some(1));
    reprice(&mut g);
    pass_turn(&mut g);
    pass_turn(&mut g);

    assert_eq!(g.current_player(), second);
    assert_eq!(g.state.players[second].holding, lot);
    assert!(g.state.bank.container_lots[1].is_empty());
    assert!(!g.state.bank.container_lots[0].is_empty());
}

// Calling the Bank

#[test]
fn calling_the_bank_is_noted_for_the_turn() {
    let (mut g, _) = game_with_cash(3, 20);
    assert!(!g.turn.bank_called);

    start(&mut g, 0);
    bid(&mut g, 2);

    assert!(g.turn.bank_called);
}

#[test]
fn the_bank_cannot_be_called_twice_in_one_turn() {
    let (mut g, _) = game_with_cash(3, 20);
    assert!(can_start(&g, 0));

    g.turn.bank_called = true;

    assert!(!can_start(&g, 0));
    assert!(
        !g.legal_actions()
            .iter()
            .any(|a| matches!(a, Action::StartAuction { .. }))
    );
}

#[test]
fn an_outbid_is_the_one_call_of_the_turn_too() {
    let (mut g, _) = held_auction(3, 0, 5);
    assert!(g.is_legal(Action::Outbid(ContainerLot)));

    g.turn.bank_called = true;

    assert!(!g.is_legal(Action::Outbid(ContainerLot)));
    assert!(
        !g.legal_actions()
            .iter()
            .any(|a| matches!(a, Action::Outbid(_)))
    );
}

#[test]
fn the_call_is_available_again_next_turn() {
    let (mut g, _) = held_auction(3, 0, 5);
    assert!(!g.turn.bank_called, "the next player's turn has begun");
    let second = g.current_player();
    outbid(&mut g, 6);
    reprice(&mut g);
    pass_turn(&mut g);
    pass_turn(&mut g);

    assert_eq!(g.current_player(), second);
    assert!(!g.turn.bank_called);
}

#[test]
fn three_or_four_players_can_only_have_one_auction_at_a_time() {
    for players in [3, 4] {
        let (g, _) = held_auction(players, 0, 5);

        for lot in 0..BANK_LOTS {
            assert!(!can_start(&g, lot), "{players} players, lot {lot}");
        }
        assert!(g.is_legal(Action::Outbid(ContainerLot)));
    }
}

#[test]
fn five_players_still_have_one_container_lot_auction_at_a_time() {
    let (g, _) = held_auction(5, 0, 5);

    for lot in 0..BANK_LOTS {
        assert!(!can_start(&g, lot), "lot {lot}");
    }
}

// Winning an auction

#[test]
fn the_holder_wins_the_lot_at_the_start_of_their_next_turn() {
    let (mut g, holder) = held_auction(3, 0, 5);
    let lot = g.state.bank.container_lots[0];
    assert!(lot.total() > 0);
    pass_turn(&mut g);
    assert_eq!(tile(&g).map(|a| a.holder), Some(holder), "not yet");
    pass_turn(&mut g);

    assert_eq!(g.current_player(), holder);
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.turn.actions_left, 2);
    assert_eq!(g.state.players[holder].holding, lot, "the lot is theirs");
    assert!(g.state.bank.container_lots[0].is_empty());
    assert_eq!(tile(&g), None, "the token and tile are back in the supply");
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_winning_bid_is_paid_to_the_bank_not_the_supply() {
    let (mut g, holder) = held_auction(3, 0, 5);
    let (minted, burned) = (g.state.cash_minted, g.state.cash_burned);
    pass_turn(&mut g);
    pass_turn(&mut g);

    assert_eq!(
        g.state.bank.cash_lots,
        [3, 4, 4],
        "$5 goes 2, 2, 1 on top of $1, $2, $3"
    );
    assert_eq!(cash(&g, holder), 15);
    assert_eq!((g.state.cash_minted, g.state.cash_burned), (minted, burned));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn the_other_lots_are_untouched_by_a_win() {
    let (mut g, _) = held_auction(3, 1, 3);
    let before = g.state.bank.container_lots;
    pass_turn(&mut g);
    pass_turn(&mut g);

    assert_eq!(g.state.bank.container_lots[0], before[0]);
    assert_eq!(g.state.bank.container_lots[2], before[2]);
    assert!(g.state.bank.container_lots[1].is_empty());
}

#[test]
fn the_winner_cannot_call_the_bank_in_the_turn_they_win() {
    let (mut g, holder) = held_auction(3, 0, 5);
    pass_turn(&mut g);
    pass_turn(&mut g);
    assert_eq!(g.current_player(), holder);
    assert!(g.turn.won_auction);

    assert!(cash(&g, holder) > 0 && g.state.bank.container_lots[1].total() > 0);
    assert!(!can_start(&g, 1), "an auction was just won");
}

#[test]
fn a_player_who_did_not_win_can_start_the_next_auction() {
    let (mut g, _) = held_auction(3, 0, 5);
    pass_turn(&mut g);
    pass_turn(&mut g);
    pass_turn(&mut g);

    assert!(!g.turn.won_auction);
    assert!(can_start(&g, 1), "no auction is active any more");
}

#[test]
fn the_winner_may_call_the_bank_on_their_following_turn() {
    let (mut g, holder) = held_auction(3, 0, 5);
    // The win is the holder's turn after two passes, and their following turn is three passes on.
    for _ in 0..5 {
        pass_turn(&mut g);
    }

    assert_eq!(g.current_player(), holder);
    assert!(!g.turn.won_auction);
    assert!(can_start(&g, 1));
}

#[test]
fn a_win_comes_after_the_interest_and_tile_cash_cannot_pay_it() {
    let (mut g, holder) = game_with_cash(3, 20);
    start(&mut g, 0);
    bid(&mut g, 20);
    reprice(&mut g);
    set_loans(&mut g, holder, 1);
    assert_eq!(cash(&g, holder), 0);
    pass_turn(&mut g);
    pass_turn(&mut g);

    assert_eq!(g.current_player(), holder);
    assert_eq!(
        g.state.players[holder].loans, 2,
        "a second loan paid the interest"
    );
    assert_eq!(cash(&g, holder), 9);
    assert_eq!(tile(&g), None, "and the auction was won");
    assert_eq!(
        g.state.bank.cash_lots,
        [9, 9, 9],
        "$1 interest, then $20 bid"
    );
    assert_eq!(g.check_invariants(), Ok(()));
}

/// The second player holds an auction on container lot `lot`, which holds two green containers, and is
/// in default at the start of their next turn: two loans, no cash and only a red and a blue container
/// to seize. The player to their right, who was the last to play, must deal what is seized.
fn defaulting_holder(lot: usize) -> (Game, PlayerId, PlayerId) {
    let (mut g, _) = game_with_cash(3, 20);
    pass_turn(&mut g);
    let holder = g.current_player();
    let right = (holder + 2) % 3;
    set_container_lot(&mut g, lot, counts(&[(Green, 2)]));
    start(&mut g, lot);
    bid(&mut g, 20);
    reprice(&mut g);
    set_loans(&mut g, holder, 2);
    set_scoring_area(&mut g, holder, counts(&[(Red, 1), (Blue, 1)]));
    pass_turn(&mut g);
    pass_turn(&mut g);
    (g, holder, right)
}

#[test]
fn an_auction_is_still_won_after_a_default() {
    let (mut g, holder, right) = defaulting_holder(0);

    assert!(
        matches!(g.phase, Phase::Distribute(_)),
        "the default comes first"
    );
    assert_eq!(g.current_player(), right);
    assert_eq!(tile(&g).map(|a| a.holder), Some(holder));

    g.apply(Action::Deal(Red)).unwrap();

    assert_eq!(g.phase, Phase::Main);
    assert_eq!(g.current_player(), holder);
    assert_eq!(tile(&g), None);
    assert_eq!(g.state.players[holder].holding, counts(&[(Green, 2)]));
    assert!(g.turn.won_auction);
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn seized_containers_are_not_dealt_to_the_lot_being_auctioned() {
    for lot in 0..BANK_LOTS {
        let (mut g, _, _) = defaulting_holder(lot);
        let before = g.state.bank.container_lots;
        let open: Vec<usize> = (0..BANK_LOTS).filter(|&l| l != lot).collect();

        g.apply(Action::Deal(Red)).unwrap();

        let lots = g.state.bank.container_lots;
        assert_eq!(
            lots[open[0]],
            plus(before[open[0]], Red),
            "token on lot {lot}"
        );
        assert_eq!(
            lots[open[1]],
            plus(before[open[1]], Blue),
            "token on lot {lot}"
        );
        assert!(lots[lot].is_empty(), "token on lot {lot}: it was won");
    }
}

#[test]
fn a_lot_with_a_token_is_closed_to_seized_containers_whoever_holds_it() {
    let (mut g, holder) = game_with_cash(3, 20);
    let debtor = (holder + 1) % 3;
    set_cash(&mut g, debtor, 0);
    set_loans(&mut g, debtor, 2);
    set_scoring_area(&mut g, debtor, counts(&[(Red, 1), (Blue, 1)]));
    let before = g.state.bank.container_lots;
    start(&mut g, 0);
    bid(&mut g, 5);
    reprice(&mut g);

    assert!(matches!(g.phase, Phase::Distribute(_)));
    assert_eq!(
        g.current_player(),
        holder,
        "the player to the debtor's right deals"
    );
    g.apply(Action::Deal(Red)).unwrap();

    let lots = g.state.bank.container_lots;
    assert_eq!(lots[0], before[0], "still waiting for its winner");
    assert_eq!(lots[1], plus(before[1], Red));
    assert_eq!(lots[2], plus(before[2], Blue));
    assert_eq!(tile(&g).map(|a| a.holder), Some(holder));
    assert_eq!(g.check_invariants(), Ok(()));
}

#[test]
fn a_won_lot_cannot_be_seized_because_the_interest_comes_first() {
    let (mut g, _) = game_with_cash(3, 20);
    pass_turn(&mut g);
    let holder = g.current_player();
    set_container_lot(&mut g, 0, counts(&[(Green, 2)]));
    start(&mut g, 0);
    bid(&mut g, 20);
    reprice(&mut g);
    set_loans(&mut g, holder, 2);
    let factory = g.state.players[holder].factory_stock();
    assert_eq!(factory, 1, "all there is to seize but the lot to be won");
    pass_turn(&mut g);
    pass_turn(&mut g);

    assert_eq!(g.current_player(), holder);
    assert_eq!(g.phase, Phase::Main);
    assert_eq!(
        g.state.players[holder].factory_stock(),
        0,
        "the interest cost the factory container"
    );
    assert_eq!(g.state.players[holder].holding, counts(&[(Green, 2)]));
    assert_eq!(g.check_invariants(), Ok(()));
}

// Whole games

#[test]
fn the_bank_holds_one_auction_for_three_or_four_players_and_two_for_five() {
    assert_eq!(max_active_auctions(3), 1);
    assert_eq!(max_active_auctions(4), 1);
    assert_eq!(max_active_auctions(5), 2);
}

#[test]
fn the_invariants_flag_more_auctions_than_the_bank_can_hold() {
    for (players, fine) in [(3, false), (4, false), (5, true)] {
        let (mut g, me) = game_with_cash(players, 20);
        for kind in AuctionKind::ALL {
            let bid = match kind {
                ContainerLot => Bid::Cash(0),
                AuctionKind::CashLot => Bid::Containers(ContainerBid::default()),
            };
            g.state.bank.auctions[kind] = Some(Auction {
                lot: 0,
                holder: me,
                bid,
            });
        }

        assert_eq!(g.check_invariants().is_ok(), fine, "{players} players");
    }
}
#[test]
fn random_play_with_bank_calls_keeps_every_invariant() {
    let (mut started, mut outbid_count, mut won) = ([0; 2], [0; 2], [0; 2]);
    for seed in 0..6 {
        for players in 3..=5 {
            let mut g = new_game(players, GameLength::Short, seed);
            let mut rng = Rng::new(seed + 90);
            for _ in 0..1500 {
                let Some(legal) = actions_while_playing(&g) else {
                    break;
                };
                let calls: Vec<Action> = legal
                    .iter()
                    .copied()
                    .filter(|a| matches!(a, Action::StartAuction { .. } | Action::Outbid(_)))
                    .collect();
                let pick = if !calls.is_empty() && rng.below(2) == 0 {
                    calls[rng.below(calls.len() as u32) as usize]
                } else {
                    legal[rng.below(legal.len() as u32) as usize]
                };
                let before = g.state.bank.auctions;
                g.apply(pick).unwrap();
                match pick {
                    Action::StartAuction { kind, .. } => started[kind.index()] += 1,
                    Action::Outbid(kind) => outbid_count[kind.index()] += 1,
                    _ => {}
                }
                for kind in AuctionKind::ALL {
                    let ended = before[kind].is_some() && g.state.bank.auctions[kind].is_none();
                    won[kind.index()] += usize::from(ended);
                }
                assert_eq!(g.check_invariants(), Ok(()), "after {pick:?}");
            }
        }
    }
    for kind in AuctionKind::ALL {
        let i = kind.index();
        assert!(
            started[i] > 50,
            "only {} {kind:?} auctions were started",
            started[i]
        );
        assert!(
            outbid_count[i] > 80,
            "only {} {kind:?} bids were outbid",
            outbid_count[i]
        );
        assert!(won[i] > 50, "only {} {kind:?} auctions were won", won[i]);
    }
}
