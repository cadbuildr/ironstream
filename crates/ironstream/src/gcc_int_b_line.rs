// FILE: gcc_int_b_line.rs
// occt: GccInt_BLine

use crate::gcc_int_i_type::GccIntIType;
use crate::gcc_int_bisec::GccIntBisec;

/// Line as a bisecting curve.
pub struct GccIntBLine {
    x0: f64,
    y0: f64,
    dx: f64,
    dy: f64,
}

impl GccIntBLine {
    pub fn new(x0: f64, y0: f64, dx: f64, dy: f64) -> Self {
        GccIntBLine { x0, y0, dx, dy }
    }
}

impl GccIntBisec for GccIntBLine {
    fn arc_type(&self) -> GccIntIType {
        GccIntIType::Lin
    }

    fn line(&self) -> (f64, f64, f64, f64) {
        (self.x0, self.y0, self.dx, self.dy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_b_line() {
        let lin = GccIntBLine::new(1.0, 2.0, 1.0, 0.0);
        assert_eq!(lin.arc_type(), GccIntIType::Lin);
    }

    #[test]
    fn test_line_properties() {
        let lin = GccIntBLine::new(3.0, 4.0, 0.0, 1.0);
        let (x0, y0, dx, dy) = lin.line();
        assert!((x0 - 3.0).abs() < 1e-10);
        assert!((y0 - 4.0).abs() < 1e-10);
        assert!(dx.abs() < 1e-10);
        assert!((dy - 1.0).abs() < 1e-10);
    }
}
