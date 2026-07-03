// FILE: gcc_int_b_hyper.rs
// occt: GccInt_BHyper

use crate::gcc_int_i_type::GccIntIType;
use crate::gcc_int_bisec::GccIntBisec;

/// Hyperbola as a bisecting curve.
pub struct GccIntBHyper {
    cx: f64,
    cy: f64,
    major: f64,
    minor: f64,
    angle: f64,
}

impl GccIntBHyper {
    pub fn new(cx: f64, cy: f64, major: f64, minor: f64, angle: f64) -> Self {
        GccIntBHyper { cx, cy, major, minor, angle }
    }
}

impl GccIntBisec for GccIntBHyper {
    fn arc_type(&self) -> GccIntIType {
        GccIntIType::Hpr
    }

    fn hyperbola(&self) -> (f64, f64, f64, f64, f64) {
        (self.cx, self.cy, self.major, self.minor, self.angle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_b_hyper() {
        let hyp = GccIntBHyper::new(1.0, 2.0, 3.0, 2.0, 0.5);
        assert_eq!(hyp.arc_type(), GccIntIType::Hpr);
    }

    #[test]
    fn test_hyperbola_properties() {
        let hyp = GccIntBHyper::new(0.0, 0.0, 1.0, 1.0, 0.0);
        let (cx, cy, maj, min, angle) = hyp.hyperbola();
        assert!(cx.abs() < 1e-10);
        assert!(cy.abs() < 1e-10);
        assert!((maj - 1.0).abs() < 1e-10);
    }
}
