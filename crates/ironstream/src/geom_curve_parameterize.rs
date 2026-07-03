// FILE: geom_curve_parameterize.rs
//! `geom_curve_parameterize` — reparameterization strategies for point clouds
//! and sampled curves, mirroring OpenCascade's `Approx_ParametrizationType` and
//! the parametrization logic used inside `GeomConvert_ApproxCurve`.
//!
//! Three classic strategies are provided:
//!
//! * [`ParametrizationType::ChordLength`]  — cumulative Euclidean chord lengths,
//!   normalized to `[0, 1]`.  This is the default in most OCCT approximation
//!   algorithms and gives the best shape fidelity for non-uniform point clouds.
//! * [`ParametrizationType::Centripetal`]  — square-root of chord lengths
//!   (Lee 1989).  Works better than pure chord-length when the curvature varies
//!   widely or the input contains near-zero-length segments.
//! * [`ParametrizationType::Uniform`]      — equal spacing in parameter, ignoring
//!   the actual geometry.  Fast but poor for non-uniform point distributions.
//!
//! All functions and types are pure Rust with no external crate dependencies;
//! only `f64::sqrt` from `std` is used.

// occt: Approx_ParametrizationType

/// The three standard reparameterization types used by OCCT's approximation
/// algorithms.
// occt: Approx_ParametrizationType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParametrizationType {
    /// Cumulative Euclidean chord lengths, normalized to `[0, 1]`.
    ///
    /// Maps to `Approx_ChordLength` in OCCT.
    ChordLength,

    /// Square-root (centripetal) of cumulative chord lengths, normalized to
    /// `[0, 1]`.
    ///
    /// Maps to `Approx_Centripetal` in OCCT.
    Centripetal,

    /// Uniform spacing — parameter `t_i = i / (n - 1)` regardless of geometry.
    ///
    /// Maps to `Approx_IsoParametric` in OCCT.
    Uniform,
}

// ─────────────────────────── free-standing functions ─────────────────────────

/// Euclidean distance between two 3-D points stored as `[f64; 3]`.
#[inline]
fn dist3(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    let dz = b[2] - a[2];
    f64::sqrt(dx * dx + dy * dy + dz * dz)
}

/// Compute chord-length parameters for a polyline of 3-D points.
///
/// Returns a `Vec<f64>` of length `pts.len()` with values in `[0, 1]`:
///
/// ```text
/// t[0] = 0
/// t[i] = (sum of chord lengths from 0..i) / total_chord_length
/// t[n-1] = 1
/// ```
///
/// When the total chord length is zero (all points coincide) every parameter
/// is set to zero.
///
/// # Panics
///
/// Panics if `pts` is empty.
pub fn chord_length_params(pts: &[[f64; 3]]) -> Vec<f64> {
    assert!(!pts.is_empty(), "chord_length_params: pts must be non-empty");
    let n = pts.len();
    let mut t = vec![0.0f64; n];
    for i in 1..n {
        t[i] = t[i - 1] + dist3(pts[i - 1], pts[i]);
    }
    let total = t[n - 1];
    if total > 0.0 {
        for ti in t.iter_mut() {
            *ti /= total;
        }
    }
    t
}

/// Compute centripetal parameters for a polyline of 3-D points (Lee 1989).
///
/// Like [`chord_length_params`] but uses the *square root* of each chord
/// length as the incremental weight:
///
/// ```text
/// t[0] = 0
/// t[i] = (sum of sqrt(chord_length_i)) / total
/// t[n-1] = 1
/// ```
///
/// # Panics
///
/// Panics if `pts` is empty.
pub fn centripetal_params(pts: &[[f64; 3]]) -> Vec<f64> {
    assert!(!pts.is_empty(), "centripetal_params: pts must be non-empty");
    let n = pts.len();
    let mut t = vec![0.0f64; n];
    for i in 1..n {
        t[i] = t[i - 1] + f64::sqrt(dist3(pts[i - 1], pts[i]));
    }
    let total = t[n - 1];
    if total > 0.0 {
        for ti in t.iter_mut() {
            *ti /= total;
        }
    }
    t
}

/// Compute uniform parameters for `n` points.
///
/// Returns `[0.0, 1/(n-1), 2/(n-1), ..., 1.0]`.  When `n == 1` returns
/// `[0.0]`.
///
/// # Panics
///
/// Panics if `n == 0`.
pub fn uniform_params(n: usize) -> Vec<f64> {
    assert!(n > 0, "uniform_params: n must be > 0");
    if n == 1 {
        return vec![0.0];
    }
    (0..n)
        .map(|i| i as f64 / (n - 1) as f64)
        .collect()
}

// ─────────────────────────── CurveParameterizer ──────────────────────────────

/// Stateful accumulator that collects 3-D sample points and computes a
/// parameterization according to a chosen [`ParametrizationType`].
///
/// This mirrors the role of `GeomConvert_ApproxCurve`'s internal
/// parameterization pass and the `Approx_ParametrizationType` enum in OCCT.
// occt: Approx_ParametrizationType
#[derive(Clone, Debug)]
pub struct CurveParameterizer {
    /// The ordered 3-D sample points.
    pub points: Vec<[f64; 3]>,
    /// The parameterization strategy to apply.
    pub param_type: ParametrizationType,
}

impl CurveParameterizer {
    /// Create a new, empty `CurveParameterizer` with the given strategy.
    pub fn new(t: ParametrizationType) -> Self {
        Self {
            points: Vec::new(),
            param_type: t,
        }
    }

    /// Append a 3-D point to the internal sample list.
    pub fn add_point(&mut self, p: [f64; 3]) {
        self.points.push(p);
    }

    /// Compute and return parameter values according to `self.param_type`.
    ///
    /// Returns an empty `Vec` when no points have been added.
    ///
    /// The returned slice is always the same length as `self.points`.
    pub fn parameters(&self) -> Vec<f64> {
        if self.points.is_empty() {
            return Vec::new();
        }
        match self.param_type {
            ParametrizationType::ChordLength => chord_length_params(&self.points),
            ParametrizationType::Centripetal => centripetal_params(&self.points),
            ParametrizationType::Uniform => uniform_params(self.points.len()),
        }
    }

    /// Convenience method: compute chord-length parameters regardless of
    /// `self.param_type`.
    ///
    /// Returns an empty `Vec` when no points have been added.
    pub fn chord_length_params(&self) -> Vec<f64> {
        if self.points.is_empty() {
            return Vec::new();
        }
        chord_length_params(&self.points)
    }
}

// ─────────────────────────── tests ───────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-12;

    fn near(a: f64, b: f64, msg: &str) {
        assert!(
            (a - b).abs() <= EPS,
            "{msg}: got {a}, expected {b}, diff={}",
            (a - b).abs()
        );
    }

    // ── chord_length_params ───────────────────────────────────────────────────

    #[test]
    fn chord_single_point() {
        let pts = [[0.0, 0.0, 0.0]];
        let t = chord_length_params(&pts);
        assert_eq!(t.len(), 1);
        near(t[0], 0.0, "single point param");
    }

    #[test]
    fn chord_two_equal_points_zero_length() {
        let pts = [[1.0, 2.0, 3.0], [1.0, 2.0, 3.0]];
        let t = chord_length_params(&pts);
        near(t[0], 0.0, "zero-len first");
        near(t[1], 0.0, "zero-len second (total=0)");
    }

    #[test]
    fn chord_uniform_x_axis() {
        // Points equally spaced on X axis → chord params should equal uniform.
        let pts: Vec<[f64; 3]> = (0..5).map(|i| [i as f64, 0.0, 0.0]).collect();
        let t = chord_length_params(&pts);
        assert_eq!(t.len(), 5);
        near(t[0], 0.0, "chord first");
        near(t[4], 1.0, "chord last");
        near(t[2], 0.5, "chord mid");
        near(t[1], 0.25, "chord quarter");
    }

    #[test]
    fn chord_non_uniform_spacing() {
        // Two segments of lengths 1 and 3 → params 0, 0.25, 1.0.
        let pts = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [4.0, 0.0, 0.0]];
        let t = chord_length_params(&pts);
        near(t[0], 0.0, "non-uni first");
        near(t[1], 0.25, "non-uni mid");
        near(t[2], 1.0, "non-uni last");
    }

    // ── centripetal_params ────────────────────────────────────────────────────

    #[test]
    fn centripetal_single_point() {
        let pts = [[3.0, 4.0, 0.0]];
        let t = centripetal_params(&pts);
        assert_eq!(t.len(), 1);
        near(t[0], 0.0, "centripetal single");
    }

    #[test]
    fn centripetal_two_equal_points() {
        let pts = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
        let t = centripetal_params(&pts);
        near(t[0], 0.0, "centripetal zero-len first");
        near(t[1], 0.0, "centripetal zero-len second");
    }

    #[test]
    fn centripetal_uniform_x_axis() {
        // Equal chord lengths → centripetal and chord-length should agree.
        let pts: Vec<[f64; 3]> = (0..5).map(|i| [i as f64, 0.0, 0.0]).collect();
        let tc = centripetal_params(&pts);
        let tl = chord_length_params(&pts);
        for (a, b) in tc.iter().zip(tl.iter()) {
            near(*a, *b, "centripetal == chord for uniform spacing");
        }
    }

    #[test]
    fn centripetal_normalised_bounds() {
        let pts = [
            [0.0, 0.0, 0.0],
            [3.0, 4.0, 0.0], // chord = 5, sqrt = ~2.236
            [6.0, 4.0, 0.0], // chord = 3, sqrt = ~1.732
        ];
        let t = centripetal_params(&pts);
        near(t[0], 0.0, "centripetal first=0");
        near(t[2], 1.0, "centripetal last=1");
        assert!(t[1] > 0.0 && t[1] < 1.0, "centripetal mid in (0,1)");
    }

    // ── uniform_params ────────────────────────────────────────────────────────

    #[test]
    fn uniform_single() {
        let t = uniform_params(1);
        assert_eq!(t, vec![0.0]);
    }

    #[test]
    fn uniform_two() {
        let t = uniform_params(2);
        near(t[0], 0.0, "uniform 2 first");
        near(t[1], 1.0, "uniform 2 last");
    }

    #[test]
    fn uniform_five() {
        let t = uniform_params(5);
        assert_eq!(t.len(), 5);
        near(t[0], 0.0, "uniform first");
        near(t[4], 1.0, "uniform last");
        near(t[2], 0.5, "uniform mid");
        near(t[1], 0.25, "uniform quarter");
        near(t[3], 0.75, "uniform three-quarter");
    }

    // ── CurveParameterizer ────────────────────────────────────────────────────

    #[test]
    fn parameterizer_empty() {
        let p = CurveParameterizer::new(ParametrizationType::ChordLength);
        assert!(p.parameters().is_empty());
        assert!(p.chord_length_params().is_empty());
    }

    #[test]
    fn parameterizer_chord_via_parameters() {
        let mut p = CurveParameterizer::new(ParametrizationType::ChordLength);
        for i in 0..5 {
            p.add_point([i as f64, 0.0, 0.0]);
        }
        let t = p.parameters();
        near(t[0], 0.0, "parameterizer chord first");
        near(t[4], 1.0, "parameterizer chord last");
        near(t[2], 0.5, "parameterizer chord mid");
    }

    #[test]
    fn parameterizer_centripetal_via_parameters() {
        let mut p = CurveParameterizer::new(ParametrizationType::Centripetal);
        for i in 0..4 {
            p.add_point([i as f64, 0.0, 0.0]);
        }
        let t = p.parameters();
        // For equal spacing centripetal == chord == uniform.
        let expected = uniform_params(4);
        for (a, b) in t.iter().zip(expected.iter()) {
            near(*a, *b, "centripetal uniform match");
        }
    }

    #[test]
    fn parameterizer_uniform_via_parameters() {
        let mut p = CurveParameterizer::new(ParametrizationType::Uniform);
        // Add non-uniform points — params should still be uniform.
        p.add_point([0.0, 0.0, 0.0]);
        p.add_point([10.0, 0.0, 0.0]); // long segment
        p.add_point([11.0, 0.0, 0.0]); // short segment
        let t = p.parameters();
        near(t[0], 0.0, "uniform param first");
        near(t[1], 0.5, "uniform param mid");
        near(t[2], 1.0, "uniform param last");
    }

    #[test]
    fn parameterizer_chord_length_params_method() {
        let mut p = CurveParameterizer::new(ParametrizationType::Uniform);
        // Even though param_type is Uniform, chord_length_params() uses chords.
        p.add_point([0.0, 0.0, 0.0]);
        p.add_point([1.0, 0.0, 0.0]);
        p.add_point([3.0, 0.0, 0.0]);
        let t = p.chord_length_params();
        near(t[0], 0.0, "cl_method first");
        near(t[1], 1.0 / 3.0, "cl_method mid");
        near(t[2], 1.0, "cl_method last");
    }

    #[test]
    fn parameterizer_add_point_accumulates() {
        let mut p = CurveParameterizer::new(ParametrizationType::ChordLength);
        p.add_point([0.0, 0.0, 0.0]);
        assert_eq!(p.points.len(), 1);
        p.add_point([1.0, 0.0, 0.0]);
        assert_eq!(p.points.len(), 2);
    }

    #[test]
    fn parameterizer_preserves_param_type() {
        let p = CurveParameterizer::new(ParametrizationType::Centripetal);
        assert_eq!(p.param_type, ParametrizationType::Centripetal);
    }
}
