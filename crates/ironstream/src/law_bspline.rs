// FILE: rust/ironstream/crates/ironstream/src/law_bspline.rs
//! `Law_BSpline` — piecewise B-Spline law (scalar function on a parameter
//! interval), mirroring OpenCascade's `Law_BSpline`
//! (`src/ModelingAlgorithms/TKGeomAlgo/Law/Law_BSpline.hxx`).
//!
//! A [`LawBSpline`] encodes a scalar B-spline defined by:
//! - a sequence of scalar **poles** (control values),
//! - a sequence of **distinct knots** with per-knot **multiplicities**,
//! - a polynomial **degree**,
//! - a **periodic** flag.
//!
//! Evaluation uses the Cox-de Boor recursion (Piegl-Tiller, "The NURBS Book",
//! §2.2, Algorithm A2.1/A2.2).
//!
//! Indexing mirrors the OCCT convention: the public `pole`, `knot`, and
//! `multiplicity` accessors use **0-based** indices consistent with Rust
//! idioms (OCCT uses 1-based; callers that port OCCT code should subtract 1).

// occt: Law_BSpline

/// A piecewise B-Spline law — a scalar function defined over a parameter
/// interval by B-spline basis functions.
///
/// # Representation
/// The spline is stored as distinct knots with multiplicities rather than the
/// expanded flat knot vector, matching the OCCT `Law_BSpline` data layout.
/// Internally, evaluation uses a flat knot vector derived on construction.
///
/// # Evaluation
/// [`LawBSpline::value`] implements Cox-de Boor recursion over the flat knot
/// vector. For a clamped non-periodic spline the parameter domain is
/// `[knots[0], knots[last]]`.
// occt: Law_BSpline
#[derive(Clone, Debug)]
pub struct LawBSpline {
    /// Scalar control values ("poles" in OCCT terminology).
    poles: Vec<f64>,
    /// Distinct knot values (strictly increasing).
    knots: Vec<f64>,
    /// Multiplicity of each distinct knot; same length as `knots`.
    mults: Vec<u32>,
    /// Polynomial degree of each B-spline segment.
    degree: u32,
    /// Whether the spline is periodic.
    is_periodic: bool,
    /// Expanded (flat) knot vector derived from `knots` and `mults`.
    /// Length == `poles.len() + degree + 1` for a non-periodic spline.
    flat_knots: Vec<f64>,
}

impl LawBSpline {
    /// Construct a new B-spline law.
    ///
    /// # Arguments
    /// * `poles`       – scalar control values.
    /// * `knots`       – strictly increasing distinct knot values.
    /// * `mults`       – multiplicity of each knot; must have the same length
    ///                   as `knots`.
    /// * `degree`      – polynomial degree (≥ 1).
    /// * `is_periodic` – whether the spline wraps around.
    ///
    /// # Panics
    /// Panics when:
    /// - `degree == 0`,
    /// - `knots` and `mults` have different lengths,
    /// - fewer than 2 distinct knots are provided,
    /// - the flat knot count does not equal `poles.len() + degree + 1`.
    pub fn new(
        poles: Vec<f64>,
        knots: Vec<f64>,
        mults: Vec<u32>,
        degree: u32,
        is_periodic: bool,
    ) -> Self {
        assert!(degree >= 1, "LawBSpline: degree must be >= 1");
        assert_eq!(
            knots.len(),
            mults.len(),
            "LawBSpline: knots and mults must have the same length"
        );
        assert!(knots.len() >= 2, "LawBSpline: need at least 2 distinct knots");

        // Build the flat knot vector.
        let mut flat_knots: Vec<f64> = Vec::new();
        for (&k, &m) in knots.iter().zip(mults.iter()) {
            for _ in 0..m {
                flat_knots.push(k);
            }
        }

        let expected = poles.len() + degree as usize + 1;
        assert_eq!(
            flat_knots.len(),
            expected,
            "LawBSpline: flat knot count ({}) != poles.len() ({}) + degree ({}) + 1",
            flat_knots.len(),
            poles.len(),
            degree
        );

        Self {
            poles,
            knots,
            mults,
            degree,
            is_periodic,
            flat_knots,
        }
    }

    // ── Accessors ────────────────────────────────────────────────────────────

    /// Number of poles.
    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }

    /// Number of distinct knots.
    pub fn nb_knots(&self) -> usize {
        self.knots.len()
    }

    /// Polynomial degree.
    pub fn degree(&self) -> u32 {
        self.degree
    }

    /// Whether the spline is periodic.
    pub fn is_periodic(&self) -> bool {
        self.is_periodic
    }

    /// Return the pole at index `i` (0-based).
    ///
    /// # Panics
    /// Panics if `i >= nb_poles()`.
    pub fn pole(&self, i: usize) -> f64 {
        self.poles[i]
    }

    /// Return the distinct knot at index `i` (0-based).
    ///
    /// # Panics
    /// Panics if `i >= nb_knots()`.
    pub fn knot(&self, i: usize) -> f64 {
        self.knots[i]
    }

    /// Return the multiplicity of knot `i` (0-based).
    ///
    /// # Panics
    /// Panics if `i >= nb_knots()`.
    pub fn multiplicity(&self, i: usize) -> u32 {
        self.mults[i]
    }

    /// Set the pole at index `i` (0-based).
    ///
    /// # Panics
    /// Panics if `i >= nb_poles()`.
    pub fn set_pole(&mut self, i: usize, v: f64) {
        self.poles[i] = v;
    }

    /// Set the distinct knot at index `i` (0-based) and rebuild the flat knot
    /// vector.
    ///
    /// # Panics
    /// Panics if `i >= nb_knots()`.
    pub fn set_knot(&mut self, i: usize, v: f64) {
        self.knots[i] = v;
        self.rebuild_flat_knots();
    }

    // ── Evaluation ───────────────────────────────────────────────────────────

    /// Evaluate the B-spline law at parameter `t` using Cox-de Boor recursion.
    ///
    /// The parameter is clamped to the valid domain
    /// `[flat_knots[degree], flat_knots[n - degree - 1]]` before evaluation,
    /// matching the OCCT behaviour for non-periodic splines.
    pub fn value(&self, t: f64) -> f64 {
        let p = self.degree as usize;
        let fk = &self.flat_knots;
        let n = self.poles.len(); // number of poles

        // Clamp to the valid parameter domain.
        let t_min = fk[p];
        let t_max = fk[n]; // fk[n] == fk[poles.len() + degree] - degree == fk[n+p] - p offset
        // For a clamped spline: valid domain is [fk[p] .. fk[n]], where n = poles.len().
        // fk has length n + p + 1, so last valid param is fk[n].
        let t = t.clamp(t_min, t_max);

        // Find the knot span index k such that fk[k] <= t < fk[k+1].
        let k = self.find_span(t);

        // Cox-de Boor: initialise d[j] = pole[k-p+j] for j in 0..=p.
        let mut d: Vec<f64> = (0..=p).map(|j| self.poles[k - p + j]).collect();

        // Triangular de Boor reduction.
        for r in 1..=p {
            for j in (r..=p).rev() {
                let i = k - p + j; // global pole index
                let denom = fk[i + p - r + 1] - fk[i];
                let alpha = if denom.abs() < f64::EPSILON * 8.0 {
                    0.0
                } else {
                    (t - fk[i]) / denom
                };
                d[j] = (1.0 - alpha) * d[j - 1] + alpha * d[j];
            }
        }

        d[p]
    }

    // ── Internal helpers ─────────────────────────────────────────────────────

    /// Find the knot span index `k` such that `flat_knots[k] <= t < flat_knots[k+1]`.
    ///
    /// Uses the standard binary-search algorithm (Piegl-Tiller A2.1), handling
    /// the special case `t == last valid parameter` by returning `n - 1` where
    /// `n == poles.len()`.
    fn find_span(&self, t: f64) -> usize {
        let p = self.degree as usize;
        let n = self.poles.len(); // n poles => valid spans 0..n-1
        let fk = &self.flat_knots;

        // Special case: t at the upper boundary.
        if t >= fk[n] {
            // Walk back past any repeated end knots.
            let mut k = n;
            while k > p && (fk[k] - fk[n]).abs() < f64::EPSILON * 8.0 {
                k -= 1;
            }
            return k;
        }

        // Binary search in fk[p..=n].
        let mut lo = p;
        let mut hi = n;
        let mut mid = (lo + hi) / 2;
        while t < fk[mid] || t >= fk[mid + 1] {
            if t < fk[mid] {
                hi = mid;
            } else {
                lo = mid;
            }
            mid = (lo + hi) / 2;
        }
        mid
    }

    /// Rebuild the flat knot vector from `knots` and `mults` after a knot is
    /// mutated via [`set_knot`](Self::set_knot).
    fn rebuild_flat_knots(&mut self) {
        self.flat_knots.clear();
        for (&k, &m) in self.knots.iter().zip(self.mults.iter()) {
            for _ in 0..m {
                self.flat_knots.push(k);
            }
        }
    }
}
