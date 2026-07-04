// FILE: step_geom_b_spline_surface_form.rs
// occt: StepGeom_BSplineSurfaceForm

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSplineSurfaceForm {
    PlaneSurface = 0,
    CylindricalSurface = 1,
    ConicalSurface = 2,
    SphericalSurface = 3,
    ToroidalSurface = 4,
    SurfaceOfRevolution = 5,
    RuledSurface = 6,
    GeneralizedCone = 7,
    QuadricSurface = 8,
    Unspecified = 9,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_form_values() {
        assert_eq!(BSplineSurfaceForm::PlaneSurface as i32, 0);
        assert_eq!(BSplineSurfaceForm::CylindricalSurface as i32, 1);
        assert_eq!(BSplineSurfaceForm::Unspecified as i32, 9);
    }

    #[test]
    fn test_surface_form_equality() {
        assert_eq!(
            BSplineSurfaceForm::PlaneSurface,
            BSplineSurfaceForm::PlaneSurface
        );
        assert_ne!(
            BSplineSurfaceForm::PlaneSurface,
            BSplineSurfaceForm::CylindricalSurface
        );
    }
}
