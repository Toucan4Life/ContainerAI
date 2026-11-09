//! Buying containers from an opponent's factory or harbor district.

use crate::action::District;
use crate::color::{Color, ColorCounts};
use crate::game::Game;
use crate::phase::{Buying, Phase};
use crate::state::PlayerId;

impl Game {
    /// Containers the active player can take in from a `district` purchase:
    /// factory purchases go to the harbor, harbor purchases onto the ship.
    fn purchase_room(&self, district: District) -> u32 {
        let me = &self.state.players[self.state.active_player];
        match district {
            District::Factory => me.harbor_room(),
            District::Harbor => me.ship_room(),
        }
    }

    /// Whether the active player could buy at least one container from `seller`'s `district`.
    pub(crate) fn can_start_buying(&self, seller: PlayerId, district: District) -> bool {
        let me = self.state.active_player;
        let buyer = &self.state.players[me];
        seller < self.state.num_players
            && seller != me
            && self.purchase_room(district) > 0
            && self.state.players[seller]
                .lots(district)
                .iter()
                .enumerate()
                .any(|(lot, containers)| {
                    !containers.is_empty() && district.lot_price(lot) <= buyer.cash
                })
    }

    pub(crate) fn start_buying(&mut self, seller: PlayerId, district: District, optional: bool) {
        self.phase = Phase::Buy(Buying {
            seller,
            district,
            bought: ColorCounts::default(),
            next: 0,
            optional,
        });
    }

    pub(crate) fn can_buy(&self, buying: &Buying, lot: usize, color: Color) -> bool {
        let district = buying.district;
        let buyer = &self.state.players[self.state.active_player];
        lot < district.lot_count()
            && Buying::order_key(lot, color) >= buying.next
            && self.state.players[buying.seller].lots(district)[lot][color] > 0
            && district.lot_price(lot) <= buyer.cash
            && self.purchase_room(district) > buying.bought.total()
    }

    /// Buys one container, paying the seller. The purchase ends by itself once nothing more can be bought.
    pub(crate) fn buy(&mut self, lot: usize, color: Color) {
        let Phase::Buy(mut buying) = self.phase else {
            unreachable!("buy outside of a purchase");
        };
        let district = buying.district;
        let price = district.lot_price(lot);
        self.state.players[self.state.active_player].cash -= price;
        let seller = &mut self.state.players[buying.seller];
        seller.cash += price;
        seller.lots_mut(district)[lot][color] -= 1;
        buying.bought[color] += 1;
        buying.next = Buying::order_key(lot, color);
        self.phase = Phase::Buy(buying);

        let can_buy_more = (0..district.lot_count()).any(|lot| {
            Color::ALL
                .into_iter()
                .any(|c| self.can_buy(&buying, lot, c))
        });
        if !can_buy_more {
            self.done_buying();
        }
    }

    /// Delivers what was bought: factory containers are priced in the harbor, harbor containers are loaded.
    pub(crate) fn done_buying(&mut self) {
        let Phase::Buy(buying) = self.phase else {
            unreachable!("done_buying outside of a purchase");
        };
        match buying.district {
            District::Factory => self.begin_arrange(District::Harbor, buying.bought),
            District::Harbor => {
                let me = self.state.active_player;
                self.state.players[me].ship.add_all(&buying.bought);
                self.finish_action();
            }
        }
    }
}
