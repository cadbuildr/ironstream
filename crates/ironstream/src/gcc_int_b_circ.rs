// FILE: gcc_int_b_circ.rs
// occt: GccInt_BCirc

use crate::gcc_int_i_type::GccIntIType;
use crate::gcc_int_bisec::GccIntBisec;

/// Circle as a bisecting curve.
pub struct GccIntBCirc {
    cx: f64,
    cy: f64,
    radius: f64,
}

impl GccIntBCirc {
    pub fn new(cx: f64, cy: f64, radius: f64) -> Self {
        GccIntBCirc { cx, cy, radius }
    }
}

impl GccIntBisec for GccIntBCirc {
    fn arc_type(&self) -> GccIntIType {
        GccIntIType::Cir
    }

    fn circle(&self) -> (f64, f64, f64) {
        (self.cx, self.cy, self.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_b_circ() {
        let circ = GccIntBCirc::new(1.0, 2.0, 3.0);
        assert_eq!(circ.arc_type(), GccIntIType::Cir);
    }

    #[test]
    fn test_circle_properties() {
        let circ = GccIntBCirc::new(5.0, 6.0, 7.0);
        let (cx, cy, r) = circ.circle();
        assert!((cx - 5.0).abs() < 1e-10);
        assert!((cy - 6.0).abs() < 1e-10);
        assert!((r - 7.0).abs() < 1e-10);
    }
}
