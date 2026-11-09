//! Final scoring cards and end-of-game scoring.

use crate::color::{Color, NUM_COLORS};
use crate::state::{PlayerId, PlayerState, State};

/// Cash lost per outstanding loan at the end of the game: $10 repaid plus $1 interest.
pub const LOAN_PAYOFF: u32 = 11;
const SHIP_AND_HOLDING_VALUE: u32 = 3;
const HARBOR_VALUE: u32 = 2;

/// A secret final-scoring card, identified by the color it values at $10.
///
/// All five cards are rotations of the color cycle Red → Blue → Yellow → White → Green.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScoringCard {
    pub ten: Color,
}

impl ScoringCard {
    pub const ALL: [ScoringCard; 5] = [
        ScoringCard { ten: Color::Red },
        ScoringCard { ten: Color::Blue },
        ScoringCard { ten: Color::Yellow },
        ScoringCard { ten: Color::White },
        ScoringCard { ten: Color::Green },
    ];

    /// The color whose value is $10 with a full set of colors and $5 without.
    pub fn two_value_color(self) -> Color {
        Color::from_index((self.ten.index() + 1) % NUM_COLORS)
    }

    /// Cash value of one container of `color` in the scoring area.
    ///
    /// `full_set` says whether the player collected every color (before discarding).
    pub fn value_of(self, color: Color, full_set: bool) -> u32 {
        match (color.index() + NUM_COLORS - self.ten.index()) % NUM_COLORS {
            0 => 10,
            1 if full_set => 10,
            1 => 5,
            2 => 6,
            3 => 4,
            _ => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScoreBreakdown {
    pub cash: u32,
    pub island: u32,
    pub leftovers: u32,
    /// Amount lost to outstanding loans (a positive number).
    pub loans: u32,
    pub total: i32,
    pub discarded: Option<Color>,
}

/// Scores one player's final position.
///
/// A tie for the most common color is resolved the way a player would: the two-value
/// color must go if it is tied, otherwise the cheapest tied color is discarded.
pub fn score_player(p: &PlayerState) -> ScoreBreakdown {
    let card = p.scoring_card;
    let island = &p.scoring_area;
    let full_set = island.has_all_colors();
    let most = island.max_count();

    let discarded = if most == 0 {
        None
    } else if island[card.two_value_color()] == most {
        Some(card.two_value_color())
    } else {
        Color::ALL
            .into_iter()
            .filter(|&c| island[c] == most)
            .min_by_key(|&c| card.value_of(c, full_set))
    };

    let island_value: u32 = Color::ALL
        .into_iter()
        .filter(|&c| Some(c) != discarded)
        .map(|c| u32::from(island[c]) * card.value_of(c, full_set))
        .sum();
    let leftovers = SHIP_AND_HOLDING_VALUE * (p.ship.total() + p.holding.total())
        + HARBOR_VALUE * p.harbor_stock();
    let loans = LOAN_PAYOFF * u32::from(p.loans);

    ScoreBreakdown {
        cash: p.cash,
        island: island_value,
        leftovers,
        loans,
        total: (p.cash + island_value + leftovers) as i32 - loans as i32,
        discarded,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FinalResults {
    pub scores: Vec<ScoreBreakdown>,
    /// Several players share the victory only if they tie on cash and factory containers.
    pub winners: Vec<PlayerId>,
}

pub fn final_results(state: &State) -> FinalResults {
    let scores: Vec<ScoreBreakdown> = state.players().iter().map(score_player).collect();
    let best = scores
        .iter()
        .map(|s| s.total)
        .max()
        .expect("at least one player");
    let tied: Vec<PlayerId> = (0..scores.len())
        .filter(|&i| scores[i].total == best)
        .collect();
    let most_in_factory = tied
        .iter()
        .map(|&i| state.players[i].factory_stock())
        .max()
        .expect("at least one tied player");
    let winners = tied
        .into_iter()
        .filter(|&i| state.players[i].factory_stock() == most_in_factory)
        .collect();
    FinalResults { scores, winners }
}
