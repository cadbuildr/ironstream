// FILE: math_utils_random_o_o.rs
// occt: MathUtils_Random

/// Random number generator.
pub struct Random {
    seed: u32,
    state: u32,
}

impl Random {
    pub fn new(seed: u32) -> Self {
        Self { seed, state: seed }
    }

    pub fn next(&mut self) -> f64 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        ((self.state / 65536) % 32768) as f64 / 32768.0
    }
}

impl Default for Random {
    fn default() -> Self {
        Self::new(1)
    }
}
