// FILE: math_utils_random_o.rs
// occt-note: MathUtils::RandomGenerator

/// High-quality pseudo-random number generator based on xoshiro256**.
/// Has 256-bit state (period 2^256 - 1), passes BigCrush tests.
pub struct RandomGenerator {
    state: [u64; 4],
}

impl RandomGenerator {
    /// Initialize with a seed value.
    pub fn new(seed: u64) -> Self {
        let mut gen = RandomGenerator {
            state: [0, 0, 0, 0],
        };
        gen.set_seed(seed);
        gen
    }

    /// Re-seed the generator.
    pub fn set_seed(&mut self, seed: u64) {
        let mut s = seed;
        self.state[0] = Self::splitmix64(&mut s);
        self.state[1] = Self::splitmix64(&mut s);
        self.state[2] = Self::splitmix64(&mut s);
        self.state[3] = Self::splitmix64(&mut s);
    }

    /// Generate next 64-bit unsigned integer.
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
    fn test_random_seed() {
        let mut gen1 = RandomGenerator::new(42);
        let mut gen2 = RandomGenerator::new(42);

        for _ in 0..10 {
            assert_eq!(gen1.next_int(), gen2.next_int());
        }
    }

    #[test]
    fn test_random_different_seeds() {
        let mut gen1 = RandomGenerator::new(42);
        let mut gen2 = RandomGenerator::new(43);

        let mut same_count = 0;
        for _ in 0..100 {
            if gen1.next_int() == gen2.next_int() {
                same_count += 1;
            }
        }
        assert!(same_count < 5); // Very unlikely to be equal by chance
    }

    #[test]
    fn test_random_real() {
        let mut gen = RandomGenerator::new(1);
        for _ in 0..1000 {
            let r = gen.next_real();
            assert!(r >= 0.0 && r < 1.0);
        }
    }

    #[test]
    fn test_random_reseed() {
        let mut gen = RandomGenerator::new(42);
        let first = gen.next_int();
        gen.set_seed(42);
        let second = gen.next_int();
        assert_eq!(first, second);
    }
}
