// FILE: gcc_int_b_elips.rs
// occt: GccInt_BElips

use crate::gcc_int_i_type::GccIntIType;
use crate::gcc_int_bisec::GccIntBisec;

/// Ellipse as a bisecting curve.
pub struct GccIntBElips {
    cx: f64,
    cy: f64,
    major: f64,
    minor: f64,
    angle: f64,
}

impl GccIntBElips {
    pub fn new(cx: f64, cy: f64, major: f64, minor: f64, angle: f64) -> Self {
        GccIntBElips { cx, cy, major, minor, angle }
    }
}

impl GccIntBisec for GccIntBElips {
    fn arc_type(&self) -> GccIntIType {
        GccIntIType::Ell
    }

    fn ellipse(&self) -> (f64, f64, f64, f64, f64) {
        (self.cx, self.cy, self.major, self.minor, self.angle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_b_elips() {
        let ell = GccIntBElips::new(1.0, 2.0, 3.0, 2.0, 0.5);
        assert_eq!(ell.arc_type(), GccIntIType::Ell);
    }

    #[test]
    fn test_ellipse_properties() {
        let ell = GccIntBElips::new(5.0, 6.0, 7.0, 5.0, 0.785);
        let (cx, cy, maj, min, angle) = ell.ellipse();
        assert!((cx - 5.0).abs() < 1e-10);
        assert!((cy - 6.0).abs() < 1e-10);
        assert!((maj - 7.0).abs() < 1e-10);
        assert!((min - 5.0).abs() < 1e-10);
        assert!((angle - 0.785).abs() < 1e-10);
    }
}
