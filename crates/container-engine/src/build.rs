//! The Build action: new factories and warehouses.

use crate::action::Building;
use crate::game::Game;

/// Cost of the next factory, indexed by factories already owned. The first one is free.
const FACTORY_COSTS: [u32; 4] = [0, 6, 9, 12];
/// Cost of the next warehouse, indexed by warehouses already owned. The first one is free.
const WAREHOUSE_COSTS: [u32; 5] = [0, 4, 5, 6, 7];

/// `None` once the factory track is full.
pub fn factory_build_cost(owned: u32) -> Option<u32> {
    FACTORY_COSTS.get(owned as usize).copied()
}

/// `None` once the warehouse track is full.
pub fn warehouse_build_cost(owned: u32) -> Option<u32> {
    WAREHOUSE_COSTS.get(owned as usize).copied()
}

impl Game {
    pub(crate) fn can_build(&self, building: Building) -> bool {
        let p = &self.state.players[self.state.active_player];
        match building {
            Building::Factory(color) => {
                !p.has_factory(color)
                    && self.state.supply.factories[color] > 0
                    && factory_build_cost(p.factory_count()).is_some_and(|cost| p.cash >= cost)
            }
            Building::Warehouse => {
                self.state.supply.warehouses > 0
                    && warehouse_build_cost(u32::from(p.warehouses))
                        .is_some_and(|cost| p.cash >= cost)
            }
        }
    }

    /// Pays the cost to the supply and places the building.
    pub(crate) fn build(&mut self, building: Building) {
        let player = &mut self.state.players[self.state.active_player];
        let cost = match building {
            Building::Factory(color) => {
                let cost = factory_build_cost(player.factory_count()).expect("legal build");
                player.factories[color.index()] = true;
                self.state.supply.factories[color] -= 1;
                cost
            }
            Building::Warehouse => {
                let cost = warehouse_build_cost(u32::from(player.warehouses)).expect("legal build");
                player.warehouses += 1;
                self.state.supply.warehouses -= 1;
                cost
            }
        };
        player.cash -= cost;
        self.state.cash_burned += cost;
        self.finish_action();
    }
}
