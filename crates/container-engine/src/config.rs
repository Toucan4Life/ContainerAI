//! Game configuration and the per-player-count supply tables.

pub const MIN_PLAYERS: usize = 3;
pub const MAX_PLAYERS: usize = 5;

/// Engine actions after which a game that has not ended counts as stalled. The batch environment and
/// the arena share it, so a stall means the same in training, in Python evaluation and in the arena.
/// The longest game that ever ended in a 5,400-game audit took 1,923 actions (`docs/equilibrium_spec.md`).
pub const STALL_HORIZON: usize = 2_500;

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
}

impl GameConfig {
    pub fn new(num_players: usize) -> Self {
        Self {
            num_players,
            length: GameLength::Standard,
            seed: 0,
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
