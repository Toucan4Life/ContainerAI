//! Actions a player can take, and the districts they act on.

use crate::color::Color;
use crate::state::{AuctionKind, FACTORY_LOTS, HARBOR_LOTS, PlayerId, ShipLocation};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum District {
    Factory,
    Harbor,
}

impl District {
    pub const fn lot_count(self) -> usize {
        match self {
            District::Factory => FACTORY_LOTS,
            District::Harbor => HARBOR_LOTS,
        }
    }

    /// What a container in lot `lot` costs: $1 to $4 in a factory district, $2 to $6 in a harbor.
    pub const fn lot_price(self, lot: usize) -> u32 {
        match self {
            District::Factory => lot as u32 + 1,
            District::Harbor => lot as u32 + 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Building {
    /// A new factory of a color the player does not own yet.
    Factory(Color),
    Warehouse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    /// Pay $1 union wages to the player on the right, then take a container from each factory.
    Produce,
    /// While producing more than fits, pick a color to produce. Colors are picked in color order.
    ProduceColor(Color),
    /// Build a factory or a warehouse, paying the supply.
    Build(Building),
    /// Choose an opponent to buy containers from; their factory containers are bought one by one.
    FactoryPurchase(PlayerId),
    /// While buying: take one container from a lot of the seller, paying that lot's price.
    Buy { lot: usize, color: Color },
    /// Stop buying. At least one container must have been bought, unless this is a free anchor action.
    DoneBuying,
    /// Move the ship: from the ocean to the Bank or an opponent's harbor, from anywhere else to the ocean.
    Sail(ShipLocation),
    /// Buy at the harbor the ship is docked at, or load from the holding area when docked at the Bank.
    HarborPurchase,
    /// While loading at the Bank: move one container from the holding area onto the ship, for free.
    Load(Color),
    /// Stop loading. At least one container must have been loaded, unless this is a free anchor action.
    DoneLoading,
    /// Pick up a whole district and lay its containers out again. With nothing to move it is a pass.
    Reprice(District),
    /// Put the next container of the hand into a lot (0 is the cheapest lot).
    Place(usize),
    /// In a delivery auction: secretly put this much cash on the table. In a runoff it is added to the
    /// bid already there.
    Bid(u32),
    /// When bids may be revised, while laying out a bid: show the cash just put down as this many cash
    /// cards.
    BidCards(u32),
    /// When bids may be revised, while laying out a bid: put this many $0 bluff cards with the cash cards.
    BidBluffs(u8),
    /// When bids may be revised, once every stack is down: leave your stack as it is. The stacks are
    /// revealed when everyone who could still revise theirs has done so.
    ReadyToReveal,
    /// After a delivery auction's bids are in: sell the cargo to this bidder, at their bid.
    Accept(PlayerId),
    /// After a delivery auction's bids are in: pay the winning bid to the Bank and keep the cargo.
    BuyOut,
    /// A free action: take a $10 loan from the supply. A bidder or deliverer may also do it before deciding.
    RequestLoan,
    /// A free action at the main node of your own turn: pay $10 back to the supply to clear a loan.
    RepayLoan,
    /// In default: the Bank seizes one container from the current place. Places without lots use lot 0.
    Seize { lot: usize, color: Color },
    /// In default: put a seized container of this color into the next Bank lot.
    Deal(Color),
    /// A Call Bank action: put the auction token on this Bank lot, and open the bidding on it.
    StartAuction { kind: AuctionKind, lot: usize },
    /// A Call Bank action: outbid the opponent who holds the active auction of this kind.
    Outbid(AuctionKind),
    /// While placing a Bank bid: put this much cash on the cash bid tile.
    BidCash(u32),
    /// While placing a container bid: take one container from a lot of your factory or harbor district
    /// and put it on the container bid tile. Containers are picked in one fixed order: factory district
    /// first, then lot by lot and color by color.
    BidContainer {
        district: District,
        lot: usize,
        color: Color,
    },
    /// Put the containers picked so far on the container bid tile.
    DoneBidding,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionError {
    Illegal(Action),
}
