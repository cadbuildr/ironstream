// FILE: geom_spiral.rs
// occt: (spiral curves: Archimedean, Logarithmic, Fermat)

use std::f64::consts::PI;

// occt: (Archimedean spiral) r = a + b*theta
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArchimedeanSpiral {
    pub a: f64,
    pub b: f64,
    pub center: [f64; 3],
}

impl ArchimedeanSpiral {
    pub fn new(a: f64, b: f64, center: [f64; 3]) -> Self {
        Self { a, b, center }
    }

    pub fn point(&self, theta: f64) -> [f64; 3] {
        let r = self.a + self.b * theta;
        [
            self.center[0] + r * theta.cos(),
            self.center[1] + r * theta.sin(),
            self.center[2],
        ]
    }

    pub fn radius(&self, theta: f64) -> f64 {
        (self.a + self.b * theta).abs()
    }
}

// occt: (Logarithmic spiral) r = a * exp(b*theta)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LogarithmicSpiral {
    pub a: f64,
    pub b: f64,
    pub center: [f64; 3],
}

impl LogarithmicSpiral {
    pub fn new(a: f64, b: f64, center: [f64; 3]) -> Self {
        assert!(a > 0.0);
        Self { a, b, center }
    }

    pub fn point(&self, theta: f64) -> [f64; 3] {
        let r = self.a * (self.b * theta).exp();
        [
            self.center[0] + r * theta.cos(),
            self.center[1] + r * theta.sin(),
            self.center[2],
        ]
    }

    pub fn radius(&self, theta: f64) -> f64 {
        self.a * (self.b * theta).exp()
    }
}

// occt: (Fermat spiral) r = a * sqrt(theta)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FermatSpiral {
    pub a: f64,
    pub center: [f64; 3],
}

impl FermatSpiral {
    pub fn new(a: f64, center: [f64; 3]) -> Self {
        assert!(a > 0.0);
        Self { a, center }
    }

    pub fn point(&self, theta: f64) -> Option<[f64; 3]> {
        if theta < 0.0 { return None; }
        let r = self.a * theta.sqrt();
        Some([
            self.center[0] + r * theta.cos(),
            self.center[1] + r * theta.sin(),
            self.center[2],
        ])
    }

    pub fn radius(&self, theta: f64) -> f64 {
        if theta < 0.0 { return 0.0; }
        self.a * theta.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn archimedean_at_zero() {
        let s = ArchimedeanSpiral::new(1.0, 0.5, [0.0, 0.0, 0.0]);
        let p = s.point(0.0);
        assert!((p[0] - 1.0).abs() < 1e-10);
        assert!(p[1].abs() < 1e-10);
    }

    #[test]
    fn logarithmic_grows() {
        let s = LogarithmicSpiral::new(1.0, 0.1, [0.0, 0.0, 0.0]);
        assert!(s.radius(1.0) > s.radius(0.0));
    }

    #[test]
    fn fermat_radius_at_pi() {
        let s = FermatSpiral::new(1.0, [0.0, 0.0, 0.0]);
        assert!((s.radius(PI) - PI.sqrt()).abs() < 1e-10);
    }
}
