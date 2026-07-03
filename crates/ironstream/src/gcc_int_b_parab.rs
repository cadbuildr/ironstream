// FILE: gcc_int_b_parab.rs
// occt: GccInt_BParab

use crate::gcc_int_i_type::GccIntIType;
use crate::gcc_int_bisec::GccIntBisec;

/// Parabola as a bisecting curve.
pub struct GccIntBParab {
    fx: f64,
    fy: f64,
    vx: f64,
    vy: f64,
    angle: f64,
}

impl GccIntBParab {
    pub fn new(fx: f64, fy: f64, vx: f64, vy: f64, angle: f64) -> Self {
        GccIntBParab { fx, fy, vx, vy, angle }
    }
}

impl GccIntBisec for GccIntBParab {
    fn arc_type(&self) -> GccIntIType {
        GccIntIType::Par
    }

    fn parabola(&self) -> (f64, f64, f64, f64, f64) {
        (self.fx, self.fy, self.vx, self.vy, self.angle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_b_parab() {
        let par = GccIntBParab::new(1.0, 2.0, 0.5, 1.0, 0.5);
        assert_eq!(par.arc_type(), GccIntIType::Par);
    }

    #[test]
    fn test_parabola_properties() {
        let par = GccIntBParab::new(0.0, 1.0, 0.0, 0.0, 0.0);
        let (fx, fy, vx, vy, angle) = par.parabola();
        assert!(fx.abs() < 1e-10);
        assert!((fy - 1.0).abs() < 1e-10);
        assert!(vx.abs() < 1e-10);
    }
}
