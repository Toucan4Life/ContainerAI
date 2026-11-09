use super::new_game;
use crate::color::Color::{self, Blue, Green, Red, White, Yellow};
use crate::color::ColorCounts;
use crate::config::GameLength;
use crate::scoring::{ScoringCard, final_results, score_player};
use crate::state::{PlayerState, State};

fn counts(r: u8, b: u8, y: u8, w: u8, g: u8) -> ColorCounts {
    ColorCounts([r, b, y, w, g])
}

fn player(card: Color, island: ColorCounts, cash: u32, loans: u8) -> PlayerState {
    PlayerState {
        cash,
        loans,
        scoring_card: ScoringCard { ten: card },
        scoring_area: island,
        ..PlayerState::default()
    }
}

fn state_with(players: Vec<PlayerState>) -> State {
    let mut state = new_game(players.len(), GameLength::Standard, 0).state;
    for (seat, p) in players.into_iter().enumerate() {
        state.players[seat] = p;
    }
    state
}

#[test]
fn scoring_cards_are_rotations_of_the_color_cycle() {
    // (color worth $10, two-value color, $6 color, $4 color, $2 color)
    let table = [
        (Red, Blue, Yellow, White, Green),
        (Blue, Yellow, White, Green, Red),
        (Yellow, White, Green, Red, Blue),
        (White, Green, Red, Blue, Yellow),
        (Green, Red, Blue, Yellow, White),
    ];
    for (ten, two, six, four, one) in table {
        let card = ScoringCard { ten };
        assert_eq!(card.two_value_color(), two, "{ten:?} card");
        assert_eq!(card.value_of(ten, false), 10);
        assert_eq!(card.value_of(two, true), 10);
        assert_eq!(card.value_of(two, false), 5);
        assert_eq!(card.value_of(six, true), 6);
        assert_eq!(card.value_of(six, false), 6);
        assert_eq!(card.value_of(four, false), 4);
        assert_eq!(card.value_of(one, false), 2);
    }
}

#[test]
fn there_are_five_distinct_cards_one_per_ten_dollar_color() {
    let tens: Vec<Color> = ScoringCard::ALL.iter().map(|c| c.ten).collect();
    assert_eq!(tens, Color::ALL.to_vec());
}

#[test]
fn rulebook_example_orange_scores_55() {
    // W10 card. Island W4 R1 B4 (tie W/B, discards B), 4 on ship, $8, 1 loan.
    let mut orange = player(White, counts(1, 4, 0, 4, 0), 8, 1);
    orange.ship = counts(4, 0, 0, 0, 0);
    let s = score_player(&orange);
    assert_eq!(s.discarded, Some(Blue));
    assert_eq!(s.cash, 8);
    assert_eq!(s.island, 46);
    assert_eq!(s.leftovers, 12);
    assert_eq!(s.loans, 11);
    assert_eq!(s.total, 55);
}

#[test]
fn rulebook_example_purple_scores_62() {
    // G10 card. Island G2 R1: G is most common and is discarded; R (two-value) is $5.
    let mut purple = player(Green, counts(1, 0, 0, 0, 2), 51, 0);
    purple.harbor_lots[0] = counts(0, 0, 1, 0, 0);
    purple.harbor_lots[1] = counts(0, 1, 0, 0, 0);
    purple.harbor_lots[2] = counts(0, 0, 0, 1, 0);
    let s = score_player(&purple);
    assert_eq!(s.discarded, Some(Green));
    assert_eq!(s.island, 5);
    assert_eq!(s.leftovers, 6);
    assert_eq!(s.total, 62);
}

#[test]
fn rulebook_example_blue_scores_71() {
    // B10 card. Island W2 R1 G1 Y1 B1: discards W; Y (two-value) is $10 with one of each color.
    let mut blue = player(Blue, counts(1, 1, 1, 2, 1), 36, 0);
    blue.holding = counts(0, 0, 2, 1, 0);
    let s = score_player(&blue);
    assert_eq!(s.discarded, Some(White));
    assert_eq!(s.island, 26);
    assert_eq!(s.leftovers, 9);
    assert_eq!(s.total, 71);
}

#[test]
fn rulebook_example_final_results_name_blue_the_winner() {
    let mut orange = player(White, counts(1, 4, 0, 4, 0), 8, 1);
    orange.ship = counts(4, 0, 0, 0, 0);
    let mut purple = player(Green, counts(1, 0, 0, 0, 2), 51, 0);
    purple.harbor_lots[0] = counts(0, 0, 1, 1, 1);
    let mut blue = player(Blue, counts(1, 1, 1, 2, 1), 36, 0);
    blue.holding = counts(0, 0, 2, 1, 0);

    let results = final_results(&state_with(vec![orange, purple, blue]));
    let totals: Vec<i32> = results.scores.iter().map(|s| s.total).collect();
    assert_eq!(totals, vec![55, 62, 71]);
    assert_eq!(results.winners, vec![2]);
}

#[test]
fn empty_scoring_area_scores_nothing_and_discards_nothing() {
    let s = score_player(&player(Red, ColorCounts::default(), 10, 0));
    assert_eq!(s.discarded, None);
    assert_eq!(s.island, 0);
    assert_eq!(s.total, 10);
}

#[test]
fn unique_most_common_color_is_discarded_even_if_it_is_the_ten_dollar_color() {
    let s = score_player(&player(Red, counts(3, 0, 1, 0, 0), 0, 0));
    assert_eq!(s.discarded, Some(Red));
    assert_eq!(s.island, 6, "only the yellow container (worth $6) remains");
}

#[test]
fn tie_without_the_two_value_color_discards_the_cheapest_tied_color() {
    let s = score_player(&player(Red, counts(2, 0, 0, 2, 0), 0, 0));
    assert_eq!(s.discarded, Some(White));
    assert_eq!(s.island, 20);

    let s = score_player(&player(Red, counts(0, 0, 2, 2, 2), 0, 0));
    assert_eq!(s.discarded, Some(Green));
    assert_eq!(s.island, 2 * 6 + 2 * 4);
}

#[test]
fn tie_including_the_two_value_color_forces_discarding_it() {
    // Red card: Blue is the two-value color. Discarding Green would be better, but is not allowed.
    let s = score_player(&player(Red, counts(0, 2, 0, 0, 2), 0, 0));
    assert_eq!(s.discarded, Some(Blue));
    assert_eq!(s.island, 4);

    let s = score_player(&player(Red, counts(2, 2, 0, 0, 2), 0, 0));
    assert_eq!(s.discarded, Some(Blue));
    assert_eq!(s.island, 2 * 10 + 2 * 2);
}

#[test]
fn two_value_color_is_five_without_a_full_set() {
    // Red card, island R2 B1 Y1: Red discarded, Blue is $5, Yellow is $6.
    let s = score_player(&player(Red, counts(2, 1, 1, 0, 0), 0, 0));
    assert_eq!(s.discarded, Some(Red));
    assert_eq!(s.island, 5 + 6);
}

#[test]
fn two_value_color_is_ten_with_a_full_set() {
    // Red card, island R2 B1 Y1 W1 G1: Red discarded, Blue is $10.
    let s = score_player(&player(Red, counts(2, 1, 1, 1, 1), 0, 0));
    assert_eq!(s.discarded, Some(Red));
    assert_eq!(s.island, 10 + 6 + 4 + 2);
}

#[test]
fn discarded_color_still_counts_toward_the_full_set() {
    // Red card, island R1 B1 Y1 W1 G3: Green is discarded but completes the set.
    let s = score_player(&player(Red, counts(1, 1, 1, 1, 3), 0, 0));
    assert_eq!(s.discarded, Some(Green));
    assert_eq!(s.island, 10 + 10 + 6 + 4);
}

#[test]
fn discarded_two_value_color_scores_nothing() {
    // Red card, island B3 R1: Blue (the two-value color) is the unique most common.
    let s = score_player(&player(Red, counts(1, 3, 0, 0, 0), 0, 0));
    assert_eq!(s.discarded, Some(Blue));
    assert_eq!(s.island, 10);
}

#[test]
fn leftover_containers_are_worth_by_location_not_color() {
    let mut p = player(Red, ColorCounts::default(), 0, 0);
    p.ship = counts(2, 1, 0, 0, 0);
    p.holding = counts(0, 0, 0, 2, 0);
    p.harbor_lots[0] = counts(1, 0, 0, 0, 0);
    p.harbor_lots[4] = counts(0, 0, 0, 0, 1);
    p.factory_lots[0] = counts(3, 0, 0, 0, 0);
    p.factory_lots[3] = counts(0, 2, 0, 0, 0);
    let s = score_player(&p);
    assert_eq!(s.leftovers, (3 + 2) * 3 + 2 * 2);
}

#[test]
fn each_loan_costs_eleven_and_totals_can_go_negative() {
    let s = score_player(&player(Red, ColorCounts::default(), 5, 2));
    assert_eq!(s.loans, 22);
    assert_eq!(s.total, -17);
}

#[test]
fn highest_total_wins() {
    let state = state_with(vec![
        player(Red, ColorCounts::default(), 30, 0),
        player(Red, ColorCounts::default(), 41, 0),
        player(Red, ColorCounts::default(), 12, 0),
    ]);
    assert_eq!(final_results(&state).winners, vec![1]);
}

#[test]
fn tied_total_goes_to_most_containers_left_in_the_factory_district() {
    let mut a = player(Red, ColorCounts::default(), 40, 0);
    a.factory_lots[0] = counts(1, 0, 0, 0, 0);
    let mut b = player(Red, ColorCounts::default(), 40, 0);
    b.factory_lots[2] = counts(0, 2, 1, 0, 0);
    let c = player(Red, ColorCounts::default(), 10, 0);
    assert_eq!(final_results(&state_with(vec![a, b, c])).winners, vec![1]);
}

#[test]
fn still_tied_players_share_the_victory() {
    let mut a = player(Red, ColorCounts::default(), 40, 0);
    a.factory_lots[0] = counts(1, 1, 0, 0, 0);
    let b = player(Red, ColorCounts::default(), 10, 0);
    let mut c = player(Red, ColorCounts::default(), 40, 0);
    c.factory_lots[1] = counts(0, 0, 2, 0, 0);
    assert_eq!(
        final_results(&state_with(vec![a, b, c])).winners,
        vec![0, 2]
    );
}

#[test]
fn factory_tie_break_only_applies_among_players_tied_for_first() {
    let mut loser = player(Red, ColorCounts::default(), 39, 0);
    loser.factory_lots[0] = counts(5, 0, 0, 0, 0);
    let winner = player(Red, ColorCounts::default(), 40, 0);
    let other = player(Red, ColorCounts::default(), 1, 0);
    assert_eq!(
        final_results(&state_with(vec![loser, winner, other])).winners,
        vec![1]
    );
}
