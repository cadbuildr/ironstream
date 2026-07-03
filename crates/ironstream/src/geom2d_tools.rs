// FILE: geom2d_tools.rs
//! `Geom2dTools` — 2D curve adaptor/convert helper utilities, mirroring the
//! OpenCascade `Geom2dAdaptor` and `Geom2dConvert` namespaces.
//!
//! Provides:
//! - [`Geom2dContinuity`]  — continuity classification for 2D curves
//! - [`Geom2dCurveInfo`]   — lightweight 2D adaptor-curve descriptor
//! - [`Geom2dTool`]        — static utility namespace (convert / sampling helpers)
//! - [`CurveTool2d`]       — B-spline curve tool with pole/knot management
//! - [`AdaptorCurve2d`]    — lightweight parameter-range adaptor for 2D curves
//! - [`convert_bspline_2d`] — construct a `CurveTool2d` from raw poles/degree
//! - [`sample_curve_2d`]   — uniformly sample a `CurveTool2d`

// ---------------------------------------------------------------------------
// Geom2dContinuity
// ---------------------------------------------------------------------------

/// Continuity classification for 2D curves.
///
/// Mirrors `GeomAbs_Shape` from the OCCT `GeomAbs` package as applied to
/// 2D-curve contexts (`Geom2dAdaptor_Curve::Continuity()`).
// occt: GeomAbs_Shape for 2D — enum C0, G1, C1, G2, C2, C3, CN
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Geom2dContinuity {
    /// Positional continuity only (no tangent constraint).
    C0,
    /// Geometric tangent continuity (direction, not magnitude).
    G1,
    /// Parametric tangent continuity (C1: first derivative continuous).
    C1,
    /// Geometric curvature continuity.
    G2,
    /// Parametric curvature continuity (C2: second derivative continuous).
    C2,
    /// Third-derivative continuity.
    C3,
    /// Infinite-order continuity (analytic).
    CN,
}

// ---------------------------------------------------------------------------
// Geom2dCurveInfo
// ---------------------------------------------------------------------------

/// Lightweight descriptor for a 2D curve, carrying the information that
/// `Geom2dAdaptor_Curve` surfaces from an underlying `Geom2d_Curve`.
///
/// This is a pure-data record with no C++ handle dependency.
// occt: Geom2dAdaptor_Curve info
pub struct Geom2dCurveInfo {
    curve_type: String,
    first: f64,
    last: f64,
    degree: u32,
    nb_poles: usize,
    continuity: Geom2dContinuity,
}

impl Geom2dCurveInfo {
    /// Construct a new curve descriptor with the given type name and parameter
    /// range.  `degree` defaults to `0` and `nb_poles` to `0`; set them with
    /// the mutating accessors when known.
    ///
    /// Corresponds to constructing a `Geom2dAdaptor_Curve` then querying its
    /// `GetType()`, `FirstParameter()`, and `LastParameter()`.
    pub fn new(curve_type: &str, first: f64, last: f64) -> Self {
        Self {
            curve_type: curve_type.to_owned(),
            first,
            last,
            degree: 0,
            nb_poles: 0,
            continuity: Geom2dContinuity::C0,
        }
    }

    /// The string name of the curve kind (e.g. `"Line"`, `"Circle"`,
    /// `"BSpline"`, …).
    // occt: Geom2dAdaptor_Curve::GetType() → GeomAbs_CurveType
    pub fn curve_type(&self) -> &str {
        &self.curve_type
    }

    /// First (smallest) parameter of the curve's valid range.
    // occt: Geom2dAdaptor_Curve::FirstParameter()
    pub fn first(&self) -> f64 {
        self.first
    }

    /// Last (largest) parameter of the curve's valid range.
    // occt: Geom2dAdaptor_Curve::LastParameter()
    pub fn last(&self) -> f64 {
        self.last
    }

    /// Polynomial degree (meaningful for Bezier / B-spline curves).
    // occt: Geom2dAdaptor_Curve::Degree()
    pub fn degree(&self) -> u32 {
        self.degree
    }

    /// Number of control poles (meaningful for Bezier / B-spline curves).
    // occt: Geom2dAdaptor_Curve::NbPoles()
    pub fn nb_poles(&self) -> usize {
        self.nb_poles
    }

    /// Continuity class of the underlying curve.
    // occt: Geom2dAdaptor_Curve::Continuity()
    pub fn continuity(&self) -> Geom2dContinuity {
        self.continuity
    }

    /// Override the continuity classification.
    pub fn set_continuity(&mut self, c: Geom2dContinuity) {
        self.continuity = c;
    }

    /// Returns `true` when the first and last parameters are within `1e-10`
    /// of each other, indicating a closed (periodic) curve.
    // occt: Geom2dAdaptor_Curve::IsClosed()
    pub fn is_closed(&self) -> bool {
        (self.first - self.last).abs() < 1.0e-10
    }
}

// ---------------------------------------------------------------------------
// Geom2dTool
// ---------------------------------------------------------------------------

/// Static utility namespace for 2D curve operations.
///
/// All methods are free functions; `Geom2dTool` itself carries no state.
// occt: Geom2dConvert utility namespace
pub struct Geom2dTool;

impl Geom2dTool {
    /// Compute the total arc length as the product of the number of segments
    /// and the average segment length.
    ///
    /// Mirrors the arithmetic used in `Geom2dConvert` when distributing
    /// sampling points across a composed curve.
    pub fn total_length(nb_segments: usize, avg_segment_length: f64) -> f64 {
        nb_segments as f64 * avg_segment_length
    }

    /// Build a uniform parameter vector over `[first, last]` with `nb_points`
    /// samples (inclusive of both endpoints).
    ///
    /// When `nb_points == 1`, the single value returned is `first`.
    ///
    /// Mirrors the linspace sampling used in `Geom2dConvert` internal helpers
    /// (e.g. when splitting a B-spline into Bezier arcs).
    pub fn parameter_range(first: f64, last: f64, nb_points: u32) -> Vec<f64> {
        if nb_points == 0 {
            return Vec::new();
        }
        if nb_points == 1 {
            return vec![first];
        }
        let n = nb_points as usize;
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            out.push(first + (last - first) * (i as f64 / (n - 1) as f64));
        }
        out
    }

    /// Map a parameter `t` in `[first, last]` to its reverse counterpart.
    ///
    /// The reversal formula is `first + last - t`, which maps `first ↔ last`
    /// and preserves the midpoint.  Used when reversing curve orientation.
    // occt: Geom2dConvert reverse-parameter utility
    pub fn reverse_parameter(t: f64, first: f64, last: f64) -> f64 {
        first + last - t
    }

    /// Construct a [`Geom2dCurveInfo`] describing a B-spline curve from its
    /// degree and pole count.
    ///
    /// The parameter range defaults to `[0.0, 1.0]` and the continuity to
    /// `C0`; callers should call [`Geom2dCurveInfo::set_continuity`] when a
    /// more precise continuity class is known.
    // occt: Geom2dConvert — derive curve info from degree/nb_poles
    pub fn curve_info_from_degree(degree: u32, nb_poles: usize) -> Geom2dCurveInfo {
        let mut info = Geom2dCurveInfo::new("BSpline", 0.0, 1.0);
        info.degree = degree;
        info.nb_poles = nb_poles;
        info
    }
}

// ---------------------------------------------------------------------------
// CurveTool2d
// ---------------------------------------------------------------------------

/// B-spline curve tool for 2D geometry, holding degree, knots, and poles.
///
/// Provides construction, mutation, and de Boor evaluation over the B-spline
/// basis.  The knot vector is stored in flat (repeated) form; callers are
/// responsible for ensuring the knot vector is valid for the chosen degree and
/// pole count before calling [`CurveTool2d::evaluate`].
// occt: Geom2dConvert
pub struct CurveTool2d {
    /// Polynomial degree of the B-spline basis.
    pub degree: u32,
    /// Flat (repeated) knot vector.
    pub knots: Vec<f64>,
    /// Control poles in 2D, each `[x, y]`.
    pub poles: Vec<[f64; 2]>,
}

impl CurveTool2d {
    /// Construct an empty `CurveTool2d` with degree `1` and no poles or knots.
    ///
    /// Mirrors `Geom2dConvert` tool initialisation before populating data.
    pub fn new() -> Self {
        Self { degree: 1, knots: Vec::new(), poles: Vec::new() }
    }

    /// Append a control pole at `(x, y)`.
    pub fn add_pole(&mut self, x: f64, y: f64) {
        self.poles.push([x, y]);
    }

    /// Set the B-spline degree.
    pub fn set_degree(&mut self, d: u32) {
        self.degree = d;
    }

    /// Evaluate the B-spline at parameter `t` using de Boor's algorithm.
    ///
    /// Returns `[0.0, 0.0]` when there are no poles.  When the knot vector is
    /// empty or shorter than `2 * (degree + 1)`, the evaluation degrades
    /// gracefully by clamping the active span.
    ///
    /// Mirrors `Geom2dAdaptor_Curve::Value(U)`.
    // occt: Geom2dAdaptor
    pub fn evaluate(&self, t: f64) -> [f64; 2] {
        let n = self.poles.len();
        if n == 0 {
            return [0.0, 0.0];
        }

        let p = self.degree as usize;
        let fk = &self.knots;

        // Locate the knot span containing t.
        let m = fk.len();
        let mut span = p;
        if m > 0 {
            for i in p..m.saturating_sub(p + 1) {
                if t < fk[i + 1] {
                    span = i;
                    break;
                }
                span = i;
            }
        }

        // de Boor in 2D.
        let mut d: Vec<[f64; 2]> = Vec::with_capacity(p + 1);
        for j in 0..=p {
            let idx = if span + j >= p { span + j - p } else { 0 };
            let idx = idx.min(n - 1);
            d.push(self.poles[idx]);
        }

        for r in 1..=p {
            for j in (r..=p).rev() {
                let i = span + j - p;
                let denom = if m > 0 && i + p + 1 - r < m && i < m {
                    fk[i + p + 1 - r] - fk[i]
                } else {
                    0.0
                };
                let alpha = if denom.abs() < 1.0e-15 {
                    0.0
                } else {
                    (t - fk[i]) / denom
                };
                d[j][0] = (1.0 - alpha) * d[j - 1][0] + alpha * d[j][0];
                d[j][1] = (1.0 - alpha) * d[j - 1][1] + alpha * d[j][1];
            }
        }

        d[p]
    }
}

impl Default for CurveTool2d {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// AdaptorCurve2d
// ---------------------------------------------------------------------------

/// Lightweight parameter-range adaptor for a 2D curve.
///
/// Stores the start and end parameter values of a curve's valid domain and
/// provides simple derived quantities (length of interval, midpoint).
///
/// Mirrors `Geom2dAdaptor_Curve::FirstParameter()` /
/// `Geom2dAdaptor_Curve::LastParameter()` and the arithmetic performed on the
/// parameter range in OCCT adaptor utilities.
// occt: Geom2dAdaptor
pub struct AdaptorCurve2d {
    /// First parameter of the curve domain.
    pub start: f64,
    /// Last parameter of the curve domain.
    pub end: f64,
}

impl AdaptorCurve2d {
    /// Construct an adaptor for the parameter interval `[start, end]`.
    pub fn new(start: f64, end: f64) -> Self {
        Self { start, end }
    }

    /// Length of the parameter interval: `end - start`.
    ///
    /// Note: this is the *parametric* length, not the arc length.
    pub fn length(&self) -> f64 {
        self.end - self.start
    }

    /// Midpoint of the parameter interval: `(start + end) / 2`.
    pub fn midpoint(&self) -> f64 {
        (self.start + self.end) * 0.5
    }
}

// ---------------------------------------------------------------------------
// Free functions
// ---------------------------------------------------------------------------

/// Build a [`CurveTool2d`] from an array of 2D poles and a polynomial degree.
///
/// A clamped uniform knot vector is generated automatically: the first and last
/// knot values each repeat `degree + 1` times (so the curve interpolates its
/// end-poles), and interior knots are distributed uniformly.
///
/// Mirrors `Geom2dConvert::CurveToBSplineCurve` when applied to a set of
/// defining poles.
// occt: Geom2dConvert
pub fn convert_bspline_2d(poles: &[[f64; 2]], degree: u32) -> CurveTool2d {
    let n = poles.len();
    let p = degree as usize;

    // Build a clamped uniform flat knot vector.
    // Total knot count = n + p + 1 for a valid B-spline.
    let total_knots = if n > 0 { n + p + 1 } else { 2 * (p + 1) };
    let interior = if total_knots > 2 * (p + 1) {
        total_knots - 2 * (p + 1)
    } else {
        0
    };

    let mut knots = Vec::with_capacity(total_knots);
    // p+1 leading zeros.
    for _ in 0..=p {
        knots.push(0.0_f64);
    }
    // Interior knots uniformly spaced in (0, 1).
    for i in 1..=interior {
        knots.push(i as f64 / (interior + 1) as f64);
    }
    // p+1 trailing ones.
    for _ in 0..=p {
        knots.push(1.0_f64);
    }

    CurveTool2d { degree, knots, poles: poles.to_vec() }
}

/// Uniformly sample a [`CurveTool2d`] at `n` parameter values.
///
/// The parameter range `[0.0, 1.0]` is divided into `n` equal intervals; the
/// function evaluates the curve at each sample using
/// [`CurveTool2d::evaluate`].  When `n == 0` an empty vector is returned.
/// When `n == 1` only the point at `t = 0.0` is returned.
///
/// Mirrors OCCT `GCPnts_UniformParameter` usage over a `Geom2dAdaptor_Curve`.
// occt: Geom2dAdaptor
pub fn sample_curve_2d(curve: &CurveTool2d, n: usize) -> Vec<[f64; 2]> {
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![curve.evaluate(0.0)];
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f64 / (n - 1) as f64;
        out.push(curve.evaluate(t));
    }
    out
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Geom2dContinuity ---

    #[test]
    fn continuity_ordering() {
        assert!(Geom2dContinuity::C0 < Geom2dContinuity::G1);
        assert!(Geom2dContinuity::G1 < Geom2dContinuity::C1);
        assert!(Geom2dContinuity::C1 < Geom2dContinuity::G2);
        assert!(Geom2dContinuity::G2 < Geom2dContinuity::C2);
        assert!(Geom2dContinuity::C2 < Geom2dContinuity::C3);
        assert!(Geom2dContinuity::C3 < Geom2dContinuity::CN);
    }

    #[test]
    fn continuity_equality() {
        assert_eq!(Geom2dContinuity::C2, Geom2dContinuity::C2);
        assert_ne!(Geom2dContinuity::C0, Geom2dContinuity::CN);
    }

    // --- Geom2dCurveInfo ---

    #[test]
    fn curve_info_new_defaults() {
        let info = Geom2dCurveInfo::new("Line", 0.0, 1.0);
        assert_eq!(info.curve_type(), "Line");
        assert_eq!(info.first(), 0.0);
        assert_eq!(info.last(), 1.0);
        assert_eq!(info.degree(), 0);
        assert_eq!(info.nb_poles(), 0);
        assert_eq!(info.continuity(), Geom2dContinuity::C0);
    }

    #[test]
    fn curve_info_is_closed_open() {
        let info = Geom2dCurveInfo::new("BSpline", 0.0, 1.0);
        assert!(!info.is_closed());
    }

    #[test]
    fn curve_info_is_closed_when_first_equals_last() {
        // Identical first and last → closed.
        let info = Geom2dCurveInfo::new("Circle", 0.0, 0.0);
        assert!(info.is_closed());
    }

    #[test]
    fn curve_info_is_closed_near_zero() {
        // Within 1e-10 → closed.
        let info = Geom2dCurveInfo::new("Circle", 0.0, 5.0e-11);
        assert!(info.is_closed());
    }

    #[test]
    fn curve_info_is_closed_just_outside_tolerance() {
        let info = Geom2dCurveInfo::new("Circle", 0.0, 2.0e-10);
        assert!(!info.is_closed());
    }

    #[test]
    fn curve_info_set_continuity() {
        let mut info = Geom2dCurveInfo::new("Ellipse", 0.0, 6.283185);
        info.set_continuity(Geom2dContinuity::CN);
        assert_eq!(info.continuity(), Geom2dContinuity::CN);
    }

    #[test]
    fn curve_info_fields_roundtrip() {
        let mut info = Geom2dCurveInfo::new("BSpline", -1.0, 2.5);
        info.degree = 3;
        info.nb_poles = 6;
        info.set_continuity(Geom2dContinuity::C2);
        assert_eq!(info.curve_type(), "BSpline");
        assert_eq!(info.first(), -1.0);
        assert_eq!(info.last(), 2.5);
        assert_eq!(info.degree(), 3);
        assert_eq!(info.nb_poles(), 6);
        assert_eq!(info.continuity(), Geom2dContinuity::C2);
    }

    // --- Geom2dTool ---

    #[test]
    fn total_length_zero_segments() {
        assert_eq!(Geom2dTool::total_length(0, 5.0), 0.0);
    }

    #[test]
    fn total_length_basic() {
        assert!((Geom2dTool::total_length(4, 2.5) - 10.0).abs() < 1.0e-12);
    }

    #[test]
    fn total_length_fractional() {
        let result = Geom2dTool::total_length(3, 1.0 / 3.0);
        assert!((result - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn parameter_range_zero_points() {
        let v = Geom2dTool::parameter_range(0.0, 1.0, 0);
        assert!(v.is_empty());
    }

    #[test]
    fn parameter_range_one_point() {
        let v = Geom2dTool::parameter_range(3.0, 7.0, 1);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], 3.0);
    }

    #[test]
    fn parameter_range_two_points() {
        let v = Geom2dTool::parameter_range(0.0, 1.0, 2);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 0.0);
        assert_eq!(v[1], 1.0);
    }

    #[test]
    fn parameter_range_five_points_linspace() {
        let v = Geom2dTool::parameter_range(0.0, 4.0, 5);
        assert_eq!(v.len(), 5);
        assert!((v[0] - 0.0).abs() < 1.0e-12);
        assert!((v[1] - 1.0).abs() < 1.0e-12);
        assert!((v[2] - 2.0).abs() < 1.0e-12);
        assert!((v[3] - 3.0).abs() < 1.0e-12);
        assert!((v[4] - 4.0).abs() < 1.0e-12);
    }

    #[test]
    fn parameter_range_endpoints_exact() {
        let first = -3.7;
        let last = 8.1;
        let v = Geom2dTool::parameter_range(first, last, 100);
        assert_eq!(v.len(), 100);
        assert!((v[0] - first).abs() < 1.0e-12);
        assert!((v[99] - last).abs() < 1.0e-12);
    }

    #[test]
    fn reverse_parameter_endpoints() {
        let (first, last) = (0.0, 1.0);
        assert!((Geom2dTool::reverse_parameter(first, first, last) - last).abs() < 1.0e-12);
        assert!((Geom2dTool::reverse_parameter(last, first, last) - first).abs() < 1.0e-12);
    }

    #[test]
    fn reverse_parameter_midpoint_invariant() {
        let (first, last) = (1.0, 5.0);
        let mid = (first + last) * 0.5;
        let rev = Geom2dTool::reverse_parameter(mid, first, last);
        assert!((rev - mid).abs() < 1.0e-12);
    }

    #[test]
    fn reverse_parameter_arbitrary() {
        let (first, last, t) = (0.0, 10.0, 3.0);
        assert!((Geom2dTool::reverse_parameter(t, first, last) - 7.0).abs() < 1.0e-12);
    }

    #[test]
    fn reverse_parameter_is_involution() {
        let (first, last, t) = (2.0, 8.0, 5.5);
        let r1 = Geom2dTool::reverse_parameter(t, first, last);
        let r2 = Geom2dTool::reverse_parameter(r1, first, last);
        assert!((r2 - t).abs() < 1.0e-12);
    }

    #[test]
    fn curve_info_from_degree_type_is_bspline() {
        let info = Geom2dTool::curve_info_from_degree(3, 7);
        assert_eq!(info.curve_type(), "BSpline");
    }

    #[test]
    fn curve_info_from_degree_fields() {
        let info = Geom2dTool::curve_info_from_degree(2, 5);
        assert_eq!(info.degree(), 2);
        assert_eq!(info.nb_poles(), 5);
        assert_eq!(info.first(), 0.0);
        assert_eq!(info.last(), 1.0);
    }

    #[test]
    fn curve_info_from_degree_default_continuity_c0() {
        let info = Geom2dTool::curve_info_from_degree(1, 2);
        assert_eq!(info.continuity(), Geom2dContinuity::C0);
    }

    #[test]
    fn curve_info_from_degree_not_closed() {
        let info = Geom2dTool::curve_info_from_degree(4, 8);
        assert!(!info.is_closed());
    }

    // --- CurveTool2d ---

    #[test]
    fn curve_tool2d_new_empty() {
        let c = CurveTool2d::new();
        assert_eq!(c.degree, 1);
        assert!(c.knots.is_empty());
        assert!(c.poles.is_empty());
    }

    #[test]
    fn curve_tool2d_add_pole() {
        let mut c = CurveTool2d::new();
        c.add_pole(1.0, 2.0);
        c.add_pole(3.0, 4.0);
        assert_eq!(c.poles.len(), 2);
        assert_eq!(c.poles[0], [1.0, 2.0]);
        assert_eq!(c.poles[1], [3.0, 4.0]);
    }

    #[test]
    fn curve_tool2d_set_degree() {
        let mut c = CurveTool2d::new();
        c.set_degree(3);
        assert_eq!(c.degree, 3);
    }

    #[test]
    fn curve_tool2d_evaluate_no_poles_returns_origin() {
        let c = CurveTool2d::new();
        let pt = c.evaluate(0.5);
        assert_eq!(pt, [0.0, 0.0]);
    }

    #[test]
    fn curve_tool2d_evaluate_linear_interpolates() {
        // Linear (degree-1) B-spline between (0,0) and (1,1).
        // Flat knot vector: [0, 0, 1, 1].
        let c = CurveTool2d {
            degree: 1,
            knots: vec![0.0, 0.0, 1.0, 1.0],
            poles: vec![[0.0, 0.0], [1.0, 1.0]],
        };
        let mid = c.evaluate(0.5);
        assert!((mid[0] - 0.5).abs() < 1.0e-10, "x: {}", mid[0]);
        assert!((mid[1] - 0.5).abs() < 1.0e-10, "y: {}", mid[1]);
    }

    #[test]
    fn curve_tool2d_evaluate_at_start_end() {
        let c = CurveTool2d {
            degree: 1,
            knots: vec![0.0, 0.0, 1.0, 1.0],
            poles: vec![[2.0, 3.0], [5.0, 7.0]],
        };
        let start = c.evaluate(0.0);
        let end = c.evaluate(1.0);
        assert!((start[0] - 2.0).abs() < 1.0e-10);
        assert!((start[1] - 3.0).abs() < 1.0e-10);
        assert!((end[0] - 5.0).abs() < 1.0e-10);
        assert!((end[1] - 7.0).abs() < 1.0e-10);
    }

    #[test]
    fn curve_tool2d_default_equals_new() {
        let a = CurveTool2d::new();
        let b = CurveTool2d::default();
        assert_eq!(a.degree, b.degree);
        assert_eq!(a.poles.len(), b.poles.len());
        assert_eq!(a.knots.len(), b.knots.len());
    }

    // --- AdaptorCurve2d ---

    #[test]
    fn adaptor_curve2d_new() {
        let a = AdaptorCurve2d::new(0.0, 1.0);
        assert_eq!(a.start, 0.0);
        assert_eq!(a.end, 1.0);
    }

    #[test]
    fn adaptor_curve2d_length() {
        let a = AdaptorCurve2d::new(2.0, 5.0);
        assert!((a.length() - 3.0).abs() < 1.0e-12);
    }

    #[test]
    fn adaptor_curve2d_length_zero() {
        let a = AdaptorCurve2d::new(3.0, 3.0);
        assert!((a.length() - 0.0).abs() < 1.0e-12);
    }

    #[test]
    fn adaptor_curve2d_midpoint() {
        let a = AdaptorCurve2d::new(0.0, 2.0);
        assert!((a.midpoint() - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn adaptor_curve2d_midpoint_asymmetric() {
        let a = AdaptorCurve2d::new(1.0, 5.0);
        assert!((a.midpoint() - 3.0).abs() < 1.0e-12);
    }

    #[test]
    fn adaptor_curve2d_negative_range() {
        let a = AdaptorCurve2d::new(-3.0, 1.0);
        assert!((a.length() - 4.0).abs() < 1.0e-12);
        assert!((a.midpoint() - (-1.0)).abs() < 1.0e-12);
    }

    // --- convert_bspline_2d ---

    #[test]
    fn convert_bspline_2d_degree_preserved() {
        let poles = [[0.0, 0.0], [1.0, 1.0], [2.0, 0.0]];
        let c = convert_bspline_2d(&poles, 2);
        assert_eq!(c.degree, 2);
    }

    #[test]
    fn convert_bspline_2d_poles_preserved() {
        let poles = [[0.0, 0.0], [1.0, 2.0], [3.0, 1.0]];
        let c = convert_bspline_2d(&poles, 2);
        assert_eq!(c.poles.len(), 3);
        assert_eq!(c.poles[0], [0.0, 0.0]);
        assert_eq!(c.poles[2], [3.0, 1.0]);
    }

    #[test]
    fn convert_bspline_2d_knot_vector_clamped() {
        // Degree-2, 3 poles: knot count = 3 + 2 + 1 = 6.
        // Clamped: [0, 0, 0, 1, 1, 1].
        let poles = [[0.0, 0.0], [1.0, 1.0], [2.0, 0.0]];
        let c = convert_bspline_2d(&poles, 2);
        assert_eq!(c.knots.len(), 6);
        assert_eq!(c.knots[0], 0.0);
        assert_eq!(c.knots[5], 1.0);
    }

    #[test]
    fn convert_bspline_2d_evaluate_endpoints_match_poles() {
        let poles = [[0.0, 0.0], [1.0, 2.0], [2.0, 0.0]];
        let c = convert_bspline_2d(&poles, 2);
        let start = c.evaluate(0.0);
        let end = c.evaluate(1.0);
        assert!((start[0] - 0.0).abs() < 1.0e-10, "start x: {}", start[0]);
        assert!((start[1] - 0.0).abs() < 1.0e-10, "start y: {}", start[1]);
        assert!((end[0] - 2.0).abs() < 1.0e-10, "end x: {}", end[0]);
        assert!((end[1] - 0.0).abs() < 1.0e-10, "end y: {}", end[1]);
    }

    // --- sample_curve_2d ---

    #[test]
    fn sample_curve_2d_zero_samples() {
        let c = convert_bspline_2d(&[[0.0, 0.0], [1.0, 1.0]], 1);
        let s = sample_curve_2d(&c, 0);
        assert!(s.is_empty());
    }

    #[test]
    fn sample_curve_2d_one_sample() {
        let c = convert_bspline_2d(&[[0.0, 0.0], [1.0, 1.0]], 1);
        let s = sample_curve_2d(&c, 1);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn sample_curve_2d_count() {
        let c = convert_bspline_2d(&[[0.0, 0.0], [1.0, 1.0]], 1);
        let s = sample_curve_2d(&c, 5);
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn sample_curve_2d_linear_midpoint() {
        // Linear B-spline: sample at t=0.5 should be midpoint of poles.
        let c = CurveTool2d {
            degree: 1,
            knots: vec![0.0, 0.0, 1.0, 1.0],
            poles: vec![[0.0, 0.0], [2.0, 4.0]],
        };
        let s = sample_curve_2d(&c, 3); // t = 0, 0.5, 1
        assert!((s[1][0] - 1.0).abs() < 1.0e-10, "mid x: {}", s[1][0]);
        assert!((s[1][1] - 2.0).abs() < 1.0e-10, "mid y: {}", s[1][1]);
    }

    #[test]
    fn sample_curve_2d_endpoints_at_t0_and_t1() {
        let c = CurveTool2d {
            degree: 1,
            knots: vec![0.0, 0.0, 1.0, 1.0],
            poles: vec![[1.0, 2.0], [3.0, 4.0]],
        };
        let s = sample_curve_2d(&c, 4);
        assert!((s[0][0] - 1.0).abs() < 1.0e-10);
        assert!((s[3][0] - 3.0).abs() < 1.0e-10);
    }
}
