//! Board state: players, Off-Shore Bank, supply.

use std::ops::{Index, IndexMut};

use crate::action::District;
use crate::color::{Color, ColorCounts, NUM_COLORS};
use crate::config::{GameConfig, MAX_PLAYERS};
use crate::rng::Rng;
use crate::scoring::ScoringCard;

pub type PlayerId = usize;

pub const STARTING_CASH: u32 = 20;
/// Cash in the Bank's three cash lots at the start ($1 + $2 + $3).
pub const BANK_START_CASH: u32 = 6;
pub const SHIP_CAPACITY: u32 = 5;
pub const FACTORY_LOTS: usize = 4;
pub const HARBOR_LOTS: usize = 5;
pub const BANK_LOTS: usize = 3;
/// Containers each factory adds to the factory storage limit.
pub const FACTORY_STORAGE: u32 = 2;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ShipLocation {
    #[default]
    Ocean,
    Island,
    Bank,
    Harbor(PlayerId),
}

/// Everything a player owns. Factory lot `i` sells at $(i + 1), harbor lot `i` at $(i + 2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerState {
    pub cash: u32,
    pub loans: u8,
    pub scoring_card: ScoringCard,
    pub factories: [bool; NUM_COLORS],
    pub warehouses: u8,
    pub factory_lots: [ColorCounts; FACTORY_LOTS],
    pub harbor_lots: [ColorCounts; HARBOR_LOTS],
    pub ship_location: ShipLocation,
    pub ship: ColorCounts,
    /// Containers won at the Off-Shore Bank, waiting to be loaded onto the ship.
    pub holding: ColorCounts,
    /// Containers delivered to Container Island.
    pub scoring_area: ColorCounts,
    /// Reserve tokens in the factory district, one per factory container on the container bid tile.
    /// They take up the room the containers left behind.
    pub reserve_factory: u8,
    /// Reserve tokens in the harbor district, one per harbor container on the container bid tile.
    pub reserve_harbor: u8,
}

impl Default for PlayerState {
    /// An empty placeholder, used for seats beyond the player count.
    fn default() -> Self {
        Self {
            cash: 0,
            loans: 0,
            scoring_card: ScoringCard::ALL[0],
            factories: [false; NUM_COLORS],
            warehouses: 0,
            factory_lots: [ColorCounts::default(); FACTORY_LOTS],
            harbor_lots: [ColorCounts::default(); HARBOR_LOTS],
            ship_location: ShipLocation::Ocean,
            ship: ColorCounts::default(),
            holding: ColorCounts::default(),
            scoring_area: ColorCounts::default(),
            reserve_factory: 0,
            reserve_harbor: 0,
        }
    }
}

impl PlayerState {
    pub fn has_factory(&self, color: Color) -> bool {
        self.factories[color.index()]
    }

    pub fn factory_colors(&self) -> impl Iterator<Item = Color> + '_ {
        Color::ALL.into_iter().filter(|&c| self.has_factory(c))
    }

    pub fn factory_count(&self) -> u32 {
        self.factories.iter().filter(|&&f| f).count() as u32
    }

    pub fn factory_limit(&self) -> u32 {
        FACTORY_STORAGE * self.factory_count()
    }

    pub fn harbor_limit(&self) -> u32 {
        u32::from(self.warehouses)
    }

    pub fn factory_stock(&self) -> u32 {
        self.factory_lots.iter().map(ColorCounts::total).sum()
    }

    /// Containers that still fit in the factory district, not counting the room kept by reserve tokens.
    pub fn factory_room(&self) -> u32 {
        self.factory_limit()
            .saturating_sub(self.factory_stock() + u32::from(self.reserve_factory))
    }

    pub fn harbor_stock(&self) -> u32 {
        self.harbor_lots.iter().map(ColorCounts::total).sum()
    }

    /// Containers that still fit in the harbor district, not counting the room kept by reserve tokens.
    pub fn harbor_room(&self) -> u32 {
        self.harbor_limit()
            .saturating_sub(self.harbor_stock() + u32::from(self.reserve_harbor))
    }

    /// Containers that can be put on the container bid tile: those in the factory and harbor districts.
    pub fn bid_stock(&self) -> u32 {
        self.factory_stock() + self.harbor_stock()
    }

    /// Containers that still fit on the ship.
    pub fn ship_room(&self) -> u32 {
        SHIP_CAPACITY.saturating_sub(self.ship.total())
    }

    pub fn lots(&self, district: District) -> &[ColorCounts] {
        match district {
            District::Factory => &self.factory_lots,
            District::Harbor => &self.harbor_lots,
        }
    }

    pub fn lots_mut(&mut self, district: District) -> &mut [ColorCounts] {
        match district {
            District::Factory => &mut self.factory_lots,
            District::Harbor => &mut self.harbor_lots,
        }
    }

    pub fn stock(&self, district: District) -> u32 {
        self.lots(district).iter().map(ColorCounts::total).sum()
    }
}

/// What the winner of a Bank auction receives. A container lot is bid on with cash, a cash lot with
/// containers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AuctionKind {
    ContainerLot,
    CashLot,
}

impl AuctionKind {
    pub const ALL: [AuctionKind; 2] = [AuctionKind::ContainerLot, AuctionKind::CashLot];

    pub const fn index(self) -> usize {
        self as usize
    }
}

/// The containers on the container bid tile. Those from the factory district lie on the right of the
/// tile and those from the harbor district on its left, so the beaten bidder knows where each came from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ContainerBid {
    pub factory: ColorCounts,
    pub harbor: ColorCounts,
}

impl ContainerBid {
    pub fn total(&self) -> u32 {
        self.factory.total() + self.harbor.total()
    }

    /// Every container of the bid, whichever district it came from.
    pub fn all(&self) -> ColorCounts {
        let mut all = self.factory;
        all.add_all(&self.harbor);
        all
    }
}

/// What lies on a bid tile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bid {
    /// Cash on the cash bid tile. It is out of the bidder's hand until they are outbid or win.
    Cash(u32),
    /// Containers on the container bid tile. They are on none of the bidder's lots until the bidder is
    /// outbid or wins.
    Containers(ContainerBid),
}

impl Bid {
    /// What a higher bid has to beat: dollars for a cash bid, the number of containers otherwise.
    pub fn value(&self) -> u32 {
        match self {
            Bid::Cash(dollars) => *dollars,
            Bid::Containers(bid) => bid.total(),
        }
    }
}

/// An active Bank auction: the auction token is on `lot`, and `holder` has the highest bid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Auction {
    pub lot: usize,
    pub holder: PlayerId,
    pub bid: Bid,
}

/// The Bank's active auctions, at most one of each kind.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Auctions([Option<Auction>; 2]);

impl Auctions {
    pub fn active(&self) -> usize {
        self.0.iter().flatten().count()
    }

    /// Cash lying on bid tiles, which is in nobody's hand.
    pub fn cash_on_tiles(&self) -> u32 {
        self.0
            .iter()
            .flatten()
            .map(|auction| match auction.bid {
                Bid::Cash(dollars) => dollars,
                Bid::Containers(_) => 0,
            })
            .sum()
    }

    /// Containers lying on bid tiles, which are on no lot of any player.
    pub fn containers_on_tiles(&self) -> ColorCounts {
        let mut all = ColorCounts::default();
        for auction in self.0.iter().flatten() {
            if let Bid::Containers(bid) = auction.bid {
                all.add_all(&bid.all());
            }
        }
        all
    }
}

impl Index<AuctionKind> for Auctions {
    type Output = Option<Auction>;

    fn index(&self, kind: AuctionKind) -> &Option<Auction> {
        &self.0[kind.index()]
    }
}

impl IndexMut<AuctionKind> for Auctions {
    fn index_mut(&mut self, kind: AuctionKind) -> &mut Option<Auction> {
        &mut self.0[kind.index()]
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BankState {
    pub container_lots: [ColorCounts; BANK_LOTS],
    pub cash_lots: [u32; BANK_LOTS],
    pub auctions: Auctions,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Supply {
    pub containers: ColorCounts,
    pub factories: ColorCounts,
    pub warehouses: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    /// Only the first `num_players` seats are in play.
    pub players: [PlayerState; MAX_PLAYERS],
    pub num_players: usize,
    pub bank: BankState,
    pub supply: Supply,
    pub first_player: PlayerId,
    pub active_player: PlayerId,
    /// Cash created by the supply (loans, delivery subsidies); for conservation checks.
    pub cash_minted: u32,
    /// Cash paid to or returned to the supply (building costs, repaid loans).
    pub cash_burned: u32,
}

impl State {
    pub fn setup(config: &GameConfig, rng: &mut Rng) -> State {
        let n = config.num_players;
        let mut supply = Supply {
            containers: ColorCounts([config.containers_per_color(); NUM_COLORS]),
            factories: ColorCounts([config.factories_per_color(); NUM_COLORS]),
            warehouses: config.total_warehouses(),
        };

        // Three random colors go to the Bank (two in lot I, one in lot II).
        let mut bank = BankState {
            cash_lots: [1, 2, 3],
            ..BankState::default()
        };
        let mut bank_colors = Color::ALL;
        rng.shuffle(&mut bank_colors);
        for (i, &color) in bank_colors[..3].iter().enumerate() {
            supply.containers[color] -= 1;
            bank.container_lots[usize::from(i == 2)][color] += 1;
        }

        let mut factory_colors = Color::ALL;
        rng.shuffle(&mut factory_colors);
        let mut cards = ScoringCard::ALL;
        rng.shuffle(&mut cards);

        let mut players: [PlayerState; MAX_PLAYERS] = Default::default();
        for (seat, player) in players.iter_mut().enumerate().take(n) {
            let color = factory_colors[seat];
            supply.factories[color] -= 1;
            supply.warehouses -= 1;
            supply.containers[color] -= 1;
            player.cash = STARTING_CASH;
            player.warehouses = 1;
            player.factories[color.index()] = true;
            player.factory_lots[1][color] = 1;
            player.scoring_card = cards[seat];
        }

        let first_player = rng.below(n as u32) as usize;
        State {
            players,
            num_players: n,
            bank,
            supply,
            first_player,
            active_player: first_player,
            cash_minted: 0,
            cash_burned: 0,
        }
    }

    pub fn players(&self) -> &[PlayerState] {
        &self.players[..self.num_players]
    }

    /// The seat to the right of `player`: the one who plays just before them.
    pub fn player_on_right(&self, player: PlayerId) -> PlayerId {
        (player + self.num_players - 1) % self.num_players
    }

    /// Containers per color across every place a container can rest.
    pub fn container_totals(&self) -> [u32; NUM_COLORS] {
        let mut totals = [0u32; NUM_COLORS];
        let mut add = |counts: &ColorCounts| {
            for c in Color::ALL {
                totals[c.index()] += u32::from(counts[c]);
            }
        };
        add(&self.supply.containers);
        add(&self.bank.auctions.containers_on_tiles());
        self.bank.container_lots.iter().for_each(&mut add);
        for p in self.players() {
            p.factory_lots.iter().for_each(&mut add);
            p.harbor_lots.iter().for_each(&mut add);
            add(&p.ship);
            add(&p.holding);
            add(&p.scoring_area);
        }
        totals
    }
}
