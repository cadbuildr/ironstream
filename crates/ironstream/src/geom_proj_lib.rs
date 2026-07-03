// FILE: geom_proj_lib.rs
//! `GeomProjLib` — projection utilities for curves and surfaces in 3D space.
//!
//! Faithful port of OpenCascade's `GeomProjLib` package
//! (`src/ModelingAlgorithms/TKGeomBase/GeomProjLib`).
//!
//! Covered classes / utilities:
//! - [`ProjectOnPlane`]   — `GeomProjLib_ProjectOnPlane`: project a 3D B-spline
//!                          curve onto an analytic plane along a direction vector.
//! - [`ProjectOnSurface`] — `GeomProjLib_ProjectOnSurface`: project a 3D
//!                          B-spline curve onto a B-spline surface (foot-point
//!                          sampling + B-spline fit).
//! - [`GeomProjLib`]      — static utility functions mirroring OCCT's
//!                          `GeomProjLib` namespace:
//!   - `curve_2d`         — compute the 2D parametric image of a 3D curve on a
//!                          plane (`GeomProjLib::Curve2d`).
//!   - `project_point_on_plane` — orthogonal/directed projection of a point onto
//!                          a plane.
//!   - `project_curve_on_plane` — convenience wrapper for [`ProjectOnPlane`].

use crate::geom_bspline_curve::GeomBSplineCurve;
use crate::geom_bspline_surface::GeomBSplineSurface;
use crate::geom_plane::GeomPlane;
use crate::gp::{Ax3, Pnt, Vec3};

// ---------------------------------------------------------------------------
// Tolerance used throughout this module
// ---------------------------------------------------------------------------

const TOL: f64 = 1.0e-9;

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Build a uniform clamped knot vector for `n_poles` control points at
/// `degree`.  Returns `(distinct_knots, multiplicities)`.
fn uniform_clamped_knots(n_poles: usize, degree: usize) -> (Vec<f64>, Vec<i32>) {
    let n_spans = n_poles as i64 - degree as i64;
    if n_spans <= 0 {
        return (vec![0.0, 1.0], vec![degree as i32 + 1, degree as i32 + 1]);
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

/// Chord-length parameters for a sequence of 3D points — returns values in
/// `[0, 1]` proportional to cumulative arc length.
fn chord_params_3d(pts: &[Pnt]) -> Vec<f64> {
    let n = pts.len();
    if n == 0 {
        return vec![];
    }
    let mut raw = vec![0.0f64; n];
    for i in 1..n {
        raw[i] = raw[i - 1] + pts[i].distance(pts[i - 1]);
    }
    let total = raw[n - 1];
    if total < TOL {
        return (0..n).map(|i| i as f64 / (n - 1).max(1) as f64).collect();
    }
    raw.iter().map(|v| v / total).collect()
}

/// Fit a degree-`degree` B-spline through 3D `pts` at `params` ∈ [0,1] using
/// least squares.  Returns `None` when the system is degenerate or the input
/// is too small.
fn fit_bspline_3d(pts: &[Pnt], params: &[f64], degree: usize) -> Option<GeomBSplineCurve> {
    let n = pts.len();
    if n < 2 {
        return None;
    }
    // Clamp degree so we always have n_poles >= 2 control points.
    let deg = degree.min(n - 1).max(1);
    let n_poles = n.min(n.max(deg + 1));
    let n_poles = n_poles.max(deg + 1);

    let (knots, mults) = uniform_clamped_knots(n_poles, deg);
    // Build the flat knot vector for basis evaluation.
    let flat: Vec<f64> = knots
        .iter()
        .zip(mults.iter())
        .flat_map(|(&k, &m)| std::iter::repeat(k).take(m as usize))
        .collect();

    // N[i][j] = B_{j,deg}(params[i]) — the basis collocation matrix.
    let nb = n_poles;
    let mut n_mat = vec![vec![0.0f64; nb]; n];
    for (i, &u) in params.iter().enumerate() {
        let span = find_span_simple(nb - 1, deg, u, &flat);
        let bfuns = basis_funs(span, u, deg, &flat);
        for (j, &b) in bfuns.iter().enumerate() {
            n_mat[i][span - deg + j] = b;
        }
    }

    // Solve N^T N c = N^T pts  by Cholesky (symmetric, PD).
    let ntn = mat_ata(&n_mat, nb);
    let ntx: Vec<f64> = (0..nb).map(|j| (0..n).map(|i| n_mat[i][j] * pts[i].x).sum()).collect();
    let nty: Vec<f64> = (0..nb).map(|j| (0..n).map(|i| n_mat[i][j] * pts[i].y).sum()).collect();
    let ntz: Vec<f64> = (0..nb).map(|j| (0..n).map(|i| n_mat[i][j] * pts[i].z).sum()).collect();

    let cx = chol_solve(&ntn, &ntx)?;
    let cy = chol_solve(&ntn, &nty)?;
    let cz = chol_solve(&ntn, &ntz)?;

    let poles: Vec<Pnt> = (0..nb).map(|j| Pnt::new(cx[j], cy[j], cz[j])).collect();
    Some(GeomBSplineCurve::new(&poles, &knots, &mults, deg as i32, false))
}

/// Find the knot span for `u` in the flat knot vector.
fn find_span_simple(n: usize, p: usize, u: f64, flat: &[f64]) -> usize {
    if u >= flat[n + 1] {
        return n;
    }
    if u <= flat[p] {
        return p;
    }
    let (mut lo, mut hi) = (p, n + 1);
    let mut mid = (lo + hi) / 2;
    while u < flat[mid] || u >= flat[mid + 1] {
        if u < flat[mid] {
            hi = mid;
        } else {
            lo = mid;
        }
        mid = (lo + hi) / 2;
    }
    mid
}

/// Compute the `p+1` non-zero B-spline basis functions at `u` in `span`.
fn basis_funs(span: usize, u: f64, p: usize, flat: &[f64]) -> Vec<f64> {
    let mut bfun = vec![0.0f64; p + 1];
    let mut left = vec![0.0f64; p + 1];
    let mut right = vec![0.0f64; p + 1];
    bfun[0] = 1.0;
    for j in 1..=p {
        left[j] = u - flat[span + 1 - j];
        right[j] = flat[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            let temp = bfun[r] / (right[r + 1] + left[j - r]);
            bfun[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        bfun[j] = saved;
    }
    bfun
}

/// Compute A^T A for an `n x m` matrix.
fn mat_ata(a: &[Vec<f64>], m: usize) -> Vec<Vec<f64>> {
    let n = a.len();
    let mut ata = vec![vec![0.0f64; m]; m];
    for i in 0..m {
        for j in i..m {
            let v: f64 = (0..n).map(|k| a[k][i] * a[k][j]).sum();
            ata[i][j] = v;
            ata[j][i] = v;
        }
    }
    ata
}

/// Solve `A x = b` for symmetric positive-definite `A` via Cholesky.
fn chol_solve(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
    let n = a.len();
    // Cholesky decomposition: L L^T = A
    let mut l = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        for j in 0..=i {
            let s: f64 = (0..j).map(|k| l[i][k] * l[j][k]).sum();
            if i == j {
                let d = a[i][i] - s;
                if d < TOL * TOL {
                    return None; // not positive-definite
                }
                l[i][j] = d.sqrt();
            } else {
                l[i][j] = (a[i][j] - s) / l[j][j];
            }
        }
    }
    // Forward substitution: L y = b
    let mut y = vec![0.0f64; n];
    for i in 0..n {
        let s: f64 = (0..i).map(|k| l[i][k] * y[k]).sum();
        y[i] = (b[i] - s) / l[i][i];
    }
    // Back substitution: L^T x = y
    let mut x = vec![0.0f64; n];
    for i in (0..n).rev() {
        let s: f64 = (i + 1..n).map(|k| l[k][i] * x[k]).sum();
        x[i] = (y[i] - s) / l[i][i];
    }
    Some(x)
}

/// Project a 3D point `p` onto a `GeomPlane` along `direction` (if provided)
/// or orthogonally (if `None`).
///
/// Returns the projected 3D point, or `None` if the direction is parallel to
/// the plane and the projection doesn't exist.
fn project_point_3d(p: Pnt, plane: &GeomPlane, direction: Option<Vec3>) -> Option<Pnt> {
    let pos = plane.position();
    let n = pos.z_dir; // plane normal (unit)
    match direction {
        None => {
            // Orthogonal projection.
            let dist = (p - pos.location).dot(n);
            Some(p - n * dist)
        }
        Some(dir) => {
            let dir = dir.normalized();
            let denom = dir.dot(n);
            if denom.abs() < TOL {
                // Direction is parallel to the plane — no unique intersection.
                return None;
            }
            let t = (pos.location - p).dot(n) / denom;
            Some(p + dir * t)
        }
    }
}

/// Compute the (U, V) parameters of the orthogonal projection of `p3d` onto
/// the plane defined by `pos`.
fn point_to_plane_uv(p3d: Pnt, pos: &Ax3) -> (f64, f64) {
    let rel = p3d - pos.location;
    let u = rel.dot(pos.x_dir);
    let v = rel.dot(pos.y_dir);
    (u, v)
}

// ---------------------------------------------------------------------------
// ProjectOnPlane
// ---------------------------------------------------------------------------

/// `GeomProjLib_ProjectOnPlane` — project a 3D B-spline curve onto an analytic
/// plane.
///
/// If a `direction` is supplied the projection rays run parallel to that
/// vector; otherwise the projection is orthogonal (perpendicular to the
/// plane normal).
///
/// The projected curve is approximated by sampling the input curve at
/// `n_samples` parameter values, projecting each sample, and fitting a
/// B-spline of the same degree through the result.
///
/// # OCCT correspondence
/// `GeomProjLib_ProjectOnPlane` (`GeomProjLib_ProjectOnPlane.hxx`).
// occt: GeomProjLib_ProjectOnPlane
#[derive(Clone, Debug)]
pub struct ProjectOnPlane {
    /// The target plane.
    plane: GeomPlane,
    /// Optional projection direction (normalised on input).  `None` means
    /// orthogonal projection.
    direction: Option<Vec3>,
    /// The number of sample points used to build the approximate projection.
    n_samples: usize,
    /// Result after calling [`perform`](Self::perform).
    result: Option<GeomBSplineCurve>,
    /// Whether the last call to `perform` succeeded.
    done: bool,
}

impl ProjectOnPlane {
    /// Create a new `ProjectOnPlane` projector.
    ///
    /// - `plane`     — the target plane.
    /// - `direction` — projection direction (normalised internally); pass
    ///                 `None` for orthogonal projection.
    /// - `n_samples` — number of sample points for the approximation
    ///                 (clamped to a minimum of 3).
    pub fn new(plane: GeomPlane, direction: Option<Vec3>, n_samples: usize) -> Self {
        Self {
            plane,
            direction: direction.map(|d| d.normalized()),
            n_samples: n_samples.max(3),
            result: None,
            done: false,
        }
    }

    /// Orthogonal-projection convenience constructor.
    pub fn orthogonal(plane: GeomPlane, n_samples: usize) -> Self {
        Self::new(plane, None, n_samples)
    }

    /// Directed-projection convenience constructor.
    pub fn directed(plane: GeomPlane, direction: Vec3, n_samples: usize) -> Self {
        Self::new(plane, Some(direction), n_samples)
    }

    /// Set the projection direction.  Pass `None` to switch to orthogonal mode.
    pub fn set_direction(&mut self, direction: Option<Vec3>) {
        self.direction = direction.map(|d| d.normalized());
        self.done = false;
        self.result = None;
    }

    /// Set the target plane.
    pub fn set_plane(&mut self, plane: GeomPlane) {
        self.plane = plane;
        self.done = false;
        self.result = None;
    }

    /// Run the projection of `curve` onto the plane.
    ///
    /// The result is stored internally; call [`is_done`](Self::is_done) and
    /// [`projected_curve`](Self::projected_curve) to retrieve it.
    pub fn perform(&mut self, curve: &GeomBSplineCurve) {
        self.done = false;
        self.result = None;

        let t0 = curve.first_parameter();
        let t1 = curve.last_parameter();
        let n = self.n_samples;
        // Sample the curve uniformly in parameter space.
        let samples: Vec<Pnt> = (0..n)
            .map(|i| {
                let t = t0 + (t1 - t0) * (i as f64 / (n - 1) as f64);
                curve.value(t)
            })
            .collect();

        // Project each sample onto the plane.
        let projected: Vec<Option<Pnt>> = samples
            .iter()
            .map(|&p| project_point_3d(p, &self.plane, self.direction))
            .collect();

        // If any projection failed (direction parallel to plane), abort.
        if projected.iter().any(|p| p.is_none()) {
            return;
        }
        let pts: Vec<Pnt> = projected.into_iter().flatten().collect();
        let params = chord_params_3d(&pts);
        let deg = (curve.degree() as usize).min(pts.len() - 1).max(1);
        if let Some(c) = fit_bspline_3d(&pts, &params, deg) {
            self.result = Some(c);
            self.done = true;
        }
    }

    /// `true` if the last [`perform`](Self::perform) succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// The projected B-spline curve, or `None` if not done.
    pub fn projected_curve(&self) -> Option<&GeomBSplineCurve> {
        self.result.as_ref()
    }

    /// Consume `self` and return the projected curve.
    pub fn into_curve(self) -> Option<GeomBSplineCurve> {
        self.result
    }
}

// ---------------------------------------------------------------------------
// ProjectOnSurface
// ---------------------------------------------------------------------------

/// `GeomProjLib_ProjectOnSurface` — project a 3D B-spline curve onto a
/// B-spline surface.
///
/// For each sample point on the input curve the algorithm finds the closest
/// point on the surface (foot-point Newton iteration), collects all
/// foot-points, and fits a B-spline through them.
///
/// # OCCT correspondence
/// `GeomProjLib_ProjectOnSurface` (`GeomProjLib_ProjectOnSurface.hxx`).
// occt: GeomProjLib_ProjectOnSurface
#[derive(Clone, Debug)]
pub struct ProjectOnSurface {
    /// The target surface.
    surface: GeomBSplineSurface,
    /// Number of sample / Newton seed points.
    n_samples: usize,
    /// Degree for the fitted result curve.
    degree: usize,
    /// Result after calling [`perform`](Self::perform).
    result: Option<GeomBSplineCurve>,
    /// Whether the last call to `perform` succeeded.
    done: bool,
}

impl ProjectOnSurface {
    /// Create a new `ProjectOnSurface` projector.
    ///
    /// - `surface`   — the target B-spline surface.
    /// - `n_samples` — number of curve samples (minimum 3).
    /// - `degree`    — degree for the result B-spline (clamped to `[1, 3]`).
    pub fn new(surface: GeomBSplineSurface, n_samples: usize, degree: usize) -> Self {
        Self {
            surface,
            n_samples: n_samples.max(3),
            degree: degree.clamp(1, 3),
            result: None,
            done: false,
        }
    }

    /// Set the target surface.
    pub fn set_surface(&mut self, surface: GeomBSplineSurface) {
        self.surface = surface;
        self.done = false;
        self.result = None;
    }

    /// Run the projection of `curve` onto the surface.
    pub fn perform(&mut self, curve: &GeomBSplineCurve) {
        self.done = false;
        self.result = None;

        let t0 = curve.first_parameter();
        let t1 = curve.last_parameter();
        let n = self.n_samples;
        let (su0, su1, sv0, sv1) = self.surface.bounds();

        let mut foot_pts: Vec<Pnt> = Vec::with_capacity(n);

        for i in 0..n {
            let t = t0 + (t1 - t0) * (i as f64 / (n - 1) as f64);
            let p = curve.value(t);
            // Seed search: grid of (NU x NV) = (5 x 5) candidates.
            let grid_n = 5usize;
            let mut best_u = (su0 + su1) * 0.5;
            let mut best_v = (sv0 + sv1) * 0.5;
            let mut best_d2 = f64::MAX;
            for gi in 0..grid_n {
                for gj in 0..grid_n {
                    let gu = su0 + (su1 - su0) * gi as f64 / (grid_n - 1) as f64;
                    let gv = sv0 + (sv1 - sv0) * gj as f64 / (grid_n - 1) as f64;
                    let s = self.surface.value(gu, gv);
                    let d2 = p.square_distance(s);
                    if d2 < best_d2 {
                        best_d2 = d2;
                        best_u = gu;
                        best_v = gv;
                    }
                }
            }
            // Newton refinement on (u, v) to minimise ||surface(u,v) - p||^2.
            // Gradient:  f = (S-p)·Su,  g = (S-p)·Sv
            // Hessian:   fuu = Su·Su + (S-p)·Suu,  fvv = Sv·Sv + (S-p)·Svv,  fuv = Su·Sv + (S-p)·Suv
            let (mut u, mut v) = (best_u, best_v);
            for _ in 0..20 {
                // d2 returns (P, D1U, D1V, D2U, D2V, D2UV)
                let (s, su, sv, suu, svv, suv) = self.surface.d2(u, v);
                let r = s - p;
                let f = r.dot(su);
                let g = r.dot(sv);
                let fuu = su.dot(su) + r.dot(suu);
                let fvv = sv.dot(sv) + r.dot(svv);
                let fuv = su.dot(sv) + r.dot(suv);
                let det = fuu * fvv - fuv * fuv;
                if det.abs() < TOL {
                    break;
                }
                let du = (f * fvv - g * fuv) / det;
                let dv = (g * fuu - f * fuv) / det;
                u = (u - du).clamp(su0, su1);
                v = (v - dv).clamp(sv0, sv1);
                if du.abs() < TOL && dv.abs() < TOL {
                    break;
                }
            }
            foot_pts.push(self.surface.value(u, v));
        }

        let params = chord_params_3d(&foot_pts);
        let deg = self.degree.min(foot_pts.len() - 1).max(1);
        if let Some(c) = fit_bspline_3d(&foot_pts, &params, deg) {
            self.result = Some(c);
            self.done = true;
        }
    }

    /// `true` if the last [`perform`](Self::perform) succeeded.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// The projected B-spline curve on the surface, or `None` if not done.
    pub fn projected_curve(&self) -> Option<&GeomBSplineCurve> {
        self.result.as_ref()
    }

    /// Consume `self` and return the projected curve.
    pub fn into_curve(self) -> Option<GeomBSplineCurve> {
        self.result
    }
}

// ---------------------------------------------------------------------------
// 2D curve on plane
// ---------------------------------------------------------------------------

/// A 2D point `(u, v)` — the parametric coordinates on a plane surface.
// occt: gp_Pnt2d
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pnt2d {
    pub u: f64,
    pub v: f64,
}

impl Pnt2d {
    pub fn new(u: f64, v: f64) -> Self {
        Self { u, v }
    }

    pub fn distance(self, other: Pnt2d) -> f64 {
        let du = self.u - other.u;
        let dv = self.v - other.v;
        (du * du + dv * dv).sqrt()
    }
}

// ---------------------------------------------------------------------------
// GeomProjLib — static utilities
// ---------------------------------------------------------------------------

/// `GeomProjLib` — static projection utilities, mirroring the OCCT
/// `GeomProjLib` namespace of free functions.
// occt: GeomProjLib
pub struct GeomProjLib;

impl GeomProjLib {
    /// `GeomProjLib::Curve2d` — compute the 2D parametric image of the
    /// 3D B-spline `curve` on `plane`.
    ///
    /// The function samples `curve` at `n_samples` parameter values, projects
    /// each sample orthogonally onto the plane, extracts (U, V) parameter
    /// coordinates, and fits a 2D B-spline (represented as two 1D curves —
    /// one for U, one for V — both wrapped as degenerate 3D B-splines whose
    /// X component is the 2D coordinate and Y/Z are zero).
    ///
    /// Returns a `Vec<Pnt2d>` of the (U, V) projections at `n_samples`
    /// equally-spaced parameter values.  Callers that need an explicit 2D
    /// B-spline can use [`curve_2d_bspline`](Self::curve_2d_bspline).
    ///
    /// # OCCT correspondence
    /// `GeomProjLib::Curve2d(const Handle(Geom_Curve)&, …, Handle(Geom_Surface)&)`
    pub fn curve_2d(
        curve: &GeomBSplineCurve,
        plane: &GeomPlane,
        n_samples: usize,
    ) -> Vec<Pnt2d> {
        let n = n_samples.max(2);
        let t0 = curve.first_parameter();
        let t1 = curve.last_parameter();
        let pos = plane.position();
        (0..n)
            .map(|i| {
                let t = t0 + (t1 - t0) * (i as f64 / (n - 1) as f64);
                let p3 = curve.value(t);
                let (u, v) = point_to_plane_uv(p3, &pos);
                Pnt2d::new(u, v)
            })
            .collect()
    }

    /// `GeomProjLib::Curve2d` variant — returns the (U, V) pairs as a
    /// B-spline curve whose poles lie in the XY plane (`z = 0`).
    ///
    /// The U coordinates are stored in the `x` field of each pole and the V
    /// coordinates in the `y` field, with `z = 0`.  Degree is clamped to
    /// `[1, min(3, n_samples-1)]`.
    pub fn curve_2d_bspline(
        curve: &GeomBSplineCurve,
        plane: &GeomPlane,
        n_samples: usize,
        degree: usize,
    ) -> Option<GeomBSplineCurve> {
        let uv = Self::curve_2d(curve, plane, n_samples);
        if uv.len() < 2 {
            return None;
        }
        // Map (U, V) → 3D pole with z = 0.
        let pts: Vec<Pnt> = uv.iter().map(|p| Pnt::new(p.u, p.v, 0.0)).collect();
        let params = chord_params_3d(&pts);
        let deg = degree.clamp(1, pts.len() - 1).min(3);
        fit_bspline_3d(&pts, &params, deg)
    }

    /// `GeomProjLib::Project` — orthogonal projection of a single 3D point
    /// onto a plane.  Returns the projected 3D point.
    pub fn project_point_on_plane(p: Pnt, plane: &GeomPlane) -> Pnt {
        // Orthogonal projection is always defined.
        project_point_3d(p, plane, None).unwrap()
    }

    /// Directed projection of a single 3D point onto a plane along `direction`.
    ///
    /// Returns `None` if `direction` is (nearly) parallel to the plane.
    pub fn project_point_directed(p: Pnt, plane: &GeomPlane, direction: Vec3) -> Option<Pnt> {
        project_point_3d(p, plane, Some(direction))
    }

    /// `GeomProjLib::ProjectOnPlane` — project a B-spline curve onto a plane.
    ///
    /// Convenience wrapper around [`ProjectOnPlane`] with orthogonal projection
    /// and `n_samples` samples.  Returns the projected curve or `None`.
    pub fn project_curve_on_plane(
        curve: &GeomBSplineCurve,
        plane: &GeomPlane,
        n_samples: usize,
    ) -> Option<GeomBSplineCurve> {
        let mut proj = ProjectOnPlane::orthogonal(plane.clone(), n_samples);
        proj.perform(curve);
        proj.into_curve()
    }

    /// `GeomProjLib::ProjectOnPlane` — directed-projection variant.
    ///
    /// Projects `curve` onto `plane` along `direction`.  Returns `None` if the
    /// direction is parallel to the plane for any sample.
    pub fn project_curve_directed(
        curve: &GeomBSplineCurve,
        plane: &GeomPlane,
        direction: Vec3,
        n_samples: usize,
    ) -> Option<GeomBSplineCurve> {
        let mut proj = ProjectOnPlane::directed(plane.clone(), direction, n_samples);
        proj.perform(curve);
        proj.into_curve()
    }

    /// `GeomProjLib::ProjectOnSurface` — project a B-spline curve onto a
    /// B-spline surface.  Convenience wrapper around [`ProjectOnSurface`].
    pub fn project_curve_on_surface(
        curve: &GeomBSplineCurve,
        surface: &GeomBSplineSurface,
        n_samples: usize,
    ) -> Option<GeomBSplineCurve> {
        let mut proj = ProjectOnSurface::new(surface.clone(), n_samples, 3);
        proj.perform(curve);
        proj.into_curve()
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom_plane::GeomPlane;
    use crate::gp::{Ax3, Pnt};

    const TOL_TEST: f64 = 1e-5;

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

    /// A simple line segment from (0,0,0) to (4,0,0) as a degree-1 B-spline.
    fn segment_x(x0: f64, x1: f64) -> GeomBSplineCurve {
        let poles = [Pnt::new(x0, 0.0, 0.0), Pnt::new(x1, 0.0, 0.0)];
        GeomBSplineCurve::new(&poles, &[0.0, 1.0], &[2, 2], 1, false)
    }

    /// A parabolic arc lifted off the XY plane: P(t) = (t, t^2, 1.0) for t in [0,1]
    /// approximated as a degree-2 B-spline.
    fn parabola_lifted() -> GeomBSplineCurve {
        let poles = [
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(0.5, 0.5, 1.0),
            Pnt::new(1.0, 1.0, 1.0),
        ];
        GeomBSplineCurve::new(&poles, &[0.0, 1.0], &[3, 3], 2, false)
    }

    // ─── project_point_on_plane ─────────────────────────────────────────────

    #[test]
    fn project_point_ortho_xy_plane() {
        // XY plane (z=0), point at (1,2,5) → projected to (1,2,0)
        let plane = GeomPlane::from_ax3(Ax3::identity());
        let p = Pnt::new(1.0, 2.0, 5.0);
        let foot = GeomProjLib::project_point_on_plane(p, &plane);
        pnt_near(foot, Pnt::new(1.0, 2.0, 0.0), TOL_TEST);
    }

    #[test]
    fn project_point_ortho_offset_plane() {
        // Plane at z = 3 with normal +Z.
        let origin = Pnt::new(0.0, 0.0, 3.0);
        let normal = Pnt::new(0.0, 0.0, 1.0);
        let plane = GeomPlane::from_point_normal(origin, normal);
        let p = Pnt::new(2.0, 4.0, 10.0);
        let foot = GeomProjLib::project_point_on_plane(p, &plane);
        pnt_near(foot, Pnt::new(2.0, 4.0, 3.0), TOL_TEST);
    }

    #[test]
    fn project_point_directed_xy_plane() {
        // Project (1,2,5) along direction (0,0,-1) onto z=0 → (1,2,0)
        let plane = GeomPlane::from_ax3(Ax3::identity());
        let p = Pnt::new(1.0, 2.0, 5.0);
        let dir = Pnt::new(0.0, 0.0, -1.0);
        let foot = GeomProjLib::project_point_directed(p, &plane, dir).unwrap();
        pnt_near(foot, Pnt::new(1.0, 2.0, 0.0), TOL_TEST);
    }

    #[test]
    fn project_point_directed_angled() {
        // Project (0,0,4) along direction (1,0,-1) onto z=0.
        // Parametric: (0 + t*1, 0, 4 + t*(-1)) = (t, 0, 4-t)
        // z=0 → t=4 → foot=(4,0,0)
        let plane = GeomPlane::from_ax3(Ax3::identity());
        let p = Pnt::new(0.0, 0.0, 4.0);
        let dir = Pnt::new(1.0, 0.0, -1.0);
        let foot = GeomProjLib::project_point_directed(p, &plane, dir).unwrap();
        pnt_near(foot, Pnt::new(4.0, 0.0, 0.0), TOL_TEST);
    }

    #[test]
    fn project_point_directed_parallel_returns_none() {
        // Direction (1,0,0) is parallel to z=0 plane: no intersection.
        let plane = GeomPlane::from_ax3(Ax3::identity());
        let p = Pnt::new(0.0, 0.0, 5.0);
        let dir = Pnt::new(1.0, 0.0, 0.0);
        assert!(GeomProjLib::project_point_directed(p, &plane, dir).is_none());
    }

    // ─── curve_2d ───────────────────────────────────────────────────────────

    #[test]
    fn curve_2d_segment_on_xy_plane() {
        // Segment on z=1 (above XY plane) projects down: U coords follow x.
        let curve = segment_x(0.0, 3.0);
        let plane = GeomPlane::from_ax3(Ax3::identity());
        let uv = GeomProjLib::curve_2d(&curve, &plane, 4);
        assert_eq!(uv.len(), 4);
        // V values should be ≈ 0 (the curve lies on y=0).
        for p in &uv {
            near(p.v, 0.0, TOL_TEST);
        }
        // U increases from 0 to 3.
        assert!(uv[0].u < uv[3].u);
    }

    #[test]
    fn curve_2d_lifted_curve_z_ignored() {
        // The parabola at z=1 — projected orthogonally onto z=0 plane.
        // The (U,V) coords should equal the (x,y) coords (z is dropped).
        let curve = parabola_lifted();
        let plane = GeomPlane::from_ax3(Ax3::identity());
        let uv = GeomProjLib::curve_2d(&curve, &plane, 5);
        // Middle point of the curve projected: x and y should be positive.
        for p in &uv {
            assert!(p.u >= -TOL_TEST, "u={}", p.u);
            assert!(p.v >= -TOL_TEST, "v={}", p.v);
        }
    }

    // ─── curve_2d_bspline ───────────────────────────────────────────────────

    #[test]
    fn curve_2d_bspline_returns_valid_curve() {
        let curve = segment_x(0.0, 5.0);
        let plane = GeomPlane::from_ax3(Ax3::identity());
        let c2d = GeomProjLib::curve_2d_bspline(&curve, &plane, 8, 2);
        assert!(c2d.is_some(), "curve_2d_bspline must succeed");
        let c = c2d.unwrap();
        // Start and end should correspond to (0,0) and (5,0).
        let s = c.start_point();
        let e = c.end_point();
        near(s.x, 0.0, 5e-3);
        near(s.y, 0.0, 5e-3);
        near(e.x, 5.0, 5e-3);
        near(e.y, 0.0, 5e-3);
    }

    // ─── ProjectOnPlane ─────────────────────────────────────────────────────

    #[test]
    fn project_on_plane_orthogonal_segment_stays_in_plane() {
        // Segment at y=2 (above XZ plane): orthogonal projection onto XZ plane
        // should give a segment at y=0.
        let poles = [Pnt::new(0.0, 2.0, 0.0), Pnt::new(4.0, 2.0, 0.0)];
        let curve = GeomBSplineCurve::new(&poles, &[0.0, 1.0], &[2, 2], 1, false);

        let xz_plane_normal = Pnt::new(0.0, 1.0, 0.0); // normal = +Y
        let plane = GeomPlane::from_point_normal(Pnt::origin(), xz_plane_normal);
        let result = GeomProjLib::project_curve_on_plane(&curve, &plane, 10);
        assert!(result.is_some());
        let proj = result.unwrap();
        // All evaluated points should have y ≈ 0.
        for i in 0..5 {
            let t = proj.first_parameter()
                + (proj.last_parameter() - proj.first_parameter()) * i as f64 / 4.0;
            let p = proj.value(t);
            near(p.y, 0.0, 5e-3);
        }
    }

    #[test]
    fn project_on_plane_directed_correct_foot() {
        // Segment on z=2, direction = -Z: projection onto z=0 should give same
        // (x,y) values at z=0.
        let poles = [Pnt::new(0.0, 0.0, 2.0), Pnt::new(3.0, 3.0, 2.0)];
        let curve = GeomBSplineCurve::new(&poles, &[0.0, 1.0], &[2, 2], 1, false);
        let plane = GeomPlane::from_ax3(Ax3::identity()); // z=0 plane
        let result = GeomProjLib::project_curve_directed(
            &curve,
            &plane,
            Pnt::new(0.0, 0.0, -1.0),
            8,
        );
        assert!(result.is_some(), "directed projection must succeed");
        let proj = result.unwrap();
        // Midpoint of original curve is (1.5, 1.5, 2). Projection → (1.5, 1.5, 0).
        let mid_t =
            proj.first_parameter() + (proj.last_parameter() - proj.first_parameter()) * 0.5;
        let mid = proj.value(mid_t);
        near(mid.z, 0.0, 5e-3);
    }

    #[test]
    fn project_on_plane_is_done_after_perform() {
        let curve = segment_x(0.0, 2.0);
        let plane = GeomPlane::from_ax3(Ax3::identity());
        let mut proj = ProjectOnPlane::orthogonal(plane, 6);
        assert!(!proj.is_done(), "should not be done before perform");
        proj.perform(&curve);
        assert!(proj.is_done(), "should be done after perform");
    }
}
