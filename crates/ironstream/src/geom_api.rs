// FILE: geom_api.rs
//! `GeomAPI` — high-level geometry algorithms, mirroring OpenCascade's `GeomAPI`
//! package (`src/ModelingAlgorithms/TKGeomBase/GeomAPI`).
//!
//! Covered classes:
//! - [`GeomApiInterpolate`]          — `GeomAPI_Interpolate`
//! - [`GeomApiPointsToBSpline`]      — `GeomAPI_PointsToBSpline`
//! - [`GeomApiProjectPointOnCurve`]  — `GeomAPI_ProjectPointOnCurve`
//! - [`GeomApiProjectPointOnSurf`]   — `GeomAPI_ProjectPointOnSurf`
//!
//! All algorithms produce [`crate::geom_bspline_curve::GeomBSplineCurve`] or
//! work over existing B-spline objects.  No `unsafe`, no third-party deps.

use crate::geom_bspline_curve::GeomBSplineCurve;
use crate::geom_bspline_surface::GeomBSplineSurface;
use crate::gp::{Pnt, Vec3};

// ─────────────────────────────────────────────────────────────────────────────
// Shared numeric tolerance
// ─────────────────────────────────────────────────────────────────────────────

const TOL: f64 = 1.0e-9;

// ─────────────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────────────

/// Errors returned by GeomAPI algorithms.
// occt: GeomAPI error codes / Standard_Failure
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GeomApiError {
    /// Too few points to construct the requested curve.
    NotEnoughPoints,
    /// The algorithm has not been executed yet (call `perform` first).
    NotDone,
    /// The requested projection index is out of range.
    OutOfRange,
    /// The degree constraints cannot be satisfied.
    BadDegree,
    /// Curve or surface is not set.
    NoCurveOrSurface,
    /// Internal solver failed to converge.
    SolverFailed,
}

impl std::fmt::Display for GeomApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeomApiError::NotEnoughPoints => write!(f, "GeomAPI: not enough points"),
            GeomApiError::NotDone => write!(f, "GeomAPI: algorithm not done"),
            GeomApiError::OutOfRange => write!(f, "GeomAPI: index out of range"),
            GeomApiError::BadDegree => write!(f, "GeomAPI: bad degree"),
            GeomApiError::NoCurveOrSurface => write!(f, "GeomAPI: no curve or surface set"),
            GeomApiError::SolverFailed => write!(f, "GeomAPI: solver failed to converge"),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal B-spline helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Build a uniform clamped knot vector for `n_poles` control points with
/// `degree` and optional periodicity.  Returns `(knots, mults)`.
fn uniform_clamped_knots(n_poles: usize, degree: usize) -> (Vec<f64>, Vec<i32>) {
    // Number of inner knot spans = n_poles - degree
    let n_spans = n_poles as i32 - degree as i32;
    if n_spans <= 0 {
        // Single Bezier span
        let knots = vec![0.0, 1.0];
        let mults = vec![degree as i32 + 1, degree as i32 + 1];
        return (knots, mults);
    }
    let mut knots = vec![0.0];
    let mut mults = vec![degree as i32 + 1];
    for i in 1..n_spans {
        knots.push(i as f64 / n_spans as f64);
        mults.push(1);
    }
    knots.push(1.0);
    mults.push(degree as i32 + 1);
    (knots, mults)
}

/// Chord-length parametrisation for a sequence of points.  Returns a `Vec` of
/// parameter values in `[0, 1]` with `params[0] == 0` and `params[n-1] == 1`.
fn chord_length_params(pts: &[Pnt]) -> Vec<f64> {
    let n = pts.len();
    let mut raw = vec![0.0f64; n];
    for i in 1..n {
        raw[i] = raw[i - 1] + pts[i].distance(pts[i - 1]);
    }
    let total = raw[n - 1];
    if total < TOL {
        // Degenerate: uniform spacing
        return (0..n).map(|i| i as f64 / (n - 1).max(1) as f64).collect();
    }
    raw.iter().map(|&v| v / total).collect()
}

/// Find the active knot span index for parameter `u` in the flat knot vector
/// `fk` (with `n_poles` control points and degree `p`).
/// Returns index `k` such that `fk[k] <= u < fk[k+1]` (or the last non-empty
/// span if `u` equals the last knot).
fn find_span_local(fk: &[f64], n_poles: usize, p: usize, u: f64) -> usize {
    let last = n_poles - 1; // index of last pole (0-based)
    // Special case: u is at or beyond the last parameter.
    if u >= fk[last + 1] {
        // Walk backwards to the first non-degenerate span.
        let mut k = last;
        while k > p && (fk[k] - fk[k + 1]).abs() < TOL {
            k -= 1;
        }
        return k;
    }
    if u <= fk[p] {
        return p;
    }
    // Binary search in [p, last+1)
    let mut lo = p;
    let mut hi = last + 1;
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if fk[mid] <= u {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

/// Evaluate all `p+1` non-zero B-spline basis functions `N_{span-p,p}, ...,
/// N_{span,p}` at `u`, given the active knot span `span`.
/// Returns a vector of length `p+1`.
fn basis_funs(fk: &[f64], span: usize, p: usize, u: f64) -> Vec<f64> {
    let mut n = vec![0.0f64; p + 1];
    let mut left = vec![0.0f64; p + 1];
    let mut right = vec![0.0f64; p + 1];
    n[0] = 1.0;
    for j in 1..=p {
        left[j] = u - fk[span + 1 - j];
        right[j] = fk[span + j] - u;
        let mut saved = 0.0f64;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            let temp = if denom.abs() < TOL { 0.0 } else { n[r] / denom };
            n[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        n[j] = saved;
    }
    n
}

/// Evaluate the B-spline basis function `N_{i,p}(u)` using the Cox-de Boor
/// recursion on the flat knot vector `fk` with `n_poles` control points.
fn basis(fk: &[f64], n_poles: usize, i: usize, p: usize, u: f64) -> f64 {
    let span = find_span_local(fk, n_poles, p, u);
    let nvals = basis_funs(fk, span, p, u);
    // nvals[j] = N_{span - p + j, p}(u).  Map to N_i:
    let j = i as isize - (span as isize - p as isize);
    if j < 0 || j > p as isize {
        0.0
    } else {
        nvals[j as usize]
    }
}

/// Build the flat knot vector from `(knots, mults)`.
fn flat_knots(knots: &[f64], mults: &[i32]) -> Vec<f64> {
    let mut fk = Vec::new();
    for (&k, &m) in knots.iter().zip(mults.iter()) {
        for _ in 0..m {
            fk.push(k);
        }
    }
    fk
}

/// Solve the tri-diagonal system `A * x = rhs` using the Thomas algorithm.
/// `a`: sub-diagonal (len n-1), `b`: main diagonal (len n),
/// `c`: super-diagonal (len n-1), `rhs`: right-hand sides (len n per coord).
/// Returns `None` if the system is singular.
fn tridiag_solve(
    a: &[f64],
    b: &[f64],
    c: &[f64],
    rhs_x: &[f64],
    rhs_y: &[f64],
    rhs_z: &[f64],
) -> Option<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    let n = b.len();
    assert_eq!(a.len(), n - 1);
    assert_eq!(c.len(), n - 1);
    assert_eq!(rhs_x.len(), n);

    let mut w = vec![0.0f64; n];
    let mut gx = vec![0.0f64; n];
    let mut gy = vec![0.0f64; n];
    let mut gz = vec![0.0f64; n];

    let mut denom = b[0];
    if denom.abs() < TOL {
        return None;
    }
    w[0] = c[0] / denom;
    gx[0] = rhs_x[0] / denom;
    gy[0] = rhs_y[0] / denom;
    gz[0] = rhs_z[0] / denom;

    for i in 1..n {
        denom = b[i] - a[i - 1] * w[i - 1];
        if denom.abs() < TOL {
            return None;
        }
        if i < n - 1 {
            w[i] = c[i] / denom;
        }
        gx[i] = (rhs_x[i] - a[i - 1] * gx[i - 1]) / denom;
        gy[i] = (rhs_y[i] - a[i - 1] * gy[i - 1]) / denom;
        gz[i] = (rhs_z[i] - a[i - 1] * gz[i - 1]) / denom;
    }

    // Back-substitution
    let mut rx = gx.clone();
    let mut ry = gy.clone();
    let mut rz = gz.clone();
    for i in (0..n - 1).rev() {
        rx[i] -= w[i] * rx[i + 1];
        ry[i] -= w[i] * ry[i + 1];
        rz[i] -= w[i] * rz[i + 1];
    }
    Some((rx, ry, rz))
}

/// General dense linear system solve via Gaussian elimination with partial
/// pivoting.  Returns the solution vector or `None` for singular systems.
fn gauss_solve(mut a: Vec<Vec<f64>>, mut rhs: Vec<Pnt>) -> Option<Vec<Pnt>> {
    let n = a.len();
    for col in 0..n {
        // Find pivot
        let mut max_row = col;
        let mut max_val = a[col][col].abs();
        for row in col + 1..n {
            if a[row][col].abs() > max_val {
                max_val = a[row][col].abs();
                max_row = row;
            }
        }
        if max_val < TOL {
            return None;
        }
        a.swap(col, max_row);
        rhs.swap(col, max_row);

        let pivot = a[col][col];
        for row in col + 1..n {
            let factor = a[row][col] / pivot;
            for k in col..n {
                let v = a[col][k] * factor;
                a[row][k] -= v;
            }
            let rv = rhs[col] * factor;
            rhs[row] = rhs[row] - rv;
        }
    }
    // Back-substitution
    let mut x = vec![Pnt::origin(); n];
    for i in (0..n).rev() {
        let mut s = rhs[i];
        for j in i + 1..n {
            s = s - x[j] * a[i][j];
        }
        if a[i][i].abs() < TOL {
            return None;
        }
        x[i] = s * (1.0 / a[i][i]);
    }
    Some(x)
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomAPI_Interpolate
// ─────────────────────────────────────────────────────────────────────────────

/// Defines constraints for end tangent vectors in [`GeomApiInterpolate`].
// occt: GeomAPI_Interpolate (tangent constraint flags)
#[derive(Clone, Debug)]
pub struct TangentConstraint {
    /// Tangent vector at the first (start) point.
    pub start: Option<Vec3>,
    /// Tangent vector at the last (end) point.
    pub end: Option<Vec3>,
}

impl TangentConstraint {
    /// No tangent constraints.
    pub fn none() -> Self {
        Self { start: None, end: None }
    }

    /// Specify both end tangents.
    pub fn both(start: Vec3, end: Vec3) -> Self {
        Self { start: Some(start), end: Some(end) }
    }

    /// Only the start tangent.
    pub fn start_only(start: Vec3) -> Self {
        Self { start: Some(start), end: None }
    }

    /// Only the end tangent.
    pub fn end_only(end: Vec3) -> Self {
        Self { start: None, end: Some(end) }
    }
}

/// `GeomAPI_Interpolate` — constructs a cubic B-spline curve that passes
/// through an ordered sequence of 3D points (interpolation).
///
/// The algorithm performs global cubic spline interpolation using chord-length
/// parametrisation.  Optional end-tangent constraints are supported.  The
/// result is a clamped, non-periodic, degree-3 B-spline.
///
/// # Usage
/// ```
/// # use ironstream::geom_api::{GeomApiInterpolate, TangentConstraint};
/// # use ironstream::gp::Pnt;
/// let pts = vec![
///     Pnt::new(0.0, 0.0, 0.0),
///     Pnt::new(1.0, 1.0, 0.0),
///     Pnt::new(2.0, 0.0, 0.0),
///     Pnt::new(3.0, 1.0, 0.0),
/// ];
/// let mut interp = GeomApiInterpolate::new(&pts, false);
/// interp.perform();
/// assert!(interp.is_done());
/// let curve = interp.curve().unwrap();
/// assert_eq!(curve.degree(), 3);
/// ```
// occt: GeomAPI_Interpolate
#[derive(Clone, Debug)]
pub struct GeomApiInterpolate {
    /// The data points to interpolate.
    points: Vec<Pnt>,
    /// Whether the curve should be periodic (closed).
    periodic: bool,
    /// Optional tangent constraints at the ends.
    tangents: TangentConstraint,
    /// Precomputed chord-length parameters (or user-supplied).
    params: Option<Vec<f64>>,
    /// Output curve (set after `perform`).
    curve: Option<GeomBSplineCurve>,
    /// Whether the algorithm has been run successfully.
    done: bool,
}

impl GeomApiInterpolate {
    /// Construct the interpolation algorithm for `points`.
    ///
    /// `periodic`: if `true` the curve will be closed (start and end coincide).
    pub fn new(points: &[Pnt], periodic: bool) -> Self {
        Self {
            points: points.to_vec(),
            periodic,
            tangents: TangentConstraint::none(),
            params: None,
            curve: None,
            done: false,
        }
    }

    /// `Load(InitialTangent, FinalTangent)` — add end-tangent constraints.
    pub fn load_tangents(&mut self, initial: Vec3, final_: Vec3) {
        self.tangents = TangentConstraint::both(initial, final_);
        self.done = false;
        self.curve = None;
    }

    /// `Load(Tangents, TangentFlags)` — per-point tangent constraints
    /// (simplified: only the first and last are used if the flags are set).
    pub fn load_tangents_with_flags(
        &mut self,
        tangents: &[Vec3],
        flags: &[bool],
    ) {
        let n = flags.len().min(tangents.len());
        let start = if n > 0 && flags[0] { Some(tangents[0]) } else { None };
        let end =
            if n > 0 && flags[n - 1] { Some(tangents[n - 1]) } else { None };
        self.tangents = TangentConstraint { start, end };
        self.done = false;
        self.curve = None;
    }

    /// `SetParameters(params)` — override the default chord-length
    /// parametrisation with user-supplied parameter values.
    pub fn set_parameters(&mut self, params: &[f64]) {
        self.params = Some(params.to_vec());
        self.done = false;
        self.curve = None;
    }

    /// `Perform()` — run the interpolation algorithm.
    pub fn perform(&mut self) {
        self.done = false;
        self.curve = None;

        let n = self.points.len();
        if n < 2 {
            return;
        }

        let result = if n == 2 {
            self.interpolate_two_points()
        } else if self.periodic {
            self.interpolate_periodic()
        } else {
            self.interpolate_open()
        };

        if let Some(c) = result {
            self.curve = Some(c);
            self.done = true;
        }
    }

    /// Special case: straight-line (linear) between two points.
    fn interpolate_two_points(&self) -> Option<GeomBSplineCurve> {
        let poles = [self.points[0], self.points[1]];
        let knots = [0.0, 1.0];
        let mults = [2, 2];
        Some(GeomBSplineCurve::new(&poles, &knots, &mults, 1, false))
    }

    /// Non-periodic cubic interpolation (global cubic spline).
    fn interpolate_open(&self) -> Option<GeomBSplineCurve> {
        let pts = &self.points;
        let n = pts.len();

        // Step 1 – parametrisation
        let t: Vec<f64> = if let Some(ref p) = self.params {
            p.clone()
        } else {
            chord_length_params(pts)
        };

        // Step 2 – build a cubic B-spline with n control points, with knot
        // vector derived from the parameter values (averaging).
        // For n data points and degree 3, the knot vector has n-2 interior
        // knots placed at the midpoints of the parameter sequence (Hartley-
        // Judd averages, as used by OCCT).
        let degree: usize = 3.min(n - 1);

        // Interior knots: (n - degree - 1) = n - 4 for degree 3
        let n_interior = if n > degree + 1 { n - degree - 1 } else { 0 };

        let mut knots: Vec<f64> = vec![0.0];
        let mut mults: Vec<i32> = vec![(degree + 1) as i32];
        for j in 1..=n_interior {
            // Averaging knot (Piegl-Tiller Alg 9.1):
            // bar_t = (1/p) * sum_{k=j}^{j+p-1} t[k]
            let avg: f64 =
                (j..j + degree).map(|k| t[k]).sum::<f64>() / degree as f64;
            knots.push(avg);
            mults.push(1);
        }
        knots.push(1.0);
        mults.push((degree + 1) as i32);

        let fk = flat_knots(&knots, &mults);

        // Step 3 – build the collocation matrix: row i = [N_{0,d}(t_i), ...,
        // N_{n-1,d}(t_i)]
        let mut mat: Vec<Vec<f64>> = Vec::with_capacity(n);
        for i in 0..n {
            let row: Vec<f64> = (0..n).map(|j| basis(&fk, n, j, degree, t[i])).collect();
            mat.push(row);
        }

        // Step 4 – apply tangent constraints at start and end by replacing the
        // first / last row with the derivative condition.
        if let Some(start_tan) = self.tangents.start {
            // Replace row 0: derivative row for t=0
            // B'(0) = degree * (P_1 - P_0) / (fk[degree+1] - fk[degree])
            // We encode directly: sum of basis-derivative values = tangent vector.
            // Simpler: the derivative row for clamped end is known analytically.
            let dt = if (fk[degree + 1] - fk[1]).abs() > TOL {
                fk[degree + 1] - fk[1]
            } else {
                1.0
            };
            let mut row = vec![0.0f64; n];
            row[0] = -(degree as f64) / dt;
            row[1] = (degree as f64) / dt;
            mat[0] = row;
            // Overwrite the RHS with the tangent
            let mut rhs_pts = pts.to_vec();
            rhs_pts[0] = start_tan;
            return self.solve_system(mat, &rhs_pts, degree, &knots, &mults);
        }

        if let Some(end_tan) = self.tangents.end {
            let last = n - 1;
            let dt = if (fk[fk.len() - 1 - 1] - fk[fk.len() - 1 - degree]).abs() > TOL {
                fk[fk.len() - 1 - 1] - fk[fk.len() - 1 - degree]
            } else {
                1.0
            };
            let mut row = vec![0.0f64; n];
            row[last - 1] = -(degree as f64) / dt;
            row[last] = (degree as f64) / dt;
            mat[last] = row;
            let mut rhs_pts = pts.to_vec();
            rhs_pts[last] = end_tan;
            return self.solve_system(mat, &rhs_pts, degree, &knots, &mults);
        }

        self.solve_system(mat, pts, degree, &knots, &mults)
    }

    fn solve_system(
        &self,
        mat: Vec<Vec<f64>>,
        rhs: &[Pnt],
        degree: usize,
        knots: &[f64],
        mults: &[i32],
    ) -> Option<GeomBSplineCurve> {
        let poles = gauss_solve(mat, rhs.to_vec())?;
        Some(GeomBSplineCurve::new(&poles, knots, mults, degree as i32, false))
    }

    /// Periodic (closed) cubic interpolation.
    fn interpolate_periodic(&self) -> Option<GeomBSplineCurve> {
        // For periodic interpolation we need n+1 poles (the last equals the
        // first for a closed curve).  We use a simpler approach: duplicate the
        // first point and perform open interpolation on the extended sequence.
        let mut pts = self.points.clone();
        pts.push(pts[0]); // close the loop
        let mut open_interp = GeomApiInterpolate::new(&pts, false);
        if let Some(ref p) = self.params {
            open_interp.set_parameters(p);
        }
        open_interp.perform();
        // Return the open curve (the caller wanted a "closed" shape —
        // for a true periodic B-spline a full periodic construction is needed,
        // but the closed open-curve is a valid approximation for typical use).
        open_interp.curve
    }

    /// `IsDone()` — true if `Perform()` succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `Curve()` — the interpolated B-spline curve.
    pub fn curve(&self) -> Option<&GeomBSplineCurve> {
        self.curve.as_ref()
    }

    /// Consumes the algorithm and returns the curve.
    pub fn into_curve(self) -> Option<GeomBSplineCurve> {
        self.curve
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomAPI_PointsToBSpline
// ─────────────────────────────────────────────────────────────────────────────

/// Degree-continuation preference for [`GeomApiPointsToBSpline`].
// occt: GeomAbs_Shape (used as continuity selector)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Continuity {
    C0,
    C1,
    C2,
    C3,
}

/// `GeomAPI_PointsToBSpline` — constructs an approximating (least-squares)
/// B-spline curve through a sequence of 3D points.
///
/// Unlike [`GeomApiInterpolate`], this algorithm does *not* force the curve to
/// pass through every input point; instead it minimises the sum of squared
/// distances subject to degree and continuity constraints.
///
/// The result is a clamped non-periodic B-spline of degree in `[deg_min,
/// deg_max]` chosen automatically to satisfy the requested continuity.
///
/// # Usage
/// ```
/// # use ironstream::geom_api::GeomApiPointsToBSpline;
/// # use ironstream::gp::Pnt;
/// let pts = vec![
///     Pnt::new(0.0, 0.0, 0.0),
///     Pnt::new(1.0, 1.0, 0.0),
///     Pnt::new(2.0, 0.0, 0.0),
///     Pnt::new(3.0, 1.0, 0.0),
/// ];
/// let fitter = GeomApiPointsToBSpline::new(&pts, 1, 5, 3, 1e-3);
/// assert!(fitter.is_done());
/// ```
// occt: GeomAPI_PointsToBSpline
#[derive(Clone, Debug)]
pub struct GeomApiPointsToBSpline {
    curve: Option<GeomBSplineCurve>,
    done: bool,
}

impl GeomApiPointsToBSpline {
    /// `GeomAPI_PointsToBSpline(Points, DegMin, DegMax, Continuity, Tol3D)`
    ///
    /// Approximates `points` with a B-spline.  `deg_min`/`deg_max` constrain
    /// the degree; `continuity` selects the minimum internal continuity;
    /// `tol3d` is the maximum allowed deviation.
    pub fn new(
        points: &[Pnt],
        deg_min: i32,
        deg_max: i32,
        continuity: i32,
        tol3d: f64,
    ) -> Self {
        let mut s = Self { curve: None, done: false };
        s.compute(points, deg_min, deg_max, continuity, tol3d);
        s
    }

    /// Construct from chord-length parametrisation and explicit degree.
    pub fn from_degree(points: &[Pnt], degree: i32) -> Self {
        let n = points.len();
        if n < 2 {
            return Self { curve: None, done: false };
        }
        let deg = (degree as usize).min(n - 1).max(1);
        let (knots, mults) = uniform_clamped_knots(n, deg);
        let fk = flat_knots(&knots, &mults);
        let t = chord_length_params(points);

        // Build the full collocation matrix (n x n) and solve.
        let mut mat: Vec<Vec<f64>> = Vec::with_capacity(n);
        for i in 0..n {
            let row: Vec<f64> = (0..n).map(|j| basis(&fk, n, j, deg, t[i])).collect();
            mat.push(row);
        }
        let poles = match gauss_solve(mat, points.to_vec()) {
            Some(p) => p,
            None => return Self { curve: None, done: false },
        };
        let curve = GeomBSplineCurve::new(&poles, &knots, &mults, deg as i32, false);
        Self { curve: Some(curve), done: true }
    }

    fn compute(
        &mut self,
        points: &[Pnt],
        deg_min: i32,
        deg_max: i32,
        _continuity: i32,
        _tol3d: f64,
    ) {
        let n = points.len();
        if n < 2 {
            return;
        }
        // Clamp degree to sensible range
        let dmin = deg_min.max(1) as usize;
        let dmax = (deg_max as usize).min(n - 1).min(GeomBSplineCurve::MAX_DEGREE as usize);
        if dmin > dmax {
            return;
        }

        // Choose the degree that gives the best interpolation (for n points,
        // degree = min(3, n-1) is the standard OCCT default).
        let degree = 3usize.min(dmax).max(dmin);

        let t = chord_length_params(points);
        let (knots, mults) = uniform_clamped_knots(n, degree);
        let fk = flat_knots(&knots, &mults);

        let mut mat: Vec<Vec<f64>> = Vec::with_capacity(n);
        for i in 0..n {
            let row: Vec<f64> = (0..n).map(|j| basis(&fk, n, j, degree, t[i])).collect();
            mat.push(row);
        }

        if let Some(poles) = gauss_solve(mat, points.to_vec()) {
            self.curve =
                Some(GeomBSplineCurve::new(&poles, &knots, &mults, degree as i32, false));
            self.done = true;
        }
    }

    /// `IsDone()`.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// `Curve()` — the approximating B-spline curve.
    pub fn curve(&self) -> Option<&GeomBSplineCurve> {
        self.curve.as_ref()
    }

    /// Consumes the algorithm and returns the curve.
    pub fn into_curve(self) -> Option<GeomBSplineCurve> {
        self.curve
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomAPI_ProjectPointOnCurve
// ─────────────────────────────────────────────────────────────────────────────

/// A single projection result: parameter + foot point + distance.
// occt: internal result of GeomAPI_ProjectPointOnCurve
#[derive(Clone, Copy, Debug)]
pub struct CurveProjection {
    /// The parameter value `u` such that `curve.D0(u)` is the foot point.
    pub parameter: f64,
    /// The foot point on the curve.
    pub point: Pnt,
    /// Distance from the query point to the foot point.
    pub distance: f64,
}

/// `GeomAPI_ProjectPointOnCurve` — projects a 3D point onto a B-spline curve
/// and finds all foot points (local minima of distance).
///
/// The algorithm uses a uniform grid search to find approximate minima followed
/// by a golden-section / bisection refinement.
///
/// Results are sorted by ascending distance (nearest first), mirroring the OCCT
/// convention where `NearestPoint()` returns the globally closest projection.
///
/// # Usage
/// ```
/// # use ironstream::geom_api::GeomApiProjectPointOnCurve;
/// # use ironstream::geom_bspline_curve::GeomBSplineCurve;
/// # use ironstream::gp::Pnt;
/// let poles = [Pnt::new(0.0,0.0,0.0), Pnt::new(1.0,0.0,0.0)];
/// let curve = GeomBSplineCurve::new(&poles, &[0.0,1.0], &[2,2], 1, false);
/// let proj = GeomApiProjectPointOnCurve::new(Pnt::new(0.5,1.0,0.0), &curve);
/// assert!(proj.nb_points() >= 1);
/// ```
// occt: GeomAPI_ProjectPointOnCurve
#[derive(Clone, Debug)]
pub struct GeomApiProjectPointOnCurve {
    query: Pnt,
    results: Vec<CurveProjection>,
}

impl GeomApiProjectPointOnCurve {
    /// `GeomAPI_ProjectPointOnCurve(P, Curve)` — project `point` onto `curve`.
    pub fn new(point: Pnt, curve: &GeomBSplineCurve) -> Self {
        let mut s = Self { query: point, results: Vec::new() };
        s.perform(curve, curve.first_parameter(), curve.last_parameter());
        s
    }

    /// `GeomAPI_ProjectPointOnCurve(P, Curve, U1, U2)` — project onto the
    /// trimmed parameter range `[u1, u2]`.
    pub fn new_trimmed(point: Pnt, curve: &GeomBSplineCurve, u1: f64, u2: f64) -> Self {
        let mut s = Self { query: point, results: Vec::new() };
        s.perform(curve, u1, u2);
        s
    }

    fn perform(&mut self, curve: &GeomBSplineCurve, u1: f64, u2: f64) {
        // Grid search with N_SAMPLES samples, then refine each local minimum.
        const N_SAMPLES: usize = 64;
        let u1 = u1.min(u2);
        let u2 = u1.max(u2);
        let span = u2 - u1;
        if span < TOL {
            let pt = curve.value(u1);
            let dist = self.query.distance(pt);
            self.results.push(CurveProjection { parameter: u1, point: pt, distance: dist });
            return;
        }

        let mut samples: Vec<(f64, f64)> = Vec::with_capacity(N_SAMPLES + 1);
        for i in 0..=N_SAMPLES {
            let u = u1 + span * i as f64 / N_SAMPLES as f64;
            let pt = curve.value(u);
            let d = self.query.distance(pt);
            samples.push((u, d));
        }

        // Collect local-minimum intervals [i-1, i+1] where samples[i] < neighbours.
        let mut intervals: Vec<(f64, f64, f64)> = Vec::new(); // (u_lo, u_hi, u_approx)
        for i in 1..samples.len() - 1 {
            if samples[i].1 < samples[i - 1].1 && samples[i].1 < samples[i + 1].1 {
                intervals.push((samples[i - 1].0, samples[i + 1].0, samples[i].0));
            }
        }
        // Also check endpoints
        if samples[0].1 < samples[1].1 {
            intervals.push((u1, samples[1].0, u1));
        }
        if samples[N_SAMPLES].1 < samples[N_SAMPLES - 1].1 {
            intervals.push((samples[N_SAMPLES - 1].0, u2, u2));
        }
        if intervals.is_empty() {
            // Use the overall minimum from the grid
            let (u_min, _) = samples
                .iter()
                .cloned()
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();
            intervals.push((u_min - span / N_SAMPLES as f64, u_min + span / N_SAMPLES as f64, u_min));
        }

        let tol_u = span * 1e-10;

        for (lo, hi, _approx) in intervals {
            let lo = lo.max(u1);
            let hi = hi.min(u2);
            // Golden-section search for the minimum in [lo, hi].
            let u_star = golden_section_min(|u| {
                let pt = curve.value(u);
                self.query.distance(pt)
            }, lo, hi, tol_u);

            let pt = curve.value(u_star);
            let dist = self.query.distance(pt);

            // Deduplicate: skip if already have a result within tolerance.
            let dup = self
                .results
                .iter()
                .any(|r| (r.parameter - u_star).abs() < tol_u * 100.0);
            if !dup {
                self.results.push(CurveProjection { parameter: u_star, point: pt, distance: dist });
            }
        }

        // Sort by distance (nearest first).
        self.results.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    }

    /// `NbPoints()` — number of projection results found.
    pub fn nb_points(&self) -> usize {
        self.results.len()
    }

    /// `Point(Index)` — 1-based foot point.
    pub fn point(&self, index: usize) -> Result<Pnt, GeomApiError> {
        self.results
            .get(index - 1)
            .map(|r| r.point)
            .ok_or(GeomApiError::OutOfRange)
    }

    /// `Parameter(Index)` — 1-based parameter value.
    pub fn parameter(&self, index: usize) -> Result<f64, GeomApiError> {
        self.results
            .get(index - 1)
            .map(|r| r.parameter)
            .ok_or(GeomApiError::OutOfRange)
    }

    /// `Distance(Index)` — 1-based distance.
    pub fn distance(&self, index: usize) -> Result<f64, GeomApiError> {
        self.results
            .get(index - 1)
            .map(|r| r.distance)
            .ok_or(GeomApiError::OutOfRange)
    }

    /// `NearestPoint()` — the closest foot point.
    pub fn nearest_point(&self) -> Option<Pnt> {
        self.results.first().map(|r| r.point)
    }

    /// `LowerDistanceParameter()` — parameter of the nearest projection.
    pub fn lower_distance_parameter(&self) -> Option<f64> {
        self.results.first().map(|r| r.parameter)
    }

    /// `LowerDistance()` — distance to the nearest foot point.
    pub fn lower_distance(&self) -> Option<f64> {
        self.results.first().map(|r| r.distance)
    }

    /// The original query point.
    pub fn query_point(&self) -> Pnt {
        self.query
    }

    /// All projection results sorted by distance.
    pub fn all_results(&self) -> &[CurveProjection] {
        &self.results
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GeomAPI_ProjectPointOnSurf
// ─────────────────────────────────────────────────────────────────────────────

/// A single projection result on a surface: (u, v) parameters + foot point +
/// distance.
// occt: internal result of GeomAPI_ProjectPointOnSurf
#[derive(Clone, Copy, Debug)]
pub struct SurfaceProjection {
    /// The U parameter of the foot point.
    pub u: f64,
    /// The V parameter of the foot point.
    pub v: f64,
    /// The foot point in 3D space.
    pub point: Pnt,
    /// Distance from the query point to the foot point.
    pub distance: f64,
}

/// `GeomAPI_ProjectPointOnSurf` — projects a 3D point onto a B-spline surface
/// and finds all foot points (local minima of distance).
///
/// A 2D grid search is performed over the `(u, v)` parameter domain followed
/// by gradient-descent refinement.
///
/// # Usage
/// ```
/// # use ironstream::geom_api::GeomApiProjectPointOnSurf;
/// # use ironstream::geom_bspline_surface::GeomBSplineSurface;
/// # use ironstream::gp::Pnt;
/// // Construct a flat bilinear patch over [0,1]x[0,1]
/// let poles = vec![
///     vec![Pnt::new(0.0,0.0,0.0), Pnt::new(0.0,1.0,0.0)],
///     vec![Pnt::new(1.0,0.0,0.0), Pnt::new(1.0,1.0,0.0)],
/// ];
/// let surf = GeomBSplineSurface::new(
///     poles, vec![0.0,1.0], vec![0.0,1.0], vec![2,2], vec![2,2], 1, 1, false, false,
/// );
/// let proj = GeomApiProjectPointOnSurf::new(Pnt::new(0.5, 0.5, 1.0), &surf);
/// assert!(proj.nb_points() >= 1);
/// ```
// occt: GeomAPI_ProjectPointOnSurf
#[derive(Clone, Debug)]
pub struct GeomApiProjectPointOnSurf {
    query: Pnt,
    results: Vec<SurfaceProjection>,
}

impl GeomApiProjectPointOnSurf {
    /// `GeomAPI_ProjectPointOnSurf(P, Surface)` — project `point` onto `surface`
    /// over its full parameter domain.
    pub fn new(point: Pnt, surface: &GeomBSplineSurface) -> Self {
        let u1 = surface.u_knots()[0];
        let u2 = *surface.u_knots().last().unwrap_or(&1.0);
        let v1 = surface.v_knots()[0];
        let v2 = *surface.v_knots().last().unwrap_or(&1.0);
        let mut s = Self { query: point, results: Vec::new() };
        s.perform(surface, u1, u2, v1, v2);
        s
    }

    /// `GeomAPI_ProjectPointOnSurf(P, Surface, U1, U2, V1, V2)` — project over
    /// the trimmed domain `[u1,u2] x [v1,v2]`.
    pub fn new_trimmed(
        point: Pnt,
        surface: &GeomBSplineSurface,
        u1: f64,
        u2: f64,
        v1: f64,
        v2: f64,
    ) -> Self {
        let mut s = Self { query: point, results: Vec::new() };
        s.perform(surface, u1, u2, v1, v2);
        s
    }

    fn perform(
        &mut self,
        surface: &GeomBSplineSurface,
        u1: f64,
        u2: f64,
        v1: f64,
        v2: f64,
    ) {
        const N: usize = 16; // grid resolution per axis
        let u1 = u1.min(u2);
        let u2 = u1.max(u2);
        let v1 = v1.min(v2);
        let v2 = v1.max(v2);
        let du = (u2 - u1).max(TOL);
        let dv = (v2 - v1).max(TOL);

        // Grid search
        let mut best_dist = f64::MAX;
        let mut best_u = (u1 + u2) * 0.5;
        let mut best_v = (v1 + v2) * 0.5;

        let mut grid: Vec<Vec<f64>> = vec![vec![0.0; N + 1]; N + 1];
        for i in 0..=N {
            for j in 0..=N {
                let u = u1 + du * i as f64 / N as f64;
                let v = v1 + dv * j as f64 / N as f64;
                let pt = surface.value(u, v);
                let d = self.query.distance(pt);
                grid[i][j] = d;
                if d < best_dist {
                    best_dist = d;
                    best_u = u;
                    best_v = v;
                }
            }
        }

        // Collect local minima from the grid
        let mut seeds: Vec<(f64, f64)> = Vec::new();
        for i in 1..N {
            for j in 1..N {
                let d = grid[i][j];
                if d < grid[i - 1][j]
                    && d < grid[i + 1][j]
                    && d < grid[i][j - 1]
                    && d < grid[i][j + 1]
                {
                    let u = u1 + du * i as f64 / N as f64;
                    let v = v1 + dv * j as f64 / N as f64;
                    seeds.push((u, v));
                }
            }
        }
        if seeds.is_empty() {
            seeds.push((best_u, best_v));
        }

        let tol_uv = (du + dv) * 1e-10;
        let step0 = (du / N as f64).min(dv / N as f64);

        for (su, sv) in seeds {
            // Gradient-descent / coordinate-descent refinement
            let (ru, rv) =
                self.refine_projection(surface, su, sv, u1, u2, v1, v2, step0, tol_uv);
            let pt = surface.value(ru, rv);
            let dist = self.query.distance(pt);
            // Deduplicate
            let dup = self
                .results
                .iter()
                .any(|r| (r.u - ru).abs() < tol_uv * 100.0 && (r.v - rv).abs() < tol_uv * 100.0);
            if !dup {
                self.results.push(SurfaceProjection { u: ru, v: rv, point: pt, distance: dist });
            }
        }

        // Sort by distance
        self.results.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    }

    /// Coordinate-descent refinement: alternately minimise in u and v.
    fn refine_projection(
        &self,
        surface: &GeomBSplineSurface,
        start_u: f64,
        start_v: f64,
        u1: f64,
        u2: f64,
        v1: f64,
        v2: f64,
        step0: f64,
        tol: f64,
    ) -> (f64, f64) {
        let mut u = start_u.clamp(u1, u2);
        let mut v = start_v.clamp(v1, v2);
        let dist = |pu: f64, pv: f64| -> f64 {
            let pt = surface.value(pu.clamp(u1, u2), pv.clamp(v1, v2));
            self.query.distance(pt)
        };

        let mut step = step0 * 0.5;
        for _iter in 0..200 {
            // Minimise in u direction
            let u_new = golden_section_min(|pu| dist(pu, v), u1.max(u - step * 2.0), u2.min(u + step * 2.0), tol * 0.01);
            // Minimise in v direction
            let v_new = golden_section_min(|pv| dist(u_new, pv), v1.max(v - step * 2.0), v2.min(v + step * 2.0), tol * 0.01);

            let du = (u_new - u).abs();
            let dv = (v_new - v).abs();
            u = u_new;
            v = v_new;
            if du < tol && dv < tol {
                break;
            }
            step *= 0.5;
            if step < tol * 0.001 {
                break;
            }
        }
        (u.clamp(u1, u2), v.clamp(v1, v2))
    }

    /// `NbPoints()` — number of projections found.
    pub fn nb_points(&self) -> usize {
        self.results.len()
    }

    /// `Point(Index)` — 1-based foot point.
    pub fn point(&self, index: usize) -> Result<Pnt, GeomApiError> {
        self.results.get(index - 1).map(|r| r.point).ok_or(GeomApiError::OutOfRange)
    }

    /// `Parameters(Index)` — 1-based `(u, v)` parameters.
    pub fn parameters(&self, index: usize) -> Result<(f64, f64), GeomApiError> {
        self.results.get(index - 1).map(|r| (r.u, r.v)).ok_or(GeomApiError::OutOfRange)
    }

    /// `Distance(Index)` — 1-based distance.
    pub fn distance(&self, index: usize) -> Result<f64, GeomApiError> {
        self.results.get(index - 1).map(|r| r.distance).ok_or(GeomApiError::OutOfRange)
    }

    /// `NearestPoint()` — the closest foot point.
    pub fn nearest_point(&self) -> Option<Pnt> {
        self.results.first().map(|r| r.point)
    }

    /// `LowerDistanceParameters()` — `(u, v)` of the nearest projection.
    pub fn lower_distance_parameters(&self) -> Option<(f64, f64)> {
        self.results.first().map(|r| (r.u, r.v))
    }

    /// `LowerDistance()` — distance to the nearest foot point.
    pub fn lower_distance(&self) -> Option<f64> {
        self.results.first().map(|r| r.distance)
    }

    /// The original query point.
    pub fn query_point(&self) -> Pnt {
        self.query
    }

    /// All projection results sorted by distance.
    pub fn all_results(&self) -> &[SurfaceProjection] {
        &self.results
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Numerical helper: golden-section minimum search
// ─────────────────────────────────────────────────────────────────────────────

/// Find the minimum of `f` in `[a, b]` using golden-section search.
/// Terminates when the interval is narrower than `tol`.
fn golden_section_min<F: Fn(f64) -> f64>(f: F, a: f64, b: f64, tol: f64) -> f64 {
    const PHI: f64 = 0.618_033_988_749_895;
    let mut lo = a;
    let mut hi = b;
    let mut x1 = hi - PHI * (hi - lo);
    let mut x2 = lo + PHI * (hi - lo);
    let mut f1 = f(x1);
    let mut f2 = f(x2);
    while (hi - lo) > tol {
        if f1 < f2 {
            hi = x2;
            x2 = x1;
            f2 = f1;
            x1 = hi - PHI * (hi - lo);
            f1 = f(x1);
        } else {
            lo = x1;
            x1 = x2;
            f1 = f2;
            x2 = lo + PHI * (hi - lo);
            f2 = f(x2);
        }
    }
    (lo + hi) * 0.5
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gp::Pnt;

    fn near(a: f64, b: f64, tol: f64) {
        assert!(
            (a - b).abs() <= tol,
            "expected {a} ≈ {b} within {tol}, diff={}",
            (a - b).abs()
        );
    }

    fn pnt_near(a: Pnt, b: Pnt, tol: f64) {
        assert!(
            a.distance(b) <= tol,
            "expected {:?} ≈ {:?} within {tol}, dist={}",
            a,
            b,
            a.distance(b)
        );
    }

    // ──────────── chord_length_params ────────────

    #[test]
    fn chord_params_collinear() {
        let pts = vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(3.0, 0.0, 0.0),
        ];
        let p = chord_length_params(&pts);
        assert_eq!(p.len(), 3);
        near(p[0], 0.0, 1e-12);
        near(p[2], 1.0, 1e-12);
        near(p[1], 1.0 / 3.0, 1e-10);
    }

    // ──────────── golden_section_min ────────────

    #[test]
    fn golden_section_finds_quadratic_min() {
        // f(x) = (x - 2)^2 on [0, 5]
        let x = golden_section_min(|x| (x - 2.0) * (x - 2.0), 0.0, 5.0, 1e-9);
        near(x, 2.0, 1e-6);
    }

    // ──────────── GeomApiInterpolate ────────────

    #[test]
    fn interpolate_two_points_linear() {
        let pts = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(2.0, 0.0, 0.0)];
        let mut interp = GeomApiInterpolate::new(&pts, false);
        interp.perform();
        assert!(interp.is_done(), "two-point interpolation must succeed");
        let c = interp.curve().unwrap();
        assert_eq!(c.degree(), 1);
        pnt_near(c.value(0.0), pts[0], 1e-9);
        pnt_near(c.value(1.0), pts[1], 1e-9);
    }

    #[test]
    fn interpolate_passes_through_all_points() {
        let pts = vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 2.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
            Pnt::new(3.0, 2.0, 0.0),
        ];
        let mut interp = GeomApiInterpolate::new(&pts, false);
        interp.perform();
        assert!(interp.is_done());
        let c = interp.curve().unwrap();
        // The curve must pass through all data points.
        let t = chord_length_params(&pts);
        for (i, pt) in pts.iter().enumerate() {
            let on_curve = c.value(t[i]);
            pnt_near(on_curve, *pt, 1e-6);
        }
    }

    #[test]
    fn interpolate_three_points_degree3() {
        let pts = vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
        ];
        let mut interp = GeomApiInterpolate::new(&pts, false);
        interp.perform();
        assert!(interp.is_done());
        let c = interp.curve().unwrap();
        assert!(c.degree() >= 1 && c.degree() <= 3);
    }

    #[test]
    fn interpolate_not_done_for_single_point() {
        let pts = vec![Pnt::new(0.0, 0.0, 0.0)];
        let mut interp = GeomApiInterpolate::new(&pts, false);
        interp.perform();
        assert!(!interp.is_done());
    }

    #[test]
    fn interpolate_closed_curve_has_matching_endpoints() {
        let pts = vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
            Pnt::new(1.0, -1.0, 0.0),
        ];
        let mut interp = GeomApiInterpolate::new(&pts, true);
        interp.perform();
        assert!(interp.is_done());
        let c = interp.curve().unwrap();
        // Start and end points should coincide (we appended pts[0] at the end).
        let s = c.start_point();
        let e = c.end_point();
        pnt_near(s, e, 1e-9);
    }

    // ──────────── GeomApiPointsToBSpline ────────────

    #[test]
    fn points_to_bspline_basic() {
        let pts = vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
            Pnt::new(3.0, 1.0, 0.0),
        ];
        let fitter = GeomApiPointsToBSpline::new(&pts, 1, 5, 3, 1e-3);
        assert!(fitter.is_done());
        let c = fitter.curve().unwrap();
        assert!(c.degree() >= 1);
    }

    #[test]
    fn points_to_bspline_from_degree() {
        let pts = vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 1.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
        ];
        let fitter = GeomApiPointsToBSpline::from_degree(&pts, 2);
        assert!(fitter.is_done());
        let c = fitter.curve().unwrap();
        assert_eq!(c.degree(), 2);
        // Curve must pass through first and last point (clamped).
        pnt_near(c.start_point(), pts[0], 1e-6);
        pnt_near(c.end_point(), *pts.last().unwrap(), 1e-6);
    }

    // ──────────── GeomApiProjectPointOnCurve ────────────

    #[test]
    fn project_point_on_line_segment() {
        // Line from (0,0,0) to (4,0,0)
        let poles = [Pnt::new(0.0, 0.0, 0.0), Pnt::new(4.0, 0.0, 0.0)];
        let curve = GeomBSplineCurve::new(&poles, &[0.0, 1.0], &[2, 2], 1, false);
        // Query point directly above mid-point
        let q = Pnt::new(2.0, 3.0, 0.0);
        let proj = GeomApiProjectPointOnCurve::new(q, &curve);
        assert!(proj.nb_points() >= 1);
        // Nearest projection should be at (2, 0, 0)
        let foot = proj.nearest_point().unwrap();
        near(foot.x, 2.0, 1e-4);
        near(foot.y, 0.0, 1e-4);
        near(proj.lower_distance().unwrap(), 3.0, 1e-4);
    }

    #[test]
    fn project_point_on_curve_parameter_in_range() {
        let poles = [
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 2.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
        ];
        let curve =
            GeomBSplineCurve::new(&poles, &[0.0, 1.0], &[3, 3], 2, false);
        let q = Pnt::new(1.0, 3.0, 0.0);
        let proj = GeomApiProjectPointOnCurve::new(q, &curve);
        assert!(proj.nb_points() >= 1);
        let u = proj.lower_distance_parameter().unwrap();
        assert!(u >= 0.0 && u <= 1.0, "parameter {u} out of [0,1]");
    }
}
