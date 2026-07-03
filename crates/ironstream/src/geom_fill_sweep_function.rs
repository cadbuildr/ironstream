// FILE: geom_fill_sweep_function.rs
// occt: GeomFill_SweepFunction

/// Function for sweep surface generation
pub struct SweepFunction;

impl SweepFunction {
    pub fn new() -> Self {
        SweepFunction
    }
}

impl Default for SweepFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _func = SweepFunction::new();
    }

    #[test]
    fn test_default() {
        let _func = SweepFunction::default();
    }
}
