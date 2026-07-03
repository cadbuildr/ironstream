// FILE: i_mesh_tools_model_algo.rs
// occt: IMeshTools_ModelAlgo

/// Interface class providing API for algorithms to update or modify discrete model.
pub trait ModelAlgo {
    /// Performs processing of the given model.
    /// Returns true on success, false on error.
    fn perform_internal(&self, model: &str, params: &str) -> bool;

    /// Exceptions protected processing of the model.
    fn perform(&self, model: &str, params: &str) -> bool {
        // In Rust, exceptions are not common; we just delegate to internal.
        self.perform_internal(model, params)
    }
}

/// Default implementation for testing.
pub struct DefaultModelAlgo {
    success: bool,
}

impl DefaultModelAlgo {
    /// Create algorithm that will succeed.
    pub fn new(success: bool) -> Self {
        DefaultModelAlgo { success }
    }
}

impl ModelAlgo for DefaultModelAlgo {
    fn perform_internal(&self, _model: &str, _params: &str) -> bool {
        self.success
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_algo_success() {
        let algo = DefaultModelAlgo::new(true);
        assert!(algo.perform("model", "params"));
    }

    #[test]
    fn test_default_algo_failure() {
        let algo = DefaultModelAlgo::new(false);
        assert!(!algo.perform("model", "params"));
    }

    #[test]
    fn test_perform_delegates() {
        let algo = DefaultModelAlgo::new(true);
        assert_eq!(
            algo.perform("model", "params"),
            algo.perform_internal("model", "params")
        );
    }
}
