//! Default: the Bank seizes containers from a player who cannot pay their interest.

use crate::color::{Color, ColorCounts};
use crate::game::Game;
use crate::phase::{Distributing, Phase, Seizing};
use crate::state::{HARBOR_LOTS, PlayerId, PlayerState};

/// Where the Bank looks for containers to seize.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Place {
    ScoringArea,
    Ship,
    Holding,
    Harbor,
    Factory,
}

impl Place {
    /// The Bank seizes from the first of these where the player has a container.
    const IN_ORDER: [Place; 5] = [
        Place::ScoringArea,
        Place::Ship,
        Place::Holding,
        Place::Harbor,
        Place::Factory,
    ];

    /// The place's containers by lot. Places that have no lots count as a single lot.
    fn lots(self, player: &PlayerState) -> &[ColorCounts] {
        match self {
            Place::ScoringArea => std::slice::from_ref(&player.scoring_area),
            Place::Ship => std::slice::from_ref(&player.ship),
            Place::Holding => std::slice::from_ref(&player.holding),
            Place::Harbor => &player.harbor_lots,
            Place::Factory => &player.factory_lots,
        }
    }

    fn lots_mut(self, player: &mut PlayerState) -> &mut [ColorCounts] {
        match self {
            Place::ScoringArea => std::slice::from_mut(&mut player.scoring_area),
            Place::Ship => std::slice::from_mut(&mut player.ship),
            Place::Holding => std::slice::from_mut(&mut player.holding),
            Place::Harbor => &mut player.harbor_lots,
            Place::Factory => &mut player.factory_lots,
        }
    }

    fn first_with_containers(player: &PlayerState) -> Option<Place> {
        Place::IN_ORDER
            .into_iter()
            .find(|place| place.lots(player).iter().any(|lot| !lot.is_empty()))
    }
}

/// The only color left in a hand, if there is just one.
fn only_color(hand: &ColorCounts) -> Option<Color> {
    let mut colors = hand.iter_nonzero().map(|(color, _)| color);
    let first = colors.next()?;
    colors.next().is_none().then_some(first)
}

impl Game {
    /// The interest could not be paid on `unpaid` loans: the Bank seizes a container for each.
    pub(crate) fn begin_seizure(&mut self, debtor: PlayerId, unpaid: u8) {
        self.phase = Phase::Seize(Seizing {
            debtor,
            remaining: unpaid,
            seized: ColorCounts::default(),
            next: 0,
        });
        self.advance_seizure();
    }

    /// A container may be taken if it is in the first place the debtor has any, comes after the
    /// previous pick, and leaves enough after it in that order to take one for every unpaid loan.
    pub(crate) fn can_seize(&self, seizing: &Seizing, lot: usize, color: Color) -> bool {
        let debtor = &self.state.players[seizing.debtor];
        let Some(place) = Place::first_with_containers(debtor) else {
            return false;
        };
        let lots = place.lots(debtor);
        if lots.get(lot).is_none_or(|tally| tally[color] == 0) {
            return false;
        }
        let key = Seizing::order_key(lot, color);
        if key < seizing.next {
            return false;
        }
        let here: u32 = lots.iter().map(ColorCounts::total).sum();
        let from_key: u32 = lots
            .iter()
            .enumerate()
            .flat_map(|(l, tally)| tally.iter_nonzero().map(move |(c, n)| (l, c, n)))
            .filter(|&(l, c, _)| Seizing::order_key(l, c) >= key)
            .map(|(_, _, n)| u32::from(n))
            .sum();
        from_key >= u32::from(seizing.remaining).min(here)
    }

    pub(crate) fn seize(&mut self, lot: usize, color: Color) {
        let Phase::Seize(mut seizing) = self.phase else {
            unreachable!("seize outside of a default");
        };
        self.take_seized(&mut seizing, lot, color);
        self.phase = Phase::Seize(seizing);
        self.advance_seizure();
    }

    fn take_seized(&mut self, seizing: &mut Seizing, lot: usize, color: Color) {
        let debtor = &mut self.state.players[seizing.debtor];
        let place = Place::first_with_containers(debtor).expect("the container was chosen from it");
        place.lots_mut(debtor)[lot][color] -= 1;
        seizing.seized[color] += 1;
        seizing.remaining -= 1;
        seizing.next = if place.lots(debtor).iter().all(ColorCounts::is_empty) {
            0
        } else {
            Seizing::order_key(lot, color)
        };
    }

    /// Takes every container that needs no choice. Stops when the player on the right has to choose,
    /// and otherwise goes on to dealing out whatever was seized.
    fn advance_seizure(&mut self) {
        loop {
            let Phase::Seize(mut seizing) = self.phase else {
                unreachable!("advancing a seizure outside of a default");
            };
            let debtor = &self.state.players[seizing.debtor];
            // Interest the Bank finds nothing to seize for is forgiven for this turn.
            if seizing.remaining == 0 || Place::first_with_containers(debtor).is_none() {
                self.begin_dealing(&seizing);
                return;
            }
            let mut picks = (0..HARBOR_LOTS)
                .flat_map(|lot| Color::ALL.into_iter().map(move |color| (lot, color)))
                .filter(|&(lot, color)| self.can_seize(&seizing, lot, color));
            match (picks.next(), picks.next()) {
                (Some((lot, color)), None) => {
                    self.take_seized(&mut seizing, lot, color);
                    self.phase = Phase::Seize(seizing);
                }
                (Some(_), Some(_)) => return,
                (None, _) => unreachable!("a place with containers has one to take"),
            }
        }
    }

    fn begin_dealing(&mut self, seizing: &Seizing) {
        self.phase = Phase::Distribute(Distributing {
            decider: self.state.player_on_right(seizing.debtor),
            hand: seizing.seized,
            dealt: 0,
            won_lot: None,
        });
        self.advance_dealing();
    }

    pub(crate) fn deal(&mut self, color: Color) {
        let Phase::Distribute(mut dealing) = self.phase else {
            unreachable!("deal outside of a default");
        };
        self.deal_into_lot(&mut dealing, color);
        self.phase = Phase::Distribute(dealing);
        self.advance_dealing();
    }

    /// Deals what needs no choice, which is whatever is left once only one color remains. Once
    /// everything is dealt, the winner of a container bid collects their cash lot.
    pub(crate) fn advance_dealing(&mut self) {
        let Phase::Distribute(mut dealing) = self.phase else {
            unreachable!("advancing the dealing outside of a Bank payment");
        };
        while let Some(color) = only_color(&dealing.hand) {
            self.deal_into_lot(&mut dealing, color);
        }
        if dealing.hand.is_empty() {
            if let Some(lot) = dealing.won_lot {
                self.collect_cash_lot(dealing.decider, lot);
            }
            self.win_auctions();
        } else {
            self.phase = Phase::Distribute(dealing);
        }
    }

    /// One container goes to the next Bank lot, starting at lot I and going on from there.
    fn deal_into_lot(&mut self, dealing: &mut Distributing, color: Color) {
        dealing.hand[color] -= 1;
        let lot = self
            .state
            .bank
            .container_lot_for(usize::from(dealing.dealt));
        self.state.bank.container_lots[lot][color] += 1;
        dealing.dealt += 1;
    }
}
