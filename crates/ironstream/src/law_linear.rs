// FILE: law_linear.rs
// occt: Law_Linear, Law_Constant
// occt-ref: Law_Interpol, Law_Composite
//       Law_Function (abstract), Law_BSplineKnotSplit

use std::f64::consts::PI;

/// Abstract law function interface.
// occt-ref: Law_Function
pub trait LawFunction: Send + Sync {
    fn value(&self, x: f64) -> f64;
    fn d1(&self, x: f64) -> (f64, f64);
    fn continuity(&self) -> usize;
    fn nb_intervals(&self, continuity: usize) -> usize;
    fn interval_first(&self) -> f64;
    fn interval_last(&self) -> f64;
}

/// Linear law: f(x) = a + b*(x - x0).
/// occt: Law_Linear
#[derive(Clone, Debug)]
pub struct LawLinear {
    pub x0: f64,
    pub x1: f64,
    pub v0: f64,
    pub v1: f64,
}

impl LawLinear {
    pub fn new(x0: f64, v0: f64, x1: f64, v1: f64) -> Self {
        assert!(x1 > x0, "Law_Linear: x1 must be > x0");
        Self { x0, x1, v0, v1 }
    }

    fn t(&self, x: f64) -> f64 {
        let span = self.x1 - self.x0;
        if span < 1e-15 { 0.0 } else { (x - self.x0) / span }
    }
}

impl LawFunction for LawLinear {
    fn value(&self, x: f64) -> f64 {
        let t = self.t(x).clamp(0.0, 1.0);
        self.v0 * (1.0 - t) + self.v1 * t
    }

    fn d1(&self, x: f64) -> (f64, f64) {
        let v = self.value(x);
        let span = self.x1 - self.x0;
        let dv = if span > 1e-15 { (self.v1 - self.v0) / span } else { 0.0 };
        (v, dv)
    }

    fn continuity(&self) -> usize { 1 }
    fn nb_intervals(&self, _c: usize) -> usize { 1 }
    fn interval_first(&self) -> f64 { self.x0 }
    fn interval_last(&self) -> f64 { self.x1 }
}

/// Constant law: f(x) = c.
/// occt: Law_Constant
#[derive(Clone, Debug)]
pub struct LawConstant {
    pub x0: f64,
    pub x1: f64,
    pub value: f64,
}

impl LawConstant {
    pub fn new(x0: f64, x1: f64, value: f64) -> Self {
        Self { x0, x1, value }
    }
}

impl LawFunction for LawConstant {
    fn value(&self, _x: f64) -> f64 { self.value }
    fn d1(&self, _x: f64) -> (f64, f64) { (self.value, 0.0) }
    fn continuity(&self) -> usize { usize::MAX }
    fn nb_intervals(&self, _c: usize) -> usize { 1 }
    fn interval_first(&self) -> f64 { self.x0 }
    fn interval_last(&self) -> f64 { self.x1 }
}

/// Piecewise linear interpolating law through a set of (x, v) knots.
// occt-ref: Law_Interpol
#[derive(Clone, Debug)]
pub struct LawInterpol {
    pub knots: Vec<f64>,
    pub values: Vec<f64>,
    pub is_closed: bool,
}

impl LawInterpol {
    pub fn new() -> Self {
        Self { knots: Vec::new(), values: Vec::new(), is_closed: false }
    }

    pub fn set_is_closed(&mut self, c: bool) { self.is_closed = c; }

    pub fn set(&mut self, params: Vec<f64>, values: Vec<f64>) {
        assert_eq!(params.len(), values.len(), "params and values must have same length");
        let mut pairs: Vec<(f64, f64)> = params.into_iter().zip(values).collect();
        pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        self.knots = pairs.iter().map(|p| p.0).collect();
        self.values = pairs.iter().map(|p| p.1).collect();
    }

    pub fn nb_points(&self) -> usize { self.knots.len() }
}

impl Default for LawInterpol {
    fn default() -> Self { Self::new() }
}

impl LawFunction for LawInterpol {
    fn value(&self, x: f64) -> f64 {
        let n = self.knots.len();
        if n == 0 { return 0.0; }
        if n == 1 { return self.values[0]; }
        let x = x.clamp(self.knots[0], self.knots[n - 1]);
        // binary search for interval
        let i = self.knots.partition_point(|&k| k <= x).saturating_sub(1).min(n - 2);
        let x0 = self.knots[i];
        let x1 = self.knots[i + 1];
        let v0 = self.values[i];
        let v1 = self.values[i + 1];
        let span = x1 - x0;
        if span < 1e-15 { return v0; }
        let t = (x - x0) / span;
        v0 * (1.0 - t) + v1 * t
    }

    fn d1(&self, x: f64) -> (f64, f64) {
        let v = self.value(x);
        let n = self.knots.len();
        if n < 2 { return (v, 0.0); }
        let x_c = x.clamp(self.knots[0], self.knots[n - 1]);
        let i = self.knots.partition_point(|&k| k <= x_c).saturating_sub(1).min(n - 2);
        let span = self.knots[i + 1] - self.knots[i];
        let dv = if span > 1e-15 { (self.values[i + 1] - self.values[i]) / span } else { 0.0 };
        (v, dv)
    }

    fn continuity(&self) -> usize { 0 }  // C0 only for piecewise linear
    fn nb_intervals(&self, _c: usize) -> usize { self.knots.len().saturating_sub(1).max(1) }
    fn interval_first(&self) -> f64 { self.knots.first().copied().unwrap_or(0.0) }
    fn interval_last(&self) -> f64 { self.knots.last().copied().unwrap_or(1.0) }
}

/// S-curve (smooth) law: cubic Hermite interpolation at endpoints.
/// occt: Law_S // (Law_Interpol with tangent conditions)
#[derive(Clone, Debug)]
pub struct LawS {
    pub x0: f64,
    pub x1: f64,
    pub v0: f64,
    pub v1: f64,
}

impl LawS {
    pub fn new(x0: f64, v0: f64, x1: f64, v1: f64) -> Self {
        Self { x0, x1, v0, v1 }
    }

    fn t(&self, x: f64) -> f64 {
        let span = self.x1 - self.x0;
        if span < 1e-15 { 0.0 } else { ((x - self.x0) / span).clamp(0.0, 1.0) }
    }
}

impl LawFunction for LawS {
    /// Smooth cubic Hermite (with zero tangent at both ends).
    fn value(&self, x: f64) -> f64 {
        let t = self.t(x);
        // smoothstep: 3t^2 - 2t^3
        let s = t * t * (3.0 - 2.0 * t);
        self.v0 * (1.0 - s) + self.v1 * s
    }

    fn d1(&self, x: f64) -> (f64, f64) {
        let v = self.value(x);
        let t = self.t(x);
        let span = self.x1 - self.x0;
        let ds_dt = 6.0 * t * (1.0 - t);
        let dv_dt = (self.v1 - self.v0) * ds_dt;
        let dv = if span > 1e-15 { dv_dt / span } else { 0.0 };
        (v, dv)
    }

    fn continuity(&self) -> usize { 1 }
    fn nb_intervals(&self, _c: usize) -> usize { 1 }
    fn interval_first(&self) -> f64 { self.x0 }
    fn interval_last(&self) -> f64 { self.x1 }
}

/// Composite law: a sequence of law segments chained by parameter.
// occt-ref: Law_Composite
#[derive(Clone, Debug, Default)]
pub struct LawComposite {
    /// Each segment: (x_start, x_end, constant_value_stub)
    pub segments: Vec<(f64, f64, f64)>,
}

impl LawComposite {
    pub fn new() -> Self { Self::default() }

    pub fn add_constant_segment(&mut self, x0: f64, x1: f64, v: f64) {
        self.segments.push((x0, x1, v));
    }

    pub fn nb_laws(&self) -> usize { self.segments.len() }

    pub fn first(&self) -> f64 { self.segments.first().map(|s| s.0).unwrap_or(0.0) }
    pub fn last(&self) -> f64 { self.segments.last().map(|s| s.1).unwrap_or(1.0) }
}

impl LawFunction for LawComposite {
    fn value(&self, x: f64) -> f64 {
        for &(x0, x1, v) in &self.segments {
            if x <= x1 { return v; }
        }
        self.segments.last().map(|s| s.2).unwrap_or(0.0)
    }

    fn d1(&self, x: f64) -> (f64, f64) { (self.value(x), 0.0) }

    fn continuity(&self) -> usize { 0 }
    fn nb_intervals(&self, _c: usize) -> usize { self.segments.len() }
    fn interval_first(&self) -> f64 { self.first() }
    fn interval_last(&self) -> f64 { self.last() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn law_linear_values() {
        let l = LawLinear::new(0.0, 0.0, 1.0, 10.0);
        assert!((l.value(0.0) - 0.0).abs() < 1e-12);
        assert!((l.value(1.0) - 10.0).abs() < 1e-12);
        assert!((l.value(0.5) - 5.0).abs() < 1e-12);
        let (v, dv) = l.d1(0.5);
        assert!((v - 5.0).abs() < 1e-12);
        assert!((dv - 10.0).abs() < 1e-12);
    }

    #[test]
    fn law_constant_values() {
        let l = LawConstant::new(0.0, 1.0, 7.0);
        assert!((l.value(0.3) - 7.0).abs() < 1e-12);
        let (v, dv) = l.d1(0.5);
        assert!((v - 7.0).abs() < 1e-12);
        assert!(dv.abs() < 1e-12);
        assert_eq!(l.continuity(), usize::MAX);
    }

    #[test]
    fn law_interpol_piecewise() {
        let mut l = LawInterpol::new();
        l.set(vec![0.0, 0.5, 1.0], vec![0.0, 5.0, 10.0]);
        assert!((l.value(0.0) - 0.0).abs() < 1e-10);
        assert!((l.value(0.5) - 5.0).abs() < 1e-10);
        assert!((l.value(0.25) - 2.5).abs() < 1e-10);
        assert!((l.value(1.0) - 10.0).abs() < 1e-10);
        assert_eq!(l.nb_points(), 3);
    }

    #[test]
    fn law_s_smooth() {
        let l = LawS::new(0.0, 0.0, 1.0, 1.0);
        assert!((l.value(0.0) - 0.0).abs() < 1e-12);
        assert!((l.value(1.0) - 1.0).abs() < 1e-12);
        // at t=0.5, smoothstep(0.5) = 0.5
        assert!((l.value(0.5) - 0.5).abs() < 1e-10);
        // derivative at endpoints should be 0
        let (_, d0) = l.d1(0.0);
        assert!(d0.abs() < 1e-10);
    }

    #[test]
    fn law_composite_segments() {
        let mut c = LawComposite::new();
        c.add_constant_segment(0.0, 0.5, 1.0);
        c.add_constant_segment(0.5, 1.0, 2.0);
        assert_eq!(c.nb_laws(), 2);
        assert!((c.value(0.3) - 1.0).abs() < 1e-12);
        assert!((c.value(0.7) - 2.0).abs() < 1e-12);
    }
}
