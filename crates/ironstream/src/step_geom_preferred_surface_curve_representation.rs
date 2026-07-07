// FILE: step_geom_preferred_surface_curve_representation.rs
// occt: StepGeom_PreferredSurfaceCurveRepresentation

/// Enumeration for preferred curve representation on a surface
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepGeomPreferredSurfaceCurveRepresentation {
    /// Use the 2D parametric curve
    Pcurve = 0,
    /// Use the 3D curve
    Curve3D = 1,
    /// Use the surface approximation
    SurfaceApproximation = 2,
}

impl StepGeomPreferredSurfaceCurveRepresentation {
    pub fn is_pcurve(&self) -> bool {
        matches!(self, StepGeomPreferredSurfaceCurveRepresentation::Pcurve)
    }

    pub fn is_curve_3d(&self) -> bool {
        matches!(self, StepGeomPreferredSurfaceCurveRepresentation::Curve3D)
    }

    pub fn is_surface_approximation(&self) -> bool {
        matches!(self, StepGeomPreferredSurfaceCurveRepresentation::SurfaceApproximation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcurve() {
        let rep = StepGeomPreferredSurfaceCurveRepresentation::Pcurve;
        assert!(rep.is_pcurve());
        assert!(!rep.is_curve_3d());
    }

    #[test]
    fn test_curve_3d() {
        let rep = StepGeomPreferredSurfaceCurveRepresentation::Curve3D;
        assert!(rep.is_curve_3d());
        assert!(!rep.is_pcurve());
    }

    #[test]
    fn test_surface_approximation() {
        let rep = StepGeomPreferredSurfaceCurveRepresentation::SurfaceApproximation;
        assert!(rep.is_surface_approximation());
    }
}
