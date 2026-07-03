// FILE: math_bullard_generator.rs
// occt: math_BullardGenerator

/// Random number generator based on Bullard algorithm.
pub struct BullardGenerator {
    state: u64,
}

impl BullardGenerator {
    /// Create new generator with seed.
    pub fn new(seed: u64) -> Self {
        BullardGenerator {
            state: seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407),
        }
    }

    /// Generate next value in [0, 1).
    pub fn next(&mut self) -> f64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.state >> 11) as f64 * (1.0 / 9007199254740992.0)
    }

    /// Generate next integer.
    pub fn next_int(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bullard_generator_range() {
        let mut gen = BullardGenerator::new(42);
        for _ in 0..1000 {
            let val = gen.next();
            assert!(val >= 0.0 && val < 1.0);
        }
    }

    #[test]
    fn test_bullard_generator_deterministic() {
        let mut gen1 = BullardGenerator::new(42);
        let mut gen2 = BullardGenerator::new(42);

        for _ in 0..10 {
            assert_eq!(gen1.next_int(), gen2.next_int());
        }
    }

    #[test]
    fn test_bullard_generator_different_seeds() {
        let mut gen1 = BullardGenerator::new(42);
        let mut gen2 = BullardGenerator::new(43);

        let mut diff_count = 0;
        for _ in 0..100 {
            if (gen1.next() - gen2.next()).abs() > 1e-10 {
                diff_count += 1;
            }
        }
        assert!(diff_count > 90);
    }
}
