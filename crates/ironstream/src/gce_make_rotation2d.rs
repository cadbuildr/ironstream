// FILE: gce_make_rotation2d.rs
// occt: gce_MakeRotation2d

use crate::gp2d::Pnt2d;
use crate::gp_trsf2d::Trsf2d;

/// Construction algorithm for gp_Trsf2d rotation in 2D.
/// Implements an elementary construction algorithm for a rotation in 2D space.
pub struct GceMakeRotation2d {
    rotation: Trsf2d,
}

impl GceMakeRotation2d {
    /// Constructs a rotation around a point in 2D.
    /// `point` is the rotation center, `angle` is in radians.
    pub fn new(point: Pnt2d, angle: f64) -> Self {
        let mut trsf = Trsf2d::identity();
        trsf.set_rotation(point, angle);
        GceMakeRotation2d { rotation: trsf }
    }

    /// Returns the constructed transformation.
    pub fn value(&self) -> Trsf2d {
        self.rotation
    }

    /// Alias for value() returning a copy.
    pub fn operator(&self) -> Trsf2d {
        self.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn test_make_rotation2d_90_degrees() {
        let center = Pnt2d::new(0.0, 0.0);
        let maker = GceMakeRotation2d::new(center, FRAC_PI_2);
        let trsf = maker.value();

        // Rotate (1, 0) by 90 degrees around origin, should give ~(0, 1)
        let p = Pnt2d::new(1.0, 0.0);
        let p_rot = trsf.apply(p);
        assert!((p_rot.x).abs() < 1e-10);
        assert!((p_rot.y - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_make_rotation2d_around_point() {
        let center = Pnt2d::new(1.0, 1.0);
        let maker = GceMakeRotation2d::new(center, FRAC_PI_2);
        let trsf = maker.value();

        // After rotation around (1, 1), the center itself should not move
        let p_rot = trsf.apply(center);
        assert!((p_rot.x - center.x).abs() < 1e-10);
        assert!((p_rot.y - center.y).abs() < 1e-10);
    }

    #[test]
    fn test_operator_alias() {
        let center = Pnt2d::new(0.0, 0.0);
        let maker = GceMakeRotation2d::new(center, FRAC_PI_2);
        let v1 = maker.value();
        let v2 = maker.operator();
        // Check that both give the same result
        assert_eq!(v1.scale_factor(), v2.scale_factor());
    }
}
