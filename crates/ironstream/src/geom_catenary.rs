// FILE: geom_catenary.rs
// occt: Geom_Curve (catenary), Law_Interpolate

use std::f64::consts::PI;

// occt: (catenary curve)
/// Catenary: y = a * cosh(x/a) in 3-D.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Catenary {
    pub a: f64,
    pub center: [f64; 3],
}

impl Catenary {
    pub fn new(a: f64) -> Self {
        assert!(a > 0.0, "Catenary: a must be positive");
        Self { a, center: [0.0, 0.0, 0.0] }
    }

    pub fn with_center(a: f64, center: [f64; 3]) -> Self {
        assert!(a > 0.0);
        Self { a, center }
    }

    pub fn point(&self, x: f64) -> [f64; 3] {
        let y = self.a * (x / self.a).cosh();
        [self.center[0] + x, self.center[1] + y, self.center[2]]
    }

    pub fn tangent(&self, x: f64) -> [f64; 3] {
        let s = (x / self.a).sinh();
        let len = (1.0 + s * s).sqrt();
        [1.0 / len, s / len, 0.0]
    }

    pub fn arc_length(&self, x: f64) -> f64 {
        self.a * (x / self.a).sinh()
    }

    pub fn curvature(&self, x: f64) -> f64 {
        let c = (x / self.a).cosh();
        1.0 / (self.a * c * c)
    }
}

// occt: (clothoid / Euler spiral)
#[derive(Clone, Copy, Debug)]
pub struct Clothoid {
    pub a: f64,
}

impl Clothoid {
    pub fn new(a: f64) -> Self {
        assert!(a > 0.0);
        Self { a }
    }

    pub fn curvature(&self, s: f64) -> f64 {
        s / (self.a * self.a)
    }

    pub fn radius(&self, s: f64) -> f64 {
        if s.abs() < 1e-14 { return f64::INFINITY; }
        self.a * self.a / s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catenary_minimum_at_zero() {
        let c = Catenary::new(1.0);
        let p = c.point(0.0);
        assert!((p[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn catenary_symmetric() {
        let c = Catenary::new(2.0);
        let l = c.point(-1.0);
        let r = c.point(1.0);
        assert!((l[1] - r[1]).abs() < 1e-10);
    }

    #[test]
    fn catenary_arc_length() {
        let c = Catenary::new(1.0);
        let s = c.arc_length(1.0);
        assert!((s - 1.0_f64.sinh()).abs() < 1e-10);
    }

    #[test]
    fn clothoid_curvature_linear() {
        let cl = Clothoid::new(2.0);
        let k1 = cl.curvature(1.0);
        let k2 = cl.curvature(2.0);
        assert!((k2 / k1 - 2.0).abs() < 1e-10);
    }
}
