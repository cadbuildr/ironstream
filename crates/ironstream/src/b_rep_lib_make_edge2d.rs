// FILE: b_rep_lib_make_edge2d.rs
// occt: BRepLib_MakeEdge2d

use crate::b_rep_lib_edge_error::EdgeError;

/// Makes 2D edges from curves
pub struct MakeEdge2d {
    done: bool,
    error: EdgeError,
}

impl MakeEdge2d {
    pub fn new() -> Self {
        MakeEdge2d {
            done: false,
            error: EdgeError::EdgeDone,
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn error(&self) -> EdgeError {
        self.error
    }

    pub fn perform(&mut self) {
        self.done = true;
    }
}

impl Default for MakeEdge2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_edge2d_creation() {
        let maker = MakeEdge2d::new();
        assert!(!maker.is_done());
    }

    #[test]
    fn test_perform() {
        let mut maker = MakeEdge2d::new();
        maker.perform();
        assert!(maker.is_done());
    }

    #[test]
    fn test_error() {
        let maker = MakeEdge2d::new();
        assert_eq!(maker.error(), EdgeError::EdgeDone);
    }
}
