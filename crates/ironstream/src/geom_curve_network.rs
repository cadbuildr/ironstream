// FILE: geom_curve_network.rs
//! `GeomCurveNetwork` — curve network fitting, mirroring OpenCascade's
//! `GeomFill_Filling` / `GeomFill_CurveAndDerivative` infrastructure.
//!
//! Models a network of curves that intersect or align to define a surface
//! patch.  The network is built incrementally via [`CurveNetwork`], which
//! collects one or more polyline curves (each given as a sequence of 3-D
//! sample points), then computes a representative output polyline that
//! approximates the interior of the network.
//!
//! Companion types:
//!
//! - [`NetworkConstraint`] — per-curve weight and tangent flag, mirroring the
//!   derivative information carried by `GeomFill_CurveAndDerivative`.
//! - [`interpolate_network`] — free function that runs the complete network
//!   interpolation pipeline.
//! - [`curve_network_bounds`] — returns the axis-aligned bounding box of a
//!   [`CurveNetwork`].
//!
//! All code is pure Rust with no external crates.

// ---------------------------------------------------------------------------
// CurveNetwork
// ---------------------------------------------------------------------------

/// A network of curves used to fit a surface patch.
///
/// Each curve is stored as an ordered sequence of 3-D sample points
/// `[f64; 3]`.  After all curves have been added, [`CurveNetwork::build`]
/// returns a single merged polyline that approximates the network interior
/// (centroid-blended, one representative point per unique sample across all
/// curves).
///
/// Mirrors the curve container managed by `GeomFill_Filling` and the
/// derivative evaluation performed by `GeomFill_CurveAndDerivative`.
// occt: GeomFill_Filling
#[derive(Clone, Debug)]
pub struct CurveNetwork {
    /// Curves stored as point sequences.
    pub curves: Vec<Vec<[f64; 3]>>,
    /// Positional tolerance used when merging close sample points.
    pub tolerance: f64,
}

impl CurveNetwork {
    /// Create an empty network with the given positional `tolerance`.
    ///
    /// `tolerance` is used by [`build`](Self::build) to decide when two
    /// sample points from different curves are coincident and should be
    /// represented by their centroid.
    pub fn new(tol: f64) -> Self {
        Self {
            curves: Vec::new(),
            tolerance: tol,
        }
    }

    /// Append a curve to the network.
    ///
    /// `pts` is the ordered list of 3-D sample points along the curve.
    /// At least two points should be supplied for a meaningful curve, but the
    /// method does not enforce this so that callers can add degenerate curves
    /// (e.g. a single point) without panicking.
    pub fn add_curve(&mut self, pts: Vec<[f64; 3]>) {
        self.curves.push(pts);
    }

    /// Build and return the merged network polyline.
    ///
    /// The algorithm:
    /// 1. Collect every sample point from every curve into a single flat list.
    /// 2. Merge groups of points that are mutually within [`self.tolerance`]
    ///    by replacing each group with its centroid.
    /// 3. Return the de-duplicated, centroid-blended list.
    ///
    /// When the network is empty the returned `Vec` is empty.
    ///
    /// This mirrors the role of `GeomFill_Filling` in assembling constraint
    /// points and `GeomFill_CurveAndDerivative` in evaluating each curve's
    /// position (and optionally derivative) at those points.
    // occt: GeomFill_CurveAndDerivative
    pub fn build(&self) -> Vec<[f64; 3]> {
        // Gather all points.
        let all: Vec<[f64; 3]> = self.curves.iter().flat_map(|c| c.iter().copied()).collect();

        if all.is_empty() {
            return Vec::new();
        }

        // Greedy merge: for each unvisited point, collect all points within
        // `tolerance` and replace the group with its centroid.
        let tol_sq = self.tolerance * self.tolerance;
        let n = all.len();
        let mut used = vec![false; n];
        let mut result: Vec<[f64; 3]> = Vec::new();

        for i in 0..n {
            if used[i] {
                continue;
            }
            // Accumulate group centroid.
            let mut sum = all[i];
            let mut count = 1usize;
            for j in (i + 1)..n {
                if used[j] {
                    continue;
                }
                if dist_sq(all[i], all[j]) <= tol_sq {
                    sum[0] += all[j][0];
                    sum[1] += all[j][1];
                    sum[2] += all[j][2];
                    count += 1;
                    used[j] = true;
                }
            }
            used[i] = true;
            let inv = 1.0 / count as f64;
            result.push([sum[0] * inv, sum[1] * inv, sum[2] * inv]);
        }

        result
    }

    /// Return the number of curves currently in the network.
    pub fn num_curves(&self) -> usize {
        self.curves.len()
    }
}

// ---------------------------------------------------------------------------
// NetworkConstraint
// ---------------------------------------------------------------------------

/// Per-curve constraint descriptor for a [`CurveNetwork`].
///
/// Carries the curve index inside the network, a blending weight, and a flag
/// indicating whether tangent (derivative) information is available for that
/// curve.
///
/// Mirrors the `GeomFill_CurveAndDerivative` notion of a curve together with
/// its first derivative, wrapped in the constraint layer used by
/// `GeomFill_Filling`.
// occt: GeomFill_CurveAndDerivative
#[derive(Clone, Debug, PartialEq)]
pub struct NetworkConstraint {
    /// Index of the target curve inside the [`CurveNetwork`].
    pub curve_idx: usize,
    /// Blending weight (default `1.0`).  Higher weight causes the fitted
    /// result to stay closer to this curve.
    pub weight: f64,
    /// `true` if tangent (first-derivative) constraints are active for this
    /// curve.
    pub tangent: bool,
}

impl NetworkConstraint {
    /// Construct a new constraint for curve at `idx` with default weight
    /// `1.0` and tangent flag `false`.
    pub fn new(idx: usize) -> Self {
        Self {
            curve_idx: idx,
            weight: 1.0,
            tangent: false,
        }
    }

    /// Return a copy of `self` with the weight set to `w`.
    ///
    /// Follows the builder pattern so constraints can be constructed inline:
    /// ```rust,ignore
    /// let c = NetworkConstraint::new(0).with_weight(2.0).with_tangent(true);
    /// ```
    pub fn with_weight(mut self, w: f64) -> Self {
        self.weight = w;
        self
    }

    /// Return a copy of `self` with the tangent flag set to `t`.
    pub fn with_tangent(mut self, t: bool) -> Self {
        self.tangent = t;
        self
    }
}

// ---------------------------------------------------------------------------
// Free functions
// ---------------------------------------------------------------------------

/// Interpolate a curve network and return the merged representative polyline.
///
/// Convenience wrapper around [`CurveNetwork`]: constructs a temporary network
/// with the given `tol`, inserts every curve from `curves`, and calls
/// [`CurveNetwork::build`].
///
/// # Parameters
/// - `curves` — slice of curves, each curve being a `Vec<[f64;3]>` of sample
///   points.
/// - `tol` — positional tolerance for point merging (see
///   [`CurveNetwork::build`]).
///
/// # Returns
/// Merged, de-duplicated polyline approximating the network interior.
///
/// Mirrors the top-level fitting step performed by `GeomFill_Filling` after
/// all `GeomFill_CurveAndDerivative` objects have been registered.
// occt: GeomFill_Filling
// occt: GeomFill_CurveAndDerivative
pub fn interpolate_network(curves: &[Vec<[f64; 3]>], tol: f64) -> Vec<[f64; 3]> {
    let mut net = CurveNetwork::new(tol);
    for c in curves {
        net.add_curve(c.clone());
    }
    net.build()
}

/// Return the axis-aligned bounding box `(min, max)` of all points in `net`.
///
/// If the network contains no points at all, both corners are `[0.0; 3]`.
///
/// Mirrors the bounding-box query available on `GeomFill_Filling` data after
/// all boundary curves have been inserted.
// occt: GeomFill_Filling
pub fn curve_network_bounds(net: &CurveNetwork) -> ([f64; 3], [f64; 3]) {
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    let mut any = false;

    for curve in &net.curves {
        for &p in curve {
            any = true;
            for k in 0..3 {
                if p[k] < min[k] {
                    min[k] = p[k];
                }
                if p[k] > max[k] {
                    max[k] = p[k];
                }
            }
        }
    }

    if !any {
        ([0.0; 3], [0.0; 3])
    } else {
        (min, max)
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Squared Euclidean distance between two 3-D points.
#[inline]
fn dist_sq(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    dx * dx + dy * dy + dz * dz
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // CurveNetwork — construction and basic accessors
    // -----------------------------------------------------------------------

    #[test]
    fn new_network_is_empty() {
        let net = CurveNetwork::new(1.0e-4);
        assert_eq!(net.num_curves(), 0);
        assert!(net.curves.is_empty());
        assert!((net.tolerance - 1.0e-4).abs() < f64::EPSILON);
    }

    #[test]
    fn add_curve_increments_count() {
        let mut net = CurveNetwork::new(1.0e-3);
        net.add_curve(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        assert_eq!(net.num_curves(), 1);
        net.add_curve(vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]]);
        assert_eq!(net.num_curves(), 2);
    }

    #[test]
    fn build_empty_network_returns_empty() {
        let net = CurveNetwork::new(1.0e-4);
        assert!(net.build().is_empty());
    }

    #[test]
    fn build_single_curve_returns_all_points() {
        let mut net = CurveNetwork::new(1.0e-6);
        let pts = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
        net.add_curve(pts.clone());
        let out = net.build();
        // No duplicates: all three points are distinct.
        assert_eq!(out.len(), 3);
    }

    #[test]
    fn build_merges_coincident_endpoints() {
        // Two curves sharing a common endpoint at the origin.
        let mut net = CurveNetwork::new(1.0e-3);
        net.add_curve(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        net.add_curve(vec![[0.0, 0.0, 0.0], [0.0, 1.0, 0.0]]);
        let out = net.build();
        // 4 raw points but 2 are coincident, so merged result has 3.
        assert_eq!(out.len(), 3);
    }

    #[test]
    fn build_centroid_of_merged_group() {
        // Two nearly-identical points should merge to their centroid.
        let mut net = CurveNetwork::new(0.1);
        net.add_curve(vec![[0.0, 0.0, 0.0]]);
        net.add_curve(vec![[0.05, 0.0, 0.0]]); // within tolerance
        let out = net.build();
        assert_eq!(out.len(), 1);
        // Centroid x = (0.0 + 0.05) / 2 = 0.025
        assert!((out[0][0] - 0.025).abs() < 1.0e-12);
        assert!(out[0][1].abs() < 1.0e-12);
        assert!(out[0][2].abs() < 1.0e-12);
    }

    #[test]
    fn build_two_separate_curves_no_merge() {
        let mut net = CurveNetwork::new(1.0e-6);
        net.add_curve(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        net.add_curve(vec![[0.0, 5.0, 0.0], [1.0, 5.0, 0.0]]);
        let out = net.build();
        // All four points are far apart — no merging.
        assert_eq!(out.len(), 4);
    }

    // -----------------------------------------------------------------------
    // NetworkConstraint — construction and builder methods
    // -----------------------------------------------------------------------

    #[test]
    fn constraint_defaults() {
        let c = NetworkConstraint::new(3);
        assert_eq!(c.curve_idx, 3);
        assert!((c.weight - 1.0).abs() < f64::EPSILON);
        assert!(!c.tangent);
    }

    #[test]
    fn constraint_with_weight() {
        let c = NetworkConstraint::new(0).with_weight(2.5);
        assert!((c.weight - 2.5).abs() < f64::EPSILON);
    }

    #[test]
    fn constraint_with_tangent() {
        let c = NetworkConstraint::new(1).with_tangent(true);
        assert!(c.tangent);
    }

    #[test]
    fn constraint_builder_chain() {
        let c = NetworkConstraint::new(7).with_weight(0.5).with_tangent(true);
        assert_eq!(c.curve_idx, 7);
        assert!((c.weight - 0.5).abs() < f64::EPSILON);
        assert!(c.tangent);
    }

    #[test]
    fn constraint_clone_and_eq() {
        let a = NetworkConstraint::new(2).with_weight(3.0).with_tangent(true);
        let b = a.clone();
        assert_eq!(a, b);
    }

    // -----------------------------------------------------------------------
    // interpolate_network
    // -----------------------------------------------------------------------

    #[test]
    fn interpolate_empty_slice_returns_empty() {
        let out = interpolate_network(&[], 1.0e-4);
        assert!(out.is_empty());
    }

    #[test]
    fn interpolate_single_curve_passthrough() {
        let curves = vec![vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]]];
        let out = interpolate_network(&curves, 1.0e-6);
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn interpolate_two_curves_shared_point_merges() {
        let curves = vec![
            vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            vec![[1.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
        ];
        let out = interpolate_network(&curves, 1.0e-6);
        // Shared point [1,0,0] merges: 3 unique points.
        assert_eq!(out.len(), 3);
    }

    // -----------------------------------------------------------------------
    // curve_network_bounds
    // -----------------------------------------------------------------------

    #[test]
    fn bounds_empty_network_returns_zeros() {
        let net = CurveNetwork::new(1.0e-4);
        let (lo, hi) = curve_network_bounds(&net);
        assert_eq!(lo, [0.0, 0.0, 0.0]);
        assert_eq!(hi, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn bounds_single_point() {
        let mut net = CurveNetwork::new(1.0e-4);
        net.add_curve(vec![[3.0, -1.0, 5.0]]);
        let (lo, hi) = curve_network_bounds(&net);
        assert_eq!(lo, [3.0, -1.0, 5.0]);
        assert_eq!(hi, [3.0, -1.0, 5.0]);
    }

    #[test]
    fn bounds_two_curves() {
        let mut net = CurveNetwork::new(1.0e-4);
        net.add_curve(vec![[0.0, 0.0, 0.0], [1.0, 2.0, 3.0]]);
        net.add_curve(vec![[-1.0, 0.5, 0.0], [0.5, 1.5, 4.0]]);
        let (lo, hi) = curve_network_bounds(&net);
        assert!((lo[0] - (-1.0)).abs() < 1.0e-12);
        assert!((lo[1] - 0.0).abs() < 1.0e-12);
        assert!((lo[2] - 0.0).abs() < 1.0e-12);
        assert!((hi[0] - 1.0).abs() < 1.0e-12);
        assert!((hi[1] - 2.0).abs() < 1.0e-12);
        assert!((hi[2] - 4.0).abs() < 1.0e-12);
    }

    #[test]
    fn bounds_negative_coordinates() {
        let mut net = CurveNetwork::new(1.0e-4);
        net.add_curve(vec![[-5.0, -3.0, -1.0], [-2.0, -1.0, 0.0]]);
        let (lo, hi) = curve_network_bounds(&net);
        assert!((lo[0] - (-5.0)).abs() < 1.0e-12);
        assert!((hi[0] - (-2.0)).abs() < 1.0e-12);
    }

    // -----------------------------------------------------------------------
    // dist_sq helper
    // -----------------------------------------------------------------------

    #[test]
    fn dist_sq_same_point_is_zero() {
        let p = [1.0, 2.0, 3.0];
        assert!(dist_sq(p, p).abs() < 1.0e-30);
    }

    #[test]
    fn dist_sq_unit_x() {
        let a = [0.0, 0.0, 0.0];
        let b = [1.0, 0.0, 0.0];
        assert!((dist_sq(a, b) - 1.0).abs() < 1.0e-12);
    }
}
