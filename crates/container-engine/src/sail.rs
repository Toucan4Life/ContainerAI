//! Sailing the ship, and the anchor actions at the Off-Shore Bank and at opponents' harbors.

use crate::action::District;
use crate::color::Color;
use crate::game::Game;
use crate::phase::{Loading, Phase};
use crate::state::ShipLocation;

impl Game {
    pub(crate) fn can_sail(&self, destination: ShipLocation) -> bool {
        let me = self.state.active_player;
        let at = self.state.players[me].ship_location;
        let from_ocean = at == ShipLocation::Ocean;
        match destination {
            ShipLocation::Ocean => !from_ocean,
            ShipLocation::Bank => from_ocean,
            ShipLocation::Harbor(owner) => {
                from_ocean && owner != me && owner < self.state.num_players
            }
            // There is no point delivering an empty ship.
            ShipLocation::Island => from_ocean && !self.state.players[me].ship.is_empty(),
        }
    }

    /// Moves the ship, then offers the free anchor action of the place it arrives at.
    pub(crate) fn sail(&mut self, destination: ShipLocation) {
        let me = self.state.active_player;
        self.state.players[me].ship_location = destination;
        match destination {
            ShipLocation::Harbor(owner) if self.can_start_buying(owner, District::Harbor) => {
                self.start_buying(owner, District::Harbor, true);
            }
            ShipLocation::Bank if self.can_load() => self.start_loading(true),
            ShipLocation::Island => self.start_delivery(),
            _ => self.finish_action(),
        }
    }

    /// The paid Harbor Purchase action: buy at the harbor the ship is docked at,
    /// or load from the holding area when docked at the Bank.
    pub(crate) fn can_harbor_purchase(&self) -> bool {
        match self.state.players[self.state.active_player].ship_location {
            ShipLocation::Harbor(owner) => self.can_start_buying(owner, District::Harbor),
            ShipLocation::Bank => self.can_load(),
            ShipLocation::Ocean | ShipLocation::Island => false,
        }
    }

    pub(crate) fn harbor_purchase(&mut self) {
        match self.state.players[self.state.active_player].ship_location {
            ShipLocation::Harbor(owner) => self.start_buying(owner, District::Harbor, false),
            ShipLocation::Bank => self.start_loading(false),
            ShipLocation::Ocean | ShipLocation::Island => {
                unreachable!("harbor purchase while not docked at a harbor or the Bank")
            }
        }
    }

    /// Whether there is anything to load and room for it.
    fn can_load(&self) -> bool {
        let me = &self.state.players[self.state.active_player];
        !me.holding.is_empty() && me.ship_room() > 0
    }

    fn start_loading(&mut self, optional: bool) {
        self.phase = Phase::Load(Loading {
            next_color: 0,
            loaded: 0,
            optional,
        });
    }

    pub(crate) fn can_load_color(&self, loading: &Loading, color: Color) -> bool {
        let me = &self.state.players[self.state.active_player];
        color.index() >= usize::from(loading.next_color)
            && me.holding[color] > 0
            && me.ship_room() > 0
    }

    /// Loads one container for free. Loading ends by itself once nothing more can be loaded.
    pub(crate) fn load(&mut self, color: Color) {
        let Phase::Load(mut loading) = self.phase else {
            unreachable!("load outside of loading");
        };
        let me = &mut self.state.players[self.state.active_player];
        me.holding[color] -= 1;
        me.ship[color] += 1;
        loading.loaded += 1;
        loading.next_color = color.index() as u8;
        self.phase = Phase::Load(loading);

        if !Color::ALL
            .into_iter()
            .any(|c| self.can_load_color(&loading, c))
        {
            self.finish_action();
        }
    }
}
