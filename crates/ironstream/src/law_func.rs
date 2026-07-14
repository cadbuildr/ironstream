// FILE: law_func.rs
// occt: Law_BSpFunc
// occt-ref: Law_Composite, Law_Interpol, Law_Linear

/// Continuity class for law functions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LawContinuity {
    C0, C1, C2,
}

impl Default for LawContinuity {
    fn default() -> Self { Self::C1 }
}

// occt-ref: Law_Linear // — linear function f(t) = a*t + b
#[derive(Clone, Debug)]
pub struct LawLinear {
    pub t_first: f64,
    pub t_last: f64,
    pub v_first: f64,
    pub v_last: f64,
}

impl LawLinear {
    pub fn new(t_first: f64, t_last: f64, v_first: f64, v_last: f64) -> Self {
        Self { t_first, t_last, v_first, v_last }
    }

    pub fn value(&self, t: f64) -> f64 {
        if (self.t_last - self.t_first).abs() < 1e-15 { return self.v_first; }
        let u = (t - self.t_first) / (self.t_last - self.t_first);
        self.v_first + u * (self.v_last - self.v_first)
    }

    pub fn d1(&self, t: f64) -> (f64, f64) {
        (self.value(t), (self.v_last - self.v_first) / (self.t_last - self.t_first + 1e-30))
    }

    pub fn bounds(&self) -> (f64, f64) { (self.t_first, self.t_last) }
    pub fn continuity(&self) -> LawContinuity { LawContinuity::C1 }
    pub fn is_constant(&self) -> bool { (self.v_last - self.v_first).abs() < 1e-12 }
}

// occt-ref: Law_Interpol // — interpolated law through a set of (param, value) pairs
#[derive(Clone, Debug, Default)]
pub struct LawInterpol {
    pub params: Vec<f64>,
    pub values: Vec<f64>,
    pub periodic: bool,
    pub is_built: bool,
}

impl LawInterpol {
    pub fn new() -> Self { Self::default() }

    pub fn set_cubic(&mut self, params: Vec<f64>, values: Vec<f64>, periodic: bool) {
        assert_eq!(params.len(), values.len());
        self.params = params;
        self.values = values;
        self.periodic = periodic;
        self.is_built = !self.params.is_empty();
    }

    pub fn value(&self, t: f64) -> f64 {
        if self.params.is_empty() { return 0.0; }
        if t <= self.params[0] { return self.values[0]; }
        let n = self.params.len();
        if t >= self.params[n - 1] { return self.values[n - 1]; }
        // Linear interpolation between bracketing knots
        for i in 0..n - 1 {
            if t >= self.params[i] && t <= self.params[i + 1] {
                let u = (t - self.params[i]) / (self.params[i + 1] - self.params[i]);
                return self.values[i] + u * (self.values[i + 1] - self.values[i]);
            }
        }
        *self.values.last().unwrap()
    }

    pub fn bounds(&self) -> (f64, f64) {
        if self.params.is_empty() { return (0.0, 1.0); }
        (*self.params.first().unwrap(), *self.params.last().unwrap())
    }

    pub fn nb_intervals(&self) -> usize { self.params.len().saturating_sub(1) }
    pub fn is_done(&self) -> bool { self.is_built }
}

/// A segment in a composite law.
#[derive(Clone, Debug)]
pub struct LawSegment {
    pub t_first: f64,
    pub t_last: f64,
    pub kind: LawSegmentKind,
}

#[derive(Clone, Debug)]
pub enum LawSegmentKind {
    Linear(LawLinear),
    Interpol(LawInterpol),
    Constant(f64),
}

impl LawSegment {
    pub fn linear(t_first: f64, t_last: f64, v_first: f64, v_last: f64) -> Self {
        Self { t_first, t_last, kind: LawSegmentKind::Linear(LawLinear::new(t_first, t_last, v_first, v_last)) }
    }

    pub fn constant(t_first: f64, t_last: f64, v: f64) -> Self {
        Self { t_first, t_last, kind: LawSegmentKind::Constant(v) }
    }

    pub fn value(&self, t: f64) -> f64 {
        match &self.kind {
            LawSegmentKind::Linear(l) => l.value(t),
            LawSegmentKind::Interpol(i) => i.value(t),
            LawSegmentKind::Constant(v) => *v,
        }
    }
}

// occt-ref: Law_Composite // — piecewise law made of sub-laws on contiguous intervals
#[derive(Clone, Debug, Default)]
pub struct LawComposite {
    pub segments: Vec<LawSegment>,
    pub t_first: f64,
    pub t_last: f64,
}

impl LawComposite {
    pub fn new() -> Self { Self::default() }

    pub fn add_segment(&mut self, seg: LawSegment) {
        if self.segments.is_empty() { self.t_first = seg.t_first; }
        self.t_last = seg.t_last;
        self.segments.push(seg);
    }

    pub fn value(&self, t: f64) -> f64 {
        for seg in &self.segments {
            if t >= seg.t_first && t <= seg.t_last { return seg.value(t); }
        }
        self.segments.last().map(|s| s.value(s.t_last)).unwrap_or(0.0)
    }

    pub fn nb_laws(&self) -> usize { self.segments.len() }
    pub fn bounds(&self) -> (f64, f64) { (self.t_first, self.t_last) }
    pub fn is_empty(&self) -> bool { self.segments.is_empty() }

    pub fn law_at(&self, i: usize) -> Option<&LawSegment> { self.segments.get(i) }
}

// occt: Law_BSpFunc // — B-Spline-based law function
#[derive(Clone, Debug)]
pub struct LawBSpFunc {
    pub knots: Vec<f64>,
    pub poles: Vec<f64>,
    pub mults: Vec<u32>,
    pub degree: u32,
    pub is_periodic: bool,
}

impl LawBSpFunc {
    pub fn new(degree: u32, knots: Vec<f64>, poles: Vec<f64>, mults: Vec<u32>) -> Self {
        Self { knots, poles, mults, degree, is_periodic: false }
    }

    pub fn value(&self, t: f64) -> f64 {
        if self.poles.is_empty() { return 0.0; }
        // Stub: clamp to pole range
        if self.knots.is_empty() { return self.poles[0]; }
        let first = *self.knots.first().unwrap();
        let last = *self.knots.last().unwrap();
        let u = ((t - first) / (last - first + 1e-30)).clamp(0.0, 1.0);
        let idx = ((u * (self.poles.len() - 1) as f64) as usize).min(self.poles.len() - 1);
        self.poles[idx]
    }

    pub fn bounds(&self) -> (f64, f64) {
        if self.knots.is_empty() { return (0.0, 1.0); }
        (*self.knots.first().unwrap(), *self.knots.last().unwrap())
    }

    pub fn nb_poles(&self) -> usize { self.poles.len() }
    pub fn nb_knots(&self) -> usize { self.knots.len() }
    pub fn degree(&self) -> u32 { self.degree }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn law_linear_value() {
        let l = LawLinear::new(0.0, 1.0, 2.0, 4.0);
        assert!((l.value(0.0) - 2.0).abs() < 1e-10);
        assert!((l.value(1.0) - 4.0).abs() < 1e-10);
        assert!((l.value(0.5) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn law_linear_constant() {
        let l = LawLinear::new(0.0, 1.0, 5.0, 5.0);
        assert!(l.is_constant());
    }

    #[test]
    fn law_interpol_basic() {
        let mut li = LawInterpol::new();
        li.set_cubic(vec![0.0, 0.5, 1.0], vec![1.0, 2.0, 1.0], false);
        assert!(li.is_done());
        assert!((li.value(0.0) - 1.0).abs() < 1e-10);
        assert!((li.value(1.0) - 1.0).abs() < 1e-10);
        assert!(li.nb_intervals() == 2);
    }

    #[test]
    fn law_composite_basic() {
        let mut c = LawComposite::new();
        c.add_segment(LawSegment::linear(0.0, 0.5, 1.0, 2.0));
        c.add_segment(LawSegment::constant(0.5, 1.0, 2.0));
        assert_eq!(c.nb_laws(), 2);
        assert!((c.value(0.25) - 1.5).abs() < 1e-10);
        assert!((c.value(0.75) - 2.0).abs() < 1e-10);
    }

    #[test]
    fn law_bspfunc_basic() {
        let f = LawBSpFunc::new(3,
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
            vec![1.0, 1.5, 2.0, 1.0],
            vec![4, 4]);
        assert_eq!(f.nb_poles(), 4);
        let (a, b) = f.bounds();
        assert!(a < b);
    }
}
