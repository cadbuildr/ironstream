// FILE: rust/ironstream/crates/ironstream/src/law_ext.rs
//! Extended scalar law utilities mirroring OpenCascade's
//! `Law_BSplineKnotSplitting` and extended `Law_Interpolate` helpers.
//!
//! This module provides:
//!
//! - [`KnotSplitting`] — determines where a B-spline knot sequence can be
//!   split into independent Bézier or C^k-continuous segments, mirroring
//!   `Law_BSplineKnotSplitting`.
//! - [`LawS`] — a scaled smooth-step law that maps `[0, 1]` to a user-defined
//!   range via the S-curve polynomial, with first-derivative support.
//! - [`law_compose`] — pointwise composition of two sampled laws.
//! - [`law_scale`] — pointwise scaling of a sampled law by a constant factor.
//!
//! All types are zero-dependency and use only `std`.

// occt: Law_BSplineKnotSplitting

// ─────────────────────────────────────────────────────────────────────────────
// KnotSplitting
// ─────────────────────────────────────────────────────────────────────────────

/// Determines the indices at which a B-spline knot sequence can be split into
/// independent sub-curves, mirroring OpenCascade's `Law_BSplineKnotSplitting`.
///
/// A split occurs at knot index `i` when the multiplicity of that knot equals
/// or exceeds the requested continuity order, meaning the spline can be broken
/// at that parameter without loss of information about either sub-curve.
///
/// # Usage
/// ```ignore
/// let ks = KnotSplitting::new(knots, mults);
/// // Split at C^0 boundaries (multiplicity == degree + 1 for a degree-d spline):
/// ks.compute_splits(continuity);
/// let n = ks.num_intervals();
/// ```
// occt: Law_BSplineKnotSplitting
#[derive(Clone, Debug)]
pub struct KnotSplitting {
    /// Strictly increasing distinct knot values.
    pub knots: Vec<f64>,
    /// Multiplicity of each distinct knot; same length as `knots`.
    pub multiplicities: Vec<u32>,
    /// 0-based indices into `knots` at which the sequence can be split for
    /// the most recently requested continuity order.
    ///
    /// Populated by [`KnotSplitting::compute_splits`]; empty until that method
    /// is called.
    pub split_indices: Vec<usize>,
}

impl KnotSplitting {
    /// Construct a new `KnotSplitting` from distinct knot values and their
    /// multiplicities.
    ///
    /// The `split_indices` field is initially empty; call
    /// [`compute_splits`](Self::compute_splits) to populate it.
    ///
    /// # Panics
    /// Panics when `knots` and `mults` have different lengths or when fewer
    /// than two distinct knots are provided.
    pub fn new(knots: Vec<f64>, mults: Vec<u32>) -> Self {
        assert_eq!(
            knots.len(),
            mults.len(),
            "KnotSplitting: knots and mults must have the same length"
        );
        assert!(
            knots.len() >= 2,
            "KnotSplitting: at least two distinct knots are required"
        );
        Self {
            knots,
            multiplicities: mults,
            split_indices: Vec::new(),
        }
    }

    /// Compute the indices at which the knot sequence can be split while
    /// preserving at least `continuity`-order continuity across each split.
    ///
    /// A split is recorded at knot index `i` (for interior knots only, i.e.
    /// `0 < i < knots.len() - 1`) whenever the multiplicity at that knot is
    /// strictly greater than `continuity`.  This corresponds to the OCCT
    /// `Law_BSplineKnotSplitting` criterion: the degree of the spline minus the
    /// required continuity order gives the minimum multiplicity threshold.
    ///
    /// The first and last knots (boundary knots) are never recorded as split
    /// indices because splitting there would yield an empty sub-interval.
    ///
    /// After this call [`split_indices`](Self::split_indices) is fully replaced.
    pub fn compute_splits(&mut self, continuity: u32) {
        self.split_indices.clear();
        let n = self.knots.len();
        // Examine only interior knots: indices 1 .. n-2 inclusive.
        for i in 1..n.saturating_sub(1) {
            if self.multiplicities[i] > continuity {
                self.split_indices.push(i);
            }
        }
    }

    /// Return the number of parameter intervals defined by the current set of
    /// split indices.
    ///
    /// If no splits have been computed this equals 1 (the whole knot range is
    /// one interval). Otherwise it equals `split_indices.len() + 1`.
    pub fn num_intervals(&self) -> usize {
        self.split_indices.len() + 1
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LawS
// ─────────────────────────────────────────────────────────────────────────────

/// A scaled S-curve law that maps the normalised parameter `t ∈ [0, 1]` to a
/// user-supplied range `[0, scale_factor]` using the smooth-step polynomial
/// `s(t) = 3t² − 2t³`.
///
/// The derivative is `s'(t) = 6t − 6t²`, scaled by `scale_factor`.
///
/// `law_values` is an optional pre-sampled cache of evaluated values; it is not
/// used by [`evaluate`](Self::evaluate) or [`derivative`](Self::derivative)
/// (which are computed analytically) but is available for export or plotting.
///
/// # Relationship to OCCT
/// This corresponds to a simplified, parameterisation-independent form of
/// `Law_S` (see `Law_S.hxx`) with an explicit scale factor and pre-sampled
/// cache, suitable for use in sweeping and evolution algorithms.
// occt: Law_BSplineKnotSplitting
#[derive(Clone, Debug)]
pub struct LawS {
    /// The output range upper bound: `evaluate(1.0) == scale_factor`.
    pub scale_factor: f64,
    /// Optional pre-sampled law values for export / debugging.
    pub law_values: Vec<f64>,
}

impl LawS {
    /// Construct a new `LawS` with the given `scale` factor and an empty
    /// `law_values` cache.
    pub fn new(scale: f64) -> Self {
        Self {
            scale_factor: scale,
            law_values: Vec::new(),
        }
    }

    /// Evaluate the scaled S-curve at normalised parameter `t`.
    ///
    /// `t` is clamped to `[0, 1]` before evaluation, so the return value is
    /// always in `[0, scale_factor]` (assuming `scale_factor >= 0`).
    ///
    /// Uses the smooth-step formula `3t² − 2t³` scaled by `scale_factor`.
    pub fn evaluate(&self, t: f64) -> f64 {
        let u = t.clamp(0.0, 1.0);
        self.scale_factor * u * u * (3.0 - 2.0 * u)
    }

    /// Evaluate the first derivative of the S-curve at normalised parameter `t`.
    ///
    /// `t` is clamped to `[0, 1]`. The derivative of `3t² − 2t³` is
    /// `6t − 6t²`, scaled by `scale_factor`.
    pub fn derivative(&self, t: f64) -> f64 {
        let u = t.clamp(0.0, 1.0);
        self.scale_factor * 6.0 * u * (1.0 - u)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Free functions
// ─────────────────────────────────────────────────────────────────────────────

/// Pointwise compose two sampled laws `f` and `g` into a new law whose `i`-th
/// value is `f[i] * g[i]`.
///
/// This is the simplest form of law composition used in sweep evolution: each
/// position along the spine contributes both a profile scaling (`f`) and a
/// twist/orientation factor (`g`), and the combined law is their product at
/// each sample point.
///
/// If `f` and `g` differ in length the result has length `min(f.len(), g.len())`
/// and the trailing samples of the longer slice are ignored, matching the OCCT
/// convention for truncated law arrays.
///
/// # Examples
/// ```ignore
/// let f = vec![1.0, 2.0, 3.0];
/// let g = vec![0.5, 1.0, 2.0];
/// let h = law_compose(&f, &g);
/// assert_eq!(h, vec![0.5, 2.0, 6.0]);
/// ```
pub fn law_compose(f: &[f64], g: &[f64]) -> Vec<f64> {
    let len = f.len().min(g.len());
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        out.push(f[i] * g[i]);
    }
    out
}

/// Pointwise scale a sampled law by a constant `factor`.
///
/// Returns a new `Vec<f64>` of the same length as `values` where every element
/// has been multiplied by `factor`.  Corresponds to the scalar multiplication
/// used in `Law_Scale` within OCCT's evolution law algebra.
///
/// # Examples
/// ```ignore
/// let v = vec![1.0, 2.0, 4.0];
/// let scaled = law_scale(&v, 0.5);
/// assert_eq!(scaled, vec![0.5, 1.0, 2.0]);
/// ```
pub fn law_scale(values: &[f64], factor: f64) -> Vec<f64> {
    values.iter().map(|&v| v * factor).collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── KnotSplitting ─────────────────────────────────────────────────────────

    #[test]
    fn knot_splitting_new_basic() {
        let ks = KnotSplitting::new(vec![0.0, 0.5, 1.0], vec![3, 1, 3]);
        assert_eq!(ks.knots.len(), 3);
        assert_eq!(ks.multiplicities.len(), 3);
        assert!(ks.split_indices.is_empty());
    }

    #[test]
    #[should_panic]
    fn knot_splitting_mismatched_lengths_panics() {
        let _ = KnotSplitting::new(vec![0.0, 1.0], vec![1, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn knot_splitting_too_few_knots_panics() {
        let _ = KnotSplitting::new(vec![0.0], vec![4]);
    }

    #[test]
    fn compute_splits_no_interior_splits() {
        // Interior knot has multiplicity 1, continuity 2 → no splits recorded.
        let mut ks = KnotSplitting::new(vec![0.0, 0.5, 1.0], vec![3, 1, 3]);
        ks.compute_splits(2);
        assert!(ks.split_indices.is_empty());
        assert_eq!(ks.num_intervals(), 1);
    }

    #[test]
    fn compute_splits_high_multiplicity() {
        // Interior knot at index 1 has multiplicity 3, continuity 2 → split.
        let mut ks = KnotSplitting::new(vec![0.0, 0.5, 1.0], vec![3, 3, 3]);
        ks.compute_splits(2);
        assert_eq!(ks.split_indices, vec![1]);
        assert_eq!(ks.num_intervals(), 2);
    }

    #[test]
    fn compute_splits_multiple_interior() {
        // Three interior knots with multiplicity 4 each, continuity 3.
        let knots = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let mults = vec![4, 4, 4, 4, 4];
        let mut ks = KnotSplitting::new(knots, mults);
        ks.compute_splits(3);
        // Indices 1, 2, 3 are interior and have multiplicity 4 > 3.
        assert_eq!(ks.split_indices, vec![1, 2, 3]);
        assert_eq!(ks.num_intervals(), 4);
    }

    #[test]
    fn compute_splits_boundary_knots_never_recorded() {
        // Even if boundary multiplicities exceed continuity, they are not
        // recorded as split indices.
        let knots = vec![0.0, 0.5, 1.0];
        let mults = vec![10, 1, 10];
        let mut ks = KnotSplitting::new(knots, mults);
        ks.compute_splits(2);
        assert!(ks.split_indices.is_empty());
        assert_eq!(ks.num_intervals(), 1);
    }

    #[test]
    fn compute_splits_resets_previous_results() {
        let knots = vec![0.0, 0.5, 1.0];
        let mults = vec![3, 3, 3];
        let mut ks = KnotSplitting::new(knots, mults);
        ks.compute_splits(2); // records index 1
        assert_eq!(ks.split_indices.len(), 1);
        ks.compute_splits(3); // multiplicity 3 is not > 3 → no splits
        assert!(ks.split_indices.is_empty());
    }

    // ── LawS ──────────────────────────────────────────────────────────────────

    #[test]
    fn laws_evaluate_endpoints() {
        let law = LawS::new(2.0);
        assert!((law.evaluate(0.0) - 0.0).abs() < 1e-14);
        assert!((law.evaluate(1.0) - 2.0).abs() < 1e-14);
    }

    #[test]
    fn laws_evaluate_midpoint() {
        let law = LawS::new(1.0);
        // s(0.5) = 3*0.25 - 2*0.125 = 0.75 - 0.25 = 0.5
        assert!((law.evaluate(0.5) - 0.5).abs() < 1e-14);
    }

    #[test]
    fn laws_evaluate_clamps() {
        let law = LawS::new(3.0);
        assert!((law.evaluate(-1.0) - 0.0).abs() < 1e-14);
        assert!((law.evaluate(2.0) - 3.0).abs() < 1e-14);
    }

    #[test]
    fn laws_derivative_endpoints() {
        let law = LawS::new(1.0);
        // derivative at 0 and 1 should be 0.
        assert!((law.derivative(0.0) - 0.0).abs() < 1e-14);
        assert!((law.derivative(1.0) - 0.0).abs() < 1e-14);
    }

    #[test]
    fn laws_derivative_midpoint() {
        let law = LawS::new(1.0);
        // s'(0.5) = 6*0.5*0.5 = 1.5
        assert!((law.derivative(0.5) - 1.5).abs() < 1e-14);
    }

    #[test]
    fn laws_scale_factor_applied() {
        let law = LawS::new(4.0);
        assert!((law.evaluate(0.5) - 2.0).abs() < 1e-14);
        assert!((law.derivative(0.5) - 6.0).abs() < 1e-14);
    }

    #[test]
    fn laws_new_law_values_empty() {
        let law = LawS::new(1.0);
        assert!(law.law_values.is_empty());
    }

    // ── law_compose ───────────────────────────────────────────────────────────

    #[test]
    fn law_compose_same_length() {
        let f = vec![1.0, 2.0, 3.0];
        let g = vec![0.5, 1.0, 2.0];
        let h = law_compose(&f, &g);
        assert_eq!(h, vec![0.5, 2.0, 6.0]);
    }

    #[test]
    fn law_compose_f_shorter() {
        let f = vec![2.0, 3.0];
        let g = vec![1.0, 2.0, 4.0];
        let h = law_compose(&f, &g);
        assert_eq!(h, vec![2.0, 6.0]);
    }

    #[test]
    fn law_compose_g_shorter() {
        let f = vec![1.0, 2.0, 4.0];
        let g = vec![2.0, 3.0];
        let h = law_compose(&f, &g);
        assert_eq!(h, vec![2.0, 6.0]);
    }

    #[test]
    fn law_compose_empty() {
        let h = law_compose(&[], &[1.0, 2.0]);
        assert!(h.is_empty());
    }

    // ── law_scale ─────────────────────────────────────────────────────────────

    #[test]
    fn law_scale_basic() {
        let v = vec![1.0, 2.0, 4.0];
        let s = law_scale(&v, 0.5);
        assert_eq!(s, vec![0.5, 1.0, 2.0]);
    }

    #[test]
    fn law_scale_by_zero() {
        let v = vec![1.0, 2.0, 3.0];
        let s = law_scale(&v, 0.0);
        assert_eq!(s, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn law_scale_empty() {
        let s = law_scale(&[], 3.0);
        assert!(s.is_empty());
    }

    #[test]
    fn law_scale_negative_factor() {
        let v = vec![1.0, -2.0, 3.0];
        let s = law_scale(&v, -1.0);
        assert_eq!(s, vec![-1.0, 2.0, -3.0]);
    }
}
