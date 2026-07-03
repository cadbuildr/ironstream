// FILE: b_rep_algo_api_algo.rs
// occt: BRepAlgoAPI_Algo

/// Root class for BRepAlgo API algorithms.
#[derive(Clone, Debug)]
pub struct BRepAlgoApiAlgo {
    run_parallel: bool,
    fuzzy_value: f64,
}

impl BRepAlgoApiAlgo {
    /// Create an empty algo.
    pub fn new() -> Self {
        BRepAlgoApiAlgo {
            run_parallel: false,
            fuzzy_value: 1e-12,
        }
    }

    /// Get the shape (placeholder).
    pub fn shape(&self) -> Vec<u8> {
        Vec::new()
    }

    /// Clear the algorithm state.
    pub fn clear(&mut self) {
        self.run_parallel = false;
        self.fuzzy_value = 1e-12;
    }

    /// Set fuzzy value.
    pub fn set_fuzzy_value(&mut self, fuzz: f64) {
        self.fuzzy_value = fuzz;
    }

    /// Get fuzzy value.
    pub fn fuzzy_value(&self) -> f64 {
        self.fuzzy_value
    }

    /// Set parallel execution.
    pub fn set_run_parallel(&mut self, flag: bool) {
        self.run_parallel = flag;
    }

    /// Get parallel execution flag.
    pub fn run_parallel(&self) -> bool {
        self.run_parallel
    }
}

impl Default for BRepAlgoApiAlgo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let algo = BRepAlgoApiAlgo::new();
        assert!(!algo.run_parallel());
        assert!(algo.fuzzy_value() > 0.0);
    }

    #[test]
    fn test_set_fuzzy_value() {
        let mut algo = BRepAlgoApiAlgo::new();
        algo.set_fuzzy_value(0.001);
        assert_eq!(algo.fuzzy_value(), 0.001);
    }

    #[test]
    fn test_set_run_parallel() {
        let mut algo = BRepAlgoApiAlgo::new();
        algo.set_run_parallel(true);
        assert!(algo.run_parallel());
    }

    #[test]
    fn test_clear() {
        let mut algo = BRepAlgoApiAlgo::new();
        algo.set_fuzzy_value(0.1);
        algo.set_run_parallel(true);
        algo.clear();
        assert!(!algo.run_parallel());
    }
}
