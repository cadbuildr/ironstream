// FILE: rust/ironstream/crates/ironstream/src/law_composite.rs
//! `Law_Composite` — piecewise scalar evolution law, mirroring OpenCascade's
//! `Law_Composite` (and supporting types `Law_Constant`, `Law_Linear`).
//!
//! This module provides a self-contained, zero-dependency piecewise law that
//! maps a 1-D parameter `t` to a real value. It is used by sweeping and
//! offsetting algorithms to define how a scalar quantity varies along a spine.

// occt: Law_Function type — enum Constant, Linear, BSpline, Composite
/// Discriminant tag for a law segment's underlying kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LawFunctionType {
    /// A constant scalar law.
    Constant,
    /// A linear (affine) scalar law.
    Linear,
    /// A B-spline scalar law.
    BSpline,
    /// A piecewise composite law.
    Composite,
}

// ─────────────────────────────────────────────────────────────────────────────
// LawConstant
// ─────────────────────────────────────────────────────────────────────────────

// occt: Law_Constant — constant scalar function
/// A constant evolution law: `f(t) = value` for all `t` in `[first, last]`.
#[derive(Clone, Debug)]
pub struct LawConstant {
    /// The constant scalar value.
    pub value: f64,
    /// Start of the parameter domain.
    pub first: f64,
    /// End of the parameter domain.
    pub last: f64,
}

impl LawConstant {
    /// Create a constant law with the given `value` over `[first, last]`.
    ///
    /// # Panics
    /// Panics if `last <= first`.
    pub fn new(value: f64, first: f64, last: f64) -> Self {
        assert!(
            last > first,
            "LawConstant: last ({last}) must be > first ({first})"
        );
        Self { value, first, last }
    }

    /// Return the constant scalar value (does not depend on `t`).
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Evaluate the law at parameter `t`; always returns the constant value.
    pub fn evaluate(&self, _t: f64) -> f64 {
        self.value
    }

    /// Return the parameter bounds `(first, last)`.
    pub fn bounds(&self) -> (f64, f64) {
        (self.first, self.last)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LawLinear
// ─────────────────────────────────────────────────────────────────────────────

// occt: Law_Linear — linear interpolation f(first)=start, f(last)=end
/// A linear evolution law: `f(first) = start`, `f(last) = end_val`, with
/// affine interpolation in between.
#[derive(Clone, Debug)]
pub struct LawLinear {
    /// Value at `first`.
    pub start: f64,
    /// Value at `last`.
    pub end_val: f64,
    /// Start of the parameter domain.
    pub first: f64,
    /// End of the parameter domain.
    pub last: f64,
}

impl LawLinear {
    /// Create a linear law over `[first, last]` from `start` to `end_val`.
    ///
    /// # Panics
    /// Panics if `last <= first`.
    pub fn new(first: f64, last: f64, start: f64, end_val: f64) -> Self {
        assert!(
            last > first,
            "LawLinear: last ({last}) must be > first ({first})"
        );
        Self {
            start,
            end_val,
            first,
            last,
        }
    }

    /// Evaluate the law at parameter `t`.
    ///
    /// `t` is clamped to `[first, last]` before interpolation.
    pub fn evaluate(&self, t: f64) -> f64 {
        let t_clamped = t.clamp(self.first, self.last);
        let frac = (t_clamped - self.first) / (self.last - self.first);
        self.start + frac * (self.end_val - self.start)
    }

    /// Return the parameter bounds `(first, last)`.
    pub fn bounds(&self) -> (f64, f64) {
        (self.first, self.last)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LawSegment
// ─────────────────────────────────────────────────────────────────────────────

/// A single segment in a [`LawComposite`], wrapping either a constant or a
/// linear sub-law.
///
/// Used instead of a boxed closure so that the composite remains `Clone` and
/// `Debug` without any heap-allocated trait objects.
// occt: Law_Composite segment variant — enum wrapper for sub-laws
#[derive(Clone, Debug)]
pub enum LawSegment {
    /// A constant sub-law.
    Constant(LawConstant),
    /// A linear sub-law.
    Linear(LawLinear),
}

impl LawSegment {
    /// Evaluate this segment at parameter `t`.
    pub fn evaluate(&self, t: f64) -> f64 {
        match self {
            LawSegment::Constant(c) => c.evaluate(t),
            LawSegment::Linear(l) => l.evaluate(t),
        }
    }

    /// Parameter start of this segment.
    pub fn first(&self) -> f64 {
        match self {
            LawSegment::Constant(c) => c.first,
            LawSegment::Linear(l) => l.first,
        }
    }

    /// Parameter end of this segment.
    pub fn last(&self) -> f64 {
        match self {
            LawSegment::Constant(c) => c.last,
            LawSegment::Linear(l) => l.last,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LawComposite
// ─────────────────────────────────────────────────────────────────────────────

// occt: Law_Composite — piecewise law made of ordered LawSegment intervals
/// A piecewise scalar evolution law made of ordered constant or linear
/// segments.
///
/// Segments are stored in ascending parameter order. Evaluation clamps `t`
/// to the overall domain and delegates to the containing segment.
#[derive(Clone, Debug, Default)]
pub struct LawComposite {
    /// Ordered list of segments, each covering a contiguous sub-interval.
    laws: Vec<LawSegment>,
}

impl LawComposite {
    /// Create an empty composite law.
    pub fn new() -> Self {
        Self { laws: Vec::new() }
    }

    /// Append a constant segment `[first, last]` with the given `value`.
    ///
    /// # Panics
    /// Panics if `last <= first`, or if the new segment does not begin where
    /// the previous one ended.
    pub fn add_constant(&mut self, first: f64, last: f64, value: f64) {
        self.check_contiguous(first, last);
        self.laws
            .push(LawSegment::Constant(LawConstant::new(value, first, last)));
    }

    /// Append a linear segment `[first, last]` from `start` to `end_val`.
    ///
    /// # Panics
    /// Panics if `last <= first`, or if the new segment does not begin where
    /// the previous one ended.
    pub fn add_linear(&mut self, first: f64, last: f64, start: f64, end_val: f64) {
        self.check_contiguous(first, last);
        self.laws
            .push(LawSegment::Linear(LawLinear::new(first, last, start, end_val)));
    }

    /// Evaluate the composite law at parameter `t`.
    ///
    /// If `t` is outside `[bounds().0, bounds().1]` it is clamped to the
    /// nearest segment boundary before evaluation.
    ///
    /// # Panics
    /// Panics if no segments have been added.
    pub fn evaluate(&self, t: f64) -> f64 {
        assert!(!self.laws.is_empty(), "LawComposite: no segments defined");
        let (lo, hi) = self.bounds();
        let t_clamped = t.clamp(lo, hi);
        let idx = self.find_segment(t_clamped);
        self.laws[idx].evaluate(t_clamped)
    }

    /// Return the number of segments in this composite.
    pub fn nb_laws(&self) -> usize {
        self.laws.len()
    }

    /// Return the overall parameter bounds `(first, last)`.
    ///
    /// # Panics
    /// Panics if no segments have been added.
    pub fn bounds(&self) -> (f64, f64) {
        assert!(!self.laws.is_empty(), "LawComposite: no segments defined");
        (self.laws[0].first(), self.laws[self.laws.len() - 1].last())
    }

    // ── Internals ─────────────────────────────────────────────────────────────

    /// Validate that a new segment `[first, last]` is contiguous with the
    /// previous one (or is the first segment), and that `last > first`.
    fn check_contiguous(&self, first: f64, last: f64) {
        assert!(
            last > first,
            "LawComposite: last ({last}) must be > first ({first})"
        );
        if let Some(prev) = self.laws.last() {
            let prev_last = prev.last();
            assert!(
                (first - prev_last).abs() < 1e-10,
                "LawComposite: new segment first ({first}) must equal previous last ({prev_last})"
            );
        }
    }

    /// Binary search: return the index of the segment containing `t`.
    ///
    /// `t` is assumed to already be clamped to the valid domain.
    fn find_segment(&self, t: f64) -> usize {
        let n = self.laws.len();
        let mut lo = 0usize;
        let mut hi = n - 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if t < self.laws[mid].last() {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo.min(n - 1)
    }
}
