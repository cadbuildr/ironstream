// FILE: gcc_int_b_point.rs
// occt: GccInt_BPoint

use crate::gcc_int_i_type::GccIntIType;
use crate::gcc_int_bisec::GccIntBisec;

/// Point as a bisecting curve.
pub struct GccIntBPoint {
    x: f64,
    y: f64,
}

impl GccIntBPoint {
    pub fn new(x: f64, y: f64) -> Self {
        GccIntBPoint { x, y }
    }
}

impl GccIntBisec for GccIntBPoint {
    fn arc_type(&self) -> GccIntIType {
        GccIntIType::Pnt
    }

    fn point(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_b_point() {
        let pnt = GccIntBPoint::new(1.0, 2.0);
        assert_eq!(pnt.arc_type(), GccIntIType::Pnt);
    }

    #[test]
    fn test_point_properties() {
        let pnt = GccIntBPoint::new(3.5, 4.5);
        let (x, y) = pnt.point();
        assert!((x - 3.5).abs() < 1e-10);
        assert!((y - 4.5).abs() < 1e-10);
    }
}
