// FILE: law_s_interpolate.rs
// occt-ref: Law_S, Law_SInterpolate, Law_Interpol, Law_Linear

use std::f64::consts::PI;

/// S-shaped (smooth) law from t=0..1: starts at v0, ends at v1 with zero derivatives.
#[derive(Clone, Debug)]
pub struct LawS {
    pub v0: f64,
    pub v1: f64,
}

impl LawS {
    pub fn new(v0: f64, v1: f64) -> Self { Self { v0, v1 } }

    /// Smooth interpolation using a cosine blend (OCCT Law_S).
    pub fn value(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        let blend = (1.0 - (PI * t).cos()) / 2.0;
        self.v0 + (self.v1 - self.v0) * blend
    }

    pub fn d1(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        (self.v1 - self.v0) * PI / 2.0 * (PI * t).sin()
    }

    pub fn d2(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        (self.v1 - self.v0) * PI * PI / 2.0 * (PI * t).cos()
    }

    pub fn bounds(&self) -> (f64, f64) { (0.0, 1.0) }
}

/// Piecewise linear law (Law_Linear in OCCT).
#[derive(Clone, Debug)]
pub struct LawLinear {
    pub knots: Vec<f64>,
    pub values: Vec<f64>,
}

impl LawLinear {
    pub fn new(knots: Vec<f64>, values: Vec<f64>) -> Option<Self> {
        if knots.len() != values.len() || knots.len() < 2 { return None; }
        Some(Self { knots, values })
    }

    pub fn constant(v: f64) -> Self {
        Self { knots: vec![0.0, 1.0], values: vec![v, v] }
    }

    pub fn value(&self, t: f64) -> f64 {
        let n = self.knots.len();
        if t <= self.knots[0] { return self.values[0]; }
        if t >= self.knots[n-1] { return self.values[n-1]; }
        for i in 0..n-1 {
            if t >= self.knots[i] && t <= self.knots[i+1] {
                let dt = self.knots[i+1] - self.knots[i];
                if dt < 1e-14 { return self.values[i]; }
                let frac = (t - self.knots[i]) / dt;
                return self.values[i] + frac * (self.values[i+1] - self.values[i]);
            }
        }
        *self.values.last().unwrap()
    }

    pub fn d1(&self, t: f64) -> f64 {
        let n = self.knots.len();
        if t <= self.knots[0] || t >= self.knots[n-1] { return 0.0; }
        for i in 0..n-1 {
            if t >= self.knots[i] && t <= self.knots[i+1] {
                let dt = self.knots[i+1] - self.knots[i];
                if dt < 1e-14 { return 0.0; }
                return (self.values[i+1] - self.values[i]) / dt;
            }
        }
        0.0
    }

    pub fn parameter_range(&self) -> (f64, f64) {
        (*self.knots.first().unwrap(), *self.knots.last().unwrap())
    }

    pub fn is_constant(&self) -> bool {
        self.values.windows(2).all(|w| (w[0] - w[1]).abs() < 1e-12)
    }
}

/// Cubic Hermite interpolation law (Law_Interpol).
#[derive(Clone, Debug)]
pub struct LawInterpol {
    pub knots: Vec<f64>,
    pub values: Vec<f64>,
    pub derivatives: Vec<f64>,
    pub is_periodic: bool,
}

impl LawInterpol {
    pub fn new(knots: Vec<f64>, values: Vec<f64>) -> Option<Self> {
        if knots.len() != values.len() || knots.len() < 2 { return None; }
        let n = knots.len();
        let mut derivatives = vec![0.0; n];
        // Catmull-Rom style tangents
        for i in 1..n-1 {
            derivatives[i] = (values[i+1] - values[i-1]) / (knots[i+1] - knots[i-1]).max(1e-14);
        }
        Some(Self { knots, values, derivatives, is_periodic: false })
    }

    pub fn value(&self, t: f64) -> f64 {
        let n = self.knots.len();
        if t <= self.knots[0] { return self.values[0]; }
        if t >= self.knots[n-1] { return self.values[n-1]; }
        for i in 0..n-1 {
            let t0 = self.knots[i]; let t1 = self.knots[i+1];
            if t >= t0 && t <= t1 {
                let dt = t1 - t0;
                let u = (t - t0) / dt;
                let h00 = 2.0*u*u*u - 3.0*u*u + 1.0;
                let h10 = u*u*u - 2.0*u*u + u;
                let h01 = -2.0*u*u*u + 3.0*u*u;
                let h11 = u*u*u - u*u;
                return h00 * self.values[i] + h10 * dt * self.derivatives[i]
                     + h01 * self.values[i+1] + h11 * dt * self.derivatives[i+1];
            }
        }
        *self.values.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn law_s_values() {
        let s = LawS::new(0.0, 1.0);
        assert!((s.value(0.0)).abs() < 1e-10);
        assert!((s.value(1.0) - 1.0).abs() < 1e-10);
        assert!((s.value(0.5) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn law_s_zero_derivative_at_ends() {
        let s = LawS::new(0.0, 10.0);
        assert!(s.d1(0.0).abs() < 1e-10);
        assert!(s.d1(1.0).abs() < 1e-10);
    }

    #[test]
    fn law_linear_interpolation() {
        let l = LawLinear::new(vec![0.0, 1.0, 2.0], vec![0.0, 5.0, 10.0]).unwrap();
        assert!((l.value(0.5) - 2.5).abs() < 1e-10);
        assert!((l.value(1.5) - 7.5).abs() < 1e-10);
    }

    #[test]
    fn law_linear_constant() {
        let l = LawLinear::constant(3.0);
        assert!(l.is_constant());
        assert!((l.value(0.5) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn law_interpol_endpoints() {
        let l = LawInterpol::new(vec![0.0, 1.0, 2.0], vec![0.0, 5.0, 10.0]).unwrap();
        assert!((l.value(0.0)).abs() < 1e-10);
        assert!((l.value(2.0) - 10.0).abs() < 1e-10);
    }
}
