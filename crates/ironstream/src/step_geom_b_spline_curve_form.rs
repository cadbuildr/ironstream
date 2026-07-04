// FILE: step_geom_b_spline_curve_form.rs
// occt: StepGeom_BSplineCurveForm

/// StepGeom_BSplineCurveForm: Enum representing the geometric form of a B-spline curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSplineCurveForm {
    PolylineForm = 0,
    CircularArc = 1,
    EllipticArc = 2,
    ParabolicArc = 3,
    HyperbolicArc = 4,
    Unspecified = 5,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b_spline_curve_form_values() {
        assert_eq!(BSplineCurveForm::PolylineForm as i32, 0);
        assert_eq!(BSplineCurveForm::CircularArc as i32, 1);
        assert_eq!(BSplineCurveForm::EllipticArc as i32, 2);
        assert_eq!(BSplineCurveForm::ParabolicArc as i32, 3);
        assert_eq!(BSplineCurveForm::HyperbolicArc as i32, 4);
        assert_eq!(BSplineCurveForm::Unspecified as i32, 5);
    }

    #[test]
    fn test_b_spline_curve_form_equality() {
        let form = BSplineCurveForm::CircularArc;
        assert_eq!(form, BSplineCurveForm::CircularArc);
        assert_ne!(form, BSplineCurveForm::EllipticArc);
    }
}
