// FILE: math_utils_random.rs
//
// High-quality pseudo-random number generator based on xoshiro256**.
// Faithful port of OCCT's MathUtils::RandomGenerator.

// occt-note: MathUtils::RandomGenerator
/// High-quality pseudo-random number generator based on xoshiro256**.
///
/// xoshiro256** is a general-purpose PRNG designed by David Blackman
/// and Sebastiano Vigna. It has:
/// - 256-bit state (period 2^256 - 1)
/// - Passes all BigCrush statistical tests
/// - Very fast on 64-bit hardware
/// - Equidistributed to 4 dimensions
///
/// Suitable for Monte Carlo methods, stochastic optimization,
/// and any application requiring high-quality randomness.
pub struct RandomGenerator {
    state: [u64; 4],
}

impl RandomGenerator {
    /// Initialize with a seed value.
    /// Uses SplitMix64 to expand a single seed into the full 256-bit state,
    /// ensuring good initialization even from poor seeds.
    pub fn new(seed: u64) -> Self {
        let mut gen = RandomGenerator {
            state: [0; 4],
        };
        gen.set_seed(seed);
        gen
    }

    /// Re-seed the generator.
    pub fn set_seed(&mut self, seed: u64) {
        // Use SplitMix64 to initialize the state from a single seed.
        // This ensures good state initialization even from sequential seeds.
        let mut seed_state = seed;
        self.state[0] = Self::splitmix64(&mut seed_state);
        self.state[1] = Self::splitmix64(&mut seed_state);
        self.state[2] = Self::splitmix64(&mut seed_state);
        self.state[3] = Self::splitmix64(&mut seed_state);
    }

    /// Generate next 64-bit unsigned integer.
    /// Returns pseudo-random value in [0, 2^64)
    pub fn next_int(&mut self) -> u64 {
        let result = Self::rotl(self.state[1].wrapping_mul(5), 7).wrapping_mul(9);
        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = Self::rotl(self.state[3], 45);

        result
    }

    /// Generate next double in [0, 1).
    /// Uses 53 bits of randomness for full double precision.
    pub fn next_real(&mut self) -> f64 {
        (self.next_int() >> 11) as f64 * (1.0 / 9007199254740992.0)
    }

    /// 64-bit left rotation.
    #[inline]
    fn rotl(x: u64, k: u32) -> u64 {
        (x << k) | (x >> (64 - k))
    }

    /// SplitMix64 step for seed expansion.
    /// Takes a reference to the seed state and advances it.
    fn splitmix64(state: &mut u64) -> u64 {
        *state = state.wrapping_add(0x9e3779b97f4a7c15u64);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9u64);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111ebu64);
        z ^ (z >> 31)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_generator_initialization() {
        let gen = RandomGenerator::new(42);
        assert_ne!(gen.state[0], 0, "seed expansion should produce non-zero state");
        assert_ne!(gen.state[1], 0);
        assert_ne!(gen.state[2], 0);
        assert_ne!(gen.state[3], 0);
    }

    #[test]
    fn test_random_generator_deterministic() {
        let mut gen1 = RandomGenerator::new(12345);
        let mut gen2 = RandomGenerator::new(12345);

        for _ in 0..100 {
            assert_eq!(
                gen1.next_int(),
                gen2.next_int(),
                "same seed should produce same sequence"
            );
        }
    }

    #[test]
    fn test_random_generator_different_seeds() {
        let mut gen1 = RandomGenerator::new(123);
        let mut gen2 = RandomGenerator::new(456);

        let mut same_count = 0;
        for _ in 0..100 {
            if gen1.next_int() == gen2.next_int() {
                same_count += 1;
            }
        }
        // With different seeds, we should almost never get the same value
        assert!(same_count < 5, "different seeds should produce different sequences");
    }

    #[test]
    fn test_next_real_range() {
        let mut gen = RandomGenerator::new(999);
        for _ in 0..1000 {
            let val = gen.next_real();
            assert!(val >= 0.0 && val < 1.0, "next_real should return value in [0, 1)");
        }
    }

    #[test]
    fn test_next_real_distribution() {
        let mut gen = RandomGenerator::new(777);
        let mut sum = 0.0;
        let n = 10000;
        for _ in 0..n {
            sum += gen.next_real();
        }
        let mean = sum / n as f64;
        // Mean of uniform [0, 1) should be ~0.5
        assert!(
            mean > 0.48 && mean < 0.52,
            "mean of 10000 uniform samples should be close to 0.5, got {}",
            mean
        );
    }

    #[test]
    fn test_reseed() {
        let mut gen = RandomGenerator::new(100);
        let val1 = gen.next_int();

        gen.set_seed(100);
        let val2 = gen.next_int();

        assert_eq!(val1, val2, "reseeding with same value should restart sequence");
    }
}
