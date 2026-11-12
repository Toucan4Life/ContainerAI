//! A game in progress: the engine, who sits where, and a log of what happened.

use container_bots::{Opponent, create};
use container_engine::action::{Action, Building, District};
use container_engine::config::{GameConfig, GameLength};
use container_engine::color::{Color, ColorCounts};
use container_engine::game::Game;
use container_engine::phase::Phase;
use container_engine::state::{AuctionKind, PlayerState, ShipLocation};
use serde_json::{Value, json};

use crate::dto::{action_json, observation_json};

const MAX_LOG: usize = 300;
const LOT_NUMERALS: [&str; 5] = ["I", "II", "III", "IV", "V"];

struct Seat {
    name: String,
    kind: String,
    bot: Option<Box<dyn Opponent + Send>>,
}

pub struct Session {
    game: Game,
    seats: Vec<Seat>,
    viewer: usize,
    log: Vec<Value>,
    turns: u32,
    pending: Option<Pending>,
    /// The table as it was before each of the human's actions this turn.
    history: Vec<Undo>,
}

struct Undo {
    game: Game,
    log: Vec<Value>,
    turns: u32,
    pending: Option<Pending>,
    /// True when the action was taken from the main menu, i.e. it starts a fresh action.
    from_main: bool,
}

/// A multi-step action (produce, buy) whose log entry is completed once it has finished.
#[derive(Clone)]
struct Pending {
    entry: usize,
    before: Game,
    seat: usize,
    action: Action,
    /// (price, color) of what was bought so far.
    bought: Vec<(u32, Color)>,
}

impl Session {
    /// `config` is `{"seed": 1, "length": "standard", "players": [{"name": "You", "kind": "human"}, ...]}`.
    /// A kind is `human` or an opponent id from `container-bots`. At most one seat is human.
    pub fn new(config: &str) -> Result<Session, String> {
        let config: Value = serde_json::from_str(config).map_err(|e| e.to_string())?;
        let seed = config["seed"].as_u64().unwrap_or(0);
        let length = match config["length"].as_str().unwrap_or("standard") {
            "short" => GameLength::Short,
            "standard" => GameLength::Standard,
            "extended" => GameLength::Extended,
            other => return Err(format!("unknown game length {other}")),
        };
        let players = config["players"].as_array().ok_or("players must be a list")?;

        let mut seats = Vec::new();
        for (seat, player) in players.iter().enumerate() {
            let kind = player["kind"].as_str().unwrap_or("human").to_string();
            let name = player["name"]
                .as_str()
                .map_or_else(|| format!("Player {}", seat + 1), str::to_string);
            let bot = if kind == "human" {
                None
            } else {
                let bot_seed = seed.wrapping_mul(31).wrapping_add(seat as u64 + 1);
                Some(create(&kind, bot_seed).ok_or(format!("unknown opponent {kind}"))?)
            };
            seats.push(Seat { name, kind, bot });
        }
        if seats.iter().filter(|s| s.bot.is_none()).count() > 1 {
            return Err("only one seat can be human".into());
        }

        let game = Game::new(
            GameConfig::new(seats.len())
                .with_length(length)
                .with_seed(seed),
        )
        .map_err(|e| format!("{e:?}"))?;
        let viewer = seats.iter().position(|s| s.bot.is_none()).unwrap_or(0);
        let mut session = Session {
            game,
            seats,
            viewer,
            log: Vec::new(),
            turns: 0,
            pending: None,
            history: Vec::new(),
        };
        let first = session.game.state.first_player;
        session.say(Some(first), format!("{} goes first", session.seats[first].name), Vec::new());
        Ok(session)
    }

    fn say(&mut self, player: Option<usize>, text: String, detail: Vec<String>) {
        let round = self.turns / self.seats.len() as u32 + 1;
        self.log
            .push(json!({ "player": player, "text": text, "round": round, "detail": detail }));
        if self.log.len() > MAX_LOG {
            self.log.remove(0);
            if let Some(p) = &mut self.pending {
                p.entry = p.entry.saturating_sub(1);
            }
        }
    }

    pub fn is_over(&self) -> bool {
        self.game.is_terminal()
    }

    pub fn current_is_bot(&self) -> bool {
        !self.is_over() && self.seats[self.game.current_player()].bot.is_some()
    }

    /// Everything the UI draws, from the human seat's point of view.
    pub fn snapshot(&self) -> Value {
        let mut view = observation_json(
            &self
                .game
                .observation(self.viewer)
                .expect("the viewer sits at the table"),
        );
        let mine = !self.is_over() && self.game.current_player() == self.viewer && !self.current_is_bot();
        let legal: Vec<Value> = if mine {
            self.game
                .legal_actions()
                .into_iter()
                .enumerate()
                .map(|(id, a)| {
                    let mut json = action_json(a);
                    json["id"] = json!(id);
                    json
                })
                .collect()
        } else {
            Vec::new()
        };
        let n = self.seats.len() as u32;
        view["round"] = json!(self.turns / n + 1);
        view["seats"] = json!(
            self.seats
                .iter()
                .map(|s| json!({ "name": s.name, "kind": s.kind }))
                .collect::<Vec<_>>()
        );
        view["legal"] = json!(legal);
        view["log"] = json!(self.log);
        view["canStepBack"] = json!(
            self.history
                .last()
                .is_some_and(|u| matches!(u.game.phase, Phase::Arrange(_)))
        );
        view["canUndo"] = json!(!self.history.is_empty());
        view["gameOver"] = json!(self.is_over());
        view["results"] = self.results();
        view
    }

    fn results(&self) -> Value {
        let Some(results) = self.game.results() else {
            return Value::Null;
        };
        let scores: Vec<Value> = results
            .scores
            .iter()
            .map(|s| {
                json!({
                    "cash": s.cash,
                    "island": s.island,
                    "leftovers": s.leftovers,
                    "loans": s.loans,
                    "total": s.total,
                    "discarded": s.discarded.map(|c| c.index()),
                })
            })
            .collect();
        json!({ "scores": scores, "winners": results.winners })
    }

    /// The human takes the `id`th action of the list the last snapshot offered.
    pub fn act(&mut self, id: usize) -> Result<(), String> {
        if self.current_is_bot() || self.is_over() {
            return Err("it is not your turn".into());
        }
        let seat = self.game.current_player();
        let action = *self
            .game
            .legal_actions()
            .get(id)
            .ok_or_else(|| format!("no legal action {id}"))?;
        self.history.push(Undo {
            game: self.game.clone(),
            log: self.log.clone(),
            turns: self.turns,
            pending: self.pending.clone(),
            from_main: matches!(self.game.phase, Phase::Main),
        });
        if let Err(e) = self.play(seat, action) {
            self.history.pop();
            return Err(e);
        }
        if self.is_over() {
            self.history.clear();
        }
        Ok(())
    }

    /// Takes back just the last single step (a container placement).
    pub fn step_back(&mut self) -> Result<(), String> {
        let u = self.history.pop().ok_or("nothing to undo")?;
        self.game = u.game;
        self.log = u.log;
        self.turns = u.turns;
        self.pending = u.pending;
        Ok(())
    }

    /// Takes back the human's last action this turn, or cancels the one in progress.
    pub fn undo(&mut self) -> Result<(), String> {
        while let Some(u) = self.history.pop() {
            let from_main = u.from_main;
            self.game = u.game;
            self.log = u.log;
            self.turns = u.turns;
            self.pending = u.pending;
            if from_main {
                return Ok(());
            }
        }
        Err("nothing to undo".into())
    }

    /// The computer opponent whose turn it is takes one action. Returns false when it is not a bot's turn.
    pub fn step(&mut self) -> Result<bool, String> {
        if !self.current_is_bot() {
            return Ok(false);
        }
        self.history.clear();
        let seat = self.game.current_player();
        let obs = self
            .game
            .observation(seat)
            .expect("the current player sits at the table");
        let legal = self.game.legal_actions();
        let bot = self.seats[seat].bot.as_mut().expect("checked above");
        let action = bot.choose(&obs, &legal);
        self.play(seat, action)?;
        Ok(true)
    }

    fn play(&mut self, seat: usize, action: Action) -> Result<(), String> {
        let before = self.game.clone();
        self.game
            .apply(action)
            .map_err(|e| format!("illegal action: {e:?}"))?;
        if let (Some(p), Phase::Buy(b), Action::Buy { lot, color }) =
            (&mut self.pending, before.phase, action)
        {
            p.bought.push((b.district.lot_price(lot), color));
        }
        if let Some(text) = self.describe(&before, seat, action) {
            let detail = self.details(&before, seat, action);
            self.say(Some(seat), text, detail);
            let starts = matches!(
                (before.phase, action),
                (Phase::Main, Action::Produce | Action::FactoryPurchase(_) | Action::HarborPurchase)
            );
            if starts {
                self.pending = Some(Pending {
                    entry: self.log.len() - 1,
                    before: before.clone(),
                    seat,
                    action,
                    bought: Vec::new(),
                });
            }
        }
        self.finish_pending();
        if self.is_over() {
            self.say(None, "The game is over".into(), Vec::new());
        } else if self.game.state.active_player != before.state.active_player {
            self.turns += 1;
        }
        self.log_won_auctions(&before);
        Ok(())
    }

    /// An auction is won when its holder's turn begins and the interest is settled, with no action of
    /// their own. Without an entry the lot would just appear in their holding area or hand.
    fn log_won_auctions(&mut self, before: &Game) {
        let winner = self.game.state.active_player;
        if !self.game.turn.won_auction {
            return;
        }
        for kind in AuctionKind::ALL {
            let Some(auction) = before.state.bank.auctions[kind].filter(|a| a.holder == winner) else {
                continue;
            };
            if self.game.state.bank.auctions[kind].is_some() {
                continue;
            }
            let who = self.name(winner).to_string();
            let lot = LOT_NUMERALS.get(auction.lot).copied().unwrap_or("?");
            let text = match kind {
                AuctionKind::ContainerLot => {
                    let won = before.state.bank.container_lots[auction.lot];
                    let icons: String = won.iter_nonzero().map(|(c, n)| icons(c, usize::from(n))).collect();
                    format!("{who} wins the container auction on lot {lot} for ${}: {icons}", auction.bid.value())
                }
                AuctionKind::CashLot => format!(
                    "{who} wins the cash auction on lot {lot} for {} containers and receives ${}",
                    auction.bid.value(),
                    before.state.bank.cash_lots[auction.lot],
                ),
            };
            self.say(Some(winner), text, Vec::new());
        }
    }

    /// Once a produce or purchase is over, rewrite its log entry with what was produced or bought.
    fn finish_pending(&mut self) {
        let unfinished = matches!(
            self.game.phase,
            Phase::Produce(_) | Phase::Buy(_) | Phase::Load(_) | Phase::Arrange(_)
        );
        if unfinished || self.pending.as_ref().is_none_or(|p| p.entry >= self.log.len()) {
            return;
        }
        let Some(p) = self.pending.take() else { return };
        let list = |counts: Vec<(u32, Color)>| -> String {
            Color::ALL
                .into_iter()
                .filter_map(|c| {
                    let n = counts.iter().filter(|(_, x)| *x == c).count();
                    (n > 0).then(|| icons(c, n))
                })
                .collect::<Vec<_>>()
                .join(" ")
        };
        let who = self.name(p.seat).to_string();
        let mut detail = self.details(&p.before, p.seat, p.action);
        let text = match p.action {
            Action::Produce => {
                let made: Vec<(u32, Color)> = Color::ALL
                    .into_iter()
                    .flat_map(|c| {
                        let n = p.before.state.supply.containers[c]
                            .saturating_sub(self.game.state.supply.containers[c]);
                        std::iter::repeat((0, c)).take(n as usize)
                    })
                    .collect();
                if made.is_empty() {
                    format!("{who} produces nothing")
                } else {
                    format!("{who} produces {}", list(made))
                }
            }
            Action::FactoryPurchase(_) | Action::HarborPurchase if !p.bought.is_empty() => {
                let total: u32 = p.bought.iter().map(|(price, _)| price).sum();
                let from = match (p.action, p.before.state.players[p.seat].ship_location) {
                    (Action::FactoryPurchase(seller), _) => {
                        if seller == self.viewer {
                            "your factories".into()
                        } else {
                            format!("{}'s factories", self.name(seller))
                        }
                    }
                    (_, ShipLocation::Harbor(owner)) if owner == self.viewer => "your harbor".into(),
                    (_, ShipLocation::Harbor(owner)) => format!("{}'s harbor", self.name(owner)),
                    _ => "the harbor".into(),
                };
                let mut priced = p.bought.clone();
                priced.sort_by_key(|&(price, c)| (price, c.index()));
                let each: Vec<String> = priced
                    .iter()
                    .map(|&(price, c)| format!("{} ${price}", icons(c, 1)))
                    .collect();
                let (seller, to) = match p.action {
                    Action::FactoryPurchase(seller) => (seller, format!("{who}'s harbor")),
                    _ => match p.before.state.players[p.seat].ship_location {
                        ShipLocation::Harbor(owner) => (owner, format!("{who}'s ship")),
                        _ => (p.seat, format!("{who}'s ship")),
                    },
                };
                let mut lines = vec![
                    format!("Bought: {}", each.join("  ")),
                    format!("Paid ${total} to {}", self.name(seller)),
                    format!("Moved to {to}"),
                ];
                lines.extend(detail.iter().filter(|l| l.starts_with("Your cash")).cloned());
                detail = lines;
                format!("{who} buys {} from {from} for ${total}", list(p.bought.clone()))
            }
            _ => return,
        };
        self.log[p.entry]["text"] = json!(text);
        self.log[p.entry]["detail"] = json!(detail);
    }

    /// Facts for the log tooltip, all of which are public at the table: what moved where, the sale
    /// amounts, and the viewer's own cash.
    fn details(&self, before: &Game, seat: usize, action: Action) -> Vec<String> {
        let after = &self.game;
        let mut lines = Vec::new();
        match (before.phase, action) {
            (Phase::Decide(d), Action::Accept(winner)) => {
                lines.push(format!(
                    "{} pays ${}; the Bank adds ${}",
                    self.name(winner),
                    d.top,
                    d.top
                ));
            }
            (Phase::Decide(d), Action::BuyOut) => {
                lines.push(format!("${} goes to the Bank", d.top));
            }
            (Phase::Main, Action::Build(_)) if seat != self.viewer => {
                lines.push("Paid to the supply".into());
            }
            _ => {}
        }

        let areas: [(&str, fn(&PlayerState) -> ColorCounts); 5] = [
            ("factory district", |p| sum(p.factory_lots.iter())),
            ("harbor district", |p| sum(p.harbor_lots.iter())),
            ("ship", |p| p.ship),
            ("holding area", |p| p.holding),
            ("Island", |p| p.scoring_area),
        ];
        // Mid-way through producing or pricing, containers sit in the player's hand, not on the board.
        let settled = !matches!(after.phase, Phase::Arrange(_) | Phase::Produce(_));
        let seats = if settled { self.seats.len() } else { 0 };
        for (player, (b, a)) in before.state.players[..seats]
            .iter()
            .zip(&after.state.players[..seats])
            .enumerate()
        {
            for (area, count) in &areas {
                let (old, new) = (count(b), count(a));
                let change = |sign: &str, cond: fn(u8, u8) -> bool| {
                    let parts: Vec<String> = Color::ALL
                        .into_iter()
                        .filter(|&c| cond(new[c], old[c]))
                        .map(|c| {
                            let n = new[c].abs_diff(old[c]);
                            format!("{sign}{}", icons(c, n as usize))
                        })
                        .collect();
                    parts.join(" ")
                };
                let gained = change("+", |n, o| n > o);
                let lost = change("-", |n, o| n < o);
                for text in [gained, lost].into_iter().filter(|t| !t.is_empty()) {
                    lines.push(format!("{} · {area}: {text}", self.name(player)));
                }
            }
        }
        let (old, new) = (
            before.state.players[self.viewer].cash,
            after.state.players[self.viewer].cash,
        );
        if old != new {
            lines.push(format!("Your cash: ${old} → ${new}"));
        }
        lines
    }

    fn name(&self, seat: usize) -> &str {
        &self.seats[seat].name
    }

    /// A log line for the actions that matter to the table; the many small picks are left out.
    fn describe(&self, before: &Game, seat: usize, action: Action) -> Option<String> {
        let who = self.name(seat);
        let lot_name = |lot: usize| LOT_NUMERALS.get(lot).copied().unwrap_or("?");
        let kind_name = |k: AuctionKind| match k {
            AuctionKind::ContainerLot => "container",
            AuctionKind::CashLot => "cash",
        };
        let text = match (before.phase, action) {
            (Phase::Main, Action::Produce) => format!("{who} produces"),
            (Phase::Main, Action::Build(Building::Factory(c))) => {
                format!("{who} builds a {} factory", icons(c, 1))
            }
            (Phase::Main, Action::Build(Building::Warehouse)) => {
                format!("{who} builds a warehouse")
            }
            (Phase::Main, Action::FactoryPurchase(seller)) => {
                format!("{who} buys from {}'s factories", self.name(seller))
            }
            (Phase::Main, Action::HarborPurchase) => match before.state.players[seat].ship_location
            {
                ShipLocation::Harbor(owner) => {
                    format!("{who} buys from {}'s harbor", self.name(owner))
                }
                _ => format!("{who} loads the ship from the holding area"),
            },
            (Phase::Main, Action::Sail(to)) => match to {
                ShipLocation::Ocean => format!("{who} sails out to the ocean"),
                ShipLocation::Island => format!("{who} sails to the Island"),
                ShipLocation::Bank => format!("{who} sails to the Bank"),
                ShipLocation::Harbor(owner) => {
                    format!("{who} sails to {}'s harbor", self.name(owner))
                }
            },
            (Phase::Main, Action::Reprice(d)) => format!(
                "{who} reprices the {} district",
                if d == District::Factory { "factory" } else { "harbor" }
            ),
            (_, Action::RequestLoan) => format!("{who} takes a $10 loan"),
            (_, Action::RepayLoan) => format!("{who} repays a loan"),
            (Phase::Main, Action::StartAuction { kind, lot }) => format!(
                "{who} opens the {} auction on lot {}",
                kind_name(kind),
                lot_name(lot)
            ),
            (Phase::Main, Action::Outbid(kind)) => {
                format!("{who} outbids the {} auction", kind_name(kind))
            }
            (Phase::Bid(_), Action::Bid(_)) => format!("{who} places a hidden bid"),
            (Phase::Decide(d), Action::Accept(winner)) => format!(
                "{who} sells the cargo to {} for ${} and receives ${} with the Bank's match",
                self.name(winner),
                d.top,
                2 * d.top
            ),
            (Phase::Decide(d), Action::BuyOut) => {
                format!("{who} buys out their own cargo for ${}", d.top)
            }
            (Phase::BankBid(_), Action::BidCash(amount)) => format!("{who} bids ${amount}"),
            (Phase::BankBid(_), Action::DoneBidding) => format!("{who} bids containers"),
            _ => return None,
        };
        Some(text)
    }
}

/// `{c<index>}` markers that the frontend draws as container icons, `n` of them.
fn icons(color: Color, n: usize) -> String {
    format!("{{c{}}}", color.index()).repeat(n)
}

/// The containers of every lot added together.
fn sum<'a>(lots: impl Iterator<Item = &'a ColorCounts>) -> ColorCounts {
    let mut all = ColorCounts::default();
    for lot in lots {
        all.add_all(lot);
    }
    all
}
