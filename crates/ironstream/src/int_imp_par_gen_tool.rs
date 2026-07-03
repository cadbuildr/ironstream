// FILE: int_imp_par_gen_tool.rs
// occt: IntImpParGen_Tool

use crate::int_imp_par_gen::{Position, IntImpParGen};

/// Tool functions for implicit/parametric curve intersection.
/// Provides helper functions for domain normalization and positioning.
pub struct IntImpParGenTool;

impl IntImpParGenTool {
    /// Normalizes a parameter to lie within the domain.
    pub fn normalize_on_domain(param: f64, dom_first: f64, dom_last: f64) -> f64 {
        IntImpParGen::normalize_on_domain(param, dom_first, dom_last)
    }

    /// Determines the position of a point relative to a domain.
    pub fn determine_position(
        pnt: (f64, f64),
        dom_first: f64,
        dom_last: f64,
        tol: f64,
    ) -> Position {
        IntImpParGen::determine_position(pnt, dom_first, dom_last, tol)
    }

    /// Determines transition at an intersection point with normals.
    pub fn determine_transition(
        pos1: Position,
        mut tan1: (f64, f64),
        norm1: (f64, f64),
        pos2: Position,
        mut tan2: (f64, f64),
        norm2: (f64, f64),
        tol: f64,
    ) -> (TransitionInfo, TransitionInfo) {
        let (trans1, trans2) =
            IntImpParGen::determine_transition_with_normals(pos1, tan1, norm1, pos2, tan2, norm2, tol);

        let info1 = TransitionInfo {
            position: pos1,
            transition_type: trans1.trans_type as i32,
            is_tangent: trans1.is_tangent,
        };
        let info2 = TransitionInfo {
            position: pos2,
            transition_type: trans2.trans_type as i32,
            is_tangent: trans2.is_tangent,
        };

        (info1, info2)
    }
}

/// Represents transition information at an intersection.
#[derive(Clone, Debug)]
pub struct TransitionInfo {
    pub position: Position,
    pub transition_type: i32,
    pub is_tangent: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_inside() {
        let param = IntImpParGenTool::normalize_on_domain(5.0, 0.0, 10.0);
        assert_eq!(param, 5.0);
    }

    #[test]
    fn test_normalize_below() {
        let param = IntImpParGenTool::normalize_on_domain(-5.0, 0.0, 10.0);
        assert_eq!(param, 0.0);
    }

    #[test]
    fn test_normalize_above() {
        let param = IntImpParGenTool::normalize_on_domain(15.0, 0.0, 10.0);
        assert_eq!(param, 10.0);
    }

    #[test]
    fn test_determine_position_inside() {
        let pos = IntImpParGenTool::determine_position((5.0, 0.0), 0.0, 10.0, 1e-7);
        assert_eq!(pos, Position::Inside);
    }

    #[test]
    fn test_determine_position_first() {
        let pos = IntImpParGenTool::determine_position((0.0, 0.0), 0.0, 10.0, 1e-7);
        assert_eq!(pos, Position::OnFirstBoundary);
    }

    #[test]
    fn test_determine_position_last() {
        let pos = IntImpParGenTool::determine_position((10.0, 0.0), 0.0, 10.0, 1e-7);
        assert_eq!(pos, Position::OnLastBoundary);
    }

    #[test]
    fn test_determine_position_out() {
        let pos = IntImpParGenTool::determine_position((15.0, 0.0), 0.0, 10.0, 1e-7);
        assert_eq!(pos, Position::OutOfRange);
    }

    #[test]
    fn test_determine_transition() {
        let (trans1, trans2) = IntImpParGenTool::determine_transition(
            Position::Inside,
            (1.0, 0.0),
            (0.0, 1.0),
            Position::Inside,
            (0.0, 1.0),
            (1.0, 0.0),
            1e-7,
        );
        assert_eq!(trans1.position, Position::Inside);
        assert_eq!(trans2.position, Position::Inside);
    }
}
