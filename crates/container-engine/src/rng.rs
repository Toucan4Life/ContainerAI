//! Small deterministic PRNG (xoshiro256** seeded through SplitMix64).
//!
//! Implemented in-crate so that games replay identically for a given seed,
//! independent of any external crate version.

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rng {
    s: [u64; 4],
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        let mut sm = seed;
        Self::from_state([
            splitmix64(&mut sm),
            splitmix64(&mut sm),
            splitmix64(&mut sm),
            splitmix64(&mut sm),
        ])
    }

    pub fn from_state(s: [u64; 4]) -> Self {
        assert!(s != [0; 4], "xoshiro256** state must not be all zero");
        Self { s }
    }

    pub fn next_u64(&mut self) -> u64 {
        let s = &mut self.s;
        let result = s[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = s[1] << 17;
        s[2] ^= s[0];
        s[3] ^= s[1];
        s[1] ^= s[2];
        s[0] ^= s[3];
        s[2] ^= t;
        s[3] = s[3].rotate_left(45);
        result
    }

    /// Uniform integer in `0..n` (Lemire's unbiased bounded generation).
    pub fn below(&mut self, n: u32) -> u32 {
        assert!(n > 0, "below(0) is undefined");
        let range = u64::from(n);
        let mut m = (self.next_u64() >> 32) * range;
        if (m & 0xFFFF_FFFF) < range {
            let threshold = (1u64 << 32).wrapping_sub(range) % range;
            while (m & 0xFFFF_FFFF) < threshold {
                m = (self.next_u64() >> 32) * range;
            }
        }
        (m >> 32) as u32
    }

    /// Fisher–Yates shuffle.
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = self.below(i as u32 + 1) as usize;
            items.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitmix64_matches_reference_vector() {
        let mut s = 0u64;
        assert_eq!(splitmix64(&mut s), 0xE220_A839_7B1D_CDAF);
        assert_eq!(splitmix64(&mut s), 0x6E78_9E6A_A1B9_65F4);
    }

    #[test]
    fn xoshiro_matches_reference_vector() {
        let mut rng = Rng::from_state([1, 2, 3, 4]);
        assert_eq!(rng.next_u64(), 11520);
        assert_eq!(rng.next_u64(), 0);
        assert_eq!(rng.next_u64(), 1_509_978_240);
        assert_eq!(rng.next_u64(), 1_215_971_899_390_074_240);
    }

    #[test]
    fn same_seed_gives_same_sequence() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn different_seeds_give_different_sequences() {
        let mut a = Rng::new(1);
        let mut b = Rng::new(2);
        let same = (0..16).filter(|_| a.next_u64() == b.next_u64()).count();
        assert!(same < 16);
    }

    #[test]
    fn below_stays_in_bounds_and_covers_range() {
        let mut rng = Rng::new(7);
        let mut seen = [false; 6];
        for _ in 0..1000 {
            let v = rng.below(6);
            assert!(v < 6);
            seen[v as usize] = true;
        }
        assert!(seen.iter().all(|&s| s));
    }

    #[test]
    fn below_one_is_always_zero() {
        let mut rng = Rng::new(3);
        for _ in 0..50 {
            assert_eq!(rng.below(1), 0);
        }
    }

    #[test]
    fn below_is_roughly_uniform() {
        let mut rng = Rng::new(99);
        let mut buckets = [0u32; 5];
        let n = 50_000;
        for _ in 0..n {
            buckets[rng.below(5) as usize] += 1;
        }
        for b in buckets {
            let expected = n / 5;
            assert!(
                b > expected * 9 / 10 && b < expected * 11 / 10,
                "{buckets:?}"
            );
        }
    }

    #[test]
    fn shuffle_is_a_permutation() {
        let mut rng = Rng::new(5);
        let mut v: Vec<u32> = (0..20).collect();
        rng.shuffle(&mut v);
        let mut sorted = v.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..20).collect::<Vec<_>>());
    }

    #[test]
    fn shuffle_is_deterministic_and_actually_shuffles() {
        let mut a: Vec<u32> = (0..20).collect();
        let mut b = a.clone();
        Rng::new(11).shuffle(&mut a);
        Rng::new(11).shuffle(&mut b);
        assert_eq!(a, b);
        assert_ne!(a, (0..20).collect::<Vec<_>>());
    }

    #[test]
    fn shuffle_handles_tiny_slices() {
        let mut rng = Rng::new(1);
        let mut empty: [u8; 0] = [];
        rng.shuffle(&mut empty);
        let mut one = [9u8];
        rng.shuffle(&mut one);
        assert_eq!(one, [9]);
    }
}
