//! Container colors and per-color tallies.

use std::ops::{Index, IndexMut};

pub const NUM_COLORS: usize = 5;

/// The five container colors, in the cyclic order used by the scoring cards.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Color {
    Red,
    Blue,
    Yellow,
    White,
    Green,
}

impl Color {
    pub const ALL: [Color; NUM_COLORS] = [
        Color::Red,
        Color::Blue,
        Color::Yellow,
        Color::White,
        Color::Green,
    ];

    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn from_index(index: usize) -> Color {
        match index {
            0 => Color::Red,
            1 => Color::Blue,
            2 => Color::Yellow,
            3 => Color::White,
            4 => Color::Green,
            _ => panic!("color index out of range"),
        }
    }
}

/// A tally of containers per color. Used for lots, districts, supplies and hands.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ColorCounts(pub [u8; NUM_COLORS]);

impl ColorCounts {
    pub fn total(&self) -> u32 {
        self.0.iter().map(|&n| u32::from(n)).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|&n| n == 0)
    }

    pub fn max_count(&self) -> u8 {
        self.0.iter().copied().max().unwrap_or(0)
    }

    /// The first color (in cycle order) with at least one container.
    pub fn first_color(&self) -> Option<Color> {
        Color::ALL.into_iter().find(|&c| self[c] > 0)
    }

    pub fn has_all_colors(&self) -> bool {
        self.0.iter().all(|&n| n > 0)
    }

    /// Removes `n` containers of `color` if enough are present; otherwise leaves the tally untouched.
    pub fn try_remove(&mut self, color: Color, n: u8) -> bool {
        match self[color].checked_sub(n) {
            Some(rest) => {
                self[color] = rest;
                true
            }
            None => false,
        }
    }

    pub fn iter_nonzero(&self) -> impl Iterator<Item = (Color, u8)> + '_ {
        Color::ALL
            .into_iter()
            .map(|c| (c, self[c]))
            .filter(|&(_, n)| n > 0)
    }

    pub fn add_all(&mut self, other: &ColorCounts) {
        for c in Color::ALL {
            self[c] += other[c];
        }
    }

    /// Subtracts `other`, panicking if it holds more than `self` (an engine bug).
    pub fn sub_all(&mut self, other: &ColorCounts) {
        for c in Color::ALL {
            self[c] = self[c]
                .checked_sub(other[c])
                .expect("sub_all underflow: removing containers that are not there");
        }
    }
}

impl Index<Color> for ColorCounts {
    type Output = u8;

    fn index(&self, color: Color) -> &u8 {
        &self.0[color.index()]
    }
}

impl IndexMut<Color> for ColorCounts {
    fn index_mut(&mut self, color: Color) -> &mut u8 {
        &mut self.0[color.index()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_follow_scoring_card_cycle_order() {
        assert_eq!(
            Color::ALL,
            [
                Color::Red,
                Color::Blue,
                Color::Yellow,
                Color::White,
                Color::Green
            ]
        );
    }

    #[test]
    fn index_and_from_index_round_trip() {
        for (i, c) in Color::ALL.iter().enumerate() {
            assert_eq!(c.index(), i);
            assert_eq!(Color::from_index(i), *c);
        }
    }

    #[test]
    fn counts_start_empty() {
        let c = ColorCounts::default();
        assert!(c.is_empty());
        assert_eq!(c.total(), 0);
        assert_eq!(c.first_color(), None);
        assert!(!c.has_all_colors());
    }

    #[test]
    fn counts_index_add_and_total() {
        let mut c = ColorCounts::default();
        c[Color::Red] += 2;
        c[Color::Green] += 1;
        assert_eq!(c[Color::Red], 2);
        assert_eq!(c[Color::Blue], 0);
        assert_eq!(c.total(), 3);
        assert!(!c.is_empty());
    }

    #[test]
    fn try_remove_only_succeeds_when_enough() {
        let mut c = ColorCounts::default();
        c[Color::Blue] = 2;
        assert!(!c.try_remove(Color::Blue, 3));
        assert_eq!(c[Color::Blue], 2, "failed removal must not modify");
        assert!(c.try_remove(Color::Blue, 2));
        assert_eq!(c[Color::Blue], 0);
    }

    #[test]
    fn first_color_is_lowest_index_present() {
        let mut c = ColorCounts::default();
        c[Color::White] = 1;
        c[Color::Yellow] = 4;
        assert_eq!(c.first_color(), Some(Color::Yellow));
    }

    #[test]
    fn has_all_colors_requires_one_of_each() {
        let mut c = ColorCounts::default();
        for color in Color::ALL {
            assert!(!c.has_all_colors());
            c[color] = 1;
        }
        assert!(c.has_all_colors());
    }

    #[test]
    fn iter_nonzero_lists_present_colors_in_order() {
        let mut c = ColorCounts::default();
        c[Color::Green] = 1;
        c[Color::Red] = 3;
        let v: Vec<_> = c.iter_nonzero().collect();
        assert_eq!(v, vec![(Color::Red, 3), (Color::Green, 1)]);
    }

    #[test]
    fn add_all_and_sub_all_combine_tallies() {
        let mut a = ColorCounts::default();
        a[Color::Red] = 1;
        let mut b = ColorCounts::default();
        b[Color::Red] = 2;
        b[Color::Blue] = 1;
        a.add_all(&b);
        assert_eq!(a[Color::Red], 3);
        assert_eq!(a[Color::Blue], 1);
        a.sub_all(&b);
        assert_eq!(a[Color::Red], 1);
        assert_eq!(a[Color::Blue], 0);
    }

    #[test]
    fn max_count_returns_largest_tally() {
        let mut c = ColorCounts::default();
        assert_eq!(c.max_count(), 0);
        c[Color::Blue] = 4;
        c[Color::Red] = 2;
        assert_eq!(c.max_count(), 4);
    }
}
