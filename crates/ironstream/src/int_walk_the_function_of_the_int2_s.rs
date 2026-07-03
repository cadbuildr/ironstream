// FILE: int_walk_the_function_of_the_int2_s.rs
// occt: IntWalk_TheFunctionOfTheInt2S

/// Function class for 2-surface intersection walking
#[derive(Clone, Debug)]
pub struct TheFunctionOfTheInt2S;

impl TheFunctionOfTheInt2S {
    /// Create new function
    pub fn new() -> Self {
        TheFunctionOfTheInt2S
    }

    /// Evaluate function at parameters
    pub fn value(&self, u1: f64, v1: f64, u2: f64, v2: f64) -> f64 {
        // Simplified: return distance-like value
        ((u1 - u2).powi(2) + (v1 - v2).powi(2)).sqrt()
    }
}

impl Default for TheFunctionOfTheInt2S {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _func = TheFunctionOfTheInt2S::new();
    }

    #[test]
    fn test_value() {
        let func = TheFunctionOfTheInt2S::new();
        let val = func.value(0.0, 0.0, 0.0, 0.0);
        assert_eq!(val, 0.0);
    }
}
