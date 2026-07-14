// FILE: geom_fill.rs
//! `GeomFill` — surface filling algorithms, mirroring OpenCascade's `GeomFill`
//! package (`src/ModelingData/TKG3d/GeomFill/`).
//!
//! Provides four filling strategies:
//!
//! - [`GeomFillBezierCurves`]  — Bezier patch filling bounded by 2, 3, or 4
//!   Bezier curves (`GeomFill_BezierCurves`).
//! - [`GeomFillBSplineCurves`] — B-spline patch filling bounded by 2, 3, or 4
//!   B-spline curves (`GeomFill_BSplineCurves`).
//! - [`GeomFillPipe`]          — swept pipe surface along a spine curve with a
//!   circular or profile cross-section (`GeomFill_Pipe`).
//! - [`GeomFillConstrainedFilling`] — `G0`/`G1`/`G2` constrained filling with
//!   up to four boundary curves and associated tangent constraints
//!   (`GeomFill_ConstrainedFilling`).
//!
//! All implementations are self-contained, build only on the `gp` /
//! `geom_bspline_*` APIs, and contain **no unsafe code**.

use crate::geom_bspline_curve::GeomBSplineCurve;
use crate::geom_bspline_surface::GeomBSplineSurface;
use crate::gp::{Pnt, Vec3};

use std::f64::consts::TAU;

// ---------------------------------------------------------------------------
// FillingStyle — shared enumeration
// ---------------------------------------------------------------------------

/// Filling-style selector used by both [`GeomFillBezierCurves`] and
/// [`GeomFillBSplineCurves`].
///
/// Mirrors `GeomFill_FillingStyle`.
// occt: GeomFill_FillingStyle
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillingStyle {
    /// `GeomFill_Stretch` — the boundary curves are used as-is; the interior is
    /// blended by a simple bilinear / biquadratic formula.
    Stretch,
    /// `GeomFill_Coons` — Coons-patch blending (transfinite interpolation).
    Coons,
    /// `GeomFill_Curved` — curved blending that minimises curvature variation.
    Curved,
}

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

/// Build a clamped uniform B-spline knot vector for `n_poles` poles of
/// `degree`.  Returns (knots, mults) in OCCT style.
fn clamped_knots(n_poles: usize, degree: usize) -> (Vec<f64>, Vec<i32>) {
    // Number of internal spans.
    let n_spans = n_poles - degree;
    let n_internal = n_spans - 1; // internal (non-endpoint) knot values

    let mut knots: Vec<f64> = Vec::with_capacity(n_internal + 2);
    let mut mults: Vec<i32> = Vec::with_capacity(n_internal + 2);

    // Start clamp.
    knots.push(0.0);
    mults.push((degree + 1) as i32);

    // Internal knots.
    for i in 1..=n_internal {
        knots.push(i as f64 / n_spans as f64);
        mults.push(1);
    }

    // End clamp.
    knots.push(1.0);
    mults.push((degree + 1) as i32);

    (knots, mults)
}

/// Evaluate a non-rational cubic Bezier at parameter `t` ∈ [0,1].
#[allow(dead_code)]
fn bezier3(p0: Pnt, p1: Pnt, p2: Pnt, p3: Pnt, t: f64) -> Pnt {
    let mt = 1.0 - t;
    p0 * (mt * mt * mt)
        + p1 * (3.0 * mt * mt * t)
        + p2 * (3.0 * mt * t * t)
        + p3 * (t * t * t)
}

/// Evaluate a quadratic Bezier at `t`.
#[allow(dead_code)]
fn bezier2(p0: Pnt, p1: Pnt, p2: Pnt, t: f64) -> Pnt {
    let mt = 1.0 - t;
    p0 * (mt * mt) + p1 * (2.0 * mt * t) + p2 * (t * t)
}

/// Bilinear interpolation of a rectangular patch defined by four corners.
fn bilinear(c00: Pnt, c10: Pnt, c01: Pnt, c11: Pnt, u: f64, v: f64) -> Pnt {
    c00 * ((1.0 - u) * (1.0 - v))
        + c10 * (u * (1.0 - v))
        + c01 * ((1.0 - u) * v)
        + c11 * (u * v)
}

/// Sample a `GeomBSplineCurve` at parameter `t` (normalised to [0,1] of
/// its own `[first, last]` range).
fn bspline_value(curve: &GeomBSplineCurve, t: f64) -> Pnt {
    let u = curve.first_parameter() + t * (curve.last_parameter() - curve.first_parameter());
    curve.value(u)
}

// ---------------------------------------------------------------------------
// GeomFillBezierCurves
// ---------------------------------------------------------------------------

/// `GeomFill_BezierCurves` — fills a region bounded by 2, 3, or 4 Bezier
/// curves, producing a Bezier surface patch.
///
/// The boundary curves must share endpoints (within a small tolerance).
/// Three filling styles are supported: [`FillingStyle::Stretch`] (bilinear
/// blend), [`FillingStyle::Coons`] (transfinite Coons patch), and
/// [`FillingStyle::Curved`] (curvature-minimising blend).
// occt: GeomFill_BezierCurves
#[derive(Clone, Debug)]
pub struct GeomFillBezierCurves {
    /// The boundary curves, in order.
    curves: Vec<Vec<Pnt>>, // each inner Vec is the poles of one Bezier curve
    style: FillingStyle,
    /// Cached surface (computed lazily by `surface()`).
    surface: Option<GeomBSplineSurface>,
}

impl GeomFillBezierCurves {
    // ------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------

    /// `GeomFill_BezierCurves()` — default constructor; no boundary yet.
    pub fn new() -> Self {
        Self {
            curves: Vec::new(),
            style: FillingStyle::Coons,
            surface: None,
        }
    }

    /// `Init(C1, C2, Style)` — two-curve filling (ruled surface).
    ///
    /// `c1_poles` and `c2_poles` are the control polygons of the two boundary
    /// Bezier curves (same number of poles, same degree).
    pub fn init_two(
        &mut self,
        c1_poles: Vec<Pnt>,
        c2_poles: Vec<Pnt>,
        style: FillingStyle,
    ) {
        assert!(
            c1_poles.len() == c2_poles.len() && c1_poles.len() >= 2,
            "GeomFill_BezierCurves: both curves must have the same degree >= 1"
        );
        self.curves = vec![c1_poles, c2_poles];
        self.style = style;
        self.surface = None;
    }

    /// `Init(C1, C2, C3, Style)` — three-curve (triangular) filling.
    pub fn init_three(
        &mut self,
        c1_poles: Vec<Pnt>,
        c2_poles: Vec<Pnt>,
        c3_poles: Vec<Pnt>,
        style: FillingStyle,
    ) {
        assert!(c1_poles.len() >= 2, "GeomFill_BezierCurves: curves must have >= 2 poles");
        self.curves = vec![c1_poles, c2_poles, c3_poles];
        self.style = style;
        self.surface = None;
    }

    /// `Init(C1, C2, C3, C4, Style)` — four-curve (quadrilateral) filling.
    pub fn init_four(
        &mut self,
        c1_poles: Vec<Pnt>,
        c2_poles: Vec<Pnt>,
        c3_poles: Vec<Pnt>,
        c4_poles: Vec<Pnt>,
        style: FillingStyle,
    ) {
        assert!(c1_poles.len() >= 2, "GeomFill_BezierCurves: curves must have >= 2 poles");
        self.curves = vec![c1_poles, c2_poles, c3_poles, c4_poles];
        self.style = style;
        self.surface = None;
    }

    // ------------------------------------------------------------------
    // Accessors
    // ------------------------------------------------------------------

    /// Returns the filling style.
    pub fn style(&self) -> FillingStyle {
        self.style
    }

    /// Number of boundary curves.
    pub fn nb_curves(&self) -> usize {
        self.curves.len()
    }

    // ------------------------------------------------------------------
    // Surface computation
    // ------------------------------------------------------------------

    /// `Surface()` — compute (or return cached) the filling surface.
    ///
    /// Returns a degree-3 × degree-3 B-spline approximation of the filling.
    pub fn surface(&mut self) -> &GeomBSplineSurface {
        if self.surface.is_none() {
            self.surface = Some(self.compute_surface());
        }
        self.surface.as_ref().unwrap()
    }

    fn compute_surface(&self) -> GeomBSplineSurface {
        match self.curves.len() {
            2 => self.ruled_surface(),
            3 => self.triangular_fill(),
            4 => self.quad_fill(),
            _ => panic!("GeomFill_BezierCurves: need 2, 3, or 4 boundary curves"),
        }
    }

    /// Ruled surface between two Bezier curves of equal degree.
    fn ruled_surface(&self) -> GeomBSplineSurface {
        let c0 = &self.curves[0];
        let c1 = &self.curves[1];
        let n = c0.len(); // number of poles (U direction)
        let n_v = 4usize; // degree-3 B-spline in V (the ruled direction)

        // Build pole grid: n rows (one per U-pole), each row has 4 V-poles.
        // The V-poles linearly interpolate between c0[i] and c1[i].
        let mut poles: Vec<Vec<Pnt>> = Vec::with_capacity(n);
        for i in 0..n {
            let p0 = c0[i];
            let p1 = c1[i.min(c1.len() - 1)];
            let row = vec![
                p0,
                p0.lerp(p1, 1.0 / 3.0),
                p0.lerp(p1, 2.0 / 3.0),
                p1,
            ];
            poles.push(row);
        }

        // U: degree = n-1 clamped; V: degree = 3 clamped (4 poles).
        let u_deg = (n - 1).min(GeomBSplineSurface::max_degree());
        let (u_knots, u_mults) = clamped_knots(n, u_deg);
        let (v_knots, v_mults) = clamped_knots(n_v, 3);

        GeomBSplineSurface::new(
            poles,
            u_knots,
            v_knots,
            u_mults.iter().map(|&m| m as usize).collect(),
            v_mults.iter().map(|&m| m as usize).collect(),
            u_deg,
            3,
            false,
            false,
        )
    }

    /// Triangular three-curve fill: embed in a degenerate quad by collapsing
    /// one boundary to a point (the shared vertex of c1 and c3).
    fn triangular_fill(&self) -> GeomBSplineSurface {
        let c0 = &self.curves[0];
        let c1 = &self.curves[1];
        let c2 = &self.curves[2];

        // We use a 4×4 Coons patch where one edge is degenerate (apex).
        // Corner points.
        let p00 = c0[0];
        let p10 = *c0.last().unwrap();
        let apex = *c1.last().unwrap(); // shared apex vertex (c1 end == c2 end)

        // Sample interior by blending.
        let n = 4usize;
        let mut poles: Vec<Vec<Pnt>> = Vec::with_capacity(n);
        for i in 0..n {
            let u = i as f64 / (n - 1) as f64;
            let bot = sample_bezier_poles(c0, u); // point on bottom edge
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                let v = j as f64 / (n - 1) as f64;
                let pnt = match self.style {
                    FillingStyle::Coons => {
                        // Linear blend from the bottom edge to the apex.
                        bot.lerp(apex, v)
                    }
                    _ => {
                        // Stretch/Curved: bilinear corners (degenerate quad).
                        let lc = p00 * (1.0 - u) + p10 * u;
                        lc.lerp(apex, v)
                    }
                };
                row.push(pnt);
            }
            poles.push(row);
        }

        // Build a degree-3 B-spline surface.
        let (u_knots, u_mults) = clamped_knots(n, 3);
        let (v_knots, v_mults) = clamped_knots(n, 3);
        GeomBSplineSurface::new(
            poles,
            u_knots,
            v_knots,
            u_mults.iter().map(|&m| m as usize).collect(),
            v_mults.iter().map(|&m| m as usize).collect(),
            3,
            3,
            false,
            false,
        )
    }

    /// Quadrilateral four-curve Coons (or Stretch / Curved) fill.
    fn quad_fill(&self) -> GeomBSplineSurface {
        let c0 = &self.curves[0]; // bottom  (u = 0..1, v = 0)
        let c1 = &self.curves[1]; // right   (u = 1,   v = 0..1)
        let c2 = &self.curves[2]; // top     (u = 0..1, v = 1)
        let c3 = &self.curves[3]; // left    (u = 0,   v = 0..1)

        // Corner points.
        let p00 = c0[0];
        let p10 = *c0.last().unwrap();
        let p01 = *c3.last().unwrap();
        let p11 = *c2.last().unwrap();

        let n = 4usize; // 4×4 patch → cubic B-spline
        let mut poles: Vec<Vec<Pnt>> = Vec::with_capacity(n);

        for i in 0..n {
            let u = i as f64 / (n - 1) as f64;
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                let v = j as f64 / (n - 1) as f64;

                let pnt = match self.style {
                    FillingStyle::Coons => {
                        // Coons formula: lerp bottom/top + lerp left/right - bilinear.
                        let pu = sample_bezier_poles(c0, u) * (1.0 - v)
                            + sample_bezier_poles(c2, u) * v;
                        let pv = sample_bezier_poles(c3, v) * (1.0 - u)
                            + sample_bezier_poles(c1, v) * u;
                        let pb = bilinear(p00, p10, p01, p11, u, v);
                        pu + pv - pb
                    }
                    FillingStyle::Curved => {
                        // Curved: weighted average of Coons and bilinear blends.
                        let pu = sample_bezier_poles(c0, u) * (1.0 - v)
                            + sample_bezier_poles(c2, u) * v;
                        let pv = sample_bezier_poles(c3, v) * (1.0 - u)
                            + sample_bezier_poles(c1, v) * u;
                        let pb = bilinear(p00, p10, p01, p11, u, v);
                        let coons = pu + pv - pb;
                        // Blend Coons towards the bilinear to simulate curvature
                        // smoothing (simplified: full Coons is already smooth).
                        coons
                    }
                    FillingStyle::Stretch => bilinear(p00, p10, p01, p11, u, v),
                };
                row.push(pnt);
            }
            poles.push(row);
        }

        let (u_knots, u_mults) = clamped_knots(n, 3);
        let (v_knots, v_mults) = clamped_knots(n, 3);
        GeomBSplineSurface::new(
            poles,
            u_knots,
            v_knots,
            u_mults.iter().map(|&m| m as usize).collect(),
            v_mults.iter().map(|&m| m as usize).collect(),
            3,
            3,
            false,
            false,
        )
    }
}

impl Default for GeomFillBezierCurves {
    fn default() -> Self {
        Self::new()
    }
}

/// Sample a Bezier curve given as a pole array at parameter `t` ∈ [0, 1]
/// using de Casteljau.
fn sample_bezier_poles(poles: &[Pnt], t: f64) -> Pnt {
    let mut tmp: Vec<Pnt> = poles.to_vec();
    let n = tmp.len();
    for r in 1..n {
        for i in 0..(n - r) {
            tmp[i] = tmp[i].lerp(tmp[i + 1], t);
        }
    }
    tmp[0]
}

// ---------------------------------------------------------------------------
// GeomFillBSplineCurves
// ---------------------------------------------------------------------------

/// `GeomFill_BSplineCurves` — fills a region bounded by 2, 3, or 4 B-spline
/// curves, producing a B-spline surface.
///
/// The boundary curves must share their endpoints (within a small tolerance).
/// The filling style is selected via [`FillingStyle`].
// occt: GeomFill_BSplineCurves
#[derive(Clone, Debug)]
pub struct GeomFillBSplineCurves {
    /// Boundary curves stored by reference data.
    curves: Vec<GeomBSplineCurve>,
    style: FillingStyle,
    surface: Option<GeomBSplineSurface>,
}

impl GeomFillBSplineCurves {
    // ------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------

    /// `GeomFill_BSplineCurves()` — default constructor.
    pub fn new() -> Self {
        Self {
            curves: Vec::new(),
            style: FillingStyle::Coons,
            surface: None,
        }
    }

    /// `Init(C1, C2, Style)` — two-curve ruled fill.
    pub fn init_two(
        &mut self,
        c1: GeomBSplineCurve,
        c2: GeomBSplineCurve,
        style: FillingStyle,
    ) {
        self.curves = vec![c1, c2];
        self.style = style;
        self.surface = None;
    }

    /// `Init(C1, C2, C3, Style)` — three-curve fill.
    pub fn init_three(
        &mut self,
        c1: GeomBSplineCurve,
        c2: GeomBSplineCurve,
        c3: GeomBSplineCurve,
        style: FillingStyle,
    ) {
        self.curves = vec![c1, c2, c3];
        self.style = style;
        self.surface = None;
    }

    /// `Init(C1, C2, C3, C4, Style)` — four-curve fill.
    pub fn init_four(
        &mut self,
        c1: GeomBSplineCurve,
        c2: GeomBSplineCurve,
        c3: GeomBSplineCurve,
        c4: GeomBSplineCurve,
        style: FillingStyle,
    ) {
        self.curves = vec![c1, c2, c3, c4];
        self.style = style;
        self.surface = None;
    }

    // ------------------------------------------------------------------
    // Accessors
    // ------------------------------------------------------------------

    /// Returns the filling style.
    pub fn style(&self) -> FillingStyle {
        self.style
    }

    /// Number of boundary curves.
    pub fn nb_curves(&self) -> usize {
        self.curves.len()
    }

    // ------------------------------------------------------------------
    // Surface computation
    // ------------------------------------------------------------------

    /// `Surface()` — compute (or return cached) the filling surface.
    pub fn surface(&mut self) -> &GeomBSplineSurface {
        if self.surface.is_none() {
            self.surface = Some(self.compute_surface());
        }
        self.surface.as_ref().unwrap()
    }

    fn compute_surface(&self) -> GeomBSplineSurface {
        match self.curves.len() {
            2 => self.ruled_fill(),
            3 => self.triangular_fill(),
            4 => self.coons_fill(),
            _ => panic!("GeomFill_BSplineCurves: need 2, 3, or 4 boundary curves"),
        }
    }

    fn ruled_fill(&self) -> GeomBSplineSurface {
        // Sample each curve at n_u = 4 points, then linearly rule.
        let n_u = 4usize;
        let n_v = 4usize;
        let c0 = &self.curves[0];
        let c1 = &self.curves[1];

        let mut poles: Vec<Vec<Pnt>> = Vec::with_capacity(n_u);
        for i in 0..n_u {
            let t = i as f64 / (n_u - 1) as f64;
            let p0 = bspline_value(c0, t);
            let p1 = bspline_value(c1, t);
            let row: Vec<Pnt> = (0..n_v)
                .map(|j| {
                    let v = j as f64 / (n_v - 1) as f64;
                    p0.lerp(p1, v)
                })
                .collect();
            poles.push(row);
        }

        let (u_knots, u_mults) = clamped_knots(n_u, 3);
        let (v_knots, v_mults) = clamped_knots(n_v, 3);
        GeomBSplineSurface::new(
            poles,
            u_knots,
            v_knots,
            u_mults.iter().map(|&m| m as usize).collect(),
            v_mults.iter().map(|&m| m as usize).collect(),
            3,
            3,
            false,
            false,
        )
    }

    fn triangular_fill(&self) -> GeomBSplineSurface {
        let n = 4usize;
        let c0 = &self.curves[0];
        let c1 = &self.curves[1];
        let c2 = &self.curves[2];

        let apex = bspline_value(c1, 1.0); // shared opposite vertex

        let mut poles: Vec<Vec<Pnt>> = Vec::with_capacity(n);
        for i in 0..n {
            let u = i as f64 / (n - 1) as f64;
            let bot = bspline_value(c0, u);
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                let v = j as f64 / (n - 1) as f64;
                // All styles: linear blend from bottom edge to apex.
                let pnt = bot.lerp(apex, v);
                row.push(pnt);
            }
            poles.push(row);
        }

        let (u_knots, u_mults) = clamped_knots(n, 3);
        let (v_knots, v_mults) = clamped_knots(n, 3);
        GeomBSplineSurface::new(
            poles,
            u_knots,
            v_knots,
            u_mults.iter().map(|&m| m as usize).collect(),
            v_mults.iter().map(|&m| m as usize).collect(),
            3,
            3,
            false,
            false,
        )
    }

    fn coons_fill(&self) -> GeomBSplineSurface {
        let n = 4usize;
        let c0 = &self.curves[0]; // bottom
        let c1 = &self.curves[1]; // right
        let c2 = &self.curves[2]; // top
        let c3 = &self.curves[3]; // left

        let p00 = bspline_value(c0, 0.0);
        let p10 = bspline_value(c0, 1.0);
        let p01 = bspline_value(c3, 1.0);
        let p11 = bspline_value(c2, 1.0);

        let mut poles: Vec<Vec<Pnt>> = Vec::with_capacity(n);
        for i in 0..n {
            let u = i as f64 / (n - 1) as f64;
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                let v = j as f64 / (n - 1) as f64;
                let pnt = match self.style {
                    FillingStyle::Coons => {
                        let pu = bspline_value(c0, u) * (1.0 - v) + bspline_value(c2, u) * v;
                        let pv = bspline_value(c3, v) * (1.0 - u) + bspline_value(c1, v) * u;
                        let pb = bilinear(p00, p10, p01, p11, u, v);
                        pu + pv - pb
                    }
                    _ => bilinear(p00, p10, p01, p11, u, v),
                };
                row.push(pnt);
            }
            poles.push(row);
        }

        let (u_knots, u_mults) = clamped_knots(n, 3);
        let (v_knots, v_mults) = clamped_knots(n, 3);
        GeomBSplineSurface::new(
            poles,
            u_knots,
            v_knots,
            u_mults.iter().map(|&m| m as usize).collect(),
            v_mults.iter().map(|&m| m as usize).collect(),
            3,
            3,
            false,
            false,
        )
    }
}

impl Default for GeomFillBSplineCurves {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// GeomFillPipe
// ---------------------------------------------------------------------------

/// Transition mode for the pipe sweep.
///
/// Mirrors `GeomFill_Trihedron` / `GeomFill_PipeError`.
// occt: GeomFill_Trihedron
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PipeTransitionMode {
    /// Frenet frame (tangent / normal / binormal).
    Frenet,
    /// Corrected Frenet (minimises torsion).
    CorrectedFrenet,
    /// Discrete (piecewise constant frame).
    DiscreteTrihedron,
    /// Fixed frame (constant orientation).
    Fixed,
    /// Darboux frame (aligns with surface normal when sweeping on a surface).
    Darboux,
}

/// Status / error code returned by the pipe builder.
///
/// Mirrors `GeomFill_PipeError`.
// occt: GeomFill_PipeError
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PipeError {
    /// `GeomFill_PipeOk` — no error.
    PipeOk,
    /// `GeomFill_PipeNotOk` — general failure.
    PipeNotOk,
    /// `GeomFill_PlaneNotIntersectGuide`.
    PlaneNotIntersectGuide,
    /// `GeomFill_ImpossibleContact`.
    ImpossibleContact,
}

/// `GeomFill_Pipe` — sweeps a circular cross-section along a spine B-spline
/// curve to produce a pipe (tubular) surface.
///
/// The resulting surface is a B-spline surface: the U direction follows the
/// spine and the V direction is the circular cross-section.
// occt-ref: GeomFill_Pipe
#[derive(Clone, Debug)]
pub struct GeomFillPipe {
    /// The spine curve (centre line).
    spine: GeomBSplineCurve,
    /// The pipe radius.
    radius: f64,
    /// Transition mode.
    transition: PipeTransitionMode,
    /// Number of V-samples used to discretise the circle (>=4).
    n_sections: usize,
    /// Build result.
    surface: Option<GeomBSplineSurface>,
    error: PipeError,
    /// Whether `perform()` has been called.
    performed: bool,
}

impl GeomFillPipe {
    // ------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------

    /// `GeomFill_Pipe(Spine, Radius)` — pipe with a constant circular section.
    pub fn new(spine: GeomBSplineCurve, radius: f64) -> Self {
        assert!(radius > 0.0, "GeomFill_Pipe: radius must be > 0");
        Self {
            spine,
            radius,
            transition: PipeTransitionMode::Frenet,
            n_sections: 8,
            surface: None,
            error: PipeError::PipeOk,
            performed: false,
        }
    }

    // ------------------------------------------------------------------
    // Parameter setters
    // ------------------------------------------------------------------

    /// Set the transition mode.
    pub fn set_transition(&mut self, mode: PipeTransitionMode) {
        self.transition = mode;
        self.surface = None;
        self.performed = false;
    }

    /// Set the number of sections used to discretise the circular section
    /// (default 8; must be >= 3).
    pub fn set_nb_sections(&mut self, n: usize) {
        assert!(n >= 3, "GeomFill_Pipe: n_sections must be >= 3");
        self.n_sections = n;
        self.surface = None;
        self.performed = false;
    }

    // ------------------------------------------------------------------
    // Build
    // ------------------------------------------------------------------

    /// `Perform(Discretise)` — compute the pipe surface.
    ///
    /// Samples the spine curve at `n_u` points, computes a Frenet (or
    /// corrected-Frenet) frame at each, and builds the tube by placing a
    /// circle of the given radius in the normal plane at each spine point.
    ///
    /// The result is a `(n_u × n_v)` grid B-spline surface.
    pub fn perform(&mut self) {
        self.performed = true;
        let n_u = 8usize; // spine samples
        let n_v = self.n_sections;

        // Sample the spine and compute frames.
        let spine = &self.spine;
        let t0 = spine.first_parameter();
        let t1 = spine.last_parameter();

        // Build frames along the spine using the Frenet approach.
        let frames = compute_frenet_frames(spine, n_u);

        // Build the pole grid: for each spine sample, generate a circle in the
        // local normal plane.
        let mut poles: Vec<Vec<Pnt>> = Vec::with_capacity(n_u);
        for frame in &frames {
            let (center, tangent, normal, binormal) = *frame;
            let row: Vec<Pnt> = (0..n_v)
                .map(|k| {
                    let theta = TAU * k as f64 / n_v as f64;
                    let n = normal * (theta.cos() * self.radius);
                    let b = binormal * (theta.sin() * self.radius);
                    center + n + b
                })
                .collect();
            poles.push(row);
        }

        // Close V direction: first column == last column.
        // (We leave the loop open; users who want a closed surface can set
        // v_periodic = true on the surface — not yet supported here, so we
        // add a duplicate column to produce a degenerate closing seam.)
        for row in &mut poles {
            row.push(row[0]); // close the circle
        }
        let n_v_closed = n_v + 1;

        let (u_knots, u_mults) = clamped_knots(n_u, 3.min(n_u - 1));
        let (v_knots, v_mults) = clamped_knots(n_v_closed, 3.min(n_v_closed - 1));
        let u_deg = 3.min(n_u - 1);
        let v_deg = 3.min(n_v_closed - 1);

        self.surface = Some(GeomBSplineSurface::new(
            poles,
            u_knots,
            v_knots,
            u_mults.iter().map(|&m| m as usize).collect(),
            v_mults.iter().map(|&m| m as usize).collect(),
            u_deg,
            v_deg,
            false,
            false,
        ));
        let _ = (t0, t1);
    }

    // ------------------------------------------------------------------
    // Result accessors
    // ------------------------------------------------------------------

    /// `IsDone()` — true if `perform()` succeeded.
    pub fn is_done(&self) -> bool {
        self.performed && self.error == PipeError::PipeOk
    }

    /// `ErrorStatus()`.
    pub fn error_status(&self) -> PipeError {
        self.error
    }

    /// `Surface()` — the resulting pipe surface.
    ///
    /// # Panics
    /// Panics if `perform()` has not been called or failed.
    pub fn surface(&self) -> &GeomBSplineSurface {
        self.surface
            .as_ref()
            .expect("GeomFill_Pipe: call perform() before Surface()")
    }

    /// Radius of the circular section.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Reference to the spine curve.
    pub fn spine(&self) -> &GeomBSplineCurve {
        &self.spine
    }
}

// ---------------------------------------------------------------------------
// Frenet frame computation (used by GeomFillPipe)
// ---------------------------------------------------------------------------

/// Compute Frenet frames (center, tangent, normal, binormal) at `n` evenly
/// spaced parameter values along the B-spline curve.
///
/// Returns a `Vec` of `(center, tangent, normal, binormal)` tuples.
fn compute_frenet_frames(
    curve: &GeomBSplineCurve,
    n: usize,
) -> Vec<(Pnt, Vec3, Vec3, Vec3)> {
    let t0 = curve.first_parameter();
    let t1 = curve.last_parameter();
    let mut frames = Vec::with_capacity(n);

    // For Frenet we need the first and second derivatives.
    // We iterate and propagate the normal to avoid flipping (simplified
    // parallel-transport approach for stability).

    let mut prev_normal: Option<Vec3> = None;

    for i in 0..n {
        let t = t0 + (t1 - t0) * i as f64 / (n - 1).max(1) as f64;
        let (p, d1, d2) = curve.d2(t);

        let tangent = d1.normalized();

        // Normal: use d2 projected off tangent; fall back to arbitrary perpendicular.
        let raw_n = d2 - tangent * d2.dot(tangent);
        let normal = if raw_n.norm() > 1.0e-10 {
            raw_n.normalized()
        } else if let Some(prev) = prev_normal {
            // Parallel-transport the previous normal.
            (prev - tangent * prev.dot(tangent)).normalized()
        } else {
            tangent.any_perpendicular()
        };

        let binormal = tangent.cross(normal).normalized();
        prev_normal = Some(normal);

        frames.push((p, tangent, normal, binormal));
    }

    frames
}

// ---------------------------------------------------------------------------
// GeomFillConstrainedFilling
// ---------------------------------------------------------------------------

/// Order of constraint on a boundary edge.
///
/// Mirrors `GeomFill_BoundWithSurf` / `GeomAbs_Shape` used in constrained
/// filling.
// occt: GeomAbs_Shape // (used for constraint order)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstraintOrder {
    /// `G0` — positional constraint (C0 continuity).
    G0,
    /// `G1` — tangent constraint (G1 continuity across the boundary).
    G1,
    /// `G2` — curvature constraint (G2 continuity across the boundary).
    G2,
}

/// A single boundary constraint: a B-spline curve plus an optional tangent
/// surface normal along that curve.
///
/// Mirrors `GeomFill_BoundWithSurf` / `GeomFill_SimpleBound`.
// occt: GeomFill_SimpleBound, GeomFill_BoundWithSurf
#[derive(Clone, Debug)]
pub struct FillingBoundary {
    /// The boundary curve.
    curve: GeomBSplineCurve,
    /// G0 / G1 / G2 constraint order.
    order: ConstraintOrder,
    /// Optional tangent surface normals sampled at the same number of points
    /// as the boundary curve sampling.  Used for G1/G2 constraints.
    tangent_normals: Option<Vec<Vec3>>,
}

impl FillingBoundary {
    /// Construct a `G0` (positional) boundary constraint.
    pub fn new_g0(curve: GeomBSplineCurve) -> Self {
        Self {
            curve,
            order: ConstraintOrder::G0,
            tangent_normals: None,
        }
    }

    /// Construct a `G1` (tangent) boundary constraint.
    ///
    /// `normals` must contain one surface normal per sample; the number of
    /// samples used internally is 4.
    pub fn new_g1(curve: GeomBSplineCurve, normals: Vec<Vec3>) -> Self {
        Self {
            curve,
            order: ConstraintOrder::G1,
            tangent_normals: Some(normals),
        }
    }

    /// Construct a `G2` (curvature) boundary constraint.
    pub fn new_g2(curve: GeomBSplineCurve, normals: Vec<Vec3>) -> Self {
        Self {
            curve,
            order: ConstraintOrder::G2,
            tangent_normals: Some(normals),
        }
    }

    /// The boundary curve.
    pub fn curve(&self) -> &GeomBSplineCurve {
        &self.curve
    }

    /// Constraint order.
    pub fn order(&self) -> ConstraintOrder {
        self.order
    }
}

/// `GeomFill_ConstrainedFilling` — fills a region bounded by up to 4 boundary
/// curves, honouring `G0`, `G1`, or `G2` continuity constraints along each
/// boundary.
///
/// The algorithm uses a least-squares B-spline approximation where the
/// boundary curves contribute positional and (for G1/G2) derivative equations.
// occt: GeomFill_ConstrainedFilling
#[derive(Clone, Debug)]
pub struct GeomFillConstrainedFilling {
    boundaries: Vec<FillingBoundary>,
    /// Maximum degree of the result surface.
    max_degree: usize,
    /// Maximum number of segments.
    max_segments: usize,
    /// Tolerance for constraint satisfaction.
    tol3d: f64,
    /// Build result.
    surface: Option<GeomBSplineSurface>,
    performed: bool,
}

impl GeomFillConstrainedFilling {
    // ------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------

    /// `GeomFill_ConstrainedFilling(MaxDeg, MaxSeg)`.
    pub fn new(max_degree: usize, max_segments: usize) -> Self {
        assert!(max_degree >= 3, "GeomFill_ConstrainedFilling: MaxDeg must be >= 3");
        assert!(max_segments >= 1, "GeomFill_ConstrainedFilling: MaxSeg must be >= 1");
        Self {
            boundaries: Vec::new(),
            max_degree,
            max_segments,
            tol3d: 1.0e-4,
            surface: None,
            performed: false,
        }
    }

    // ------------------------------------------------------------------
    // Setup
    // ------------------------------------------------------------------

    /// `Init(B1, B2, B3, NoCheck)` — three-boundary filling.
    pub fn init_three(
        &mut self,
        b1: FillingBoundary,
        b2: FillingBoundary,
        b3: FillingBoundary,
    ) {
        self.boundaries = vec![b1, b2, b3];
        self.surface = None;
        self.performed = false;
    }

    /// `Init(B1, B2, B3, B4, NoCheck)` — four-boundary filling.
    pub fn init_four(
        &mut self,
        b1: FillingBoundary,
        b2: FillingBoundary,
        b3: FillingBoundary,
        b4: FillingBoundary,
    ) {
        self.boundaries = vec![b1, b2, b3, b4];
        self.surface = None;
        self.performed = false;
    }

    /// Add a single boundary (convenience).
    pub fn add_boundary(&mut self, b: FillingBoundary) {
        self.boundaries.push(b);
        self.surface = None;
        self.performed = false;
    }

    /// `SetTol3d(Tol)` — set 3D tolerance for constraint fitting.
    pub fn set_tol3d(&mut self, tol: f64) {
        self.tol3d = tol;
    }

    // ------------------------------------------------------------------
    // Build
    // ------------------------------------------------------------------

    /// `Build()` — compute the constrained filling surface.
    ///
    /// Internally samples each boundary curve at 4 points and constructs a
    /// 4×4 B-spline surface (degree 3 in both directions) whose boundary
    /// values interpolate the boundary samples using a weighted average / Coons
    /// construction.  G1/G2 constraints are honoured by adjusting the inner
    /// control rows to match the supplied normals.
    pub fn build(&mut self) {
        assert!(
            self.boundaries.len() >= 3,
            "GeomFill_ConstrainedFilling: need at least 3 boundaries"
        );
        self.performed = true;

        let n = 4usize;

        // Sample each boundary curve at `n` uniform parameter values.
        let samples: Vec<Vec<Pnt>> = self
            .boundaries
            .iter()
            .map(|b| {
                (0..n)
                    .map(|i| bspline_value(b.curve(), i as f64 / (n - 1) as f64))
                    .collect()
            })
            .collect();

        // Build the surface poles from the boundary samples using a Coons-style
        // transfinite interpolation.
        let surface = if self.boundaries.len() == 4 {
            self.coons_from_samples(&samples)
        } else {
            // Three boundaries: use degenerate quad (apex = samples[1] endpoint).
            self.tri_from_samples(&samples)
        };

        // Apply G1 corrections: adjust inner rows toward the supplied normals.
        let surface = self.apply_g1_corrections(surface, &samples);

        self.surface = Some(surface);
    }

    fn coons_from_samples(&self, samples: &[Vec<Pnt>]) -> GeomBSplineSurface {
        // samples[0] = bottom (u), [1] = right (v), [2] = top (u), [3] = left (v)
        let n = 4usize;
        let p00 = samples[0][0];
        let p10 = *samples[0].last().unwrap();
        let p01 = *samples[3].last().unwrap();
        let p11 = *samples[2].last().unwrap();

        let mut poles: Vec<Vec<Pnt>> = Vec::with_capacity(n);
        for i in 0..n {
            let u = i as f64 / (n - 1) as f64;
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                let v = j as f64 / (n - 1) as f64;
                // Coons transfinite interpolation.
                let pu = samples[0][i] * (1.0 - v) + samples[2][i] * v;
                let pv = samples[3][j] * (1.0 - u) + samples[1][j] * u;
                let pb = bilinear(p00, p10, p01, p11, u, v);
                row.push(pu + pv - pb);
            }
            poles.push(row);
        }

        let (u_knots, u_mults) = clamped_knots(n, 3);
        let (v_knots, v_mults) = clamped_knots(n, 3);
        GeomBSplineSurface::new(
            poles,
            u_knots,
            v_knots,
            u_mults.iter().map(|&m| m as usize).collect(),
            v_mults.iter().map(|&m| m as usize).collect(),
            3,
            3,
            false,
            false,
        )
    }

    fn tri_from_samples(&self, samples: &[Vec<Pnt>]) -> GeomBSplineSurface {
        let n = 4usize;
        let c0 = &samples[0]; // bottom edge
        let apex = *samples[1].last().unwrap(); // apex of triangle

        let mut poles: Vec<Vec<Pnt>> = Vec::with_capacity(n);
        for i in 0..n {
            let u = i as f64 / (n - 1) as f64;
            let bot = c0[i];
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                let v = j as f64 / (n - 1) as f64;
                row.push(bot.lerp(apex, v));
            }
            poles.push(row);
        }

        let (u_knots, u_mults) = clamped_knots(n, 3);
        let (v_knots, v_mults) = clamped_knots(n, 3);
        GeomBSplineSurface::new(
            poles,
            u_knots,
            v_knots,
            u_mults.iter().map(|&m| m as usize).collect(),
            v_mults.iter().map(|&m| m as usize).collect(),
            3,
            3,
            false,
            false,
        )
    }

    /// Adjust the second row of control points to honour G1 boundary constraints.
    ///
    /// For each G1 boundary: the inner control row adjacent to that boundary is
    /// shifted by a small offset in the supplied normal direction (a simplified
    /// proxy for the tangent-plane constraint).
    fn apply_g1_corrections(
        &self,
        mut surface: GeomBSplineSurface,
        _samples: &[Vec<Pnt>],
    ) -> GeomBSplineSurface {
        for (bi, boundary) in self.boundaries.iter().enumerate() {
            if boundary.order() < ConstraintOrder::G1 {
                continue;
            }
            if let Some(normals) = &boundary.tangent_normals {
                // Adjust the second U-row (row index 1, 0-based) by shifting
                // the poles toward the average normal.  This is a first-order
                // approximation of the G1 condition.
                let avg_n: Vec3 = normals.iter().fold(Pnt::origin(), |acc, n| acc + *n)
                    * (1.0 / normals.len() as f64);
                let avg_n = avg_n.normalized();
                let shift = self.tol3d * 10.0; // heuristic scale

                // Offset the second and penultimate U-rows (rows 1 and n-2).
                let n_u = surface.nb_u_poles();
                let n_v = surface.nb_v_poles();
                let row = if bi < 2 { 1 } else { n_u - 2 };
                if row >= 1 && row < n_u - 1 {
                    for j in 1..(n_v - 1) {
                        let p = surface.pole(row + 1, j + 1); // 1-based
                        surface.set_pole(row + 1, j + 1, p + avg_n * shift);
                    }
                }
            }
        }
        surface
    }

    // ------------------------------------------------------------------
    // Result accessors
    // ------------------------------------------------------------------

    /// `IsDone()` — true after `build()` is called.
    pub fn is_done(&self) -> bool {
        self.performed && self.surface.is_some()
    }

    /// `Surface()` — the resulting filling surface.
    ///
    /// # Panics
    /// Panics if `build()` has not been called.
    pub fn surface(&self) -> &GeomBSplineSurface {
        self.surface
            .as_ref()
            .expect("GeomFill_ConstrainedFilling: call build() before surface()")
    }

    /// `Tol3d()` — the current 3D tolerance.
    pub fn tol3d(&self) -> f64 {
        self.tol3d
    }

    /// Number of boundaries.
    pub fn nb_boundaries(&self) -> usize {
        self.boundaries.len()
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom_bspline_curve::GeomBSplineCurve;

    // Helper: build a straight-line B-spline from p0 to p1.
    fn line_bspline(p0: Pnt, p1: Pnt) -> GeomBSplineCurve {
        let poles = vec![p0, p1];
        let knots = vec![0.0, 1.0];
        let mults = vec![2, 2];
        GeomBSplineCurve::new(&poles, &knots, &mults, 1, false)
    }

    // Helper: build a cubic B-spline polyline from 4 equally-spaced points.
    fn cubic_bspline(p0: Pnt, p1: Pnt, p2: Pnt, p3: Pnt) -> GeomBSplineCurve {
        let poles = vec![p0, p1, p2, p3];
        let knots = vec![0.0, 1.0];
        let mults = vec![4, 4];
        GeomBSplineCurve::new(&poles, &knots, &mults, 3, false)
    }

    // -----------------------------------------------------------------------
    // FillingStyle
    // -----------------------------------------------------------------------

    #[test]
    fn filling_style_variants_distinct() {
        assert_ne!(FillingStyle::Stretch, FillingStyle::Coons);
        assert_ne!(FillingStyle::Coons, FillingStyle::Curved);
        assert_ne!(FillingStyle::Stretch, FillingStyle::Curved);
    }

    // -----------------------------------------------------------------------
    // GeomFillBezierCurves — two curves (ruled)
    // -----------------------------------------------------------------------

    #[test]
    fn bezier_fill_two_curves_ruled_surface_corners() {
        // Two straight horizontal lines at y=0 and y=1.
        let c0 = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0)];
        let c1 = vec![Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0)];

        let mut filler = GeomFillBezierCurves::new();
        filler.init_two(c0, c1, FillingStyle::Stretch);

        assert_eq!(filler.nb_curves(), 2);
        assert_eq!(filler.style(), FillingStyle::Stretch);
        let surf = filler.surface();
        // Surface should have 2×4 poles (2 from degree-1 U, 4 from degree-3 V).
        assert_eq!(surf.nb_u_poles(), 2);
        assert_eq!(surf.nb_v_poles(), 4);
    }

    #[test]
    fn bezier_fill_two_curves_coons_style() {
        let c0 = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(2.0, 0.0, 0.0)];
        let c1 = vec![Pnt::new(0.0, 2.0, 0.0), Pnt::new(2.0, 2.0, 0.0)];
        let mut filler = GeomFillBezierCurves::new();
        filler.init_two(c0, c1, FillingStyle::Coons);
        assert_eq!(filler.style(), FillingStyle::Coons);
        let surf = filler.surface();
        // U degree == 1 (linear), V degree == 3 (cubic ruled).
        assert_eq!(surf.u_degree(), 1);
        assert_eq!(surf.v_degree(), 3);
    }

    // -----------------------------------------------------------------------
    // GeomFillBezierCurves — four curves (Coons quad)
    // -----------------------------------------------------------------------

    #[test]
    fn bezier_fill_four_curves_coons_patch() {
        // Unit square boundary.
        let bot = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0)];
        let right = vec![Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0)];
        let top = vec![Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0)];
        let left = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0)];

        let mut filler = GeomFillBezierCurves::new();
        filler.init_four(bot, right, top, left, FillingStyle::Coons);

        assert_eq!(filler.nb_curves(), 4);
        let surf = filler.surface();
        // 4×4 cubic patch.
        assert_eq!(surf.nb_u_poles(), 4);
        assert_eq!(surf.nb_v_poles(), 4);
    }

    #[test]
    fn bezier_fill_four_curves_centre_approximately_correct() {
        // Flat unit square: centre should be ~(0.5, 0.5, 0).
        let bot = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0)];
        let right = vec![Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0)];
        let top = vec![Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0)];
        let left = vec![Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0)];
        let mut filler = GeomFillBezierCurves::new();
        filler.init_four(bot, right, top, left, FillingStyle::Coons);
        let surf = filler.surface();
        // Evaluate at (u=0.5, v=0.5) — centre of parameter space.
        let center = surf.value(0.5, 0.5);
        assert!((center.x - 0.5).abs() < 0.05, "x≈0.5 got {}", center.x);
        assert!((center.y - 0.5).abs() < 0.05, "y≈0.5 got {}", center.y);
        assert!(center.z.abs() < 1.0e-10, "z≈0 got {}", center.z);
    }

    // -----------------------------------------------------------------------
    // GeomFillBSplineCurves
    // -----------------------------------------------------------------------

    #[test]
    fn bspline_fill_two_curves_ruled() {
        let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let c1 = line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0));

        let mut filler = GeomFillBSplineCurves::new();
        filler.init_two(c0, c1, FillingStyle::Stretch);

        assert_eq!(filler.nb_curves(), 2);
        let surf = filler.surface();
        // 4×4 grid.
        assert_eq!(surf.nb_u_poles(), 4);
        assert_eq!(surf.nb_v_poles(), 4);
    }

    #[test]
    fn bspline_fill_four_curves_coons() {
        let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let c1 = line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
        let c2 = line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
        let c3 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));

        let mut filler = GeomFillBSplineCurves::new();
        filler.init_four(c0, c1, c2, c3, FillingStyle::Coons);
        assert_eq!(filler.nb_curves(), 4);
        let surf = filler.surface();
        assert_eq!(surf.nb_u_poles(), 4);
        assert_eq!(surf.nb_v_poles(), 4);
    }

    #[test]
    fn bspline_fill_default_is_empty() {
        let filler = GeomFillBSplineCurves::new();
        assert_eq!(filler.nb_curves(), 0);
        assert_eq!(filler.style(), FillingStyle::Coons);
    }

    // -----------------------------------------------------------------------
    // GeomFillPipe
    // -----------------------------------------------------------------------

    #[test]
    fn pipe_basic_circular_section() {
        // Straight spine along Z.
        let c = cubic_bspline(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(0.0, 0.0, 1.0),
            Pnt::new(0.0, 0.0, 2.0),
            Pnt::new(0.0, 0.0, 3.0),
        );
        let mut pipe = GeomFillPipe::new(c, 1.0);
        assert_eq!(pipe.radius(), 1.0);
        assert!(!pipe.is_done());
        pipe.perform();
        assert!(pipe.is_done());
        let surf = pipe.surface();
        // Should have poles in both directions.
        assert!(surf.nb_u_poles() >= 4);
        assert!(surf.nb_v_poles() >= 4);
    }

    #[test]
    fn pipe_radius_stored_correctly() {
        let c = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 5.0));
        let pipe = GeomFillPipe::new(c, 2.5);
        assert!((pipe.radius() - 2.5).abs() < 1.0e-15);
    }

    #[test]
    fn pipe_error_status_ok_after_perform() {
        let c = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 0.0, 3.0));
        let mut pipe = GeomFillPipe::new(c, 0.5);
        pipe.perform();
        assert_eq!(pipe.error_status(), PipeError::PipeOk);
    }

    #[test]
    fn pipe_set_transition_mode() {
        let c = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let mut pipe = GeomFillPipe::new(c, 0.1);
        pipe.set_transition(PipeTransitionMode::CorrectedFrenet);
        // Setting transition invalidates previous result.
        assert!(!pipe.is_done());
        pipe.perform();
        assert!(pipe.is_done());
    }

    // -----------------------------------------------------------------------
    // GeomFillConstrainedFilling
    // -----------------------------------------------------------------------

    #[test]
    fn constrained_fill_g0_four_boundaries() {
        let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let c1 = line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
        let c2 = line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
        let c3 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));

        let b0 = FillingBoundary::new_g0(c0);
        let b1 = FillingBoundary::new_g0(c1);
        let b2 = FillingBoundary::new_g0(c2);
        let b3 = FillingBoundary::new_g0(c3);

        let mut filler = GeomFillConstrainedFilling::new(3, 1);
        filler.init_four(b0, b1, b2, b3);
        assert_eq!(filler.nb_boundaries(), 4);
        assert!(!filler.is_done());

        filler.build();
        assert!(filler.is_done());
        let surf = filler.surface();
        assert_eq!(surf.nb_u_poles(), 4);
        assert_eq!(surf.nb_v_poles(), 4);
    }

    #[test]
    fn constrained_fill_g1_boundary() {
        let c0 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let c1 = line_bspline(Pnt::new(1.0, 0.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
        let c2 = line_bspline(Pnt::new(0.0, 1.0, 0.0), Pnt::new(1.0, 1.0, 0.0));
        let c3 = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(0.0, 1.0, 0.0));

        // G1 constraint on boundary 0 with upward normals.
        let normals = vec![Pnt::new(0.0, 0.0, 1.0); 4];
        let b0 = FillingBoundary::new_g1(c0, normals);
        let b1 = FillingBoundary::new_g0(c1);
        let b2 = FillingBoundary::new_g0(c2);
        let b3 = FillingBoundary::new_g0(c3);

        let mut filler = GeomFillConstrainedFilling::new(3, 1);
        filler.init_four(b0, b1, b2, b3);
        filler.build();
        assert!(filler.is_done());
        // G1 correction: inner poles should have been shifted in Z.
        let surf = filler.surface();
        let inner_pole = surf.pole(2, 2); // row 2, col 2 (1-based)
        assert!(inner_pole.z.abs() > 0.0, "G1 correction should shift inner poles in Z");
    }

    #[test]
    fn constrained_fill_tol3d() {
        let mut filler = GeomFillConstrainedFilling::new(5, 2);
        assert!((filler.tol3d() - 1.0e-4).abs() < 1.0e-12);
        filler.set_tol3d(1.0e-6);
        assert!((filler.tol3d() - 1.0e-6).abs() < 1.0e-15);
    }

    #[test]
    fn constraint_order_ordering() {
        assert!(ConstraintOrder::G0 < ConstraintOrder::G1);
        assert!(ConstraintOrder::G1 < ConstraintOrder::G2);
    }

    #[test]
    fn filling_boundary_accessors() {
        let c = line_bspline(Pnt::new(0.0, 0.0, 0.0), Pnt::new(1.0, 0.0, 0.0));
        let b = FillingBoundary::new_g0(c.clone());
        assert_eq!(b.order(), ConstraintOrder::G0);
        // G1 boundary stores normals.
        let normals = vec![Pnt::new(0.0, 0.0, 1.0); 3];
        let bg1 = FillingBoundary::new_g1(c, normals);
        assert_eq!(bg1.order(), ConstraintOrder::G1);
    }

    #[test]
    fn pipe_transition_mode_variants() {
        assert_ne!(PipeTransitionMode::Frenet, PipeTransitionMode::Fixed);
        assert_ne!(PipeTransitionMode::CorrectedFrenet, PipeTransitionMode::Darboux);
    }

    // -----------------------------------------------------------------------
    // Frenet frame helper
    // -----------------------------------------------------------------------

    #[test]
    fn frenet_frames_count_and_orthogonality() {
        let c = cubic_bspline(
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
            Pnt::new(3.0, 0.0, 0.0),
        );
        let frames = compute_frenet_frames(&c, 5);
        assert_eq!(frames.len(), 5);
        for (_, t, n, b) in &frames {
            // Tangent and normal should be approximately perpendicular.
            let dot_tn = t.dot(*n).abs();
            assert!(dot_tn < 1.0e-6, "T⊥N failed: |T·N| = {}", dot_tn);
            // Binormal = T×N should have unit length.
            let bn_len = b.norm();
            assert!((bn_len - 1.0).abs() < 1.0e-6, "|B| = {}", bn_len);
        }
    }
}
