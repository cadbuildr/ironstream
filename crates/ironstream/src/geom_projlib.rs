// FILE: geom_projlib.rs
//! `GeomProjLib` stub — low-level projection utilities modelled after OCCT's
//! `ProjLib_ProjectOnSurface` and `ProjLib_Plane` packages.
//!
//! This module provides:
//! - [`ProjectOnPlane`]   — orthogonal projection of points / curves onto an
//!                          analytic plane defined by an origin and a normal.
//! - [`ProjectOnSurface`] — foot-point projection of points / curves onto a
//!                          named parametric surface, returning (U, V) results.
//! - [`project_3d_to_2d`] — convenience free function wrapping
//!                          [`ProjectOnSurface`].
//!
//! # OCCT correspondence
//! - `ProjLib_ProjectOnSurface` (`ProjLib_ProjectOnSurface.hxx`)
//! - `ProjLib_Plane`            (`ProjLib_Plane.hxx`)

// ---------------------------------------------------------------------------
// Internal helpers (no external crates — std only)
// ---------------------------------------------------------------------------

/// Compute the dot product of two 3-D vectors.
#[inline]
fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Subtract two 3-D points / vectors: `a - b`.
#[inline]
fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// Add a scaled vector to a point: `p + t * v`.
#[inline]
fn add_scaled(p: [f64; 3], t: f64, v: [f64; 3]) -> [f64; 3] {
    [p[0] + t * v[0], p[1] + t * v[1], p[2] + t * v[2]]
}

/// Return the Euclidean length of a 3-D vector.
#[inline]
fn len3(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

/// Normalise a 3-D vector; returns the zero vector when the input is
/// degenerate (length < 1e-15).
#[inline]
fn normalise3(v: [f64; 3]) -> [f64; 3] {
    let l = len3(v);
    if l < 1.0e-15 {
        [0.0, 0.0, 0.0]
    } else {
        [v[0] / l, v[1] / l, v[2] / l]
    }
}

/// Build two unit vectors that together with `normal` form a right-handed
/// orthonormal frame for the plane (used to derive (U, V) coordinates inside
/// `ProjectOnPlane::project_point`).
fn plane_axes(normal: [f64; 3]) -> ([f64; 3], [f64; 3]) {
    // Choose an axis that is not parallel to `normal`.
    let n = normalise3(normal);
    let helper: [f64; 3] = if n[0].abs() <= n[1].abs() && n[0].abs() <= n[2].abs() {
        [1.0, 0.0, 0.0]
    } else if n[1].abs() <= n[2].abs() {
        [0.0, 1.0, 0.0]
    } else {
        [0.0, 0.0, 1.0]
    };
    // x_axis = normalise(helper × n)
    let cx = [
        helper[1] * n[2] - helper[2] * n[1],
        helper[2] * n[0] - helper[0] * n[2],
        helper[0] * n[1] - helper[1] * n[0],
    ];
    let x_axis = normalise3(cx);
    // y_axis = n × x_axis
    let y_axis = [
        n[1] * x_axis[2] - n[2] * x_axis[1],
        n[2] * x_axis[0] - n[0] * x_axis[2],
        n[0] * x_axis[1] - n[1] * x_axis[0],
    ];
    (x_axis, y_axis)
}

// ---------------------------------------------------------------------------
// ProjectOnPlane
// ---------------------------------------------------------------------------

/// Orthogonal projection of 3-D points and polyline curves onto an analytic
/// plane.
///
/// The plane is defined by an origin point and a (not necessarily unit-length)
/// normal vector.  Projection is always orthogonal (along the normal direction).
///
/// Returned 3-D coordinates lie on the plane.
///
/// # OCCT correspondence
// occt: ProjLib_Plane
#[derive(Clone, Debug)]
pub struct ProjectOnPlane {
    /// A point on the plane.
    pub plane_origin: [f64; 3],
    /// The plane normal (need not be unit-length; normalised internally).
    pub plane_normal: [f64; 3],
}

impl ProjectOnPlane {
    /// Construct a `ProjectOnPlane` from an origin point and a normal vector.
    ///
    /// The normal is stored as-is; it is normalised on each projection call so
    /// callers may pass any non-zero vector.
    ///
    /// # Example
    /// ```rust
    /// use ironstream::geom_projlib::ProjectOnPlane;
    /// let proj = ProjectOnPlane::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    /// ```
    pub fn new(origin: [f64; 3], normal: [f64; 3]) -> Self {
        Self {
            plane_origin: origin,
            plane_normal: normal,
        }
    }

    /// Orthogonally project a single 3-D point onto the plane.
    ///
    /// The returned point satisfies `dot(result - plane_origin, plane_normal) == 0`.
    ///
    /// # Formula
    /// `foot = p - dot(p - origin, n̂) · n̂`
    ///
    /// # OCCT correspondence
    /// `ProjLib_Plane::Project` (point overload)
    pub fn project_point(&self, p: [f64; 3]) -> [f64; 3] {
        let n = normalise3(self.plane_normal);
        let rel = sub3(p, self.plane_origin);
        let dist = dot3(rel, n);
        add_scaled(p, -dist, n)
    }

    /// Project every point of a polyline curve onto the plane.
    ///
    /// Each point is projected independently via [`project_point`](Self::project_point).
    /// The order and count of points is preserved.
    ///
    /// # OCCT correspondence
    /// `ProjLib_Plane::Project` (curve / polyline overload)
    pub fn project_curve(&self, pts: &[[f64; 3]]) -> Vec<[f64; 3]> {
        pts.iter().map(|&p| self.project_point(p)).collect()
    }
}

// ---------------------------------------------------------------------------
// ProjectOnSurface
// ---------------------------------------------------------------------------

/// Foot-point projection of 3-D points and polyline curves onto a named
/// parametric surface, returning (U, V) parameter pairs.
///
/// This stub stores the surface by name and a numeric tolerance; actual
/// foot-point iteration would require a concrete surface representation.  For
/// the purpose of this stub the (U, V) result is computed by a planar
/// approximation: the 3-D point is orthogonally projected onto the local
/// tangent plane at the surface origin, which is modelled here as the XY plane
/// centred at the origin.
///
/// # OCCT correspondence
// occt: ProjLib_ProjectOnSurface
#[derive(Clone, Debug)]
pub struct ProjectOnSurface {
    /// The name of the target surface (e.g. `"BSplineSurface"`, `"Plane"`, …).
    pub surface_name: String,
    /// Convergence tolerance for foot-point iteration.
    pub tolerance: f64,
    /// Internal flag set to `true` once a successful projection has been run.
    done: bool,
}

impl ProjectOnSurface {
    /// Construct a `ProjectOnSurface` for the named surface with the given
    /// tolerance.
    ///
    /// `is_done()` returns `false` until the first successful call to
    /// [`project_point`](Self::project_point) or
    /// [`project_curve`](Self::project_curve).
    ///
    /// # Example
    /// ```rust
    /// use ironstream::geom_projlib::ProjectOnSurface;
    /// let proj = ProjectOnSurface::new("BSplineSurface", 1e-7);
    /// ```
    pub fn new(surface: &str, tol: f64) -> Self {
        Self {
            surface_name: surface.to_owned(),
            tolerance: tol,
            done: false,
        }
    }

    /// Project a single 3-D point onto the surface and return its (U, V)
    /// parametric coordinates.
    ///
    /// In this stub the surface is modelled as the XY plane: `U = x`, `V = y`.
    /// Callers that require a real foot-point iteration should replace this
    /// method body with a Newton loop over a concrete surface evaluator.
    ///
    /// Sets the internal `done` flag to `true`.
    ///
    /// # OCCT correspondence
    /// `ProjLib_ProjectOnSurface::IsOnSurface` / foot-point query
    pub fn project_point(&mut self, p: [f64; 3]) -> [f64; 2] {
        self.done = true;
        // Stub: treat the surface as the XY plane.
        [p[0], p[1]]
    }

    /// Project every point of a polyline curve and return the (U, V) pairs.
    ///
    /// Each point is projected independently via
    /// [`project_point`](Self::project_point).
    ///
    /// # OCCT correspondence
    /// `ProjLib_ProjectOnSurface` (curve / polyline projection)
    pub fn project_curve(&mut self, pts: &[[f64; 3]]) -> Vec<[f64; 2]> {
        pts.iter().map(|&p| self.project_point(p)).collect()
    }

    /// Returns `true` if at least one projection has been performed
    /// successfully since construction (or the last reset).
    ///
    /// # OCCT correspondence
    /// `ProjLib_ProjectOnSurface::IsDone`
    pub fn is_done(&self) -> bool {
        self.done
    }
}

// ---------------------------------------------------------------------------
// Free function
// ---------------------------------------------------------------------------

/// Project a single 3-D point onto a named surface and return its (U, V)
/// parametric coordinates.
///
/// Constructs a temporary [`ProjectOnSurface`] with a default tolerance of
/// `1e-7` and delegates to [`ProjectOnSurface::project_point`].
///
/// # Parameters
/// - `pt`      — the 3-D point to project.
/// - `surface` — the name of the target surface (forwarded to
///               [`ProjectOnSurface::new`]).
///
/// # Returns
/// `[U, V]` parametric coordinates on the surface.
///
/// # OCCT correspondence
/// Convenience wrapper analogous to `GeomAPI_ProjectPointOnSurf` / free
/// helpers in `ProjLib`.
pub fn project_3d_to_2d(pt: [f64; 3], surface: &str) -> [f64; 2] {
    let mut proj = ProjectOnSurface::new(surface, 1.0e-7);
    proj.project_point(pt)
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ── helpers ──────────────────────────────────────────────────────────────

    fn near(a: f64, b: f64, tol: f64) {
        assert!(
            (a - b).abs() <= tol,
            "expected {a} ≈ {b} within {tol}, diff = {}",
            (a - b).abs()
        );
    }

    fn arr_near(a: [f64; 3], b: [f64; 3], tol: f64) {
        for i in 0..3 {
            near(a[i], b[i], tol);
        }
    }

    // ── ProjectOnPlane::new ──────────────────────────────────────────────────

    #[test]
    fn project_on_plane_stores_origin_and_normal() {
        let proj = ProjectOnPlane::new([1.0, 2.0, 3.0], [0.0, 0.0, 1.0]);
        assert_eq!(proj.plane_origin, [1.0, 2.0, 3.0]);
        assert_eq!(proj.plane_normal, [0.0, 0.0, 1.0]);
    }

    // ── ProjectOnPlane::project_point ────────────────────────────────────────

    #[test]
    fn project_point_onto_xy_plane() {
        // XY plane: origin=(0,0,0), normal=(0,0,1)
        // Point (3, 4, 7) → foot (3, 4, 0)
        let proj = ProjectOnPlane::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let foot = proj.project_point([3.0, 4.0, 7.0]);
        arr_near(foot, [3.0, 4.0, 0.0], 1.0e-12);
    }

    #[test]
    fn project_point_onto_offset_z_plane() {
        // Plane at z=5, normal +Z.  Point (1,2,10) → (1,2,5)
        let proj = ProjectOnPlane::new([0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
        let foot = proj.project_point([1.0, 2.0, 10.0]);
        arr_near(foot, [1.0, 2.0, 5.0], 1.0e-12);
    }

    #[test]
    fn project_point_onto_xz_plane() {
        // XZ plane: normal=(0,1,0).  Point (3,6,1) → (3,0,1)
        let proj = ProjectOnPlane::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let foot = proj.project_point([3.0, 6.0, 1.0]);
        arr_near(foot, [3.0, 0.0, 1.0], 1.0e-12);
    }

    #[test]
    fn project_point_already_on_plane_is_unchanged() {
        // Point that already lies on z=0 should be returned unchanged.
        let proj = ProjectOnPlane::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let p = [5.0, -3.0, 0.0];
        let foot = proj.project_point(p);
        arr_near(foot, p, 1.0e-12);
    }

    #[test]
    fn project_point_non_unit_normal_works() {
        // Normal (0,0,4) — not unit-length — should give the same result.
        let proj = ProjectOnPlane::new([0.0, 0.0, 0.0], [0.0, 0.0, 4.0]);
        let foot = proj.project_point([2.0, -1.0, 9.0]);
        arr_near(foot, [2.0, -1.0, 0.0], 1.0e-12);
    }

    // ── ProjectOnPlane::project_curve ────────────────────────────────────────

    #[test]
    fn project_curve_empty_returns_empty() {
        let proj = ProjectOnPlane::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert!(proj.project_curve(&[]).is_empty());
    }

    #[test]
    fn project_curve_single_point() {
        let proj = ProjectOnPlane::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let result = proj.project_curve(&[[1.0, 2.0, 5.0]]);
        assert_eq!(result.len(), 1);
        arr_near(result[0], [1.0, 2.0, 0.0], 1.0e-12);
    }

    #[test]
    fn project_curve_multiple_points_count_preserved() {
        let proj = ProjectOnPlane::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let pts: Vec<[f64; 3]> = (0..10).map(|i| [i as f64, 0.0, i as f64]).collect();
        let result = proj.project_curve(&pts);
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn project_curve_all_z_coords_zeroed_on_xy_plane() {
        let proj = ProjectOnPlane::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let pts = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [-1.0, -2.0, -3.0]];
        for foot in proj.project_curve(&pts) {
            near(foot[2], 0.0, 1.0e-12);
        }
    }

    // ── ProjectOnSurface::new ────────────────────────────────────────────────

    #[test]
    fn project_on_surface_stores_name_and_tolerance() {
        let proj = ProjectOnSurface::new("Plane", 1.0e-6);
        assert_eq!(proj.surface_name, "Plane");
        assert_eq!(proj.tolerance, 1.0e-6);
    }

    #[test]
    fn project_on_surface_not_done_after_new() {
        let proj = ProjectOnSurface::new("BSplineSurface", 1.0e-7);
        assert!(!proj.is_done());
    }

    // ── ProjectOnSurface::project_point ──────────────────────────────────────

    #[test]
    fn project_on_surface_point_returns_xy() {
        let mut proj = ProjectOnSurface::new("Plane", 1.0e-7);
        let uv = proj.project_point([3.0, 5.0, 2.0]);
        near(uv[0], 3.0, 1.0e-12);
        near(uv[1], 5.0, 1.0e-12);
    }

    #[test]
    fn project_on_surface_is_done_after_project_point() {
        let mut proj = ProjectOnSurface::new("Cone", 1.0e-7);
        proj.project_point([0.0, 0.0, 0.0]);
        assert!(proj.is_done());
    }

    // ── ProjectOnSurface::project_curve ──────────────────────────────────────

    #[test]
    fn project_on_surface_curve_empty_returns_empty() {
        let mut proj = ProjectOnSurface::new("Plane", 1.0e-7);
        assert!(proj.project_curve(&[]).is_empty());
    }

    #[test]
    fn project_on_surface_curve_count_preserved() {
        let mut proj = ProjectOnSurface::new("Plane", 1.0e-7);
        let pts: Vec<[f64; 3]> = (0..5).map(|i| [i as f64, i as f64, 0.0]).collect();
        let result = proj.project_curve(&pts);
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn project_on_surface_curve_uv_values_correct() {
        let mut proj = ProjectOnSurface::new("Plane", 1.0e-7);
        let pts = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
        let result = proj.project_curve(&pts);
        near(result[0][0], 1.0, 1.0e-12);
        near(result[0][1], 2.0, 1.0e-12);
        near(result[1][0], 4.0, 1.0e-12);
        near(result[1][1], 5.0, 1.0e-12);
    }

    #[test]
    fn project_on_surface_is_done_after_project_curve() {
        let mut proj = ProjectOnSurface::new("Plane", 1.0e-7);
        proj.project_curve(&[[1.0, 2.0, 3.0]]);
        assert!(proj.is_done());
    }

    // ── project_3d_to_2d ─────────────────────────────────────────────────────

    #[test]
    fn project_3d_to_2d_basic() {
        let uv = project_3d_to_2d([7.0, -3.0, 5.0], "BSplineSurface");
        near(uv[0], 7.0, 1.0e-12);
        near(uv[1], -3.0, 1.0e-12);
    }

    #[test]
    fn project_3d_to_2d_origin() {
        let uv = project_3d_to_2d([0.0, 0.0, 0.0], "Plane");
        near(uv[0], 0.0, 1.0e-12);
        near(uv[1], 0.0, 1.0e-12);
    }

    #[test]
    fn project_3d_to_2d_negative_coords() {
        let uv = project_3d_to_2d([-5.0, -8.0, 100.0], "Cone");
        near(uv[0], -5.0, 1.0e-12);
        near(uv[1], -8.0, 1.0e-12);
    }
}
