// FILE: gce_make_translation2d.rs
// occt: gce_MakeTranslation2d

use crate::gp2d::Pnt2d;
use crate::gp_trsf2d::Trsf2d;

/// Construction algorithm for gp_Trsf2d translation in 2D.
/// Implements elementary construction algorithms for a translation in 2D space.
pub struct GceMakeTranslation2d {
    translation: Trsf2d,
}

impl GceMakeTranslation2d {
    /// Constructs a translation from a vector in 2D.
    /// The vector defines the translation direction and magnitude.
    pub fn new_from_vector(vector: Pnt2d) -> Self {
        let mut trsf = Trsf2d::identity();
        trsf.set_translation(vector);
        GceMakeTranslation2d {
            translation: trsf,
        }
    }

    /// Constructs a translation from two points in 2D.
    /// The translation vector is from point1 to point2.
    pub fn new_from_points(point1: Pnt2d, point2: Pnt2d) -> Self {
        let vector = point2 - point1;
        Self::new_from_vector(vector)
    }

    /// Returns the constructed transformation.
    pub fn value(&self) -> Trsf2d {
        self.translation
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
    fn test_make_translation2d_from_vector() {
        let vector = Pnt2d::new(1.0, 2.0);
        let maker = GceMakeTranslation2d::new_from_vector(vector);
        let trsf = maker.value();

        // Translate origin by vector, should give vector
        let p = Pnt2d::new(0.0, 0.0);
        let p_trans = trsf.apply(p);
        assert!((p_trans.x - 1.0).abs() < 1e-10);
        assert!((p_trans.y - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_make_translation2d_from_points() {
        let p1 = Pnt2d::new(1.0, 1.0);
        let p2 = Pnt2d::new(4.0, 5.0);
        let maker = GceMakeTranslation2d::new_from_points(p1, p2);
        let trsf = maker.value();

        // Translate p1 should give p2
        let result = trsf.apply(p1);
        assert!((result.x - p2.x).abs() < 1e-10);
        assert!((result.y - p2.y).abs() < 1e-10);
    }

    #[test]
    fn test_operator_alias() {
        let vector = Pnt2d::new(1.0, 0.0);
        let maker = GceMakeTranslation2d::new_from_vector(vector);
        let v1 = maker.value();
        let v2 = maker.operator();
        assert_eq!(v1.scale_factor(), v2.scale_factor());
    }
}
