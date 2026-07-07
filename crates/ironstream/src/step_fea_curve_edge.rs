// FILE: step_fea_curve_edge.rs
// occt: StepFEA_CurveEdge

/// Mirrors OCCT enum StepFEA_CurveEdge { StepFEA_ElementEdge }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveEdge {
    ElementEdge,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant() {
        let edge = CurveEdge::ElementEdge;
        assert_eq!(edge, CurveEdge::ElementEdge);
    }

    #[test]
    fn test_copy() {
        let edge = CurveEdge::ElementEdge;
        let edge2 = edge;
        assert_eq!(edge, edge2);
    }
}
