//! Turns an observation and the legal actions into the numbers a neural opponent reads.
//!
//! Seats are encoded relative to the viewer (slot 0 is the viewer, slot 1 the player on their left, and
//! so on), so a net does not learn anything tied to a seat number. Everything is scaled to roughly 0..1.

use container_engine::action::{Action, Building, District};
use container_engine::color::{Color, ColorCounts, NUM_COLORS};
use container_engine::config::MAX_PLAYERS;
use container_engine::observation::{Observation, PhaseView, PlayerView};
use container_engine::phase::Phase;
use container_engine::scoring::ScoringCard;
use container_engine::state::{
    AuctionKind, Bid, ContainerBid, FACTORY_LOTS, HARBOR_LOTS, PlayerId, ShipLocation,
};

/// The most actions a net looks at in one decision. Longer lists are thinned out evenly.
pub const MAX_ACTIONS: usize = 48;

const PLAYER_DIM: usize =
    1 + 1 + 1 + NUM_COLORS + 1 + FACTORY_LOTS * NUM_COLORS + HARBOR_LOTS * NUM_COLORS + 3 + MAX_PLAYERS
        + NUM_COLORS * 3
        + 2;
const PHASE_KINDS: usize = 11;
const PHASE_EXTRA: usize = 40;
const BANK_DIM: usize = 3 * NUM_COLORS + 3 + 2 * (1 + 3 + MAX_PLAYERS + 1);
const GLOBAL_DIM: usize = 3 + 2 * MAX_PLAYERS + NUM_COLORS + BANK_DIM + NUM_COLORS + 1 + 6 + 1;

/// Length of the state vector.
pub const STATE_DIM: usize = MAX_PLAYERS * PLAYER_DIM + NUM_COLORS + GLOBAL_DIM + PHASE_KINDS + PHASE_EXTRA;

/// Length of the state vector of a net that also reads the opponent memory ([`crate::memory`]), whose
/// numbers follow the usual state.
pub const STATE_DIM_MEM: usize = STATE_DIM + crate::memory::MEM_DIM;

/// Action kinds the type one-hot tells apart.
const KINDS: usize = 26;
/// Length of one action's vector.
pub const ACTION_DIM: usize = KINDS + NUM_COLORS + 6 + MAX_PLAYERS + 4 + 1 + 2 + 3;

struct Out<'a> {
    buf: &'a mut [f32],
    pos: usize,
}

impl Out<'_> {
    fn put(&mut self, x: f32) {
        self.buf[self.pos] = x;
        self.pos += 1;
    }

    fn flag(&mut self, b: bool) {
        self.put(b as u8 as f32);
    }

    fn one_hot(&mut self, n: usize, hot: Option<usize>) {
        for i in 0..n {
            self.flag(hot == Some(i));
        }
    }

    fn counts(&mut self, c: &ColorCounts, scale: f32) {
        for &n in &c.0 {
            self.put(n as f32 / scale);
        }
    }

    /// A seat as a one-hot of its place relative to the viewer.
    fn seat(&mut self, obs: &Observation, seat: PlayerId) {
        self.one_hot(MAX_PLAYERS, Some(relative(obs, seat)));
    }

    fn bid(&mut self, bid: &ContainerBid) {
        self.counts(&bid.factory, 3.0);
        self.counts(&bid.harbor, 3.0);
    }
}

fn relative(obs: &Observation, seat: PlayerId) -> usize {
    (seat + obs.num_players - obs.viewer) % obs.num_players
}

fn player(out: &mut Out, obs: &Observation, p: Option<&PlayerView>) {
    let Some(p) = p else {
        for _ in 0..PLAYER_DIM {
            out.put(0.0);
        }
        return;
    };
    out.put(1.0);
    out.put(p.cash.unwrap_or(0) as f32 / 30.0);
    out.put(p.loans as f32 / 2.0);
    for &f in &p.factories {
        out.flag(f);
    }
    out.put(p.warehouses as f32 / 3.0);
    for lot in &p.factory_lots {
        out.counts(lot, 2.0);
    }
    for lot in &p.harbor_lots {
        out.counts(lot, 2.0);
    }
    let (at, seat) = match p.ship_location {
        ShipLocation::Ocean => (0, None),
        ShipLocation::Island => (1, None),
        ShipLocation::Bank => (2, None),
        ShipLocation::Harbor(s) => (3, Some(s)),
    };
    out.one_hot(3, (at < 3).then_some(at));
    match seat {
        Some(s) => out.seat(obs, s),
        None => out.one_hot(MAX_PLAYERS, None),
    }
    out.counts(&p.ship, 5.0);
    out.counts(&p.holding, 5.0);
    out.counts(&p.scoring_area, 8.0);
    out.put(p.reserve_factory as f32 / 3.0);
    out.put(p.reserve_harbor as f32 / 3.0);
}

fn phase_index(phase: &PhaseView) -> usize {
    match phase {
        PhaseView::Bid(_) => 5,
        PhaseView::Open(p) => match p {
            Phase::Main => 0,
            Phase::Produce(_) => 1,
            Phase::Buy(_) => 2,
            Phase::Load(_) => 3,
            Phase::Arrange(_) => 4,
            Phase::Bid(_) => 5,
            Phase::Decide(_) => 6,
            Phase::Seize(_) => 7,
            Phase::Distribute(_) => 8,
            Phase::BankBid(_) => 9,
            Phase::GameOver => 10,
        },
    }
}

fn district(out: &mut Out, d: District) {
    out.flag(d == District::Harbor);
}

fn phase_extra(out: &mut Out, obs: &Observation) {
    let start = out.pos;
    match &obs.phase {
        PhaseView::Bid(b) => {
            out.seat(obs, b.deliverer);
            out.put(b.base as f32 / 10.0);
            out.flag(b.runoff);
            for rel in 0..MAX_PLAYERS {
                let seat = (obs.viewer + rel) % obs.num_players;
                let real = rel < obs.num_players;
                out.put(if real { b.bids[seat] as f32 / 10.0 } else { 0.0 });
            }
            for rel in 0..MAX_PLAYERS {
                let seat = (obs.viewer + rel) % obs.num_players;
                out.flag(rel < obs.num_players && b.placed[seat]);
            }
            for rel in 0..MAX_PLAYERS {
                let seat = (obs.viewer + rel) % obs.num_players;
                out.flag(rel < obs.num_players && b.contenders[seat]);
            }
        }
        PhaseView::Open(Phase::Produce(c)) => {
            out.counts(&c.candidates, 3.0);
            out.counts(&c.picked, 3.0);
            out.put(c.to_pick as f32 / 3.0);
        }
        PhaseView::Open(Phase::Buy(b)) => {
            out.seat(obs, b.seller);
            district(out, b.district);
            out.counts(&b.bought, 3.0);
            out.flag(b.optional);
        }
        PhaseView::Open(Phase::Load(l)) => {
            out.put(l.loaded as f32 / 5.0);
            out.flag(l.optional);
        }
        PhaseView::Open(Phase::Arrange(a)) => {
            out.seat(obs, a.owner);
            district(out, a.district);
            out.counts(&a.hand, 3.0);
            out.put(a.min_lot as f32 / 5.0);
        }
        PhaseView::Open(Phase::Decide(d)) => {
            out.seat(obs, d.deliverer);
            out.put(d.top as f32 / 10.0);
            for rel in 0..MAX_PLAYERS {
                let seat = (obs.viewer + rel) % obs.num_players;
                out.flag(rel < obs.num_players && d.tied[seat]);
            }
        }
        PhaseView::Open(Phase::Seize(s)) => {
            out.seat(obs, s.debtor);
            out.put(s.remaining as f32 / 2.0);
            out.counts(&s.seized, 2.0);
        }
        PhaseView::Open(Phase::Distribute(d)) => {
            out.seat(obs, d.decider);
            out.counts(&d.hand, 3.0);
            out.put(d.dealt as f32 / 3.0);
            out.flag(d.won_lot.is_some());
        }
        PhaseView::Open(Phase::BankBid(b)) => {
            out.flag(b.kind == AuctionKind::CashLot);
            out.one_hot(3, Some(b.lot.min(2)));
            out.put(b.min as f32 / 10.0);
            out.bid(&b.picked);
        }
        PhaseView::Open(_) => {}
    }
    while out.pos < start + PHASE_EXTRA {
        out.put(0.0);
    }
}

/// Writes the state vector of `obs` into `out`, which holds [`STATE_DIM`] numbers.
pub fn encode_state(obs: &Observation, buf: &mut [f32]) {
    assert!(buf.len() >= STATE_DIM);
    let mut out = Out { buf, pos: 0 };
    for rel in 0..MAX_PLAYERS {
        let p = (rel < obs.num_players).then(|| &obs.players[(obs.viewer + rel) % obs.num_players]);
        player(&mut out, obs, p);
    }
    let card = obs.players[obs.viewer].scoring_card;
    let card = card.and_then(|c| ScoringCard::ALL.iter().position(|&x| x == c));
    out.one_hot(NUM_COLORS, card);

    out.one_hot(3, Some(obs.num_players - 3));
    out.seat(obs, obs.first_player);
    out.seat(obs, obs.active_player);
    out.counts(&obs.supply.containers, 10.0);
    for lot in &obs.bank.container_lots {
        out.counts(lot, 3.0);
    }
    for &cash in &obs.bank.cash_lots {
        out.put(cash as f32 / 10.0);
    }
    for kind in AuctionKind::ALL {
        match obs.bank.auctions[kind] {
            Some(a) => {
                out.put(1.0);
                out.one_hot(3, Some(a.lot.min(2)));
                out.seat(obs, a.holder);
                out.put(match a.bid {
                    Bid::Cash(d) => d as f32 / 10.0,
                    Bid::Containers(b) => b.total() as f32 / 5.0,
                });
            }
            None => {
                for _ in 0..1 + 3 + MAX_PLAYERS + 1 {
                    out.put(0.0);
                }
            }
        }
    }
    out.counts(&obs.supply.factories, 3.0);
    out.put(obs.supply.warehouses as f32 / 5.0);
    out.put(obs.turn.actions_left as f32 / 2.0);
    out.flag(obs.turn.produced);
    out.put(obs.turn.loans_requested as f32 / 2.0);
    out.put(obs.turn.loans_repaid as f32 / 2.0);
    out.flag(obs.turn.bank_called);
    out.flag(obs.turn.won_auction);
    out.flag(obs.last_turn);

    out.one_hot(PHASE_KINDS, Some(phase_index(&obs.phase)));
    phase_extra(&mut out, obs);
    debug_assert_eq!(out.pos, STATE_DIM, "state layout");
}

/// Writes the state vector of a net with memory, [`STATE_DIM_MEM`] numbers: the usual state, then the memory.
pub fn encode_state_mem(obs: &Observation, memory: &crate::memory::Memory, buf: &mut [f32]) {
    assert!(buf.len() >= STATE_DIM_MEM);
    encode_state(obs, buf);
    memory.encode(obs.viewer, obs.num_players, &mut buf[STATE_DIM..STATE_DIM_MEM]);
}

fn color_one_hot(out: &mut Out, color: Option<Color>) {
    out.one_hot(NUM_COLORS, color.map(Color::index));
}

/// Writes the vector of one action into `buf`, which holds [`ACTION_DIM`] numbers.
pub fn encode_action(obs: &Observation, action: Action, buf: &mut [f32]) {
    assert!(buf.len() >= ACTION_DIM);
    let me = &obs.players[obs.viewer];
    let cash = me.cash.unwrap_or(0).max(1) as f32;
    let kind;
    let mut color = None;
    let mut lot = None;
    let mut amount = None;
    let mut seat = None;
    let mut location = None;
    let mut dist = None;
    let mut auction = None;
    match action {
        Action::Produce => kind = 0,
        Action::ProduceColor(c) => (kind, color) = (1, Some(c)),
        Action::Build(Building::Factory(c)) => (kind, color) = (2, Some(c)),
        Action::Build(Building::Warehouse) => kind = 3,
        Action::FactoryPurchase(p) => (kind, seat) = (4, Some(p)),
        Action::Buy { lot: l, color: c } => (kind, lot, color) = (5, Some(l), Some(c)),
        Action::DoneBuying => kind = 6,
        Action::Sail(loc) => {
            kind = 7;
            match loc {
                ShipLocation::Ocean => location = Some(0),
                ShipLocation::Island => location = Some(1),
                ShipLocation::Bank => location = Some(2),
                ShipLocation::Harbor(p) => (location, seat) = (Some(3), Some(p)),
            }
        }
        Action::HarborPurchase => kind = 8,
        Action::Load(c) => (kind, color) = (9, Some(c)),
        Action::DoneLoading => kind = 10,
        Action::Reprice(d) => (kind, dist) = (11, Some(d)),
        Action::Place(l) => (kind, lot) = (12, Some(l)),
        Action::Bid(a) => (kind, amount) = (13, Some(a)),
        Action::Accept(p) => (kind, seat) = (14, Some(p)),
        Action::BuyOut => kind = 15,
        Action::RequestLoan => kind = 16,
        Action::RepayLoan => kind = 17,
        Action::Seize { lot: l, color: c } => (kind, lot, color) = (18, Some(l), Some(c)),
        Action::Deal(c) => (kind, color) = (19, Some(c)),
        Action::StartAuction { kind: k, lot: l } => (kind, auction, lot) = (20, Some(k), Some(l)),
        Action::Outbid(k) => (kind, auction) = (21, Some(k)),
        Action::BidCash(a) => (kind, amount) = (22, Some(a)),
        Action::BidContainer {
            district: d,
            lot: l,
            color: c,
        } => (kind, dist, lot, color) = (23, Some(d), Some(l), Some(c)),
        Action::DoneBidding => kind = 24,
    }
    let mut out = Out { buf, pos: 0 };
    out.one_hot(KINDS, Some(kind));
    color_one_hot(&mut out, color);
    out.one_hot(6, lot.map(|l| l.min(5)));
    match seat {
        Some(p) => out.seat(obs, p),
        None => out.one_hot(MAX_PLAYERS, None),
    }
    out.one_hot(4, location);
    out.flag(dist == Some(District::Harbor));
    out.flag(auction == Some(AuctionKind::CashLot));
    out.flag(auction == Some(AuctionKind::ContainerLot));
    let a = amount.unwrap_or(0) as f32;
    out.put(a / 30.0);
    out.put(a / cash);
    out.flag(amount.is_some());
    debug_assert_eq!(out.pos, ACTION_DIM, "action layout");
}

/// Picks at most [`MAX_ACTIONS`] of `n` legal actions, evenly spread, and returns their indices.
pub fn thin(n: usize) -> Vec<usize> {
    if n <= MAX_ACTIONS {
        return (0..n).collect();
    }
    (0..MAX_ACTIONS)
        .map(|i| i * (n - 1) / (MAX_ACTIONS - 1))
        .collect()
}
