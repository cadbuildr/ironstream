// FILE: step_visual_edge_or_curve.rs
// occt: StepVisual_EdgeOrCurve

/// A union type selecting either an edge or a curve in STEP representation.
#[derive(Clone, Debug, PartialEq)]
pub enum EdgeOrCurve {
    Edge(i32),
    Curve(i32),
}

impl EdgeOrCurve {
    /// Creates an EdgeOrCurve from an edge.
    pub fn edge(id: i32) -> Self {
        EdgeOrCurve::Edge(id)
    }

    /// Creates an EdgeOrCurve from a curve.
    pub fn curve(id: i32) -> Self {
        EdgeOrCurve::Curve(id)
    }

    /// Returns the case number (1 = Edge, 2 = Curve).
    pub fn case_num(&self) -> i32 {
        match self {
            EdgeOrCurve::Edge(_) => 1,
            EdgeOrCurve::Curve(_) => 2,
        }
    }

    /// Returns the ID if this is an edge.
    pub fn as_edge(&self) -> Option<i32> {
        match self {
            EdgeOrCurve::Edge(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns the ID if this is a curve.
    pub fn as_curve(&self) -> Option<i32> {
        match self {
            EdgeOrCurve::Curve(id) => Some(*id),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_or_curve_edge() {
        let eoc = EdgeOrCurve::edge(5);
        assert_eq!(eoc.case_num(), 1);
        assert_eq!(eoc.as_edge(), Some(5));
        assert_eq!(eoc.as_curve(), None);
    }

    #[test]
    fn test_edge_or_curve_curve() {
        let eoc = EdgeOrCurve::curve(10);
        assert_eq!(eoc.case_num(), 2);
        assert_eq!(eoc.as_curve(), Some(10));
        assert_eq!(eoc.as_edge(), None);
    }
}
