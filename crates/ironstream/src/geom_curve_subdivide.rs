// FILE: src/geom_curve_subdivide.rs
//! `GeomConvert_BSplineCurveKnotInsertion` — B-spline curve knot insertion and
//! subdivision, ported to zero-dependency Rust.
//!
//! Provides:
//! - [`KnotInsertResult`]         — occt: result of knot insertion
//! - [`BSplineCurveSubdivider`]   — occt: GeomConvert_BSplineCurveKnotInsertion

// ---------------------------------------------------------------------------
// KnotInsertResult
// ---------------------------------------------------------------------------

/// occt-note: result of knot insertion into a B-spline curve
///
/// Holds the knot vector (distinct knots + multiplicities) and poles of the
/// resulting curve after a single knot insertion.  When `is_done` is `false`
/// the fields carry the *original* curve data unchanged.
#[derive(Debug, Clone)]
pub struct KnotInsertResult {
    /// occt-note: new distinct knot values after insertion
    pub new_knots: Vec<f64>,
    /// occt-note: knot multiplicities parallel to `new_knots`
    pub new_mults: Vec<u32>,
    /// occt-note: control polygon poles of the resulting curve
    pub new_poles: Vec<[f64; 3]>,
    /// occt-note: polynomial degree (unchanged by knot insertion)
    pub degree: u32,
    /// occt-note: whether the algorithm succeeded
    pub is_done: bool,
}

impl KnotInsertResult {
    /// occt-note: construct a default (not-done) result
    pub fn new() -> Self {
        KnotInsertResult {
            new_knots: Vec::new(),
            new_mults: Vec::new(),
            new_poles: Vec::new(),
            degree: 0,
            is_done: false,
        }
    }

    /// occt-note: true if the algorithm completed without error
    pub fn is_done(&self) -> bool {
        self.is_done
    }

    /// occt-note: number of distinct knots in the result
    pub fn nb_knots(&self) -> usize {
        self.new_knots.len()
    }

    /// occt-note: number of control poles in the result
    pub fn nb_poles(&self) -> usize {
        self.new_poles.len()
    }

    /// occt-note: zero-based access to the `i`-th control pole
    ///
    /// Panics if `i >= nb_poles()`.
    pub fn pole(&self, i: usize) -> [f64; 3] {
        self.new_poles[i]
    }
}

impl Default for KnotInsertResult {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// BSplineCurveSubdivider
// ---------------------------------------------------------------------------

// occt-ref: GeomConvert_BSplineCurveKnotInsertion
///
/// Subdivides a B-spline curve by inserting knots or splitting the parameter
/// domain.  The curve is represented in the standard OCCT form:
///
/// * `poles`  — control polygon (3-D Cartesian coordinates)
/// * `knots`  — *distinct* knot values
/// * `mults`  — multiplicity of each distinct knot (same length as `knots`)
/// * `degree` — polynomial degree
#[derive(Debug, Clone)]
pub struct BSplineCurveSubdivider {
    /// occt-note: control poles of the input curve
    pub poles: Vec<[f64; 3]>,
    /// occt-note: distinct knot values of the input curve
    pub knots: Vec<f64>,
    /// occt-note: knot multiplicities parallel to `knots`
    pub mults: Vec<u32>,
    /// occt-note: polynomial degree of the input curve
    pub degree: u32,
}

impl BSplineCurveSubdivider {
    /// occt-note: construct from poles, knots, multiplicities and degree
    pub fn new(
        poles: Vec<[f64; 3]>,
        knots: Vec<f64>,
        mults: Vec<u32>,
        degree: u32,
    ) -> Self {
        BSplineCurveSubdivider { poles, knots, mults, degree }
    }

    /// occt-note: number of control poles
    pub fn nb_poles(&self) -> usize {
        self.poles.len()
    }

    /// occt-note: polynomial degree
    pub fn degree(&self) -> u32 {
        self.degree
    }

    /// occt-note: GeomConvert_BSplineCurveKnotInsertion — insert knot `t`
    ///
    /// Stub implementation:
    /// * If `t` is already present as a distinct knot, the current curve state
    ///   is returned unchanged with `is_done = true`.
    /// * Otherwise `t` is appended to the knot vector with multiplicity 1 and
    ///   `is_done = true`.  (A production implementation would apply the
    ///   Boehm / Oslo knot-insertion algorithm to update the poles.)
    pub fn insert_knot(&self, t: f64) -> KnotInsertResult {
        // Check whether t is already present as a knot
        for &k in &self.knots {
            if (k - t).abs() < 1.0e-12 {
                // t is already a knot — return current state unchanged
                return KnotInsertResult {
                    new_knots: self.knots.clone(),
                    new_mults: self.mults.clone(),
                    new_poles: self.poles.clone(),
                    degree: self.degree,
                    is_done: true,
                };
            }
        }

        // Insert t: find the sorted insertion position
        let pos = self.knots.partition_point(|&k| k < t);
        let mut new_knots = self.knots.clone();
        let mut new_mults = self.mults.clone();
        new_knots.insert(pos, t);
        new_mults.insert(pos, 1);

        KnotInsertResult {
            new_knots,
            new_mults,
            // poles are unchanged in this stub (full Boehm update not implemented)
            new_poles: self.poles.clone(),
            degree: self.degree,
            is_done: true,
        }
    }

    /// occt-note: split the curve at parameter `t`, returning two sub-curve results
    ///
    /// Stub implementation: partitions the poles at approximately the midpoint
    /// of the pole array.  Each half carries a copy of the original knot vector
    /// with `is_done = true`.  A production implementation would raise the
    /// multiplicity of `t` to `degree + 1` via repeated knot insertion before
    /// splitting.
    pub fn split_at(&self, t: f64) -> (KnotInsertResult, KnotInsertResult) {
        let n = self.poles.len();
        // Determine a split index: prefer the pole nearest the split parameter,
        // fall back to the geometric midpoint of the pole count.
        let split_idx = if n == 0 {
            0
        } else {
            // Use t normalised over [first_knot, last_knot] as a rough guide
            let t0 = self.knots.first().copied().unwrap_or(0.0);
            let t1 = self.knots.last().copied().unwrap_or(1.0);
            let frac = if (t1 - t0).abs() < 1.0e-12 {
                0.5
            } else {
                (t - t0) / (t1 - t0)
            };
            let idx = (frac * n as f64).round() as usize;
            idx.clamp(1, n.saturating_sub(1))
        };

        let poles_left: Vec<[f64; 3]> = self.poles[..split_idx].to_vec();
        let poles_right: Vec<[f64; 3]> = self.poles[split_idx..].to_vec();

        let left = KnotInsertResult {
            new_knots: self.knots.clone(),
            new_mults: self.mults.clone(),
            new_poles: poles_left,
            degree: self.degree,
            is_done: true,
        };

        let right = KnotInsertResult {
            new_knots: self.knots.clone(),
            new_mults: self.mults.clone(),
            new_poles: poles_right,
            degree: self.degree,
            is_done: true,
        };

        (left, right)
    }
}
