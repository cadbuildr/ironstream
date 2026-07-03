// FILE: geom_transformation.rs

use crate::gp_trsf::{Trsf, Form as TrsfForm};

/// Wraps Trsf for use as a Geom-level transformation object.
// occt: Geom_Transformation
#[derive(Debug, Clone)]
pub struct Geom_Transformation {
    trsf: Trsf,
}

impl Geom_Transformation {
    pub fn new() -> Self {
        Geom_Transformation { trsf: Trsf::default() }
    }

    pub fn from_trsf(t: Trsf) -> Self {
        Geom_Transformation { trsf: t }
    }

    pub fn invert(&self) -> Geom_Transformation {
        Geom_Transformation {
            trsf: self.trsf.inverted().unwrap_or_default(),
        }
    }

    pub fn multiplied(&self, other: &Geom_Transformation) -> Geom_Transformation {
        Geom_Transformation {
            trsf: self.trsf.multiplied(&other.trsf),
        }
    }

    /// Returns the (row, col) entry of the 3x4 transformation matrix (1-based indices).
    pub fn value(&self, row: usize, col: usize) -> f64 {
        self.trsf.value(row as i32, col as i32).unwrap_or(0.0)
    }

    pub fn is_negative(&self) -> bool {
        self.trsf.is_negative()
    }

    pub fn form(&self) -> TrsfForm {
        self.trsf.form()
    }

    pub fn scale_factor(&self) -> f64 {
        self.trsf.scale_factor()
    }

    pub fn get_trsf(&self) -> &Trsf {
        &self.trsf
    }

    pub fn set_trsf(&mut self, t: Trsf) {
        self.trsf = t;
    }
}

impl Default for Geom_Transformation {
    fn default() -> Self {
        Geom_Transformation::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp_trsf::{Trsf, Form as TrsfForm};
    use crate::gp::{Ax1, Pnt};

    const EPS: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    fn make_translation(dx: f64, dy: f64, dz: f64) -> Trsf {
        use crate::gp::Pnt;
        let mut t = Trsf::default();
        t.set_translation(Pnt::new(dx, dy, dz));
        t
    }

    fn make_scale(cx: f64, cy: f64, cz: f64, s: f64) -> Trsf {
        use crate::gp::Pnt;
        let mut t = Trsf::default();
        let _ = t.set_scale(Pnt::new(cx, cy, cz), s);
        t
    }

    #[test]
    fn test_identity() {
        let t = Geom_Transformation::new();
        assert!(matches!(t.form(), TrsfForm::Identity));
        assert!(approx_eq(t.value(1, 1), 1.0));
        assert!(approx_eq(t.value(2, 2), 1.0));
        assert!(approx_eq(t.value(3, 3), 1.0));
        assert!(approx_eq(t.value(1, 2), 0.0));
        assert!(approx_eq(t.value(1, 4), 0.0));
        assert!(approx_eq(t.value(2, 4), 0.0));
        assert!(approx_eq(t.value(3, 4), 0.0));
        assert!(approx_eq(t.scale_factor(), 1.0));
        assert!(!t.is_negative());
    }

    #[test]
    fn test_translation() {
        let trsf = make_translation(3.0, 4.0, 5.0);
        let t = Geom_Transformation::from_trsf(trsf);
        assert!(matches!(t.form(), TrsfForm::Translation));
        assert!(approx_eq(t.value(1, 1), 1.0));
        assert!(approx_eq(t.value(2, 2), 1.0));
        assert!(approx_eq(t.value(3, 3), 1.0));
        assert!(approx_eq(t.value(1, 4), 3.0));
        assert!(approx_eq(t.value(2, 4), 4.0));
        assert!(approx_eq(t.value(3, 4), 5.0));
        assert!(!t.is_negative());
    }

    #[test]
    fn test_scale() {
        let trsf = make_scale(0.0, 0.0, 0.0, 2.5);
        let t = Geom_Transformation::from_trsf(trsf);
        assert!(matches!(t.form(), TrsfForm::Scale));
        assert!(approx_eq(t.scale_factor(), 2.5));
        assert!(approx_eq(t.value(1, 1), 2.5));
        assert!(approx_eq(t.value(2, 2), 2.5));
        assert!(approx_eq(t.value(3, 3), 2.5));
        assert!(approx_eq(t.value(1, 4), 0.0));
        assert!(!t.is_negative());
    }

    #[test]
    fn test_rotation_z_90() {
        let mut trsf = Trsf::default();
        let origin = Pnt::new(0.0, 0.0, 0.0);
        let z_dir = Pnt::new(0.0, 0.0, 1.0);
        let axis = Ax1::new(origin, z_dir);
        let angle = std::f64::consts::PI / 2.0;
        trsf.set_rotation(axis, angle);
        let t = Geom_Transformation::from_trsf(trsf);
        assert!(matches!(t.form(), TrsfForm::Rotation));
        assert!(approx_eq(t.value(1, 1), 0.0));
        assert!(approx_eq(t.value(1, 2), -1.0));
        assert!(approx_eq(t.value(2, 1), 1.0));
        assert!(approx_eq(t.value(2, 2), 0.0));
        assert!(approx_eq(t.value(3, 3), 1.0));
        assert!(approx_eq(t.value(1, 4), 0.0));
        assert!(approx_eq(t.value(2, 4), 0.0));
        assert!(approx_eq(t.value(3, 4), 0.0));
        assert!(!t.is_negative());
    }

    #[test]
    fn test_invert_translation() {
        let trsf = make_translation(3.0, -1.0, 2.0);
        let t = Geom_Transformation::from_trsf(trsf);
        let inv = t.invert();
        assert!(approx_eq(inv.value(1, 4), -3.0));
        assert!(approx_eq(inv.value(2, 4), 1.0));
        assert!(approx_eq(inv.value(3, 4), -2.0));
    }

    #[test]
    fn test_invert_scale() {
        let trsf = make_scale(0.0, 0.0, 0.0, 4.0);
        let t = Geom_Transformation::from_trsf(trsf);
        let inv = t.invert();
        assert!(approx_eq(inv.scale_factor(), 0.25));
    }

    #[test]
    fn test_multiply_translations() {
        let t1 = Geom_Transformation::from_trsf(make_translation(1.0, 2.0, 3.0));
        let t2 = Geom_Transformation::from_trsf(make_translation(4.0, 5.0, 6.0));
        let composed = t1.multiplied(&t2);
        assert!(approx_eq(composed.value(1, 4), 5.0));
        assert!(approx_eq(composed.value(2, 4), 7.0));
        assert!(approx_eq(composed.value(3, 4), 9.0));
    }

    #[test]
    fn test_compose_rotation_with_inverse() {
        let mut trsf = Trsf::default();
        let origin = Pnt::new(0.0, 0.0, 0.0);
        let z_dir = Pnt::new(0.0, 0.0, 1.0);
        let axis = Ax1::new(origin, z_dir);
        trsf.set_rotation(axis, 1.23456);
        let t = Geom_Transformation::from_trsf(trsf);
        let inv = t.invert();
        let composed = t.multiplied(&inv);
        assert!(approx_eq(composed.value(1, 1), 1.0));
        assert!(approx_eq(composed.value(2, 2), 1.0));
        assert!(approx_eq(composed.value(3, 3), 1.0));
        assert!(approx_eq(composed.value(1, 2), 0.0));
        assert!(approx_eq(composed.value(1, 4), 0.0));
        assert!(approx_eq(composed.value(2, 4), 0.0));
        assert!(approx_eq(composed.value(3, 4), 0.0));
    }

    #[test]
    fn test_get_set_trsf() {
        let mut t = Geom_Transformation::new();
        let new_trsf = make_translation(7.0, 8.0, 9.0);
        t.set_trsf(new_trsf);
        assert!(approx_eq(t.value(1, 4), 7.0));
        assert!(approx_eq(t.value(2, 4), 8.0));
        assert!(approx_eq(t.value(3, 4), 9.0));
    }

    #[test]
    fn test_scale_factor_identity() {
        let t = Geom_Transformation::new();
        assert!(approx_eq(t.scale_factor(), 1.0));
    }

    #[test]
    fn test_point_mirror() {
        let mut trsf = Trsf::default();
        let pt = Pnt::new(1.0, 2.0, 3.0);
        trsf.set_mirror_point(pt);
        let t = Geom_Transformation::from_trsf(trsf);
        assert!(matches!(t.form(), TrsfForm::PntMirror));
        assert!(approx_eq(t.value(1, 1), -1.0));
        assert!(approx_eq(t.value(2, 2), -1.0));
        assert!(approx_eq(t.value(3, 3), -1.0));
        assert!(approx_eq(t.value(1, 4), 2.0));
        assert!(approx_eq(t.value(2, 4), 4.0));
        assert!(approx_eq(t.value(3, 4), 6.0));
        assert!(t.is_negative());
    }
}
