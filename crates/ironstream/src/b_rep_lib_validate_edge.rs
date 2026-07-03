// FILE: b_rep_lib_validate_edge.rs
// occt: BRepLib_ValidateEdge

/// Computes the max distance between 3D curve and curve on surface.
#[derive(Debug, Clone)]
pub struct BRepLibValidateEdge {
    is_exact_method: bool,
    is_multi_thread: bool,
    control_points_number: i32,
    tolerance_for_checking: f64,
    calculated_distance: f64,
    exit_if_tolerance_exceeded: bool,
    is_done: bool,
}

impl BRepLibValidateEdge {
    pub fn new() -> Self {
        BRepLibValidateEdge {
            is_exact_method: false,
            is_multi_thread: false,
            control_points_number: 22,
            tolerance_for_checking: f64::MAX,
            calculated_distance: 0.0,
            exit_if_tolerance_exceeded: false,
            is_done: false,
        }
    }

    pub fn set_exact_method(&mut self, is_exact: bool) {
        self.is_exact_method = is_exact;
    }

    pub fn is_exact_method(&self) -> bool {
        self.is_exact_method
    }

    pub fn set_parallel(&mut self, is_multi_thread: bool) {
        self.is_multi_thread = is_multi_thread;
    }

    pub fn is_parallel(&self) -> bool {
        self.is_multi_thread
    }

    pub fn set_control_points_number(&mut self, num: i32) {
        self.control_points_number = num;
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn get_max_distance(&self) -> f64 {
        self.calculated_distance
    }
}

impl Default for BRepLibValidateEdge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_edge_creation() {
        let ve = BRepLibValidateEdge::new();
        assert!(!ve.is_exact_method());
        assert_eq!(ve.control_points_number, 22);
        assert!(!ve.is_done());
    }

    #[test]
    fn test_validate_edge_setters() {
        let mut ve = BRepLibValidateEdge::new();
        ve.set_exact_method(true);
        ve.set_parallel(true);
        assert!(ve.is_exact_method());
        assert!(ve.is_parallel());
    }
}
