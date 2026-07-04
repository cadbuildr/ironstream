// FILE: geom_to_step_make_b_spline_curve_with_knots_and_rational_b_spline_curve.rs
// occt: GeomToStep_MakeBSplineCurveWithKnotsAndRationalBSplineCurve

#[derive(Clone, Debug)]
pub struct StepGeom_BSplineCurveWithKnotsAndRationalBSplineCurve {
    pub degree: i32,
    pub control_points: Vec<(f64, f64, f64)>,
    pub weights: Vec<f64>,
    pub knots: Vec<f64>,
}

pub struct GeomToStep_MakeBSplineCurveWithKnotsAndRationalBSplineCurve {
    done: bool,
    result: Option<StepGeom_BSplineCurveWithKnotsAndRationalBSplineCurve>,
}

impl GeomToStep_MakeBSplineCurveWithKnotsAndRationalBSplineCurve {
    pub fn new() -> Self {
        GeomToStep_MakeBSplineCurveWithKnotsAndRationalBSplineCurve {
            done: false,
            result: None,
        }
    }

    pub fn from_curve_with_weights(
        degree: i32,
        points: Vec<(f64, f64, f64)>,
        weights: Vec<f64>,
        knots: Vec<f64>,
    ) -> Self {
        let mut conv = Self::new();
        if degree > 0 && points.len() == weights.len() && !knots.is_empty() {
            conv.result = Some(StepGeom_BSplineCurveWithKnotsAndRationalBSplineCurve {
                degree,
                control_points: points,
                weights,
                knots,
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_BSplineCurveWithKnotsAndRationalBSplineCurve> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeBSplineCurveWithKnotsAndRationalBSplineCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let conv = GeomToStep_MakeBSplineCurveWithKnotsAndRationalBSplineCurve::new();
        assert!(!conv.is_done());
    }

    #[test]
    fn test_from_curve_with_weights() {
        let points = vec![(0.0, 0.0, 0.0), (1.0, 1.0, 0.0)];
        let weights = vec![1.0, 1.0];
        let knots = vec![0.0, 1.0];
        let conv = GeomToStep_MakeBSplineCurveWithKnotsAndRationalBSplineCurve::from_curve_with_weights(
            2, points, weights, knots,
        );
        assert!(conv.is_done());
    }

    #[test]
    fn test_mismatched_weights() {
        let points = vec![(0.0, 0.0, 0.0)];
        let weights = vec![1.0, 2.0];
        let conv = GeomToStep_MakeBSplineCurveWithKnotsAndRationalBSplineCurve::from_curve_with_weights(
            2, points, weights, vec![0.0, 1.0],
        );
        assert!(!conv.is_done());
    }
}
