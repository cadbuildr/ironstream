// FILE: gc_make_conical_surface.rs
// occt: GC_MakeConicalSurface

//! Construction algorithms for conical surfaces.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MakeStatus {
    Ok,
    InvalidAxis,
    InvalidAngle,
    InsufficientData,
}

/// A conical surface defined by axis, apex, semi-angle.
#[derive(Clone, Debug)]
pub struct ConicalSurface {
    /// Apex point (x, y, z)
    pub apex: (f64, f64, f64),
    /// Axis direction (normalized)
    pub axis: (f64, f64, f64),
    /// Semi-angle in radians
    pub semi_angle: f64,
}

impl ConicalSurface {
    /// Create a conical surface.
    pub fn new(apex: (f64, f64, f64), axis: (f64, f64, f64), semi_angle: f64) -> Option<Self> {
        if semi_angle <= 0.0 || semi_angle >= std::f64::consts::PI / 2.0 {
            return None;
        }

        let (ax, ay, az) = axis;
        let len = (ax * ax + ay * ay + az * az).sqrt();
        if len < 1e-15 {
            return None;
        }

        Some(ConicalSurface {
            apex,
            axis: (ax / len, ay / len, az / len),
            semi_angle,
        })
    }

    /// Get the radius at a distance from apex along axis.
    pub fn radius_at_distance(&self, distance: f64) -> f64 {
        distance * self.semi_angle.tan()
    }
}

/// Builder for conical surfaces.
pub struct MakeConicalSurface {
    surface: Option<ConicalSurface>,
    status: MakeStatus,
}

impl MakeConicalSurface {
    /// Create from apex, axis and semi-angle.
    pub fn new(
        apex: (f64, f64, f64),
        axis: (f64, f64, f64),
        semi_angle: f64,
    ) -> Self {
        match ConicalSurface::new(apex, axis, semi_angle) {
            Some(surface) => MakeConicalSurface {
                surface: Some(surface),
                status: MakeStatus::Ok,
            },
            None => MakeConicalSurface {
                surface: None,
                status: MakeStatus::InvalidAngle,
            },
        }
    }

    /// Get the constructed surface.
    pub fn value(&self) -> Option<&ConicalSurface> {
        self.surface.as_ref()
    }

    /// Get status.
    pub fn status(&self) -> MakeStatus {
        self.status
    }

    /// Check if construction succeeded.
    pub fn is_done(&self) -> bool {
        self.status == MakeStatus::Ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conical_surface_create() {
        let surf = ConicalSurface::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), std::f64::consts::PI / 6.0);
        assert!(surf.is_some());
        let s = surf.unwrap();
        assert!(s.semi_angle > 0.0);
    }

    #[test]
    fn test_conical_surface_invalid_angle() {
        let surf = ConicalSurface::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), -0.5);
        assert!(surf.is_none());
    }

    #[test]
    fn test_make_conical_surface() {
        let maker = MakeConicalSurface::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), std::f64::consts::PI / 6.0);
        assert!(maker.is_done());
        assert!(maker.value().is_some());
    }

    #[test]
    fn test_radius_at_distance() {
        let surf = ConicalSurface::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), std::f64::consts::PI / 4.0).unwrap();
        let r = surf.radius_at_distance(1.0);
        assert!((r - 1.0).abs() < 1e-9);
    }
}
