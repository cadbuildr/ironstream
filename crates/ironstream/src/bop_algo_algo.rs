// FILE: bop_algo_algo.rs
// occt: BOPAlgo_Algo

/// Base class for Boolean operation algorithms
pub struct BopAlgoAlgo {
    error_status: i32,
    warning_status: i32,
}

impl BopAlgoAlgo {
    pub fn new() -> Self {
        BopAlgoAlgo {
            error_status: 0,
            warning_status: 0,
        }
    }

    pub fn set_error(&mut self, status: i32) {
        self.error_status = status;
    }

    pub fn error_status(&self) -> i32 {
        self.error_status
    }

    pub fn set_warning(&mut self, status: i32) {
        self.warning_status = status;
    }

    pub fn warning_status(&self) -> i32 {
        self.warning_status
    }

    pub fn has_errors(&self) -> bool {
        self.error_status != 0
    }

    pub fn has_warnings(&self) -> bool {
        self.warning_status != 0
    }

    pub fn clear_status(&mut self) {
        self.error_status = 0;
        self.warning_status = 0;
    }
}

impl Default for BopAlgoAlgo {
    fn default() -> Self {
        BopAlgoAlgo::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let algo = BopAlgoAlgo::new();
        assert_eq!(algo.error_status(), 0);
        assert_eq!(algo.warning_status(), 0);
    }

    #[test]
    fn test_error_status() {
        let mut algo = BopAlgoAlgo::new();
        algo.set_error(1);
        assert_eq!(algo.error_status(), 1);
        assert_eq!(algo.has_errors(), true);
    }

    #[test]
    fn test_warning_status() {
        let mut algo = BopAlgoAlgo::new();
        algo.set_warning(1);
        assert_eq!(algo.warning_status(), 1);
        assert_eq!(algo.has_warnings(), true);
    }

    #[test]
    fn test_clear_status() {
        let mut algo = BopAlgoAlgo::new();
        algo.set_error(1);
        algo.set_warning(1);
        algo.clear_status();
        assert_eq!(algo.error_status(), 0);
        assert_eq!(algo.warning_status(), 0);
    }
}
