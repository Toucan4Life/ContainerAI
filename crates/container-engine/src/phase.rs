//! Turn bookkeeping and the decision nodes the game can be waiting at.

use crate::action::District;
use crate::color::{Color, ColorCounts, NUM_COLORS};
use crate::config::MAX_PLAYERS;
use crate::state::{AuctionKind, ContainerBid, FACTORY_LOTS, PlayerId};

/// Actions a player takes on their turn.
pub const ACTIONS_PER_TURN: u8 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Turn {
    pub actions_left: u8,
    /// Produce may only be taken once per turn.
    pub produced: bool,
    /// Loans requested and repaid this turn; a turn allows two of each.
    pub loans_requested: u8,
    pub loans_repaid: u8,
    /// The Bank may only be called once per turn.
    pub bank_called: bool,
    /// An auction won at the start of the turn rules out calling the Bank.
    pub won_auction: bool,
}

impl Turn {
    pub fn new() -> Self {
        Self {
            actions_left: ACTIONS_PER_TURN,
            produced: false,
            loans_requested: 0,
            loans_repaid: 0,
            bank_called: false,
            won_auction: false,
        }
    }
}

impl Default for Turn {
    fn default() -> Self {
        Self::new()
    }
}

/// A district whose containers are being laid out again.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Arrange {
    /// Whose district it is: the active player's, unless a beaten container bid comes back.
    pub owner: PlayerId,
    pub district: District,
    /// Containers still to be placed; they are on no lot while in hand.
    pub hand: ColorCounts,
    /// Lowest lot the next container may go to. Containers of one color go to
    /// non-decreasing lots, so every arrangement has exactly one action sequence.
    pub min_lot: u8,
    /// Harbor containers of a beaten bid, to be placed in the harbor district after the factory ones.
    pub later: ColorCounts,
}

/// Choosing which colors to produce when the factories can make more than the storage has room for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProduceChoice {
    /// One per color that can be produced: the player has the factory and the supply has a container.
    pub candidates: ColorCounts,
    /// Containers still to be chosen.
    pub to_pick: u8,
    pub picked: ColorCounts,
    /// Colors are picked in color order, so every choice has exactly one action sequence.
    pub next_color: u8,
}

impl ProduceChoice {
    pub fn new(candidates: ColorCounts, to_pick: u8) -> Self {
        Self {
            candidates,
            to_pick,
            picked: ColorCounts::default(),
            next_color: 0,
        }
    }

    /// The choice once this color has been picked.
    pub fn after(mut self, color: Color) -> Self {
        self.picked[color] += 1;
        self.to_pick -= 1;
        self.next_color = color.index() as u8 + 1;
        self
    }

    /// A color can be picked if enough candidates remain from it onwards to complete the choice.
    pub fn can_pick(&self, color: Color) -> bool {
        let from = color.index();
        let remaining: u32 = self.candidates.0[from..]
            .iter()
            .map(|&n| u32::from(n))
            .sum();
        from >= usize::from(self.next_color)
            && self.candidates[color] > 0
            && remaining >= u32::from(self.to_pick)
    }
}

/// Buying containers from an opponent's district, one at a time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Buying {
    pub seller: PlayerId,
    /// The seller's district being bought from. Factory containers go to the buyer's harbor,
    /// harbor containers onto the buyer's ship.
    pub district: District,
    /// Bought so far. They are in hand until the purchase is done.
    pub bought: ColorCounts,
    /// Containers are bought in (lot, color) order, so every purchase has exactly one action sequence.
    pub next: u8,
    /// A free anchor action may buy nothing; a paid purchase must buy something.
    pub optional: bool,
}

impl Buying {
    /// Position of a (lot, color) pair in the purchase order.
    pub fn order_key(lot: usize, color: Color) -> u8 {
        lot_color_key(lot, color)
    }
}

/// Position of a (lot, color) pair when containers are taken in lot order, then color order.
fn lot_color_key(lot: usize, color: Color) -> u8 {
    (lot * NUM_COLORS + color.index()) as u8
}

/// Loading containers from the holding area onto the ship at the Off-Shore Bank.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Loading {
    /// Colors are loaded in color order, so every selection has exactly one action sequence.
    pub next_color: u8,
    pub loaded: u32,
    /// A free anchor action may load nothing; a paid purchase must load something.
    pub optional: bool,
}

/// Where a bidder is in laying out a bid, when bids may be revised.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BidStep {
    /// Choosing how much cash to put on the table.
    Cash,
    /// Choosing how many cash cards that cash lies on the table as.
    Cards,
    /// Choosing how many $0 bluff cards to put with them.
    Bluffs,
    /// Every stack is down and counted, and the bidders look them over.
    Review,
}

/// The cards a bidder lays out for their bid: cash cards worth what they bid, and $0 bluff cards. Only
/// how many cards there are in all is public, not how many of them are bluffs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Stack {
    pub cards: u32,
    pub bluffs: u8,
}

impl Stack {
    /// How many cards lie in the stack, bluffs included.
    pub fn total(&self) -> u32 {
        self.cards + u32::from(self.bluffs)
    }
}

/// A delivery auction in which the opponents place hidden cash bids one after the other.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bidding {
    pub deliverer: PlayerId,
    /// The player who must bid now. Bids already placed are hidden from them.
    pub bidder: PlayerId,
    /// Who bids in this round: every opponent at first, then only those tied for the highest bid.
    pub contenders: [bool; MAX_PLAYERS],
    /// The runoff has begun: contenders add to the bids already on the table.
    pub runoff: bool,
    /// Cash each player has on the table. It left their hand and stays here until the auction is settled.
    pub bids: [u32; MAX_PLAYERS],
    /// The cash every contender had on the table when this round began: none at first, then the highest
    /// bid in a runoff. The stacks of cards are laid out for the cash put on top of it.
    pub base: u32,
    pub step: BidStep,
    /// Every contender has laid out a stack, so how many cards each one has is public.
    pub stacks_down: bool,
    pub stacks: [Stack; MAX_PLAYERS],
    /// Contenders who are content with the stacks as they lie. Any revision clears them all.
    pub ready: [bool; MAX_PLAYERS],
    /// How many times each contender has taken their stack back and laid out another this round.
    pub revisions: [u8; MAX_PLAYERS],
}

impl Bidding {
    /// The first round: every opponent bids from scratch, and `bidder` goes first.
    pub fn opening(deliverer: PlayerId, bidder: PlayerId, contenders: [bool; MAX_PLAYERS]) -> Self {
        Self {
            deliverer,
            bidder,
            contenders,
            runoff: false,
            bids: [0; MAX_PLAYERS],
            base: 0,
            step: BidStep::Cash,
            stacks_down: false,
            stacks: [Stack::default(); MAX_PLAYERS],
            ready: [false; MAX_PLAYERS],
            revisions: [0; MAX_PLAYERS],
        }
    }

    /// The runoff between those tied for the highest bid: each still has `top` on the table.
    pub fn runoff(
        deliverer: PlayerId,
        bidder: PlayerId,
        tied: [bool; MAX_PLAYERS],
        top: u32,
    ) -> Self {
        let mut bids = [0; MAX_PLAYERS];
        for (bid, &is_tied) in bids.iter_mut().zip(&tied) {
            if is_tied {
                *bid = top;
            }
        }
        Self {
            runoff: true,
            bids,
            base: top,
            ..Self::opening(deliverer, bidder, tied)
        }
    }

    /// The cash in a contender's stack: what they put on the table this round, on top of `base`.
    pub fn stack_amount(&self, seat: PlayerId) -> u32 {
        self.bids[seat] - self.base
    }
}

/// The deliverer deciding what to do with the highest bid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Decision {
    pub deliverer: PlayerId,
    /// The highest bid. Everyone in `tied` still has this much on the table.
    pub top: u32,
    /// Everyone holding the highest bid. If there are several, the deliverer picks the winner.
    pub tied: [bool; MAX_PLAYERS],
}

/// The Bank seizing containers from a player who could not pay their interest. The player on the
/// debtor's right chooses what is taken.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Seizing {
    pub debtor: PlayerId,
    /// Loans the interest is still unpaid on; each costs one container.
    pub remaining: u8,
    /// Taken so far. They are in hand until the Bank has them dealt out.
    pub seized: ColorCounts,
    /// Containers of one place are taken in (lot, color) order, so every selection has exactly one
    /// action sequence.
    pub next: u8,
}

impl Seizing {
    /// Position of a (lot, color) pair in the order containers are taken.
    pub fn order_key(lot: usize, color: Color) -> u8 {
        lot_color_key(lot, color)
    }
}

/// Dealing out containers over the Bank's container lots, one per lot starting at lot I: those seized in
/// a default, or those a winning container bid pays the Bank.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Distributing {
    /// Who chooses which color goes in which lot.
    pub decider: PlayerId,
    /// Still to be dealt; they are in hand until they are in a lot.
    pub hand: ColorCounts,
    pub dealt: u8,
    /// For a winning container bid, the cash lot that its winner collects once they have paid.
    pub won_lot: Option<usize>,
}

/// The active player placing a bid on a Bank bid tile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BankBidding {
    pub kind: AuctionKind,
    /// The lot the auction token is on, or is about to be placed on for an opening bid.
    pub lot: usize,
    /// The lowest bid allowed: the opening minimum, or one more than the bid to beat.
    pub min: u32,
    /// Containers picked for a container bid so far. They are in hand until the bid is done.
    pub picked: ContainerBid,
    /// Containers are picked in the order of `BankBidding::order_key`, so every bid has exactly one
    /// action sequence.
    pub next: u8,
}

impl BankBidding {
    pub fn new(kind: AuctionKind, lot: usize, min: u32) -> Self {
        Self {
            kind,
            lot,
            min,
            picked: ContainerBid::default(),
            next: 0,
        }
    }

    /// Position of a container in the picking order: the factory district first, then lot by lot and
    /// color by color.
    pub fn order_key(district: District, lot: usize, color: Color) -> u8 {
        let offset = match district {
            District::Factory => 0,
            District::Harbor => FACTORY_LOTS * NUM_COLORS,
        };
        (offset + lot * NUM_COLORS + color.index()) as u8
    }
}

/// What the game is waiting for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// The active player picks their next action.
    Main,
    /// The active player picks which colors to produce.
    Produce(ProduceChoice),
    /// The active player buys containers from an opponent.
    Buy(Buying),
    /// The active player loads containers from the holding area onto the ship.
    Load(Loading),
    Arrange(Arrange),
    /// An opponent places a hidden bid in the active player's delivery auction.
    Bid(Bidding),
    /// The active player settles their delivery auction.
    Decide(Decision),
    /// The player on the right of a defaulter chooses what the Bank seizes.
    Seize(Seizing),
    /// The seized containers are dealt out over the Bank's lots.
    Distribute(Distributing),
    /// The active player places a bid on a Bank bid tile.
    BankBid(BankBidding),
    GameOver,
}

impl Phase {
    /// Containers held in hand mid-decision, which are not on any board location.
    pub fn in_flight(&self) -> ColorCounts {
        match self {
            Phase::Arrange(a) => {
                let mut all = a.hand;
                all.add_all(&a.later);
                all
            }
            Phase::Buy(b) => b.bought,
            Phase::Seize(s) => s.seized,
            Phase::Distribute(d) => d.hand,
            Phase::BankBid(b) => b.picked.all(),
            Phase::Main
            | Phase::Produce(_)
            | Phase::Load(_)
            | Phase::Bid(_)
            | Phase::Decide(_)
            | Phase::GameOver => ColorCounts::default(),
        }
    }

    /// Cash that is on the table mid-decision, which is in nobody's hand.
    pub fn in_flight_cash(&self) -> u32 {
        match self {
            Phase::Bid(b) => b.bids.iter().sum(),
            Phase::Decide(d) => d.top * d.tied.iter().filter(|&&tied| tied).count() as u32,
            Phase::Main
            | Phase::Produce(_)
            | Phase::Buy(_)
            | Phase::Load(_)
            | Phase::Arrange(_)
            | Phase::Seize(_)
            | Phase::Distribute(_)
            | Phase::BankBid(_)
            | Phase::GameOver => 0,
        }
    }
}
