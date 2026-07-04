// FILE: step_fea_curve_edge.rs
// occt: StepFEA_CurveEdge

/// Representation of curve edge in FEA context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CurveEdge {
    Edge,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant() {
        let edge = CurveEdge::Edge;
        assert_eq!(edge, CurveEdge::Edge);
    }

    #[test]
    fn test_copy() {
        let edge = CurveEdge::Edge;
        let edge2 = edge;
        assert_eq!(edge, edge2);
    }
}
