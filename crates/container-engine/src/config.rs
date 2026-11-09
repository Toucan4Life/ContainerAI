//! Game configuration and the per-player-count supply tables.

pub const MIN_PLAYERS: usize = 3;
pub const MAX_PLAYERS: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GameLength {
    Short,
    Standard,
    Extended,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigError {
    InvalidPlayerCount(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameConfig {
    pub num_players: usize,
    pub length: GameLength,
    pub seed: u64,
    /// How many times each bidder of a delivery auction may swap their bid for another after the
    /// cards are down and before they are turned over. With none (the default) every bid is placed
    /// once, hidden, and turned over; the cards of a bid are then never part of the game.
    pub max_bid_revisions: u8,
}

impl GameConfig {
    pub fn new(num_players: usize) -> Self {
        Self {
            num_players,
            length: GameLength::Standard,
            seed: 0,
            max_bid_revisions: 0,
        }
    }

    pub fn with_length(mut self, length: GameLength) -> Self {
        self.length = length;
        self
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn with_max_bid_revisions(mut self, revisions: u8) -> Self {
        self.max_bid_revisions = revisions;
        self
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if (MIN_PLAYERS..=MAX_PLAYERS).contains(&self.num_players) {
            Ok(())
        } else {
            Err(ConfigError::InvalidPlayerCount(self.num_players))
        }
    }

    /// Containers of each color in the supply at the start of the game.
    pub fn containers_per_color(&self) -> u8 {
        let by_players = match self.length {
            GameLength::Short => [9, 11, 13],
            GameLength::Standard => [11, 14, 17],
            GameLength::Extended => [12, 16, 20],
        };
        by_players[self.num_players - MIN_PLAYERS]
    }

    /// Warehouses in play, including the free starting warehouse of each player.
    pub fn total_warehouses(&self) -> u8 {
        4 * self.num_players as u8
    }

    /// Factories of each color in play, including the starting factories.
    pub fn factories_per_color(&self) -> u8 {
        self.num_players as u8 - 1
    }

    pub fn auction_tokens(&self) -> u8 {
        if self.num_players == MAX_PLAYERS {
            2
        } else {
            1
        }
    }
}
