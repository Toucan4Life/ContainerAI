//! What one seat can see of the game: everything on the table, but nothing of the other hands.

use crate::color::{ColorCounts, NUM_COLORS};
use crate::config::MAX_PLAYERS;
use crate::game::Game;
use crate::phase::{BidStep, Bidding, Phase, Stack, Turn};
use crate::scoring::ScoringCard;
use crate::state::{
    BankState, FACTORY_LOTS, HARBOR_LOTS, PlayerId, PlayerState, ShipLocation, Supply,
};

/// A player as a seat sees them. Their cash and final scoring card are kept secret in their hand until
/// the game is over; everything else lies on the table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerView {
    pub cash: Option<u32>,
    pub scoring_card: Option<ScoringCard>,
    pub loans: u8,
    pub factories: [bool; NUM_COLORS],
    pub warehouses: u8,
    pub factory_lots: [ColorCounts; FACTORY_LOTS],
    pub harbor_lots: [ColorCounts; HARBOR_LOTS],
    pub ship_location: ShipLocation,
    pub ship: ColorCounts,
    pub holding: ColorCounts,
    pub scoring_area: ColorCounts,
    pub reserve_factory: u8,
    pub reserve_harbor: u8,
}

/// A delivery auction as a seat sees it. The cash behind a bid stays face down until the auction is
/// settled; only how many cards each stack holds is on show, once every stack is down.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BiddingView {
    pub deliverer: PlayerId,
    pub bidder: PlayerId,
    pub contenders: [bool; MAX_PLAYERS],
    pub runoff: bool,
    /// What the bidder is doing now.
    pub step: BidStep,
    /// The cash every contender has on the table beneath their stack: none in the first round, and the
    /// tied bid in a runoff. It is public.
    pub base: u32,
    /// The cash each player has on the table, as far as the seat can see it: all of their own, and of
    /// the others only what lies beneath their stacks.
    pub bids: [u32; MAX_PLAYERS],
    /// Whether the player has already bid in this round. Their bid stays face down until everyone has.
    pub placed: [bool; MAX_PLAYERS],
    /// How many cards, bluffs included, lie in each contender's stack. It is on show once every stack
    /// is down, except for a stack that is being laid out again.
    pub cards: [Option<u32>; MAX_PLAYERS],
    /// The viewer's own stack, which tells the bluff cards from the cash cards.
    pub own: Stack,
    /// Contenders who are content with the stacks as they lie.
    pub ready: [bool; MAX_PLAYERS],
    /// How many more times each contender may take their stack back and lay out another.
    pub revisions_left: [u8; MAX_PLAYERS],
}

/// The decision the game is waiting at, as a seat sees it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhaseView {
    /// A delivery auction, whose face-down bids are masked.
    Bid(BiddingView),
    /// Every other decision is taken in the open.
    Open(Phase),
}

/// The game as seen from one seat.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Observation {
    pub viewer: PlayerId,
    pub num_players: usize,
    pub first_player: PlayerId,
    pub active_player: PlayerId,
    /// Who must make the next decision.
    pub current_player: PlayerId,
    pub players: Vec<PlayerView>,
    pub bank: BankState,
    pub supply: Supply,
    pub turn: Turn,
    pub phase: PhaseView,
    /// The supply has run out, so the active player is taking the last turn.
    pub last_turn: bool,
}

impl PlayerView {
    fn new(player: &PlayerState, hand_is_known: bool) -> Self {
        Self {
            cash: hand_is_known.then_some(player.cash),
            scoring_card: hand_is_known.then_some(player.scoring_card),
            loans: player.loans,
            factories: player.factories,
            warehouses: player.warehouses,
            factory_lots: player.factory_lots,
            harbor_lots: player.harbor_lots,
            ship_location: player.ship_location,
            ship: player.ship,
            holding: player.holding,
            scoring_area: player.scoring_area,
            reserve_factory: player.reserve_factory,
            reserve_harbor: player.reserve_harbor,
        }
    }
}

impl Game {
    /// What `viewer` can see of the game, or `None` if nobody sits there.
    pub fn observation(&self, viewer: PlayerId) -> Option<Observation> {
        if viewer >= self.state.num_players {
            return None;
        }
        let over = self.is_terminal();
        let players = self
            .state
            .players()
            .iter()
            .enumerate()
            .map(|(seat, player)| PlayerView::new(player, over || seat == viewer))
            .collect();
        let phase = match self.phase {
            Phase::Bid(bidding) => PhaseView::Bid(self.bidding_view(&bidding, viewer)),
            phase => PhaseView::Open(phase),
        };
        Some(Observation {
            viewer,
            num_players: self.state.num_players,
            first_player: self.state.first_player,
            active_player: self.state.active_player,
            current_player: self.current_player(),
            players,
            bank: self.state.bank,
            supply: self.state.supply,
            turn: self.turn,
            phase,
            last_turn: self.is_last_turn(),
        })
    }

    /// A delivery auction without the cash and the cards that are still face down for `viewer`.
    fn bidding_view(&self, bidding: &Bidding, viewer: PlayerId) -> BiddingView {
        let n = self.state.num_players;
        let cap = self.config.max_bid_revisions;

        // Once every stack is down the bidder is the one looking them over, not the next to lay one out.
        let mut placed = bidding.contenders;
        if !bidding.stacks_down {
            placed = [false; MAX_PLAYERS];
            let mut seat = (bidding.deliverer + 1) % n;
            while seat != bidding.bidder {
                placed[seat] = bidding.contenders[seat];
                seat = (seat + 1) % n;
            }
        }

        // Of the cash on the table the others see only what was there before the stacks were laid out,
        // and a stack that is being laid out again is off the table until it is down once more.
        let laying_out_again = matches!(bidding.step, BidStep::Cards | BidStep::Bluffs);
        let mut bids = [0; MAX_PLAYERS];
        let mut cards = [None; MAX_PLAYERS];
        let mut revisions_left = [0; MAX_PLAYERS];
        for seat in (0..n).filter(|&seat| bidding.contenders[seat]) {
            bids[seat] = if seat == viewer {
                bidding.bids[seat]
            } else {
                bidding.base
            };
            if bidding.stacks_down && !(seat == bidding.bidder && laying_out_again) {
                cards[seat] = Some(bidding.stacks[seat].total());
            }
            revisions_left[seat] = cap - bidding.revisions[seat];
        }
        BiddingView {
            deliverer: bidding.deliverer,
            bidder: bidding.bidder,
            contenders: bidding.contenders,
            runoff: bidding.runoff,
            step: bidding.step,
            base: bidding.base,
            bids,
            placed,
            cards,
            own: bidding.stacks[viewer],
            ready: bidding.ready,
            revisions_left,
        }
    }
}
