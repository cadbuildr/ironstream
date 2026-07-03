// FILE: geom_api_ext.rs
//! High-level extrema algorithms between curves and between a curve and a
//! surface, mirroring OpenCascade's `GeomAPI_ExtremaCurveCurve` and
//! `GeomAPI_ExtremaCurveSurface` classes
//! (`src/ModelingAlgorithms/TKGeomBase/GeomAPI`).
//!
//! All types are pure-Rust stubs with zero external dependencies.  The
//! `solutions` fields and lazy distance computations provide the same public
//! contract as the OCCT counterparts so that calling code can be ported
//! one-to-one.

// ─────────────────────────────────────────────────────────────────────────────
// ExtremaCurveCurveApi
// ─────────────────────────────────────────────────────────────────────────────

/// Extrema between two 3-D curves.
///
/// Each solution is stored as `(param_on_c1, param_on_c2, distance)`.
///
/// # Usage
/// ```
/// use ironstream::geom_api_ext::ExtremaCurveCurveApi;
///
/// let api = ExtremaCurveCurveApi::new("line", "circle");
/// assert_eq!(api.nb_extrema(), 0);
/// ```
// occt: GeomAPI_ExtremaCurveCurve
#[derive(Clone, Debug)]
pub struct ExtremaCurveCurveApi {
    /// Name / tag identifying the first curve (stub: not evaluated).
    pub c1: String,
    /// Name / tag identifying the second curve (stub: not evaluated).
    pub c2: String,
    /// Extrema solutions: `(param_on_c1, param_on_c2, distance)`.
    pub solutions: Vec<(f64, f64, f64)>,
}

impl ExtremaCurveCurveApi {
    /// `GeomAPI_ExtremaCurveCurve(C1, C2)` — construct and initialise the
    /// algorithm for the two named curves.  No solutions are computed by the
    /// stub; populate `solutions` directly to simulate results.
    pub fn new(c1: &str, c2: &str) -> Self {
        Self {
            c1: c1.to_owned(),
            c2: c2.to_owned(),
            solutions: Vec::new(),
        }
    }

    /// `NbExtrema()` — number of extrema found.
    pub fn nb_extrema(&self) -> usize {
        self.solutions.len()
    }

    /// `Distance(Index)` — distance at the given 0-based solution index.
    ///
    /// Returns `f64::NAN` when `i` is out of range (mirrors OCCT's
    /// `Standard_OutOfRange` behaviour in a panic-free way).
    pub fn distance(&self, i: usize) -> f64 {
        self.solutions.get(i).map(|&(_, _, d)| d).unwrap_or(f64::NAN)
    }

    /// `Parameters(Index)` — `(u1, u2)` curve parameters at solution `i`.
    ///
    /// Returns `(NAN, NAN)` when `i` is out of range.
    pub fn parameters(&self, i: usize) -> (f64, f64) {
        self.solutions
            .get(i)
            .map(|&(u1, u2, _)| (u1, u2))
            .unwrap_or((f64::NAN, f64::NAN))
    }

    /// `LowerDistance()` — smallest distance across all solutions, or
    /// `f64::INFINITY` when there are no solutions.
    pub fn lower_distance(&self) -> f64 {
        self.solutions
            .iter()
            .map(|&(_, _, d)| d)
            .fold(f64::INFINITY, f64::min)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ExtremaCurveSurfaceApi
// ─────────────────────────────────────────────────────────────────────────────

/// Extrema between a 3-D curve and a surface.
///
/// Each solution is stored as `(param_on_curve, [u, v]_on_surface, distance)`.
///
/// # Usage
/// ```
/// use ironstream::geom_api_ext::ExtremaCurveSurfaceApi;
///
/// let api = ExtremaCurveSurfaceApi::new("bspline", "plane");
/// assert_eq!(api.nb_extrema(), 0);
/// ```
// occt: GeomAPI_ExtremaCurveSurface
#[derive(Clone, Debug)]
pub struct ExtremaCurveSurfaceApi {
    /// Name / tag identifying the curve (stub: not evaluated).
    pub curve: String,
    /// Name / tag identifying the surface (stub: not evaluated).
    pub surface: String,
    /// Extrema solutions: `(param_on_curve, [u, v]_on_surface, distance)`.
    pub solutions: Vec<(f64, [f64; 2], f64)>,
}

impl ExtremaCurveSurfaceApi {
    /// `GeomAPI_ExtremaCurveSurface(Curve, Surface)` — construct and
    /// initialise the algorithm.  No solutions are computed by the stub.
    pub fn new(curve: &str, surface: &str) -> Self {
        Self {
            curve: curve.to_owned(),
            surface: surface.to_owned(),
            solutions: Vec::new(),
        }
    }

    /// `NbExtrema()` — number of extrema found.
    pub fn nb_extrema(&self) -> usize {
        self.solutions.len()
    }

    /// `Distance(Index)` — distance at the given 0-based solution index.
    ///
    /// Returns `f64::NAN` when `i` is out of range.
    pub fn distance(&self, i: usize) -> f64 {
        self.solutions.get(i).map(|&(_, _, d)| d).unwrap_or(f64::NAN)
    }

    /// `LowerDistance()` — smallest distance across all solutions, or
    /// `f64::INFINITY` when there are no solutions.
    pub fn lower_distance(&self) -> f64 {
        self.solutions
            .iter()
            .map(|&(_, _, d)| d)
            .fold(f64::INFINITY, f64::min)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Free-function convenience wrapper
// ─────────────────────────────────────────────────────────────────────────────

/// Compute the lower distance between two named curves.
///
/// This is a thin wrapper around [`ExtremaCurveCurveApi`] for call-sites that
/// only need a single scalar result.  Returns `f64::INFINITY` when no extrema
/// have been registered in the stub.
pub fn extrema_distance(c1: &str, c2: &str) -> f64 {
    ExtremaCurveCurveApi::new(c1, c2).lower_distance()
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ──── ExtremaCurveCurveApi ────

    #[test]
    fn curve_curve_new_empty() {
        let api = ExtremaCurveCurveApi::new("line", "circle");
        assert_eq!(api.c1, "line");
        assert_eq!(api.c2, "circle");
        assert_eq!(api.nb_extrema(), 0);
    }

    #[test]
    fn curve_curve_lower_distance_no_solutions() {
        let api = ExtremaCurveCurveApi::new("a", "b");
        assert_eq!(api.lower_distance(), f64::INFINITY);
    }

    #[test]
    fn curve_curve_distance_out_of_range_is_nan() {
        let api = ExtremaCurveCurveApi::new("a", "b");
        assert!(api.distance(0).is_nan());
    }

    #[test]
    fn curve_curve_parameters_out_of_range_are_nan() {
        let api = ExtremaCurveCurveApi::new("a", "b");
        let (u1, u2) = api.parameters(0);
        assert!(u1.is_nan() && u2.is_nan());
    }

    #[test]
    fn curve_curve_single_solution() {
        let mut api = ExtremaCurveCurveApi::new("line", "line");
        api.solutions.push((0.25, 0.75, 3.5));
        assert_eq!(api.nb_extrema(), 1);
        assert_eq!(api.distance(0), 3.5);
        assert_eq!(api.parameters(0), (0.25, 0.75));
        assert_eq!(api.lower_distance(), 3.5);
    }

    #[test]
    fn curve_curve_lower_distance_picks_minimum() {
        let mut api = ExtremaCurveCurveApi::new("c1", "c2");
        api.solutions.push((0.0, 0.0, 10.0));
        api.solutions.push((0.5, 0.5, 2.0));
        api.solutions.push((1.0, 1.0, 7.0));
        assert_eq!(api.lower_distance(), 2.0);
    }

    #[test]
    fn curve_curve_multiple_solutions_indexing() {
        let mut api = ExtremaCurveCurveApi::new("c1", "c2");
        api.solutions.push((0.1, 0.2, 1.0));
        api.solutions.push((0.3, 0.4, 2.0));
        assert_eq!(api.distance(1), 2.0);
        assert_eq!(api.parameters(1), (0.3, 0.4));
        assert!(api.distance(2).is_nan());
    }

    // ──── ExtremaCurveSurfaceApi ────

    #[test]
    fn curve_surface_new_empty() {
        let api = ExtremaCurveSurfaceApi::new("bspline", "plane");
        assert_eq!(api.curve, "bspline");
        assert_eq!(api.surface, "plane");
        assert_eq!(api.nb_extrema(), 0);
    }

    #[test]
    fn curve_surface_lower_distance_no_solutions() {
        let api = ExtremaCurveSurfaceApi::new("c", "s");
        assert_eq!(api.lower_distance(), f64::INFINITY);
    }

    #[test]
    fn curve_surface_distance_out_of_range_is_nan() {
        let api = ExtremaCurveSurfaceApi::new("c", "s");
        assert!(api.distance(0).is_nan());
    }

    #[test]
    fn curve_surface_single_solution() {
        let mut api = ExtremaCurveSurfaceApi::new("line", "sphere");
        api.solutions.push((0.5, [0.1, 0.2], 4.0));
        assert_eq!(api.nb_extrema(), 1);
        assert_eq!(api.distance(0), 4.0);
        assert_eq!(api.lower_distance(), 4.0);
    }

    #[test]
    fn curve_surface_lower_distance_picks_minimum() {
        let mut api = ExtremaCurveSurfaceApi::new("c", "s");
        api.solutions.push((0.0, [0.0, 0.0], 5.0));
        api.solutions.push((0.5, [0.5, 0.5], 1.5));
        api.solutions.push((1.0, [1.0, 1.0], 3.0));
        assert_eq!(api.lower_distance(), 1.5);
    }

    #[test]
    fn curve_surface_uv_stored_in_solution() {
        let mut api = ExtremaCurveSurfaceApi::new("c", "s");
        api.solutions.push((0.3, [0.6, 0.9], 2.2));
        let (_, uv, _) = api.solutions[0];
        assert_eq!(uv, [0.6, 0.9]);
    }

    // ──── extrema_distance ────

    #[test]
    fn extrema_distance_no_solutions_returns_infinity() {
        assert_eq!(extrema_distance("line", "circle"), f64::INFINITY);
    }

    #[test]
    fn extrema_distance_reflects_curve_curve_lower_distance() {
        // The free function delegates to ExtremaCurveCurveApi; without
        // populated solutions it returns INFINITY, matching lower_distance().
        let api = ExtremaCurveCurveApi::new("line", "circle");
        assert_eq!(extrema_distance("line", "circle"), api.lower_distance());
    }
}
