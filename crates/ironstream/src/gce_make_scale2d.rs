// FILE: gce_make_scale2d.rs
// occt: gce_MakeScale2d

use crate::gp2d::Pnt2d;
use crate::gp_trsf2d::Trsf2d;

/// Construction algorithm for gp_Trsf2d scaling in 2D.
/// Implements an elementary construction algorithm for a scaling transformation in 2D space.
pub struct GceMakeScale2d {
    scale: Trsf2d,
}

impl GceMakeScale2d {
    /// Constructs a scaling transformation in 2D.
    /// `point` is the center of scaling, `scale_factor` is the scale.
    pub fn new(point: Pnt2d, scale_factor: f64) -> Self {
        let mut trsf = Trsf2d::identity();
        let _ = trsf.set_scale(point, scale_factor);
        GceMakeScale2d { scale: trsf }
    }

    /// Returns the constructed transformation.
    pub fn value(&self) -> Trsf2d {
        self.scale
    }

    /// Alias for value() returning a copy.
    pub fn operator(&self) -> Trsf2d {
        self.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_scale2d() {
        let center = Pnt2d::new(0.0, 0.0);
        let maker = GceMakeScale2d::new(center, 2.0);
        let trsf = maker.value();

        // Scale (1, 0) by 2 about origin, should give (2, 0)
        let p = Pnt2d::new(1.0, 0.0);
        let p_scaled = trsf.apply(p);
        assert!((p_scaled.x - 2.0).abs() < 1e-10);
        assert!((p_scaled.y).abs() < 1e-10);
    }

    #[test]
    fn test_make_scale2d_around_center() {
        let center = Pnt2d::new(1.0, 1.0);
        let maker = GceMakeScale2d::new(center, 2.0);
        let trsf = maker.value();

        // The center should remain unchanged
        let c_scaled = trsf.apply(center);
        assert!((c_scaled.x - center.x).abs() < 1e-10);
        assert!((c_scaled.y - center.y).abs() < 1e-10);
    }

    #[test]
    fn test_operator_alias() {
        let center = Pnt2d::new(0.0, 0.0);
        let maker = GceMakeScale2d::new(center, 2.0);
        let v1 = maker.value();
        let v2 = maker.operator();
        assert_eq!(v1.scale_factor(), v2.scale_factor());
    }
}
