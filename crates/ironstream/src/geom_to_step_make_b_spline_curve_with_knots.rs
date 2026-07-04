// FILE: geom_to_step_make_b_spline_curve_with_knots.rs
// occt: GeomToStep_MakeBSplineCurveWithKnots

#[derive(Clone, Debug)]
pub struct StepGeom_BSplineCurveWithKnots {
    pub degree: i32,
    pub control_points: Vec<(f64, f64, f64)>,
    pub knots: Vec<f64>,
    pub knot_multiplicities: Vec<i32>,
}

pub struct GeomToStep_MakeBSplineCurveWithKnots {
    done: bool,
    result: Option<StepGeom_BSplineCurveWithKnots>,
}

impl GeomToStep_MakeBSplineCurveWithKnots {
    pub fn new() -> Self {
        GeomToStep_MakeBSplineCurveWithKnots {
            done: false,
            result: None,
        }
    }

    pub fn from_degree_points_and_knots(
        degree: i32,
        points: Vec<(f64, f64, f64)>,
        knots: Vec<f64>,
        multiplicities: Vec<i32>,
    ) -> Self {
        let mut conv = Self::new();
        if degree > 0 && !points.is_empty() && !knots.is_empty() && !multiplicities.is_empty() {
            conv.result = Some(StepGeom_BSplineCurveWithKnots {
                degree,
                control_points: points,
                knots,
                knot_multiplicities: multiplicities,
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_BSplineCurveWithKnots> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeBSplineCurveWithKnots {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let conv = GeomToStep_MakeBSplineCurveWithKnots::new();
        assert!(!conv.is_done());
    }

    #[test]
    fn test_from_degree_points_and_knots() {
        let points = vec![(0.0, 0.0, 0.0), (1.0, 1.0, 0.0), (2.0, 0.0, 0.0)];
        let knots = vec![0.0, 0.5, 1.0];
        let multiplicities = vec![1, 1, 1];
        let conv = GeomToStep_MakeBSplineCurveWithKnots::from_degree_points_and_knots(
            2, points, knots, multiplicities,
        );
        assert!(conv.is_done());
    }

    #[test]
    fn test_invalid_degree() {
        let conv = GeomToStep_MakeBSplineCurveWithKnots::from_degree_points_and_knots(
            0, vec![], vec![], vec![],
        );
        assert!(!conv.is_done());
    }
}
