// FILE: step_visual_path_or_composite_curve.rs
// occt: StepVisual_PathOrCompositeCurve

/// A union type selecting a path or composite curve in STEP representation.
#[derive(Clone, Debug, PartialEq)]
pub enum PathOrCompositeCurve {
    Path(i32),
    CompositeCurve(i32),
}

impl PathOrCompositeCurve {
    /// Creates a PathOrCompositeCurve from a path.
    pub fn path(id: i32) -> Self {
        PathOrCompositeCurve::Path(id)
    }

    /// Creates a PathOrCompositeCurve from a composite curve.
    pub fn composite_curve(id: i32) -> Self {
        PathOrCompositeCurve::CompositeCurve(id)
    }

    /// Returns the case number.
    pub fn case_num(&self) -> i32 {
        match self {
            PathOrCompositeCurve::Path(_) => 1,
            PathOrCompositeCurve::CompositeCurve(_) => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_or_composite_curve_path() {
        let poc = PathOrCompositeCurve::path(1);
        assert_eq!(poc.case_num(), 1);
    }

    #[test]
    fn test_path_or_composite_curve_composite_curve() {
        let poc = PathOrCompositeCurve::composite_curve(2);
        assert_eq!(poc.case_num(), 2);
    }
}
