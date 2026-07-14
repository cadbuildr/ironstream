// FILE: crates/ironstream/src/geom_surface_tools.rs
//! `SurfaceTools` — combined surface utilities modelled after OpenCascade's
//! `GeomLib` and `BRep_Tool` packages.
//!
//! # Key items
//!
//! * [`SurfaceTools`] — static methods: retrieve surface from face, query UV
//!   parameter bounds, sample a surface grid, test planarity, and query
//!   geometric continuity.
//! * [`surface_normal`] — unit normal at a parametric point `(u, v)`.
//! * [`surface_d1`] — point and first-order partial derivatives at `(u, v)`.
//! * [`project_on_surface`] — closest `(u, v)` parameters for a 3-D point.

// ---------------------------------------------------------------------------
// SurfaceTools
// ---------------------------------------------------------------------------

/// Static surface utilities, mirroring OpenCascade's `GeomLib` helpers and the
/// surface-query subset of `BRep_Tool`.
///
/// All methods in this implementation are zero-dependency stubs that operate on
/// opaque name strings instead of live geometry objects.  They return
/// well-defined placeholder values so that calling code can be compiled and
/// tested without a full geometry kernel.
// occt: GeomLib
// occt-ref: BRep_Tool
pub struct SurfaceTools;

impl SurfaceTools {
    /// Return the name of the underlying surface associated with the named
    /// face.
    ///
    /// Mirrors `BRep_Tool::Surface(face, location)`.
    ///
    /// # Stub behaviour
    /// Returns a string of the form `"Surface(<face>)"`.
    // occt-ref: BRep_Tool
    pub fn surface_of_face(face: &str) -> String {
        format!("Surface({})", face)
    }

    /// Return the parametric bounds `([u_min, u_max], [v_min, v_max])` of the
    /// named surface.
    ///
    /// Mirrors `BRep_Tool::Surface` + `GeomLib` UV-range helpers.
    ///
    /// # Stub behaviour
    /// Always returns `([0.0, 1.0], [0.0, 1.0])`.
    // occt: GeomLib
    // occt-ref: BRep_Tool
    pub fn uv_bounds(_surface: &str) -> ([f64; 2], [f64; 2]) {
        ([0.0, 1.0], [0.0, 1.0])
    }

    /// Sample the surface on a `nu × nv` regular grid over its UV domain and
    /// return the corresponding 3-D points.
    ///
    /// The result is a `nu`-element outer `Vec` (rows, U direction) where each
    /// inner `Vec` has `nv` elements (columns, V direction).
    ///
    /// Mirrors the sampling pattern used by `GeomLib` when building an
    /// approximate surface representation.
    ///
    /// # Stub behaviour
    /// Uses [`surface_d1`] internally to evaluate the surface point at each
    /// `(u, v)` sample.  The UV domain is obtained from [`Self::uv_bounds`].
    ///
    /// # Panics
    /// Panics if `nu == 0` or `nv == 0`.
    // occt: GeomLib
    pub fn sample_surface(surface: &str, nu: usize, nv: usize) -> Vec<Vec<[f64; 3]>> {
        assert!(nu > 0, "nu must be > 0");
        assert!(nv > 0, "nv must be > 0");

        let ([u_min, u_max], [v_min, v_max]) = Self::uv_bounds(surface);

        let mut grid: Vec<Vec<[f64; 3]>> = Vec::with_capacity(nu);
        for i in 0..nu {
            let u = if nu == 1 {
                (u_min + u_max) * 0.5
            } else {
                u_min + (u_max - u_min) * (i as f64) / ((nu - 1) as f64)
            };
            let mut row: Vec<[f64; 3]> = Vec::with_capacity(nv);
            for j in 0..nv {
                let v = if nv == 1 {
                    (v_min + v_max) * 0.5
                } else {
                    v_min + (v_max - v_min) * (j as f64) / ((nv - 1) as f64)
                };
                let (pt, _du, _dv) = surface_d1(surface, u, v);
                row.push(pt);
            }
            grid.push(row);
        }
        grid
    }

    /// Test whether the named surface is planar within the given tolerance.
    ///
    /// Mirrors `GeomLib::IsBSplinesplanar` / `GeomLib::NormEstim` planarity
    /// check logic.
    ///
    /// # Stub behaviour
    /// Returns `true` when the surface name contains the substring `"Plane"` or
    /// `"plane"`, otherwise `false`.  The `tol` parameter is accepted but not
    /// used in this stub.
    // occt: GeomLib
    pub fn is_planar(surface: &str, _tol: f64) -> bool {
        surface.contains("Plane") || surface.contains("plane")
    }

    /// Return the geometric continuity order of the named surface as an integer
    /// (`0` = C0, `1` = C1, `2` = C2, …).
    ///
    /// Mirrors `GeomLib` continuity query helpers and the `GeomAbs_Shape`
    /// enum range.
    ///
    /// # Stub behaviour
    /// Always returns `2` (C2 continuity).
    // occt: GeomLib
    pub fn continuity(_s: &str) -> u32 {
        2
    }
}

// ---------------------------------------------------------------------------
// Free functions
// ---------------------------------------------------------------------------

/// Evaluate the unit surface normal at parametric coordinates `(u, v)`.
///
/// The normal is computed as the cross product of the partial derivatives
/// `d_du × d_dv`, then normalised to unit length.  If the cross product has
/// zero length (degenerate surface point) `[0.0, 0.0, 1.0]` is returned.
///
/// Mirrors `GeomLib::NormEstim` and the normal-query path in `BRep_Tool`.
///
/// # Stub behaviour
/// Delegates to [`surface_d1`] for the partial derivatives.
// occt: GeomLib
// occt-ref: BRep_Tool
pub fn surface_normal(surface: &str, u: f64, v: f64) -> [f64; 3] {
    let (_pt, du, dv) = surface_d1(surface, u, v);
    let nx = du[1] * dv[2] - du[2] * dv[1];
    let ny = du[2] * dv[0] - du[0] * dv[2];
    let nz = du[0] * dv[1] - du[1] * dv[0];
    let len = (nx * nx + ny * ny + nz * nz).sqrt();
    if len < f64::EPSILON {
        [0.0, 0.0, 1.0]
    } else {
        [nx / len, ny / len, nz / len]
    }
}

/// Evaluate the surface point and first-order partial derivatives at `(u, v)`.
///
/// Returns `(point, d_du, d_dv)` where:
/// - `point`  = position on the surface,
/// - `d_du`   = partial derivative with respect to `u`,
/// - `d_dv`   = partial derivative with respect to `v`.
///
/// Mirrors `GeomAdaptor_Surface::D1` as exposed through `GeomLib` and
/// `BRep_Tool` surface evaluation helpers.
///
/// # Stub behaviour
/// Treats every surface as the XY plane:
/// - `point  = [u, v, 0.0]`
/// - `d_du   = [1.0, 0.0, 0.0]`
/// - `d_dv   = [0.0, 1.0, 0.0]`
///
/// The `surface` parameter is accepted so real implementations can dispatch
/// on the surface kind.
// occt: GeomLib
// occt-ref: BRep_Tool
pub fn surface_d1(_surface: &str, u: f64, v: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let point = [u, v, 0.0];
    let d_du = [1.0, 0.0, 0.0];
    let d_dv = [0.0, 1.0, 0.0];
    (point, d_du, d_dv)
}

/// Find the closest `(u, v)` parameters on the named surface for a given
/// 3-D point.
///
/// Mirrors `GeomLib_Tool::Parameters` / `BRep_Tool` projection utilities that
/// invert the surface map `S(u, v) → P` to obtain `(u, v)` from `P`.
///
/// # Stub behaviour
/// Treats every surface as the XY plane and returns `[pt[0], pt[1]]`,
/// i.e. the orthogonal projection of `pt` onto that plane.  The `z` component
/// of `pt` is intentionally discarded.
// occt: GeomLib
// occt-ref: BRep_Tool
pub fn project_on_surface(pt: [f64; 3], _surface: &str) -> [f64; 2] {
    [pt[0], pt[1]]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-12;

    fn near(a: f64, b: f64) {
        assert!(
            (a - b).abs() < EPS,
            "expected {} ≈ {}, diff = {}",
            a,
            b,
            (a - b).abs()
        );
    }

    fn vec3_near(a: [f64; 3], b: [f64; 3]) {
        near(a[0], b[0]);
        near(a[1], b[1]);
        near(a[2], b[2]);
    }

    fn vec2_near(a: [f64; 2], b: [f64; 2]) {
        near(a[0], b[0]);
        near(a[1], b[1]);
    }

    // ── SurfaceTools::surface_of_face ────────────────────────────────────────

    #[test]
    fn surface_of_face_wraps_name() {
        assert_eq!(SurfaceTools::surface_of_face("F1"), "Surface(F1)");
        assert_eq!(SurfaceTools::surface_of_face("myFace"), "Surface(myFace)");
    }

    #[test]
    fn surface_of_face_empty_name() {
        assert_eq!(SurfaceTools::surface_of_face(""), "Surface()");
    }

    // ── SurfaceTools::uv_bounds ──────────────────────────────────────────────

    #[test]
    fn uv_bounds_returns_unit_square() {
        let (u, v) = SurfaceTools::uv_bounds("any_surface");
        vec2_near(u, [0.0, 1.0]);
        vec2_near(v, [0.0, 1.0]);
    }

    #[test]
    fn uv_bounds_independent_of_name() {
        let (u1, v1) = SurfaceTools::uv_bounds("Plane");
        let (u2, v2) = SurfaceTools::uv_bounds("BSpline");
        assert_eq!(u1, u2);
        assert_eq!(v1, v2);
    }

    // ── SurfaceTools::sample_surface ─────────────────────────────────────────

    #[test]
    fn sample_surface_shape() {
        let grid = SurfaceTools::sample_surface("s", 3, 4);
        assert_eq!(grid.len(), 3);
        for row in &grid {
            assert_eq!(row.len(), 4);
        }
    }

    #[test]
    fn sample_surface_1x1_is_midpoint() {
        let grid = SurfaceTools::sample_surface("s", 1, 1);
        // uv domain [0,1]x[0,1], midpoint (0.5, 0.5) → plane stub gives [0.5, 0.5, 0.0]
        vec3_near(grid[0][0], [0.5, 0.5, 0.0]);
    }

    #[test]
    fn sample_surface_corners_at_uv_extremes() {
        let grid = SurfaceTools::sample_surface("s", 2, 2);
        // plane stub: point = [u, v, 0]
        // [0][0] → (0.0, 0.0)
        vec3_near(grid[0][0], [0.0, 0.0, 0.0]);
        // [0][1] → (0.0, 1.0)
        vec3_near(grid[0][1], [0.0, 1.0, 0.0]);
        // [1][0] → (1.0, 0.0)
        vec3_near(grid[1][0], [1.0, 0.0, 0.0]);
        // [1][1] → (1.0, 1.0)
        vec3_near(grid[1][1], [1.0, 1.0, 0.0]);
    }

    #[test]
    #[should_panic(expected = "nu must be > 0")]
    fn sample_surface_panics_on_zero_nu() {
        SurfaceTools::sample_surface("s", 0, 3);
    }

    #[test]
    #[should_panic(expected = "nv must be > 0")]
    fn sample_surface_panics_on_zero_nv() {
        SurfaceTools::sample_surface("s", 3, 0);
    }

    // ── SurfaceTools::is_planar ───────────────────────────────────────────────

    #[test]
    fn is_planar_true_for_plane_names() {
        assert!(SurfaceTools::is_planar("Plane", 1e-7));
        assert!(SurfaceTools::is_planar("XYPlane", 1e-7));
        assert!(SurfaceTools::is_planar("plane_xy", 1e-7));
    }

    #[test]
    fn is_planar_false_for_other_names() {
        assert!(!SurfaceTools::is_planar("Cylinder", 1e-7));
        assert!(!SurfaceTools::is_planar("Sphere", 0.0));
        assert!(!SurfaceTools::is_planar("BSpline", 1e-3));
    }

    // ── SurfaceTools::continuity ─────────────────────────────────────────────

    #[test]
    fn continuity_returns_two() {
        assert_eq!(SurfaceTools::continuity("Plane"), 2);
        assert_eq!(SurfaceTools::continuity("BSpline"), 2);
        assert_eq!(SurfaceTools::continuity(""), 2);
    }

    // ── surface_normal ────────────────────────────────────────────────────────

    #[test]
    fn surface_normal_plane_is_z_axis() {
        for (u, v) in [(0.0, 0.0), (1.5, -0.5), (-3.0, 2.0)] {
            let n = surface_normal("any", u, v);
            vec3_near(n, [0.0, 0.0, 1.0]);
        }
    }

    #[test]
    fn surface_normal_is_unit_length() {
        let n = surface_normal("s", 0.5, 0.5);
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        near(len, 1.0);
    }

    // ── surface_d1 ───────────────────────────────────────────────────────────

    #[test]
    fn surface_d1_point_is_uv() {
        let (pt, du, dv) = surface_d1("s", 2.5, -1.0);
        vec3_near(pt, [2.5, -1.0, 0.0]);
        vec3_near(du, [1.0, 0.0, 0.0]);
        vec3_near(dv, [0.0, 1.0, 0.0]);
    }

    #[test]
    fn surface_d1_partials_are_constant() {
        for (u, v) in [(0.0, 0.0), (10.0, -5.0), (-1.0, 1.0)] {
            let (_pt, du, dv) = surface_d1("s", u, v);
            vec3_near(du, [1.0, 0.0, 0.0]);
            vec3_near(dv, [0.0, 1.0, 0.0]);
        }
    }

    #[test]
    fn surface_d1_partials_are_orthogonal() {
        let (_pt, du, dv) = surface_d1("s", 1.0, 1.0);
        let dot = du[0] * dv[0] + du[1] * dv[1] + du[2] * dv[2];
        near(dot, 0.0);
    }

    // ── project_on_surface ────────────────────────────────────────────────────

    #[test]
    fn project_drops_z_component() {
        let uv = project_on_surface([3.0, -2.0, 7.5], "s");
        vec2_near(uv, [3.0, -2.0]);
    }

    #[test]
    fn project_origin() {
        let uv = project_on_surface([0.0, 0.0, 0.0], "any");
        vec2_near(uv, [0.0, 0.0]);
    }

    #[test]
    fn project_independent_of_surface_name() {
        let pt = [1.0, 2.0, 3.0];
        assert_eq!(
            project_on_surface(pt, "Plane"),
            project_on_surface(pt, "Cylinder")
        );
    }
}
