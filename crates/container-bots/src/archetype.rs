//! Producer, Showcase and Shipper: three economic opponents on one decision framework.
//!
//! Every legal action is valued in dollars of final score and the best one is played, ties broken at
//! random. The three differ only in their [`Profile`], a handful of hand-set weights: what a container
//! is worth in a factory or a harbor, what a new factory, warehouse or delivery is worth, and the lot
//! it prices at. No action is forbidden and none has a quota, so an archetype does what the others do
//! whenever its numbers say so.
//!
//! Two guards keep a table from stalling, which hand-set weights alone do not: a bot in debt, or short
//! of cash, builds nothing (`CAPEX_BUFFER`), and a bot that has had nothing to do for a while marks
//! its stock down (`IDLE_MARKDOWN`).
//!
//! A scripted seat of the training environment is never told what the others did (`observe` is not
//! called), so the only thing an archetype learns is from `choose` itself: the top bid of its own
//! deliveries, which scales what it expects a cargo to fetch.

use container_engine::action::{Action, Building, District};
use container_engine::build::{factory_build_cost, warehouse_build_cost};
use container_engine::color::{Color, ColorCounts};
use container_engine::observation::{BiddingView, Observation, PhaseView, PlayerView};
use container_engine::phase::{Buying, Decision, Phase};
use container_engine::rng::Rng;
use container_engine::scoring::{ScoringCard, score_player};
use container_engine::state::{
    FACTORY_STORAGE, PlayerId, PlayerState, SHIP_CAPACITY, ShipLocation,
};

use crate::Opponent;

pub const PRODUCER: &str = "producer";
pub const SHOWCASE: &str = "showcase";
pub const SHIPPER: &str = "shipper";

/// What a bot needs to expect before it moves on a plan, in dollars.
const MARGIN: f64 = 0.25;
/// Value of doing nothing: any action that is worth something beats it.
const PASS: f64 = -0.01;
/// The least a legal production is worth, so that a table in which nobody is interested still ends.
const PRODUCE_FLOOR: f64 = 0.05;
/// The top bid of a delivery by the containers on the ship, before the table's own level is learned.
const TOP: [f64; 6] = [0.0, 4.0, 5.2, 6.2, 7.0, 7.6];
/// The share of a cargo's nominal worth that a bidder counts on even while it adds nothing to the
/// island yet: the most common color of an island is discarded, so the first containers score nothing.
const CARGO_FLOOR: f64 = 0.6;
/// Cash a bot keeps after building, on top of its reserve, to pay wages and interest and to buy a cargo.
/// With the loans it owes it builds nothing: a loan costs $1 a turn, more than a building returns.
const CAPEX_BUFFER: u32 = 3;
/// Decisions in a row with nothing worth doing, after which a bot marks its stock down to the cheapest
/// lot: a table where every bot waits for the others to buy would never end.
const IDLE_MARKDOWN: u32 = 4;

/// The weights that make one bot a Producer, a Showcase or a Shipper.
#[derive(Clone, Copy, Debug)]
pub struct Profile {
    /// What a container in its factory district is worth to it, at the start of the game.
    pub factory_unit: f64,
    /// What a container in its harbor is worth to it at the start of the game. Two dollars of it is
    /// what it scores at the end whatever happens, the rest is the resale.
    pub harbor_unit: f64,
    /// What a color is worth that its harbor or its cargo does not hold yet.
    pub variety: f64,
    /// What one more factory and one more warehouse are worth at the start of the game. They fall to
    /// nothing as the supply runs out.
    pub factory_appetite: f64,
    pub warehouse_appetite: f64,
    /// The lot (0 is the cheapest) it prices a container at in its factory and its harbor district.
    pub factory_lot: usize,
    pub harbor_lot: usize,
    /// How much it makes of the cash a delivery brings.
    pub ship_taste: f64,
    /// The share of what a cargo is worth to it that it bids for the cargo.
    pub bid_share: f64,
    /// Cash it keeps back besides the interest it owes.
    pub reserve: u32,
    /// The cash at which it repays a loan.
    pub repay_at: u32,
}

/// Makes the most containers and sells its own output at the cheapest lot, $1 in its factory district:
/// it values a factory above a warehouse, produces at every chance and ships only some of its stock.
pub const PRODUCER_PROFILE: Profile = Profile {
    factory_unit: 2.2,
    harbor_unit: 2.0,
    variety: 0.3,
    factory_appetite: 11.0,
    warehouse_appetite: 7.0,
    factory_lot: 0,
    harbor_lot: 1,
    ship_taste: 0.7,
    bid_share: 0.8,
    reserve: 1,
    repay_at: 16,
};

/// Keeps its harbor full of many colors at middling prices: it values a warehouse above a factory,
/// buys from factories to stock it and delivers the least of the three.
pub const SHOWCASE_PROFILE: Profile = Profile {
    factory_unit: 1.2,
    harbor_unit: 3.2,
    variety: 1.2,
    factory_appetite: 7.0,
    warehouse_appetite: 12.0,
    factory_lot: 1,
    harbor_lot: 1,
    ship_taste: 0.5,
    bid_share: 0.8,
    reserve: 2,
    repay_at: 18,
};

/// Lives off deliveries: it buys cargo wherever it is cheapest, sails whenever a delivery pays and
/// bids hard for the cargo of others. It does not aim for a size of cargo, the price of the next
/// container against what it adds to the delivery decides.
pub const SHIPPER_PROFILE: Profile = Profile {
    factory_unit: 1.2,
    harbor_unit: 2.0,
    variety: 0.6,
    factory_appetite: 9.0,
    warehouse_appetite: 9.0,
    factory_lot: 1,
    harbor_lot: 0,
    ship_taste: 1.0,
    bid_share: 0.9,
    reserve: 1,
    repay_at: 14,
};

/// Evaluation-only price-lot deviations (Stage E of the plan): the archetype index (0 producer, 1
/// showcase), its factory lot and its harbor lot. A variant equal to the base profile is a null
/// variant. The training environment seats variant `i` as seat kind `-11 - i`; they are not opponents.
pub const VARIANTS: [(usize, usize, usize); 9] = [
    (0, 0, 1), // null producer
    (0, 1, 1),
    (0, 2, 1),
    (0, 0, 2),
    (0, 1, 2),
    (1, 1, 1), // null showcase
    (1, 2, 1),
    (1, 1, 2),
    (1, 2, 2),
];

/// The archetype of [`VARIANTS`]`[index]`, with only its price lots changed.
pub fn variant(index: usize, seed: u64) -> Option<Archetype> {
    let (who, factory_lot, harbor_lot) = *VARIANTS.get(index)?;
    let base = if who == 0 { PRODUCER_PROFILE } else { SHOWCASE_PROFILE };
    Some(Archetype::new(Profile { factory_lot, harbor_lot, ..base }, seed))
}

#[derive(Clone, Debug)]
pub struct Archetype {
    profile: Profile,
    rng: Rng,
    /// What its deliveries fetched, against what `TOP` says, as a running average.
    bid_level: f64,
    /// Its latest decisions in a row in which nothing was worth doing.
    idle: u32,
}

impl Archetype {
    pub fn new(profile: Profile, seed: u64) -> Self {
        Self {
            profile,
            rng: Rng::new(seed),
            bid_level: 1.0,
            idle: 0,
        }
    }

    pub fn producer(seed: u64) -> Self {
        Self::new(PRODUCER_PROFILE, seed)
    }

    pub fn showcase(seed: u64) -> Self {
        Self::new(SHOWCASE_PROFILE, seed)
    }

    pub fn shipper(seed: u64) -> Self {
        Self::new(SHIPPER_PROFILE, seed)
    }
}

/// What the bot knows about the position it must decide in.
struct Ctx<'a> {
    obs: &'a Observation,
    me: &'a PlayerView,
    cash: u32,
    /// Cash it will not spend: the interest it owes next turn and its reserve.
    keep: u32,
    /// How far the supply is run down, from 0 to 1: the game ends when two colors are out.
    progress: f64,
    /// Containers on its ship.
    cargo: u32,
    /// Whether it has had nothing to do for a while.
    stuck: bool,
}

impl Ctx<'_> {
    fn left(&self) -> f64 {
        1.0 - self.progress
    }

    fn spendable(&self) -> u32 {
        self.cash.saturating_sub(self.keep)
    }

    /// Whether it can pay `cost` for a building and still run its business, out of debt.
    fn can_build(&self, cost: u32) -> bool {
        self.me.loans == 0 && self.spendable() >= cost + CAPEX_BUFFER
    }

    fn factories(&self) -> u32 {
        self.me.factories.iter().filter(|&&f| f).count() as u32
    }

    fn harbor_room(&self) -> u32 {
        let stock: u32 = self.me.harbor_lots.iter().map(ColorCounts::total).sum();
        u32::from(self.me.warehouses).saturating_sub(stock + u32::from(self.me.reserve_harbor))
    }

    fn factory_room(&self) -> u32 {
        let stock: u32 = self.me.factory_lots.iter().map(ColorCounts::total).sum();
        (FACTORY_STORAGE * self.factories())
            .saturating_sub(stock + u32::from(self.me.reserve_factory))
    }

    /// The colors in its harbor, as counts.
    fn harbor_colors(&self) -> ColorCounts {
        let mut held = ColorCounts::default();
        self.me.harbor_lots.iter().for_each(|lot| held.add_all(lot));
        held
    }
}

impl Opponent for Archetype {
    fn choose(&mut self, obs: &Observation, legal: &[Action]) -> Action {
        let ctx = self.context(obs);
        match obs.phase {
            PhaseView::Bid(bidding) => return self.bid(&ctx, &bidding, legal),
            PhaseView::Open(Phase::Decide(decision)) => return self.decide(&ctx, &decision, legal),
            PhaseView::Open(_) => {}
        }
        let scores: Vec<f64> = legal.iter().map(|&a| self.value(&ctx, a)).collect();
        let best = scores.iter().cloned().fold(f64::MIN, f64::max);
        let top: Vec<Action> = legal
            .iter()
            .zip(&scores)
            .filter(|&(_, &s)| s >= best - 1e-9)
            .map(|(&a, _)| a)
            .collect();
        let action = top[self.rng.below(top.len() as u32) as usize];
        if obs.phase == PhaseView::Open(Phase::Main) {
            // Repricing moves nothing, so a bot that only reprices is as stuck as one that passes.
            let moved = best > 0.0 && !matches!(action, Action::Reprice(_));
            self.idle = if moved { 0 } else { self.idle + 1 };
        }
        action
    }
}

impl Archetype {
    fn context<'a>(&self, obs: &'a Observation) -> Ctx<'a> {
        let me = &obs.players[obs.viewer];
        Ctx {
            obs,
            me,
            cash: me.cash.unwrap_or(0),
            keep: u32::from(me.loans) + self.profile.reserve,
            progress: progress(obs),
            cargo: me.ship.total(),
            stuck: self.idle >= IDLE_MARKDOWN,
        }
    }

    /// What a cargo of `k` containers brings its deliverer once sold: twice the top bid, the subsidy
    /// being as large as the bid.
    fn cargo_cash(&self, k: u32) -> f64 {
        self.profile.ship_taste * 2.0 * TOP[k.min(5) as usize] * self.bid_level
    }

    /// What the next container is worth on a ship that holds `aboard` already, bought for `price`.
    fn cargo_gain(&self, c: &Ctx, aboard: u32, new_color: bool, price: u32) -> f64 {
        let delivery = self.cargo_cash(aboard + 1) - self.cargo_cash(aboard);
        // A container still aboard at the end is worth $3.
        let sure = 3.0 * c.progress;
        delivery.max(sure) + if new_color { self.profile.variety } else { 0.0 } - f64::from(price)
    }

    /// What a container is worth in its harbor, bought for `price`.
    fn harbor_gain(&self, c: &Ctx, new_color: bool, price: u32) -> f64 {
        let unit = 2.0 + (self.profile.harbor_unit - 2.0) * c.left();
        unit + if new_color { self.profile.variety } else { 0.0 } - f64::from(price)
    }

    /// What a cargo is worth on the island of the viewer, which is what it would pay to keep it.
    fn cargo_worth(c: &Ctx, cargo: &ColorCounts) -> f64 {
        let card = c.me.scoring_card.expect("a seat sees its own card");
        island_gain(card, c.me.scoring_area, cargo)
            .max(CARGO_FLOOR * f64::from(card.cargo_value(cargo)))
    }

    /// The bid it puts on another's cargo: a share of the worth, shaded by the number of rivals.
    fn bid(&mut self, c: &Ctx, b: &BiddingView, legal: &[Action]) -> Action {
        let worth = Self::cargo_worth(c, &c.obs.players[b.deliverer].ship);
        let rivals = b.contenders.iter().filter(|&&on| on).count().max(2) as f64;
        let total = self.profile.bid_share * worth * (rivals - 1.0) / rivals;
        let add = (total - f64::from(b.base)).clamp(0.0, f64::from(c.spendable()));
        // Rounding at random keeps two bots of one kind from always tying.
        let amount = (add + f64::from(self.rng.below(1000)) / 1000.0).floor() as u32;
        let wanted = Action::Bid(amount.min(c.spendable()));
        if legal.contains(&wanted) {
            wanted
        } else {
            legal[0]
        }
    }

    /// Keeps the cargo only if that beats the cash of selling it, and else sells to the bidder with
    /// the least on its island.
    fn decide(&mut self, c: &Ctx, d: &Decision, legal: &[Action]) -> Action {
        self.learn(c, d.top);
        let top = f64::from(d.top);
        let worth = Self::cargo_worth(c, &c.me.ship);
        if legal.contains(&Action::BuyOut)
            && d.top <= c.spendable()
            && worth - top > 2.0 * top + MARGIN
        {
            return Action::BuyOut;
        }
        let area = |a: &Action| match a {
            Action::Accept(w) => c.obs.players[*w].scoring_area.total(),
            _ => u32::MAX,
        };
        let least = legal.iter().map(area).min().unwrap_or(u32::MAX);
        let tied: Vec<Action> = legal.iter().copied().filter(|a| area(a) == least).collect();
        tied[self.rng.below(tied.len() as u32) as usize]
    }

    /// Moves the expected top bid towards what its own cargo has just fetched.
    fn learn(&mut self, c: &Ctx, top: u32) {
        if c.cargo > 0 {
            let seen = f64::from(top) / TOP[c.cargo.min(5) as usize];
            self.bid_level = (0.8 * self.bid_level + 0.2 * seen).clamp(0.4, 2.5);
        }
    }

    fn value(&self, c: &Ctx, action: Action) -> f64 {
        match c.obs.phase {
            PhaseView::Open(Phase::Main) => self.main_value(c, action),
            PhaseView::Open(Phase::Produce(_)) => match action {
                Action::ProduceColor(color) => {
                    let held: u32 =
                        c.me.factory_lots
                            .iter()
                            .map(|lot| u32::from(lot[color]))
                            .sum();
                    f64::from(c.obs.supply.containers[color]) - 2.0 * f64::from(held)
                }
                _ => 0.0,
            },
            PhaseView::Open(Phase::Buy(b)) => self.buy_value(c, &b, action),
            PhaseView::Open(Phase::Arrange(a)) => match action {
                Action::Place(lot) => -(lot as f64 - self.target_lot(c, a.district) as f64).abs(),
                _ => 0.0,
            },
            PhaseView::Open(Phase::Load(_)) => match action {
                Action::DoneLoading => 0.0,
                _ => 1.0,
            },
            // The Bank's auctions, seizures and deals are not part of its game.
            _ => 0.0,
        }
    }

    /// The lot it lays a container in: its own, and the cheapest when the game is nearly over or it
    /// has been stuck.
    fn target_lot(&self, c: &Ctx, district: District) -> usize {
        if c.obs.last_turn || c.progress > 0.85 || c.stuck {
            return 0;
        }
        match district {
            District::Factory => self.profile.factory_lot,
            District::Harbor => self.profile.harbor_lot,
        }
    }

    fn main_value(&self, c: &Ctx, action: Action) -> f64 {
        let v = self.plan_value(c, action);
        // Delivering ends the turn. With a second action to spare, the ship waits for it.
        let spare = c.me.ship_location == ShipLocation::Ocean
            && c.cargo > 0
            && c.obs.turn.actions_left == 2;
        let paid = matches!(
            action,
            Action::Produce | Action::Build(_) | Action::FactoryPurchase(_) | Action::Reprice(_)
        );
        if spare && paid && v > 0.0 {
            v + self.cargo_cash(c.cargo)
        } else {
            v
        }
    }

    fn plan_value(&self, c: &Ctx, action: Action) -> f64 {
        let p = &self.profile;
        match action {
            Action::Produce => {
                let candidates = Color::ALL
                    .into_iter()
                    .filter(|&color| {
                        c.me.factories[color.index()] && c.obs.supply.containers[color] > 0
                    })
                    .count() as u32;
                let made = candidates.min(c.factory_room());
                // The wage must not come out of the cash it keeps back to pay interest.
                if made == 0 || c.spendable() == 0 {
                    -1.0
                } else {
                    (f64::from(made) * p.factory_unit * c.left() - 1.0).max(PRODUCE_FLOOR)
                }
            }
            Action::Build(Building::Factory(color)) => {
                let Some(cost) = factory_build_cost(c.factories()) else {
                    return -1.0;
                };
                if !c.can_build(cost) {
                    return -1.0;
                }
                let rivals = (0..c.obs.num_players)
                    .filter(|&s| s != c.obs.viewer && c.obs.players[s].factories[color.index()])
                    .count() as f64;
                // Among equal factories, the color with more supply left and fewer rivals.
                p.factory_appetite * c.left() - f64::from(cost)
                    + f64::from(c.obs.supply.containers[color]) / 20.0
                    - 0.3 * rivals
            }
            Action::Build(Building::Warehouse) => {
                let Some(cost) = warehouse_build_cost(u32::from(c.me.warehouses)) else {
                    return -1.0;
                };
                if !c.can_build(cost) {
                    return -1.0;
                }
                p.warehouse_appetite * c.left() - f64::from(cost)
            }
            Action::FactoryPurchase(seller) => self.restock_gain(c, seller),
            Action::HarborPurchase => match c.me.ship_location {
                ShipLocation::Harbor(seller) => self.stock_gain(c, seller, c.me.ship),
                _ => -1.0,
            },
            Action::Sail(ShipLocation::Island) => self.cargo_cash(c.cargo),
            Action::Sail(ShipLocation::Ocean) if c.cargo > 0 => self.cargo_cash(c.cargo) / 2.0,
            Action::Sail(ShipLocation::Ocean) => {
                let best = (0..c.obs.num_players)
                    .filter(|&h| h != c.obs.viewer)
                    .map(|h| self.stock_gain(c, h, ColorCounts::default()))
                    .fold(-1.0, f64::max);
                // Four actions: out to the ocean, to the harbor, back to the ocean and to the island.
                if best > 0.0 { best / 4.0 } else { PASS * 2.0 }
            }
            // With cargo aboard the ship goes to the island; it looks for cargo only with an empty hold.
            Action::Sail(ShipLocation::Harbor(h)) if c.cargo == 0 => {
                let gain = self.stock_gain(c, h, ColorCounts::default());
                if gain > 0.0 { gain / 3.0 } else { -1.0 }
            }
            Action::Reprice(district) => {
                let lots = match district {
                    District::Factory => &c.me.factory_lots[..],
                    District::Harbor => &c.me.harbor_lots[..],
                };
                let target = self.target_lot(c, district);
                let above: u32 = lots[(target + 1).min(lots.len())..]
                    .iter()
                    .map(ColorCounts::total)
                    .sum();
                match (above, district) {
                    (0, District::Factory) => PASS,
                    (0, District::Harbor) => PASS * 2.0,
                    (n, _) => 0.1 * f64::from(n),
                }
            }
            Action::RequestLoan if c.cash <= 1 && c.obs.turn.loans_repaid == 0 => 60.0,
            Action::RepayLoan
                if c.obs.turn.loans_requested == 0
                    && (c.cash >= p.repay_at || (c.progress > 0.85 && c.spendable() >= 10)) =>
            {
                50.0
            }
            Action::RequestLoan | Action::RepayLoan => -3.0,
            _ => -5.0,
        }
    }

    /// What buying from the factory district of `seller` is worth: every container that is worth its
    /// price to the harbor, up to the room it has and the cash it can spare.
    fn restock_gain(&self, c: &Ctx, seller: PlayerId) -> f64 {
        let lots = &c.obs.players[seller].factory_lots;
        let (mut room, mut budget) = (c.harbor_room(), c.spendable());
        let mut held = c.harbor_colors();
        let mut total = 0.0;
        'lots: for (lot, colors) in lots.iter().enumerate() {
            let price = District::Factory.lot_price(lot);
            for color in Color::ALL {
                for _ in 0..colors[color] {
                    if room == 0 || price > budget {
                        break 'lots;
                    }
                    let gain = self.harbor_gain(c, held[color] == 0, price) - MARGIN;
                    if gain > 0.0 {
                        total += gain;
                        room -= 1;
                        budget -= price;
                        held[color] += 1;
                    }
                }
            }
        }
        if total > 0.0 { total } else { -1.0 }
    }

    /// What loading the ship at the harbor of `seller` is worth beyond the `aboard` it holds already:
    /// every container that adds more to the delivery than it costs.
    fn stock_gain(&self, c: &Ctx, seller: PlayerId, aboard: ColorCounts) -> f64 {
        let lots = &c.obs.players[seller].harbor_lots;
        let mut budget = c.spendable();
        let mut have = aboard;
        let mut total = 0.0;
        'lots: for (lot, colors) in lots.iter().enumerate() {
            let price = District::Harbor.lot_price(lot);
            for color in Color::ALL {
                for _ in 0..colors[color] {
                    if have.total() >= SHIP_CAPACITY || price > budget {
                        break 'lots;
                    }
                    let gain = self.cargo_gain(c, have.total(), have[color] == 0, price) - MARGIN;
                    if gain > 0.0 {
                        total += gain;
                        budget -= price;
                        have[color] += 1;
                    }
                }
            }
        }
        if total > 0.0 { total } else { -1.0 }
    }

    fn buy_value(&self, c: &Ctx, b: &Buying, action: Action) -> f64 {
        let Action::Buy { lot, color } = action else {
            return 0.0;
        };
        let price = b.district.lot_price(lot);
        if price > c.spendable() {
            return -1.0;
        }
        let gain = match b.district {
            District::Factory => {
                self.harbor_gain(c, c.harbor_colors()[color] + b.bought[color] == 0, price)
            }
            District::Harbor => {
                let new_color = c.me.ship[color] + b.bought[color] == 0;
                self.cargo_gain(c, c.cargo + b.bought.total(), new_color, price)
            }
        };
        // Among equal gains the cheaper lot, which the order of buying keeps within reach.
        gain - MARGIN - 0.01 * f64::from(Buying::order_key(lot, color))
    }
}

/// How far the supply is run down: the game ends when two colors are out, so it is the share of the
/// second scarcest color that is gone.
fn progress(obs: &Observation) -> f64 {
    if obs.last_turn {
        return 1.0;
    }
    let mut supply: Vec<f64> = Color::ALL
        .iter()
        .map(|&c| f64::from(obs.supply.containers[c]))
        .collect();
    supply.sort_by(f64::total_cmp);
    (1.0 - supply[1] / (per_color(obs) - 1.5).max(1.0)).clamp(0.0, 1.0)
}

/// Containers of each color the game began with, which every container on the table adds up to. Some
/// may be in hand, so the largest sum is the safest.
fn per_color(obs: &Observation) -> f64 {
    let sum = |color: Color| -> u32 {
        let lots =
            |counts: &[ColorCounts]| counts.iter().map(|lot| u32::from(lot[color])).sum::<u32>();
        u32::from(obs.supply.containers[color])
            + lots(&obs.bank.container_lots)
            + obs
                .players
                .iter()
                .map(|p| {
                    lots(&p.factory_lots)
                        + lots(&p.harbor_lots)
                        + u32::from(p.ship[color] + p.holding[color] + p.scoring_area[color])
                })
                .sum::<u32>()
    };
    f64::from(Color::ALL.into_iter().map(sum).max().unwrap_or(0))
}

/// What `cargo` adds to the island of a player with this card and this much on it already.
fn island_gain(card: ScoringCard, area: ColorCounts, cargo: &ColorCounts) -> f64 {
    let island = |scoring_area| {
        let player = PlayerState {
            scoring_card: card,
            scoring_area,
            ..PlayerState::default()
        };
        f64::from(score_player(&player).island)
    };
    let mut after = area;
    after.add_all(cargo);
    island(after) - island(area)
}
