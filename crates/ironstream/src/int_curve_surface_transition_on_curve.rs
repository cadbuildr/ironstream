// FILE: int_curve_surface_transition_on_curve.rs
// occt: IntCurveSurface_TransitionOnCurve

/// Describes the transition of a curve relative to a surface at an intersection point.
/// Indicates whether the curve is tangent, entering, or exiting the surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntCurveSurfaceTransitionOnCurve {
    /// Curve is tangent to the surface
    Tangent = 0,
    /// Curve is entering the surface
    In = 1,
    /// Curve is exiting the surface
    Out = 2,
}

impl IntCurveSurfaceTransitionOnCurve {
    /// Convert from integer representation
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(IntCurveSurfaceTransitionOnCurve::Tangent),
            1 => Some(IntCurveSurfaceTransitionOnCurve::In),
            2 => Some(IntCurveSurfaceTransitionOnCurve::Out),
            _ => None,
        }
    }

    /// Convert to integer representation
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transition_values() {
        assert_eq!(IntCurveSurfaceTransitionOnCurve::Tangent as i32, 0);
        assert_eq!(IntCurveSurfaceTransitionOnCurve::In as i32, 1);
        assert_eq!(IntCurveSurfaceTransitionOnCurve::Out as i32, 2);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(
            IntCurveSurfaceTransitionOnCurve::from_i32(0),
            Some(IntCurveSurfaceTransitionOnCurve::Tangent)
        );
        assert_eq!(
            IntCurveSurfaceTransitionOnCurve::from_i32(1),
            Some(IntCurveSurfaceTransitionOnCurve::In)
        );
        assert_eq!(
            IntCurveSurfaceTransitionOnCurve::from_i32(2),
            Some(IntCurveSurfaceTransitionOnCurve::Out)
        );
        assert_eq!(IntCurveSurfaceTransitionOnCurve::from_i32(3), None);
    }

    #[test]
    fn test_as_i32() {
        assert_eq!(IntCurveSurfaceTransitionOnCurve::Tangent.as_i32(), 0);
        assert_eq!(IntCurveSurfaceTransitionOnCurve::In.as_i32(), 1);
        assert_eq!(IntCurveSurfaceTransitionOnCurve::Out.as_i32(), 2);
    }

    #[test]
    fn test_equality() {
        let t1 = IntCurveSurfaceTransitionOnCurve::In;
        let t2 = IntCurveSurfaceTransitionOnCurve::In;
        assert_eq!(t1, t2);
    }
}
