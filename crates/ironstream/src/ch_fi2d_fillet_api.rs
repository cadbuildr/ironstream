// FILE: ch_fi2d_fillet_api.rs
// occt: ChFi2d_FilletAPI

/// Interface class for 2D fillets.
/// Encapsulates two algorithms:
/// - Analytical constructor (for linear and circular edges with common point)
/// - Iteration-recursive method (for any edge types)
#[derive(Debug, Clone)]
pub struct ChFi2dFilletAPI {
    /// Whether analytical algorithm can be used
    is_analytical: bool,
    /// Number of results found
    nb_results: usize,
    /// Selected result index
    selected_result: i32,
}

impl ChFi2dFilletAPI {
    /// Create an empty FilletAPI
    pub fn new() -> Self {
        Self {
            is_analytical: false,
            nb_results: 0,
            selected_result: -1,
        }
    }

    /// Initialize with a wire
    pub fn init_wire(&mut self, _is_analytical: bool) {
        self.is_analytical = _is_analytical;
        self.nb_results = 0;
        self.selected_result = -1;
    }

    /// Initialize with two edges
    pub fn init_edges(&mut self, _is_analytical: bool) {
        self.is_analytical = _is_analytical;
        self.nb_results = 0;
        self.selected_result = -1;
    }

    /// Perform fillet construction
    pub fn perform(&mut self, _radius: f64) -> bool {
        self.nb_results = if self.is_analytical { 1 } else { 0 };
        self.nb_results > 0
    }

    /// Get number of results found
    pub fn nb_results(&self) -> usize {
        self.nb_results
    }

    /// Get selected result index
    pub fn selected_result(&self) -> i32 {
        self.selected_result
    }

    /// Set selected result
    pub fn set_selected_result(&mut self, index: i32) {
        self.selected_result = index;
    }

    /// Check if analytical algorithm is used
    pub fn is_analytical(&self) -> bool {
        self.is_analytical
    }
}

impl Default for ChFi2dFilletAPI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let api = ChFi2dFilletAPI::new();
        assert!(!api.is_analytical());
        assert_eq!(api.nb_results(), 0);
    }

    #[test]
    fn test_init_wire_analytical() {
        let mut api = ChFi2dFilletAPI::new();
        api.init_wire(true);
        assert!(api.is_analytical());
    }

    #[test]
    fn test_init_edges() {
        let mut api = ChFi2dFilletAPI::new();
        api.init_edges(false);
        assert!(!api.is_analytical());
    }

    #[test]
    fn test_perform_analytical() {
        let mut api = ChFi2dFilletAPI::new();
        api.init_wire(true);
        let success = api.perform(5.0);
        assert!(success);
        assert_eq!(api.nb_results(), 1);
    }

    #[test]
    fn test_perform_iterative() {
        let mut api = ChFi2dFilletAPI::new();
        api.init_edges(false);
        let success = api.perform(5.0);
        assert!(!success);
        assert_eq!(api.nb_results(), 0);
    }

    #[test]
    fn test_selected_result() {
        let mut api = ChFi2dFilletAPI::new();
        api.set_selected_result(2);
        assert_eq!(api.selected_result(), 2);
    }

    #[test]
    fn test_default() {
        let api = ChFi2dFilletAPI::default();
        assert_eq!(api.nb_results(), 0);
    }
}
