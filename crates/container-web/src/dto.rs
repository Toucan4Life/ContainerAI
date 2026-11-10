//! JSON shapes the UI reads. Colors are indices in the engine's cycle order (red, blue, yellow, white,
//! green), seats are indices, and a district is `"factory"` or `"harbor"`.

use container_engine::action::{Action, Building, District};
use container_engine::color::ColorCounts;
use container_engine::observation::{BiddingView, Observation, PhaseView, PlayerView};
use container_engine::phase::Phase;
use container_engine::state::{Auction, AuctionKind, Bid, ShipLocation};
use serde_json::{Value, json};

pub const COLOR_NAMES: [&str; 5] = ["red", "blue", "yellow", "white", "green"];

fn counts(c: &ColorCounts) -> Value {
    json!(c.0)
}

fn district(d: District) -> &'static str {
    match d {
        District::Factory => "factory",
        District::Harbor => "harbor",
    }
}

fn kind(k: AuctionKind) -> &'static str {
    match k {
        AuctionKind::ContainerLot => "container",
        AuctionKind::CashLot => "cash",
    }
}

fn location(l: ShipLocation) -> Value {
    match l {
        ShipLocation::Ocean => json!({ "at": "ocean" }),
        ShipLocation::Island => json!({ "at": "island" }),
        ShipLocation::Bank => json!({ "at": "bank" }),
        ShipLocation::Harbor(owner) => json!({ "at": "harbor", "owner": owner }),
    }
}

pub fn action_json(action: Action) -> Value {
    match action {
        Action::Produce => json!({ "type": "Produce" }),
        Action::ProduceColor(c) => json!({ "type": "ProduceColor", "color": c.index() }),
        Action::Build(Building::Factory(c)) => {
            json!({ "type": "BuildFactory", "color": c.index() })
        }
        Action::Build(Building::Warehouse) => json!({ "type": "BuildWarehouse" }),
        Action::FactoryPurchase(seller) => json!({ "type": "FactoryPurchase", "seller": seller }),
        Action::Buy { lot, color } => json!({ "type": "Buy", "lot": lot, "color": color.index() }),
        Action::DoneBuying => json!({ "type": "DoneBuying" }),
        Action::Sail(to) => json!({ "type": "Sail", "to": location(to) }),
        Action::HarborPurchase => json!({ "type": "HarborPurchase" }),
        Action::Load(c) => json!({ "type": "Load", "color": c.index() }),
        Action::DoneLoading => json!({ "type": "DoneLoading" }),
        Action::Reprice(d) => json!({ "type": "Reprice", "district": district(d) }),
        Action::Place(lot) => json!({ "type": "Place", "lot": lot }),
        Action::Bid(amount) => json!({ "type": "Bid", "amount": amount }),
        Action::Accept(player) => json!({ "type": "Accept", "player": player }),
        Action::BuyOut => json!({ "type": "BuyOut" }),
        Action::RequestLoan => json!({ "type": "RequestLoan" }),
        Action::RepayLoan => json!({ "type": "RepayLoan" }),
        Action::Seize { lot, color } => {
            json!({ "type": "Seize", "lot": lot, "color": color.index() })
        }
        Action::Deal(c) => json!({ "type": "Deal", "color": c.index() }),
        Action::StartAuction { kind: k, lot } => {
            json!({ "type": "StartAuction", "kind": kind(k), "lot": lot })
        }
        Action::Outbid(k) => json!({ "type": "Outbid", "kind": kind(k) }),
        Action::BidCash(amount) => json!({ "type": "BidCash", "amount": amount }),
        Action::BidContainer {
            district: d,
            lot,
            color,
        } => json!({
            "type": "BidContainer",
            "district": district(d),
            "lot": lot,
            "color": color.index(),
        }),
        Action::DoneBidding => json!({ "type": "DoneBidding" }),
    }
}

/// Where the Bank looks first when a player defaults, in the order the rules give.
fn seizure_place(p: &PlayerView) -> &'static str {
    let any = |lots: &[ColorCounts]| lots.iter().any(|l| !l.is_empty());
    if !p.scoring_area.is_empty() {
        "island"
    } else if !p.ship.is_empty() {
        "ship"
    } else if !p.holding.is_empty() {
        "holding"
    } else if any(&p.harbor_lots) {
        "harbor"
    } else {
        "factory"
    }
}

fn bidding_json(b: &BiddingView, n: usize) -> Value {
    json!({
        "type": "Bid",
        "deliverer": b.deliverer,
        "bidder": b.bidder,
        "contenders": &b.contenders[..n],
        "runoff": b.runoff,
        "base": b.base,
        "bids": &b.bids[..n],
        "placed": &b.placed[..n],
    })
}

fn phase_json(obs: &Observation) -> Value {
    let n = obs.num_players;
    let phase = match obs.phase {
        PhaseView::Bid(b) => return bidding_json(&b, n),
        PhaseView::Open(p) => p,
    };
    match phase {
        Phase::Main => json!({ "type": "Main" }),
        Phase::Produce(p) => json!({
            "type": "Produce",
            "candidates": counts(&p.candidates),
            "toPick": p.to_pick,
            "picked": counts(&p.picked),
        }),
        Phase::Buy(b) => json!({
            "type": "Buy",
            "seller": b.seller,
            "district": district(b.district),
            "bought": counts(&b.bought),
            "optional": b.optional,
        }),
        Phase::Load(l) => json!({ "type": "Load", "loaded": l.loaded, "optional": l.optional }),
        Phase::Arrange(a) => json!({
            "type": "Arrange",
            "owner": a.owner,
            "district": district(a.district),
            "hand": counts(&a.hand),
            "minLot": a.min_lot,
            "later": counts(&a.later),
        }),
        Phase::Bid(_) => json!({ "type": "Main" }),
        Phase::Decide(d) => json!({
            "type": "Decide",
            "deliverer": d.deliverer,
            "top": d.top,
            "tied": &d.tied[..n],
        }),
        Phase::Seize(s) => json!({
            "type": "Seize",
            "debtor": s.debtor,
            "remaining": s.remaining,
            "seized": counts(&s.seized),
            "place": seizure_place(&obs.players[s.debtor]),
        }),
        Phase::Distribute(d) => json!({
            "type": "Distribute",
            "decider": d.decider,
            "hand": counts(&d.hand),
            "dealt": d.dealt,
            "wonLot": d.won_lot,
        }),
        Phase::BankBid(b) => json!({
            "type": "BankBid",
            "kind": kind(b.kind),
            "lot": b.lot,
            "min": b.min,
            "picked": { "factory": counts(&b.picked.factory), "harbor": counts(&b.picked.harbor) },
        }),
        Phase::GameOver => json!({ "type": "GameOver" }),
    }
}

fn player_json(p: &PlayerView) -> Value {
    json!({
        "cash": p.cash,
        "scoringCard": p.scoring_card.map(|c| c.ten.index()),
        "loans": p.loans,
        "factories": p.factories,
        "warehouses": p.warehouses,
        "factoryLots": p.factory_lots.iter().map(counts).collect::<Vec<_>>(),
        "harborLots": p.harbor_lots.iter().map(counts).collect::<Vec<_>>(),
        "ship": location(p.ship_location),
        "cargo": counts(&p.ship),
        "holding": counts(&p.holding),
        "island": counts(&p.scoring_area),
        "reserveFactory": p.reserve_factory,
        "reserveHarbor": p.reserve_harbor,
    })
}

fn auction_json(k: AuctionKind, a: &Auction) -> Value {
    let bid = match a.bid {
        Bid::Cash(dollars) => json!({ "cash": dollars }),
        Bid::Containers(c) => {
            json!({ "factory": counts(&c.factory), "harbor": counts(&c.harbor) })
        }
    };
    json!({ "kind": kind(k), "lot": a.lot, "holder": a.holder, "bid": bid })
}

pub fn observation_json(obs: &Observation) -> Value {
    let auctions: Vec<Value> = AuctionKind::ALL
        .into_iter()
        .filter_map(|k| obs.bank.auctions[k].map(|a| auction_json(k, &a)))
        .collect();
    json!({
        "viewer": obs.viewer,
        "numPlayers": obs.num_players,
        "firstPlayer": obs.first_player,
        "activePlayer": obs.active_player,
        "currentPlayer": obs.current_player,
        "lastTurn": obs.last_turn,
        "turn": {
            "actionsLeft": obs.turn.actions_left,
            "produced": obs.turn.produced,
            "loansRequested": obs.turn.loans_requested,
            "loansRepaid": obs.turn.loans_repaid,
            "bankCalled": obs.turn.bank_called,
            "wonAuction": obs.turn.won_auction,
        },
        "players": obs.players.iter().map(player_json).collect::<Vec<_>>(),
        "bank": {
            "containerLots": obs.bank.container_lots.iter().map(counts).collect::<Vec<_>>(),
            "cashLots": obs.bank.cash_lots,
            "auctions": auctions,
        },
        "supply": {
            "containers": counts(&obs.supply.containers),
            "factories": counts(&obs.supply.factories),
            "warehouses": obs.supply.warehouses,
        },
        "phase": phase_json(obs),
    })
}
