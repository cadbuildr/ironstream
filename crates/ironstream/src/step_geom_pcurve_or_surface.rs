// FILE: step_geom_pcurve_or_surface.rs
// occt: StepGeom_PcurveOrSurface

/// Represents either a P-curve or a surface
#[derive(Clone, Debug)]
pub enum StepGeomPcurveOrSurface {
    Pcurve(i32),
    Surface(i32),
}

impl StepGeomPcurveOrSurface {
    pub fn is_pcurve(&self) -> bool {
        matches!(self, StepGeomPcurveOrSurface::Pcurve(_))
    }

    pub fn is_surface(&self) -> bool {
        matches!(self, StepGeomPcurveOrSurface::Surface(_))
    }

    pub fn as_pcurve(&self) -> Option<i32> {
        match self {
            StepGeomPcurveOrSurface::Pcurve(id) => Some(*id),
            _ => None,
        }
    }

    pub fn as_surface(&self) -> Option<i32> {
        match self {
            StepGeomPcurveOrSurface::Surface(id) => Some(*id),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcurve() {
        let item = StepGeomPcurveOrSurface::Pcurve(5);
        assert!(item.is_pcurve());
        assert!(!item.is_surface());
        assert_eq!(item.as_pcurve(), Some(5));
    }

    #[test]
    fn test_surface() {
        let item = StepGeomPcurveOrSurface::Surface(10);
        assert!(!item.is_pcurve());
        assert!(item.is_surface());
        assert_eq!(item.as_surface(), Some(10));
    }
}
