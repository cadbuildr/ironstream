// FILE: law_function.rs
//! `Law` — evolution law package, mirroring OpenCascade's `Law` package
//! (`src/ModelingAlgorithms/TKGeomAlgo/Law`).
//!
//! An evolution law maps a 1-D real parameter `t` to a real value (and
//! optionally its derivative). Laws are used by sweeping algorithms
//! (`BRepOffsetAPI_MakePipe`, `GeomFill_*`) to define how radius, scale,
//! rotation angle, etc. vary along the spine curve.
//!
//! # Covered types
//! - [`LawFunction`]  — abstract base trait
//! - [`LawLinear`]    — affine law:  `f(t) = v0 + (v1 - v0) * (t - t0) / (t1 - t0)`
//! - [`LawInterpol`]  — cubic-Hermite interpolation through `(t_i, v_i)` samples
//! - [`LawBSpline`]   — B-spline law defined by poles and a knot vector
//! - [`LawComposite`] — piecewise composition of other laws

use crate::precision;

// ─────────────────────────────────────────────────────────────────────────────
// Trait
// ─────────────────────────────────────────────────────────────────────────────

/// Trait shared by all evolution laws.
///
/// Mirrors `Law_Function` from OCCT.
// occt: Law_Function
pub trait LawFunction: std::fmt::Debug {
    /// Evaluate the law at parameter `t`, returning `(value, derivative)`.
    fn value_and_d1(&self, t: f64) -> (f64, f64);

    /// Evaluate the law value at parameter `t`.
    fn value(&self, t: f64) -> f64 {
        self.value_and_d1(t).0
    }

    /// Evaluate the first derivative of the law at `t`.
    fn d1(&self, t: f64) -> f64 {
        self.value_and_d1(t).1
    }

    /// First valid parameter.
    fn first_parameter(&self) -> f64;

    /// Last valid parameter.
    fn last_parameter(&self) -> f64;

    /// Bounds `(first, last)`.
    fn bounds(&self) -> (f64, f64) {
        (self.first_parameter(), self.last_parameter())
    }

    /// Continuity: `0` = C0, `1` = C1, `2` = C2, `3` = CN.
    fn continuity(&self) -> u8;
}

// ─────────────────────────────────────────────────────────────────────────────
// LawLinear
// ─────────────────────────────────────────────────────────────────────────────

/// A linear (affine) evolution law.
///
/// On `[t0, t1]` the law is:
/// ```text
/// f(t) = v0 + (v1 - v0) * (t - t0) / (t1 - t0)
/// ```
/// Outside the interval the law is clamped (OCCT `Law_Linear` behaviour).
// occt: Law_Linear
#[derive(Clone, Debug)]
pub struct LawLinear {
    t0: f64,
    t1: f64,
    v0: f64,
    v1: f64,
}

impl LawLinear {
    /// Create a linear law between `(t0, v0)` and `(t1, v1)`.
    ///
    /// # Panics
    /// Panics if `t1 <= t0` (degenerate interval).
    pub fn new(t0: f64, v0: f64, t1: f64, v1: f64) -> Self {
        assert!(
            t1 > t0 + precision::CONFUSION,
            "LawLinear: t1 must be strictly greater than t0"
        );
        Self { t0, t1, v0, v1 }
    }

    /// Set the value at the start of the interval.
    pub fn set_value_at_start(&mut self, v: f64) {
        self.v0 = v;
    }

    /// Set the value at the end of the interval.
    pub fn set_value_at_end(&mut self, v: f64) {
        self.v1 = v;
    }

    /// Value at the start of the interval.
    pub fn value_at_start(&self) -> f64 {
        self.v0
    }

    /// Value at the end of the interval.
    pub fn value_at_end(&self) -> f64 {
        self.v1
    }
}

impl LawFunction for LawLinear {
    fn value_and_d1(&self, t: f64) -> (f64, f64) {
        let len = self.t1 - self.t0;
        let slope = (self.v1 - self.v0) / len;
        let t_clamped = t.clamp(self.t0, self.t1);
        let v = self.v0 + slope * (t_clamped - self.t0);
        (v, slope)
    }

    fn first_parameter(&self) -> f64 {
        self.t0
    }

    fn last_parameter(&self) -> f64 {
        self.t1
    }

    fn continuity(&self) -> u8 {
        // Affine ⇒ infinitely differentiable (C∞), but OCCT reports C1 for
        // a single-segment linear law since it has one piece.
        1
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LawInterpol
// ─────────────────────────────────────────────────────────────────────────────

/// A C2 cubic-spline interpolation law.
///
/// Given a table of `(t_i, v_i)` samples the law constructs a natural cubic
/// spline (second derivatives zero at the endpoints) that passes exactly through
/// every sample. This mirrors `Law_Interpol` from OCCT.
///
/// # Example
/// ```
/// use ironstream::law_function::LawInterpol;
/// use ironstream::law_function::LawFunction;
/// let law = LawInterpol::new(&[0.0, 1.0, 2.0], &[1.0, 2.0, 1.0], false).unwrap();
/// // The law passes exactly through each sample.
/// assert!((law.value(0.0) - 1.0).abs() < 1e-10);
/// assert!((law.value(2.0) - 1.0).abs() < 1e-10);
/// ```
// occt: Law_Interpol
#[derive(Clone, Debug)]
pub struct LawInterpol {
    /// Parameter knots (strictly increasing).
    params: Vec<f64>,
    /// Values at each knot.
    values: Vec<f64>,
    /// Second derivatives at each knot (natural spline).
    d2: Vec<f64>,
    /// Whether the spline wraps (endpoints matched).
    #[allow(dead_code)]
    periodic: bool,
}

impl LawInterpol {
    /// Build a C2 natural cubic spline through the `(params[i], values[i])`
    /// samples.  `params` must be strictly increasing.
    ///
    /// When `periodic` is `true` the spline wraps: `values[0]` must equal
    /// `values[last]` and the first derivative is matched at the endpoints.
    ///
    /// Returns `Err` if the inputs are inconsistent.
    pub fn new(params: &[f64], values: &[f64], periodic: bool) -> Result<Self, &'static str> {
        let n = params.len();
        if n < 2 {
            return Err("LawInterpol: need at least 2 samples");
        }
        if values.len() != n {
            return Err("LawInterpol: params and values must have the same length");
        }
        for i in 1..n {
            if params[i] <= params[i - 1] + precision::CONFUSION {
                return Err("LawInterpol: params must be strictly increasing");
            }
        }
        if periodic && (values[0] - values[n - 1]).abs() > precision::CONFUSION {
            return Err("LawInterpol: periodic spline requires values[0] == values[last]");
        }

        let d2 = Self::solve_natural_spline(params, values);
        Ok(Self {
            params: params.to_vec(),
            values: values.to_vec(),
            d2,
            periodic,
        })
    }

    /// Tridiagonal system solve for natural cubic-spline second derivatives.
    ///
    /// Uses the standard three-point recurrence (Burden & Faires §3.5).
    fn solve_natural_spline(params: &[f64], values: &[f64]) -> Vec<f64> {
        let n = params.len();
        let m = n - 1; // number of intervals
        let mut h = vec![0.0; m];
        for i in 0..m {
            h[i] = params[i + 1] - params[i];
        }

        // Build the right-hand side (d[i] for i=1..m-1).
        let mut alpha = vec![0.0; m]; // alpha[1..m-1]
        for i in 1..m {
            alpha[i] =
                3.0 * ((values[i + 1] - values[i]) / h[i] - (values[i] - values[i - 1]) / h[i - 1]);
        }

        // Forward sweep.
        let mut l = vec![0.0; n];
        let mut mu = vec![0.0; n];
        let mut z = vec![0.0; n];
        l[0] = 1.0;
        mu[0] = 0.0;
        z[0] = 0.0;

        for i in 1..m {
            l[i] = 2.0 * (params[i + 1] - params[i - 1]) - h[i - 1] * mu[i - 1];
            mu[i] = h[i] / l[i];
            z[i] = (alpha[i] - h[i - 1] * z[i - 1]) / l[i];
        }

        l[m] = 1.0;
        z[m] = 0.0;

        // Back substitution — second derivatives.
        let mut d2 = vec![0.0; n];
        // d2[m] = 0 (natural BC)
        for j in (0..m).rev() {
            d2[j] = z[j] - mu[j] * d2[j + 1];
        }
        d2
    }

    /// Locate the interval index `i` such that `params[i] <= t < params[i+1]`.
    fn find_interval(&self, t: f64) -> usize {
        let n = self.params.len();
        // Binary search.
        let mut lo = 0usize;
        let mut hi = n - 2;
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if self.params[mid] <= t {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        lo
    }

    /// Evaluate the cubic polynomial in interval `i` at `t`.
    ///
    /// Returns `(value, first_derivative)`.
    fn eval_segment(&self, i: usize, t: f64) -> (f64, f64) {
        let h = self.params[i + 1] - self.params[i];
        let dt = t - self.params[i];
        let dt2 = dt * dt;
        let dt3 = dt2 * dt;

        let a = self.values[i];
        let b = (self.values[i + 1] - self.values[i]) / h
            - h * (2.0 * self.d2[i] + self.d2[i + 1]) / 6.0;
        let c = self.d2[i] / 2.0;
        let d = (self.d2[i + 1] - self.d2[i]) / (6.0 * h);

        let v = a + b * dt + c * dt2 + d * dt3;
        let dv = b + 2.0 * c * dt + 3.0 * d * dt2;
        (v, dv)
    }

    /// Number of interpolation knots.
    pub fn nb_knots(&self) -> usize {
        self.params.len()
    }

    /// Return the `i`-th knot parameter (0-based).
    pub fn knot(&self, i: usize) -> f64 {
        self.params[i]
    }

    /// Return the `i`-th knot value (0-based).
    pub fn knot_value(&self, i: usize) -> f64 {
        self.values[i]
    }
}

impl LawFunction for LawInterpol {
    fn value_and_d1(&self, t: f64) -> (f64, f64) {
        let n = self.params.len();
        let t_clamped = t.clamp(self.params[0], self.params[n - 1]);
        if t_clamped >= self.params[n - 1] {
            // At the last knot: evaluate from the last segment.
            return self.eval_segment(n - 2, t_clamped);
        }
        let i = self.find_interval(t_clamped);
        self.eval_segment(i, t_clamped)
    }

    fn first_parameter(&self) -> f64 {
        self.params[0]
    }

    fn last_parameter(&self) -> f64 {
        *self.params.last().unwrap()
    }

    fn continuity(&self) -> u8 {
        2 // C2 natural cubic spline
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LawBSpline
// ─────────────────────────────────────────────────────────────────────────────

/// A B-spline evolution law.
///
/// Defined by a clamped B-spline of arbitrary degree with scalar poles
/// (control values), a knot vector and per-knot multiplicities.
/// Mirrors `Law_BSpline` from OCCT.
///
/// The B-spline is evaluated using Cox-de Boor recursion.
// occt: Law_BSpline
#[derive(Clone, Debug)]
pub struct LawBSpline {
    /// Degree of the B-spline.
    degree: usize,
    /// Expanded (flat) knot vector of length `poles.len() + degree + 1`.
    flat_knots: Vec<f64>,
    /// Scalar control points ("poles" in OCCT terminology).
    poles: Vec<f64>,
    /// Optional rational weights; `None` ⇒ all equal to 1.
    weights: Option<Vec<f64>>,
}

impl LawBSpline {
    /// Construct from distinct knots with multiplicities.
    ///
    /// `knots` and `mults` must have the same length and `mults[0]` /
    /// `mults[last]` must equal `degree + 1` for a clamped (non-periodic)
    /// spline.
    pub fn new(
        poles: &[f64],
        knots: &[f64],
        mults: &[usize],
        degree: usize,
    ) -> Self {
        assert!(degree >= 1, "LawBSpline: degree must be >= 1");
        assert_eq!(knots.len(), mults.len(), "LawBSpline: knots and mults must have equal length");
        assert!(knots.len() >= 2, "LawBSpline: need at least 2 distinct knots");

        // Build flat knot vector.
        let mut flat_knots = Vec::new();
        for (i, &k) in knots.iter().enumerate() {
            for _ in 0..mults[i] {
                flat_knots.push(k);
            }
        }

        // Validate: flat_knots.len() == poles.len() + degree + 1
        assert_eq!(
            flat_knots.len(),
            poles.len() + degree + 1,
            "LawBSpline: flat knot count ({}) != poles ({}) + degree ({}) + 1",
            flat_knots.len(),
            poles.len(),
            degree
        );

        Self {
            degree,
            flat_knots,
            poles: poles.to_vec(),
            weights: None,
        }
    }

    /// Construct a rational B-spline (NURBS-like scalar law) with weights.
    pub fn new_rational(
        poles: &[f64],
        weights: &[f64],
        knots: &[f64],
        mults: &[usize],
        degree: usize,
    ) -> Self {
        assert_eq!(weights.len(), poles.len(), "LawBSpline: weights must match poles");
        let mut law = Self::new(poles, knots, mults, degree);
        // Only store weights when not all equal.
        let first = weights[0];
        if weights.iter().any(|&w| (w - first).abs() > precision::CONFUSION) {
            law.weights = Some(weights.to_vec());
        }
        law
    }

    /// Degree of the spline.
    pub fn degree(&self) -> usize {
        self.degree
    }

    /// Number of poles.
    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }

    /// Return the `i`-th pole (0-based).
    pub fn pole(&self, i: usize) -> f64 {
        self.poles[i]
    }

    /// Set the `i`-th pole (0-based).
    pub fn set_pole(&mut self, i: usize, v: f64) {
        self.poles[i] = v;
    }

    // ── de Boor evaluation ────────────────────────────────────────────────────

    /// Find the knot span index for parameter `t`.
    fn find_span(&self, t: f64) -> usize {
        let n = self.poles.len() - 1;
        let p = self.degree;
        let knots = &self.flat_knots;

        if t >= knots[n + 1] {
            return n;
        }
        if t <= knots[p] {
            return p;
        }
        // Binary search.
        let (mut lo, mut hi) = (p, n + 1);
        let mut mid = (lo + hi) / 2;
        while t < knots[mid] || t >= knots[mid + 1] {
            if t < knots[mid] {
                hi = mid;
            } else {
                lo = mid;
            }
            mid = (lo + hi) / 2;
        }
        mid
    }

    /// Compute B-spline basis functions N_{span-p, p}(t) .. N_{span, p}(t)
    /// (Cox-de Boor, Piegl-Tiller Algorithm A2.2).
    #[allow(dead_code)]
    fn basis_functions(&self, span: usize, t: f64) -> Vec<f64> {
        let p = self.degree;
        let knots = &self.flat_knots;
        let mut n = vec![0.0f64; p + 1];
        let mut left = vec![0.0f64; p + 1];
        let mut right = vec![0.0f64; p + 1];

        n[0] = 1.0;
        for j in 1..=p {
            left[j] = t - knots[span + 1 - j];
            right[j] = knots[span + j] - t;
            let mut saved = 0.0f64;
            for r in 0..j {
                let denom = right[r + 1] + left[j - r];
                let temp = if denom.abs() < 1e-300 { 0.0 } else { n[r] / denom };
                n[r] = saved + right[r + 1] * temp;
                saved = left[j - r] * temp;
            }
            n[j] = saved;
        }
        n
    }

    /// Evaluate both basis functions and their first derivatives.
    ///
    /// Uses the recurrence `dN_{i,p} = p * (N_{i,p-1}/Δt_l - N_{i+1,p-1}/Δt_r)`
    /// (Piegl-Tiller §2.9, derivative formula), computing N at degree p and
    /// N at degree p-1 in one pass through the de Boor triangle.
    ///
    /// Returns `(N, dN)` arrays of length `degree + 1`.
    fn basis_functions_and_d1(&self, span: usize, t: f64) -> (Vec<f64>, Vec<f64>) {
        let p = self.degree;
        let knots = &self.flat_knots;

        // ── Step 1: fill the full de Boor triangle via Algorithm A2.2. ──────
        // ndu[j][r]: when j == degree → N_{span-p+r, p}(t);
        //            the off-diagonals store knot differences for later use.
        let mut ndu = vec![vec![0.0f64; p + 1]; p + 1];
        let mut left = vec![0.0f64; p + 1];
        let mut right = vec![0.0f64; p + 1];

        ndu[0][0] = 1.0;
        for j in 1..=p {
            left[j] = t - knots[span + 1 - j];
            right[j] = knots[span + j] - t;
            let mut saved = 0.0f64;
            for r in 0..j {
                ndu[j][r] = right[r + 1] + left[j - r]; // knot difference
                let temp = if ndu[j][r].abs() < 1e-300 {
                    0.0
                } else {
                    ndu[r][j - 1] / ndu[j][r]
                };
                ndu[r][j] = saved + right[r + 1] * temp;
                saved = left[j - r] * temp;
            }
            ndu[j][j] = saved;
        }

        // N at degree p: ndu[r][p] for r = 0..=p.
        let n_vals: Vec<f64> = (0..=p).map(|r| ndu[r][p]).collect();

        if p == 0 {
            return (n_vals, vec![0.0f64; 1]);
        }

        // ── Step 2: first derivatives using the standard recurrence. ─────────
        // dN_{span-p+i, p}(t) = p * [ N_{span-p+i, p-1}(t) / (t_{span+i} - t_{span-p+i})
        //                             - N_{span-p+i+1, p-1}(t) / (t_{span+i+1} - t_{span-p+i+1}) ]
        //
        // N at degree p-1 is available in ndu[r][p-1] for r = 0..p-1 (note:
        // they span indices [span-1-(p-2), span-1] = [span-p+1, span]).
        // We need N_{span-p+i, p-1} which has local index i-0 in ndu[][p-1]
        // only for i < p (and 0 for i == p).
        let pp = p as f64;
        let mut dn = vec![0.0f64; p + 1];

        for i in 0..=p {
            let global_i = span - p + i;

            // Left knot difference: t_{global_i + p} - t_{global_i}
            let kl = knots.get(global_i + p).copied().unwrap_or(0.0);
            let kr = knots.get(global_i).copied().unwrap_or(0.0);
            let denom_l = kl - kr;

            // Right knot difference: t_{global_i + p + 1} - t_{global_i + 1}
            let kl2 = knots.get(global_i + p + 1).copied().unwrap_or(0.0);
            let kr2 = knots.get(global_i + 1).copied().unwrap_or(0.0);
            let denom_r = kl2 - kr2;

            // N_{span-p+i, p-1} is ndu[i][p-1] for i in 0..p-1, else 0.
            // N_{span-p+i+1, p-1} is ndu[i+1][p-1] for i+1 in 0..p-1, else 0.
            // (ndu[r][p-1] covers local indices 0..p.)
            let n_left = if i < p { ndu[i][p - 1] } else { 0.0 };
            let n_right = if i + 1 <= p { ndu[i + 1][p - 1] } else { 0.0 };

            let term_l = if denom_l.abs() > 1e-300 { n_left / denom_l } else { 0.0 };
            let term_r = if denom_r.abs() > 1e-300 { n_right / denom_r } else { 0.0 };
            dn[i] = pp * (term_l - term_r);
        }

        (n_vals, dn)
    }

    /// Evaluate the B-spline law, returning `(value, derivative)`.
    fn eval(&self, t: f64) -> (f64, f64) {
        let span = self.find_span(t);
        let (n_vals, dn) = self.basis_functions_and_d1(span, t);
        let p = self.degree;

        if let Some(ref w) = self.weights {
            // Rational case.
            let mut num = 0.0f64;
            let mut denom = 0.0f64;
            let mut dnum = 0.0f64;
            let mut ddenom = 0.0f64;

            for i in 0..=p {
                let idx = span - p + i;
                let wi = w[idx];
                let pi = self.poles[idx];
                num += n_vals[i] * wi * pi;
                denom += n_vals[i] * wi;
                dnum += dn[i] * wi * pi;
                ddenom += dn[i] * wi;
            }

            let v = if denom.abs() > 1e-300 { num / denom } else { 0.0 };
            let dv = if denom.abs() > 1e-300 {
                (dnum * denom - num * ddenom) / (denom * denom)
            } else {
                0.0
            };
            (v, dv)
        } else {
            // Non-rational case.
            let mut v = 0.0f64;
            let mut dv = 0.0f64;
            for i in 0..=p {
                let idx = span - p + i;
                v += n_vals[i] * self.poles[idx];
                dv += dn[i] * self.poles[idx];
            }
            (v, dv)
        }
    }
}

impl LawFunction for LawBSpline {
    fn value_and_d1(&self, t: f64) -> (f64, f64) {
        let t0 = self.flat_knots[self.degree];
        let t1 = self.flat_knots[self.flat_knots.len() - 1 - self.degree];
        let t_clamped = t.clamp(t0, t1);
        self.eval(t_clamped)
    }

    fn first_parameter(&self) -> f64 {
        self.flat_knots[self.degree]
    }

    fn last_parameter(&self) -> f64 {
        self.flat_knots[self.flat_knots.len() - 1 - self.degree]
    }

    fn continuity(&self) -> u8 {
        // A degree-p B-spline with simple inner knots has C^{p-1} continuity.
        if self.degree >= 1 { (self.degree as u8).saturating_sub(1) } else { 0 }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LawComposite
// ─────────────────────────────────────────────────────────────────────────────

/// A piecewise-composite law made of N segments, each covered by a separate
/// [`LawFunction`].
///
/// Mirrors `Law_Composite` from OCCT. Each segment `i` covers the parameter
/// interval `[breaks[i], breaks[i+1]]` and delegates evaluation to
/// `laws[i]`.  The laws are stored boxed to allow mixing different law
/// kinds.
// occt: Law_Composite
#[derive(Debug)]
pub struct LawComposite {
    /// Breakpoint parameters; length = `laws.len() + 1`.
    breaks: Vec<f64>,
    /// One law per interval.
    laws: Vec<Box<dyn LawFunction>>,
}

impl LawComposite {
    /// Construct an empty composite.  Call `add_segment` for each piece.
    pub fn new() -> Self {
        Self {
            breaks: Vec::new(),
            laws: Vec::new(),
        }
    }

    /// Append a segment `[t0, t1]` using `law`.
    ///
    /// Segments must be added in ascending parameter order and must be
    /// contiguous (`t0` of the next segment must equal `t1` of the previous
    /// one).
    ///
    /// # Panics
    /// Panics if the segment is not contiguous with the previous one or if
    /// `t1 <= t0`.
    pub fn add_segment(&mut self, t0: f64, t1: f64, law: Box<dyn LawFunction>) {
        assert!(t1 > t0 + precision::CONFUSION, "LawComposite: t1 must be > t0");
        if !self.breaks.is_empty() {
            let last = *self.breaks.last().unwrap();
            assert!(
                (t0 - last).abs() < precision::CONFUSION * 1000.0,
                "LawComposite: segments must be contiguous (expected t0={}, got {})",
                last,
                t0
            );
        } else {
            self.breaks.push(t0);
        }
        self.breaks.push(t1);
        self.laws.push(law);
    }

    /// Number of segments.
    pub fn nb_intervals(&self) -> usize {
        self.laws.len()
    }

    /// Return the breakpoints.
    pub fn break_points(&self) -> &[f64] {
        &self.breaks
    }

    /// Find which segment contains `t`, returning the segment index.
    fn find_segment(&self, t: f64) -> usize {
        let n = self.laws.len();
        if n == 0 {
            panic!("LawComposite: no segments defined");
        }
        // Binary search.
        let mut lo = 0usize;
        let mut hi = n - 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if t < self.breaks[mid + 1] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo.min(n - 1)
    }
}

impl Default for LawComposite {
    fn default() -> Self {
        Self::new()
    }
}

impl LawFunction for LawComposite {
    fn value_and_d1(&self, t: f64) -> (f64, f64) {
        let t_clamped = t.clamp(self.first_parameter(), self.last_parameter());
        let i = self.find_segment(t_clamped);
        self.laws[i].value_and_d1(t_clamped)
    }

    fn first_parameter(&self) -> f64 {
        *self.breaks.first().expect("LawComposite: no segments")
    }

    fn last_parameter(&self) -> f64 {
        *self.breaks.last().expect("LawComposite: no segments")
    }

    fn continuity(&self) -> u8 {
        // The composite is at most C0 at the junctions.
        0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: f64, b: f64, tol: f64, msg: &str) {
        assert!(
            (a - b).abs() <= tol,
            "{msg}: |{a} - {b}| = {} > tol {tol}",
            (a - b).abs()
        );
    }

    // ── LawLinear ────────────────────────────────────────────────────────────

    /// A constant law (v0 == v1) must return that value everywhere.
    #[test]
    fn linear_constant_law() {
        let law = LawLinear::new(0.0, 5.0, 1.0, 5.0);
        near(law.value(0.0), 5.0, 1e-12, "v(0)");
        near(law.value(0.5), 5.0, 1e-12, "v(0.5)");
        near(law.value(1.0), 5.0, 1e-12, "v(1)");
        near(law.d1(0.5), 0.0, 1e-12, "d1 constant");
    }

    /// A ramp from 1 to 3 over [0,2] must be exactly linear.
    #[test]
    fn linear_ramp_midpoint() {
        let law = LawLinear::new(0.0, 1.0, 2.0, 3.0);
        near(law.value(0.0), 1.0, 1e-12, "v(0)");
        near(law.value(1.0), 2.0, 1e-12, "v(1) midpoint");
        near(law.value(2.0), 3.0, 1e-12, "v(2)");
        near(law.d1(0.5), 1.0, 1e-12, "slope");
    }

    /// Value and derivative consistency via finite differences.
    #[test]
    fn linear_deriv_finite_diff() {
        let law = LawLinear::new(0.0, 2.0, 4.0, 10.0);
        let h = 1e-6;
        let t = 1.5;
        let fd = (law.value(t + h) - law.value(t - h)) / (2.0 * h);
        near(law.d1(t), fd, 1e-5, "fd vs d1");
    }

    /// value_and_d1 must agree with individual `value` / `d1` calls.
    #[test]
    fn linear_value_and_d1_consistency() {
        let law = LawLinear::new(-1.0, 0.0, 1.0, 4.0);
        let (v, d) = law.value_and_d1(0.25);
        near(v, law.value(0.25), 1e-14, "value consistency");
        near(d, law.d1(0.25), 1e-14, "d1 consistency");
    }

    // ── LawInterpol ──────────────────────────────────────────────────────────

    /// The interpolating spline must pass exactly through all samples.
    #[test]
    fn interpol_passes_through_samples() {
        let params = [0.0, 1.0, 2.0, 3.0];
        let values = [0.0, 1.0, 0.0, 1.0];
        let law = LawInterpol::new(&params, &values, false).unwrap();
        for i in 0..4 {
            near(law.value(params[i]), values[i], 1e-10, "interpolation knot");
        }
    }

    /// A spline through collinear points must be linear (d2 == 0).
    #[test]
    fn interpol_linear_data_is_linear() {
        let params = [0.0, 1.0, 2.0];
        let values = [0.0, 2.0, 4.0]; // perfectly linear
        let law = LawInterpol::new(&params, &values, false).unwrap();
        near(law.value(0.5), 1.0, 1e-10, "midpoint of linear data");
        near(law.value(1.5), 3.0, 1e-10, "second midpoint");
    }

    /// The spline derivative should be approximately consistent with finite diff.
    #[test]
    fn interpol_derivative_finite_diff() {
        let params = [0.0, 1.0, 2.0, 3.0];
        let values = [1.0, 3.0, 2.0, 4.0];
        let law = LawInterpol::new(&params, &values, false).unwrap();
        let h = 1e-5;
        let t = 1.3;
        let fd = (law.value(t + h) - law.value(t - h)) / (2.0 * h);
        near(law.d1(t), fd, 1e-4, "finite diff derivative");
    }

    // ── LawBSpline ───────────────────────────────────────────────────────────

    /// A linear B-spline with two poles must be exactly affine.
    #[test]
    fn bspline_linear_law() {
        // degree-1, 2 poles, knots [0,0,1,1]
        let poles = [1.0, 3.0];
        let knots = [0.0, 1.0];
        let mults = [2usize, 2usize];
        let law = LawBSpline::new(&poles, &knots, &mults, 1);
        near(law.value(0.0), 1.0, 1e-12, "start");
        near(law.value(1.0), 3.0, 1e-12, "end");
        near(law.value(0.5), 2.0, 1e-12, "midpoint");
    }

    /// A quadratic B-spline reproduces the expected control-net behaviour.
    #[test]
    fn bspline_quadratic_midpoint() {
        // degree-2, clamped, poles [0, 1, 0], knots [0,0,0,1,1,1]
        let poles = [0.0, 1.0, 0.0];
        let knots = [0.0, 1.0];
        let mults = [3usize, 3usize];
        let law = LawBSpline::new(&poles, &knots, &mults, 2);
        near(law.value(0.0), 0.0, 1e-12, "start = 0");
        near(law.value(1.0), 0.0, 1e-12, "end = 0");
        // At t=0.5 the Bernstein basis gives B_{1,2}(0.5) = 0.5 (the peak).
        near(law.value(0.5), 0.5, 1e-10, "peak at midpoint");
    }

    /// first_parameter / last_parameter must reflect the flat knot bounds.
    #[test]
    fn bspline_parameter_bounds() {
        let poles = [0.0, 1.0, 2.0];
        let knots = [0.0, 5.0];
        let mults = [3usize, 3usize];
        let law = LawBSpline::new(&poles, &knots, &mults, 2);
        assert_eq!(law.first_parameter(), 0.0);
        assert_eq!(law.last_parameter(), 5.0);
    }

    // ── LawComposite ─────────────────────────────────────────────────────────

    /// A composite built from two linear segments must stitch cleanly.
    #[test]
    fn composite_two_linear_segments() {
        let mut comp = LawComposite::new();
        comp.add_segment(0.0, 1.0, Box::new(LawLinear::new(0.0, 0.0, 1.0, 1.0)));
        comp.add_segment(1.0, 2.0, Box::new(LawLinear::new(1.0, 1.0, 2.0, 3.0)));

        near(comp.value(0.0), 0.0, 1e-12, "start");
        near(comp.value(0.5), 0.5, 1e-12, "mid first seg");
        near(comp.value(1.0), 1.0, 1e-10, "junction");
        near(comp.value(1.5), 2.0, 1e-12, "mid second seg");
        near(comp.value(2.0), 3.0, 1e-12, "end");
    }

    /// nb_intervals and break_points must be consistent.
    #[test]
    fn composite_metadata() {
        let mut comp = LawComposite::new();
        comp.add_segment(0.0, 1.0, Box::new(LawLinear::new(0.0, 1.0, 1.0, 2.0)));
        comp.add_segment(1.0, 3.0, Box::new(LawLinear::new(1.0, 2.0, 3.0, 6.0)));
        assert_eq!(comp.nb_intervals(), 2);
        assert_eq!(comp.break_points(), &[0.0, 1.0, 3.0]);
        assert_eq!(comp.first_parameter(), 0.0);
        assert_eq!(comp.last_parameter(), 3.0);
    }
}
