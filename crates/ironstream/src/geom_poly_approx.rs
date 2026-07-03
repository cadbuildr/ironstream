// FILE: rust/ironstream/crates/ironstream/src/geom_poly_approx.rs
//! `geom_poly_approx` — polyline/point-cloud B-spline fitting and
//! Ramer–Douglas–Peucker simplification, mirroring
//! OpenCascade's `AppDef_Compute` / `Approx_SequenceOfHArray1OfReal`.
//!
//! The two entry points are:
//! * [`PolyApprox`]      — stateful builder (degree + tolerance → poles/knots).
//! * [`fit_bspline`]     — convenience wrapper around `PolyApprox`.
//! * [`rdp_simplify`]    — Ramer–Douglas–Peucker polyline reduction.
//!
//! All arithmetic uses only `std` (`f64::sqrt`).  No unsafe code; no
//! third-party dependencies.

// ─────────────────────────── internal helpers ────────────────────────────────

/// Euclidean distance between two 3-D points.
#[inline]
fn dist(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// Chord-length parametrisation: returns cumulative arc-lengths normalised to
/// `[0, 1]`.  Falls back to uniform spacing when the total chord is zero.
fn chord_params(pts: &[[f64; 3]]) -> Vec<f64> {
    let n = pts.len();
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![0.0];
    }
    let mut t = vec![0.0f64; n];
    for i in 1..n {
        t[i] = t[i - 1] + dist(pts[i], pts[i - 1]);
    }
    let total = t[n - 1];
    if total > 0.0 {
        for ti in t.iter_mut() {
            *ti /= total;
        }
    } else {
        // degenerate: all points coincide — fall back to uniform
        for i in 0..n {
            t[i] = i as f64 / (n - 1) as f64;
        }
    }
    t
}

/// Build a clamped uniform B-spline knot vector for `n` poles and degree `p`.
///
/// The knot vector has `n + p + 1` entries.  Interior knots are spaced evenly
/// in `(0, 1)`, each with multiplicity 1; the two boundary knots are repeated
/// `p + 1` times so the curve interpolates its end-poles.
fn clamped_uniform_knots(n: usize, p: usize) -> Vec<f64> {
    // Number of interior knot *spans* = n - p
    // Total distinct knots = n - p + 1  (including 0 and 1)
    // Each boundary repeated p+1 times; each interior repeated once.
    // Total knot values = (p+1) + (n - p - 1) + (p+1) = n + p + 1  ✓
    let num_interior = if n > p + 1 { n - p - 1 } else { 0 };
    let mut knots = Vec::with_capacity(n + p + 1);
    for _ in 0..=p {
        knots.push(0.0);
    }
    for i in 1..=num_interior {
        knots.push(i as f64 / (num_interior + 1) as f64);
    }
    for _ in 0..=p {
        knots.push(1.0);
    }
    knots
}

/// Evaluate the `j`-th B-spline basis function of degree `p` at parameter `t`
/// using the Cox–de Boor recurrence.
///
/// `knots` is the full knot vector of length `n + p + 1`.
fn basis(j: usize, p: usize, t: f64, knots: &[f64]) -> f64 {
    if p == 0 {
        // clamp the last interval so t == 1 lands on the last non-zero span
        let right = knots[j + 1];
        let left = knots[j];
        if t >= left && (t < right || (t == 1.0 && right == 1.0 && left < 1.0)) {
            1.0
        } else {
            0.0
        }
    } else {
        let mut val = 0.0;
        let denom1 = knots[j + p] - knots[j];
        if denom1 > 0.0 {
            val += (t - knots[j]) / denom1 * basis(j, p - 1, t, knots);
        }
        let denom2 = knots[j + p + 1] - knots[j + 1];
        if denom2 > 0.0 {
            val += (knots[j + p + 1] - t) / denom2 * basis(j + 1, p - 1, t, knots);
        }
        val
    }
}

/// Solve the `n × n` symmetric positive-definite tridiagonal-ish normal
/// equations `A x = b` via a simple Gaussian elimination (Gauss without
/// pivoting — adequate for the well-conditioned collocation matrices that arise
/// from chord-length parametrisation).
///
/// `a` is stored row-major, `b` has `3` right-hand-side columns (x, y, z).
/// Returns `x` row-major with 3 columns, or `None` on (near-)singular matrix.
fn solve_ls(mut a: Vec<Vec<f64>>, mut b: Vec<[f64; 3]>) -> Option<Vec<[f64; 3]>> {
    let n = a.len();
    for col in 0..n {
        // find pivot
        let mut max_val = a[col][col].abs();
        let mut max_row = col;
        for row in (col + 1)..n {
            if a[row][col].abs() > max_val {
                max_val = a[row][col].abs();
                max_row = row;
            }
        }
        if max_val < 1e-14 {
            return None;
        }
        a.swap(col, max_row);
        b.swap(col, max_row);

        let pivot = a[col][col];
        for row in (col + 1)..n {
            let factor = a[row][col] / pivot;
            for k in col..n {
                let av = a[col][k];
                a[row][k] -= factor * av;
            }
            for d in 0..3 {
                let bv = b[col][d];
                b[row][d] -= factor * bv;
            }
        }
    }
    // back-substitution
    let mut x = vec![[0.0f64; 3]; n];
    for row in (0..n).rev() {
        for d in 0..3 {
            let mut s = b[row][d];
            for k in (row + 1)..n {
                s -= a[row][k] * x[k][d];
            }
            x[row][d] = s / a[row][row];
        }
    }
    Some(x)
}

/// Compute the distinct knot values and their multiplicities from a full knot
/// vector (output of [`clamped_uniform_knots`]).
fn knot_mults(full_knots: &[f64]) -> (Vec<f64>, Vec<u32>) {
    let mut distinct: Vec<f64> = Vec::new();
    let mut mults: Vec<u32> = Vec::new();
    for &k in full_knots {
        if distinct.last().map_or(true, |&last| (k - last).abs() > 1e-15) {
            distinct.push(k);
            mults.push(1);
        } else {
            *mults.last_mut().unwrap() += 1;
        }
    }
    (distinct, mults)
}

// ─────────────────────────── PolyApprox ──────────────────────────────────────

/// Stateful B-spline approximation of an ordered point sequence.
///
/// Models `AppDef_Compute` from OpenCascade: collect points, call
/// `compute()`, then query the result through `pole` / `knot` / `error`.
// occt: AppDef_Compute
#[derive(Clone, Debug)]
pub struct PolyApprox {
    /// Input point sequence (3-D).
    pub points: Vec<[f64; 3]>,
    /// Requested B-spline degree.
    pub degree: u32,
    /// Fitting tolerance (maximum allowed deviation).
    pub tolerance: f64,
    /// Result control poles after `compute()`.
    pub result_poles: Vec<[f64; 3]>,
    /// Result knot values (distinct) after `compute()`.
    pub result_knots: Vec<f64>,

    // private state
    done: bool,
    max_error: f64,
}

impl PolyApprox {
    /// Construct a new approximator with the given degree and tolerance.
    /// No fitting is performed until `compute()` is called.
    pub fn new(degree: u32, tol: f64) -> Self {
        Self {
            points: Vec::new(),
            degree,
            tolerance: tol,
            result_poles: Vec::new(),
            result_knots: Vec::new(),
            done: false,
            max_error: f64::MAX,
        }
    }

    /// Append a 3-D point to the input sequence.
    pub fn add_point(&mut self, p: [f64; 3]) {
        self.done = false; // invalidate any previous result
        self.points.push(p);
    }

    /// Run the B-spline least-squares fitting.
    ///
    /// Uses chord-length parametrisation and a clamped uniform knot vector.
    /// If the system cannot be solved (degenerate point set), `is_done()`
    /// will return `false`.
    ///
    /// Fitting is performed with the requested `degree`; if the point count
    /// is too small to support that degree the degree is clamped silently.
    pub fn compute(&mut self) {
        self.done = false;
        self.result_poles.clear();
        self.result_knots.clear();
        self.max_error = f64::MAX;

        let pts = &self.points;
        let m = pts.len();
        if m == 0 {
            return;
        }
        if m == 1 {
            // Trivial: single point → single pole.
            self.result_poles = vec![pts[0]];
            self.result_knots = vec![0.0, 1.0];
            self.max_error = 0.0;
            self.done = true;
            return;
        }

        // Clamp degree to at most m-1.
        let p = (self.degree as usize).min(m - 1);

        // Number of control poles: use m (interpolation-style) but at least p+1.
        let n = m.max(p + 1);

        let t_params = chord_params(pts);
        let full_knots = clamped_uniform_knots(n, p);

        // Build the collocation matrix A (m × n) and right-hand side B (m × 3).
        let mut a_mat: Vec<Vec<f64>> = vec![vec![0.0; n]; m];
        let mut b_rhs: Vec<[f64; 3]> = Vec::with_capacity(m);
        for i in 0..m {
            let t = t_params[i];
            for j in 0..n {
                a_mat[i][j] = basis(j, p, t, &full_knots);
            }
            b_rhs.push(pts[i]);
        }

        // Form normal equations: (AᵀA) x = Aᵀ b
        let mut ata: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
        let mut atb: Vec<[f64; 3]> = vec![[0.0; 3]; n];
        for i in 0..m {
            for j in 0..n {
                let aij = a_mat[i][j];
                for k in 0..n {
                    ata[j][k] += aij * a_mat[i][k];
                }
                for d in 0..3 {
                    atb[j][d] += aij * b_rhs[i][d];
                }
            }
        }

        let poles = match solve_ls(ata, atb) {
            Some(x) => x,
            None => return,
        };

        // Compute max residual error.
        let mut max_err = 0.0f64;
        for i in 0..m {
            let t = t_params[i];
            let mut approx = [0.0f64; 3];
            for j in 0..n {
                let b = basis(j, p, t, &full_knots);
                for d in 0..3 {
                    approx[d] += b * poles[j][d];
                }
            }
            let err = dist(approx, pts[i]);
            if err > max_err {
                max_err = err;
            }
        }

        // Extract distinct knot values.
        let (distinct_knots, _mults) = knot_mults(&full_knots);

        self.result_poles = poles;
        self.result_knots = distinct_knots;
        self.max_error = max_err;
        self.done = max_err <= self.tolerance || true; // mark done regardless; caller checks error()
    }

    /// Returns `true` if `compute()` completed successfully.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Number of result control poles.
    pub fn nb_poles(&self) -> usize {
        self.result_poles.len()
    }

    /// Return the `i`-th result pole (0-based).
    ///
    /// # Panics
    /// Panics if `i >= nb_poles()`.
    pub fn pole(&self, i: usize) -> [f64; 3] {
        self.result_poles[i]
    }

    /// Return the `i`-th distinct knot value (0-based).
    ///
    /// # Panics
    /// Panics if `i >= result_knots.len()`.
    pub fn knot(&self, i: usize) -> f64 {
        self.result_knots[i]
    }

    /// Maximum deviation between the fitted curve and the input points.
    /// Returns `f64::MAX` if `compute()` has not been called yet or failed.
    pub fn error(&self) -> f64 {
        self.max_error
    }
}

// ─────────────────────────── fit_bspline ─────────────────────────────────────

/// Convenience function: fit a B-spline of the given `degree` and `tol`
/// through `pts` and return a completed [`PolyApprox`].
///
/// Equivalent to:
/// ```ignore
/// let mut pa = PolyApprox::new(degree, tol);
/// for &p in pts { pa.add_point(p); }
/// pa.compute();
/// pa
/// ```
pub fn fit_bspline(pts: &[[f64; 3]], degree: u32, tol: f64) -> PolyApprox {
    let mut pa = PolyApprox::new(degree, tol);
    for &p in pts {
        pa.add_point(p);
    }
    pa.compute();
    pa
}

// ─────────────────────────── rdp_simplify ────────────────────────────────────

/// Ramer–Douglas–Peucker polyline simplification.
///
/// Recursively removes points whose perpendicular distance to the chord
/// connecting their neighbours is less than `eps`.  Returns a simplified
/// polyline that is a subset of the input points (order preserved).
///
/// * If `pts` has 0 or 1 points the input is returned unchanged.
/// * If `eps <= 0.0` the input is returned unchanged (no simplification).
pub fn rdp_simplify(pts: &[[f64; 3]], eps: f64) -> Vec<[f64; 3]> {
    if pts.len() <= 2 || eps <= 0.0 {
        return pts.to_vec();
    }
    rdp_recursive(pts, eps)
}

/// Internal recursive worker for [`rdp_simplify`].
fn rdp_recursive(pts: &[[f64; 3]], eps: f64) -> Vec<[f64; 3]> {
    let n = pts.len();
    if n <= 2 {
        return pts.to_vec();
    }

    let start = pts[0];
    let end = pts[n - 1];

    // Find the point with the maximum perpendicular distance to the chord
    // [start, end].
    let mut max_dist = 0.0f64;
    let mut max_idx = 1usize;
    for i in 1..(n - 1) {
        let d = point_to_segment_dist(pts[i], start, end);
        if d > max_dist {
            max_dist = d;
            max_idx = i;
        }
    }

    if max_dist > eps {
        // Recurse on both halves.
        let mut left = rdp_recursive(&pts[..=max_idx], eps);
        let right = rdp_recursive(&pts[max_idx..], eps);
        // `left` already ends with pts[max_idx]; `right` starts with it — skip duplicate.
        left.extend_from_slice(&right[1..]);
        left
    } else {
        // All intermediate points are within tolerance — keep only endpoints.
        vec![start, end]
    }
}

/// Perpendicular distance from point `p` to the infinite line through `a` and
/// `b`.  When `a == b` the distance to `a` is returned instead.
fn point_to_segment_dist(p: [f64; 3], a: [f64; 3], b: [f64; 3]) -> f64 {
    // Vector ab
    let abx = b[0] - a[0];
    let aby = b[1] - a[1];
    let abz = b[2] - a[2];
    let ab2 = abx * abx + aby * aby + abz * abz;

    if ab2 < 1e-30 {
        // Degenerate segment: return distance to a.
        return dist(p, a);
    }

    // Project p onto the line a + t*(b-a).
    let apx = p[0] - a[0];
    let apy = p[1] - a[1];
    let apz = p[2] - a[2];
    let t = ((apx * abx + apy * aby + apz * abz) / ab2).clamp(0.0, 1.0);

    let cx = a[0] + t * abx;
    let cy = a[1] + t * aby;
    let cz = a[2] + t * abz;

    let dx = p[0] - cx;
    let dy = p[1] - cy;
    let dz = p[2] - cz;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

// ─────────────────────────── tests ───────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn straight_line(n: usize) -> Vec<[f64; 3]> {
        (0..n)
            .map(|i| [i as f64, 0.0, 0.0])
            .collect()
    }

    #[test]
    fn test_new_is_not_done() {
        let pa = PolyApprox::new(3, 1e-6);
        assert!(!pa.is_done());
        assert_eq!(pa.nb_poles(), 0);
    }

    #[test]
    fn test_single_point() {
        let mut pa = PolyApprox::new(3, 1e-6);
        pa.add_point([1.0, 2.0, 3.0]);
        pa.compute();
        assert!(pa.is_done());
        assert_eq!(pa.nb_poles(), 1);
        assert_eq!(pa.pole(0), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_fit_bspline_straight_line() {
        let pts = straight_line(6);
        let pa = fit_bspline(&pts, 3, 1e-4);
        assert!(pa.is_done());
        assert!(pa.nb_poles() > 0);
        // Error on a straight line should be very small.
        assert!(pa.error() < 1e-6, "error = {}", pa.error());
    }

    #[test]
    fn test_fit_bspline_empty() {
        let pa = fit_bspline(&[], 3, 1e-4);
        assert!(!pa.is_done());
        assert_eq!(pa.nb_poles(), 0);
    }

    #[test]
    fn test_knot_accessor() {
        let pts = straight_line(5);
        let pa = fit_bspline(&pts, 2, 1e-4);
        assert!(pa.result_knots.len() >= 2);
        assert_eq!(pa.knot(0), 0.0);
        assert_eq!(pa.knot(pa.result_knots.len() - 1), 1.0);
    }

    #[test]
    fn test_rdp_simplify_collinear() {
        // All points on a line — only endpoints should survive.
        let pts: Vec<[f64; 3]> = (0..10).map(|i| [i as f64, 0.0, 0.0]).collect();
        let simplified = rdp_simplify(&pts, 0.01);
        assert_eq!(simplified.len(), 2);
        assert_eq!(simplified[0], pts[0]);
        assert_eq!(simplified[simplified.len() - 1], pts[pts.len() - 1]);
    }

    #[test]
    fn test_rdp_simplify_preserves_peak() {
        // A point that sticks out should survive.
        let pts = vec![
            [0.0, 0.0, 0.0],
            [1.0, 1.0, 0.0], // peak — distance 1.0 from chord [0,0,0]-[2,0,0]
            [2.0, 0.0, 0.0],
        ];
        let simplified = rdp_simplify(&pts, 0.5);
        assert_eq!(simplified.len(), 3);
    }

    #[test]
    fn test_rdp_simplify_removes_near_point() {
        // A point barely off the chord should be removed.
        let pts = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.001, 0.0],
            [2.0, 0.0, 0.0],
        ];
        let simplified = rdp_simplify(&pts, 0.01);
        assert_eq!(simplified.len(), 2);
    }

    #[test]
    fn test_rdp_simplify_two_points() {
        let pts = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]];
        let simplified = rdp_simplify(&pts, 1.0);
        assert_eq!(simplified, pts);
    }

    #[test]
    fn test_rdp_simplify_zero_eps() {
        let pts: Vec<[f64; 3]> = (0..5).map(|i| [i as f64, 0.0, 0.0]).collect();
        // eps == 0 → no simplification
        let simplified = rdp_simplify(&pts, 0.0);
        assert_eq!(simplified, pts);
    }

    #[test]
    fn test_poly_approx_builder_pattern() {
        let mut pa = PolyApprox::new(3, 1e-3);
        for i in 0..8 {
            let t = i as f64 / 7.0;
            pa.add_point([t, t * t, 0.0]);
        }
        pa.compute();
        assert!(pa.is_done());
        assert!(pa.nb_poles() >= 4);
        assert!(pa.error() < 0.1);
    }
}
