// FILE: geom_local_props.rs
//! `GeomLProp` local property value types — lightweight, zero-dependency structs
//! that carry pre-computed local properties of surfaces and curves.
//!
//! These are **data-only** counterparts to the full evaluator types in
//! `geom_lprop`. They hold the results (point, normal, curvatures, derivatives)
//! that a caller has already computed and let consumers query them via the same
//! API shape as OCCT's `GeomLProp_SLProps` / `GeomLProp_CLProps`.

// ---------------------------------------------------------------------------
// SurfaceLocalProps
// ---------------------------------------------------------------------------

/// Local geometric properties of a parametric surface at a given (u, v) point.
///
/// Stores the parameter pair, the evaluated point, the unit normal, and the
/// principal curvatures. All fields are set explicitly by the caller; the struct
/// does not re-evaluate the surface.
///
/// Curvature-derived quantities (`mean_curvature`, `gaussian_curvature`,
/// `shape_index`) are computed on demand from the stored `min_curvature` /
/// `max_curvature` values.
// occt: GeomLProp_SLProps
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceLocalProps {
    /// First surface parameter.
    pub u: f64,
    /// Second surface parameter.
    pub v: f64,
    /// Evaluated point on the surface.
    point: [f64; 3],
    /// Unit surface normal (only meaningful when `is_normal_defined` is true).
    normal: [f64; 3],
    /// Maximum principal curvature κ₁.
    max_curvature: f64,
    /// Minimum principal curvature κ₂.
    min_curvature: f64,
    /// Whether the normal has been set.
    is_normal_defined: bool,
    /// Whether the principal curvatures have been set.
    is_curvature_defined: bool,
}

impl SurfaceLocalProps {
    /// Create a new instance at parameter `(u, v)` with no properties populated
    /// yet. Call `set_point`, `set_normal`, and `set_curvatures` to fill it.
    // occt: GeomLProp_SLProps::GeomLProp_SLProps
    pub fn new(u: f64, v: f64) -> Self {
        SurfaceLocalProps {
            u,
            v,
            point: [0.0; 3],
            normal: [0.0; 3],
            max_curvature: 0.0,
            min_curvature: 0.0,
            is_normal_defined: false,
            is_curvature_defined: false,
        }
    }

    /// Store the evaluated surface point at `(u, v)`.
    // occt: GeomLProp_SLProps::Value
    pub fn set_point(&mut self, p: [f64; 3]) {
        self.point = p;
    }

    /// Store the unit surface normal and mark it as defined.
    // occt: GeomLProp_SLProps::Normal
    pub fn set_normal(&mut self, n: [f64; 3]) {
        self.normal = n;
        self.is_normal_defined = true;
    }

    /// Store both principal curvatures and mark them as defined.
    ///
    /// `min` and `max` may be provided in any order — the method reorders them
    /// so that `max_curvature >= min_curvature` is always satisfied.
    // occt: GeomLProp_SLProps::MaxCurvature / MinCurvature
    pub fn set_curvatures(&mut self, min: f64, max: f64) {
        self.min_curvature = min.min(max);
        self.max_curvature = min.max(max);
        self.is_curvature_defined = true;
    }

    /// The evaluated point on the surface.
    // occt: GeomLProp_SLProps::Value
    pub fn point(&self) -> [f64; 3] {
        self.point
    }

    /// The unit surface normal. Only meaningful when `is_normal_defined()` is
    /// true.
    // occt: GeomLProp_SLProps::Normal
    pub fn normal(&self) -> [f64; 3] {
        self.normal
    }

    /// Whether a unit normal has been stored.
    // occt: GeomLProp_SLProps::IsNormalDefined
    pub fn is_normal_defined(&self) -> bool {
        self.is_normal_defined
    }

    /// Whether the principal curvatures have been stored.
    // occt: GeomLProp_SLProps::IsCurvatureDefined
    pub fn is_curvature_defined(&self) -> bool {
        self.is_curvature_defined
    }

    /// Maximum principal curvature κ₁.
    // occt: GeomLProp_SLProps::MaxCurvature
    pub fn max_curvature(&self) -> f64 {
        self.max_curvature
    }

    /// Minimum principal curvature κ₂.
    // occt: GeomLProp_SLProps::MinCurvature
    pub fn min_curvature(&self) -> f64 {
        self.min_curvature
    }

    /// Mean curvature H = (κ₁ + κ₂) / 2.
    // occt: GeomLProp_SLProps::MeanCurvature
    pub fn mean_curvature(&self) -> f64 {
        (self.max_curvature + self.min_curvature) * 0.5
    }

    /// Gaussian curvature K = κ₁ · κ₂.
    // occt: GeomLProp_SLProps::GaussianCurvature
    pub fn gaussian_curvature(&self) -> f64 {
        self.max_curvature * self.min_curvature
    }

    /// Shape index S = (2/π) · atan((κ_max + κ_min) / (κ_max − κ_min)).
    ///
    /// Returns `0.0` when `κ_max == κ_min` (umbilical point / flat surface) to
    /// avoid division by zero.
    ///
    /// The shape index lies in the range (−1, 1] and classifies the local
    /// surface shape: −1 = cup, −0.5 = rut/trough, 0 = saddle, +0.5 = ridge,
    /// +1 = dome.
    // occt: GeomLProp_SLProps (shape index is a derived quantity)
    pub fn shape_index(&self) -> f64 {
        let kmax = self.max_curvature;
        let kmin = self.min_curvature;
        let diff = kmax - kmin;
        if diff == 0.0 {
            return 0.0;
        }
        (2.0 / std::f64::consts::PI) * ((kmax + kmin) / diff).atan()
    }
}

// ---------------------------------------------------------------------------
// CurveLocalProps
// ---------------------------------------------------------------------------

/// Local geometric properties of a 3D curve at a given parameter value.
///
/// Stores the parameter, the evaluated point, the first and second derivatives,
/// the scalar curvature, and the torsion. All values are set explicitly by the
/// caller.
///
/// When `set_derivatives` is called with D1 and D2, the curvature is computed
/// automatically as `|D1 × D2| / |D1|³`. The caller may override `torsion`
/// directly via `set_torsion` if it has been computed externally.
// occt: GeomLProp_CLProps
#[derive(Clone, Debug, PartialEq)]
pub struct CurveLocalProps {
    /// Curve parameter value.
    pub parameter: f64,
    /// Evaluated point P(t).
    point: [f64; 3],
    /// First derivative C'(t).
    d1: [f64; 3],
    /// Second derivative C''(t).
    d2: [f64; 3],
    /// Scalar curvature κ = |D1 × D2| / |D1|³.
    curvature: f64,
    /// Torsion τ (the second curvature of a space curve).
    torsion: f64,
}

impl CurveLocalProps {
    /// Create a new instance at curve parameter `t` with no properties
    /// populated yet. Call `set_point` and `set_derivatives` to fill it.
    // occt: GeomLProp_CLProps::GeomLProp_CLProps
    pub fn new(parameter: f64) -> Self {
        CurveLocalProps {
            parameter,
            point: [0.0; 3],
            d1: [0.0; 3],
            d2: [0.0; 3],
            curvature: 0.0,
            torsion: 0.0,
        }
    }

    /// Store the evaluated curve point P(t).
    // occt: GeomLProp_CLProps::Value
    pub fn set_point(&mut self, p: [f64; 3]) {
        self.point = p;
    }

    /// Store the first and second derivatives and compute the curvature
    /// automatically.
    ///
    /// Curvature κ = |D1 × D2| / |D1|³. If |D1| is effectively zero the
    /// curvature is set to `0.0` rather than `+∞`, matching the common
    /// defensive convention for degenerate points.
    // occt: GeomLProp_CLProps::D1 / D2 / Curvature
    pub fn set_derivatives(&mut self, d1: [f64; 3], d2: [f64; 3]) {
        self.d1 = d1;
        self.d2 = d2;

        // |D1|
        let n1_sq = d1[0] * d1[0] + d1[1] * d1[1] + d1[2] * d1[2];
        let n1 = n1_sq.sqrt();

        // D1 × D2
        let cx = d1[1] * d2[2] - d1[2] * d2[1];
        let cy = d1[2] * d2[0] - d1[0] * d2[2];
        let cz = d1[0] * d2[1] - d1[1] * d2[0];
        let cross_norm = (cx * cx + cy * cy + cz * cz).sqrt();

        let n1_cubed = n1 * n1_sq; // n1³
        if n1_cubed < f64::EPSILON {
            self.curvature = 0.0;
        } else {
            self.curvature = cross_norm / n1_cubed;
        }
    }

    /// Scalar curvature κ.
    // occt: GeomLProp_CLProps::Curvature
    pub fn curvature(&self) -> f64 {
        self.curvature
    }

    /// Torsion τ (second curvature). Returns the stored value; set externally
    /// via `set_torsion` after calling `set_derivatives`.
    // occt: GeomLProp_CLProps::Torsion
    pub fn torsion(&self) -> f64 {
        self.torsion
    }

    /// Set the torsion value directly.
    pub fn set_torsion(&mut self, torsion: f64) {
        self.torsion = torsion;
    }

    /// Unit tangent vector T̂ = D1 / |D1|.
    ///
    /// Returns `[0.0, 0.0, 0.0]` when D1 is the zero vector (degenerate point).
    // occt: GeomLProp_CLProps::Tangent
    pub fn tangent(&self) -> [f64; 3] {
        let [dx, dy, dz] = self.d1;
        let n = (dx * dx + dy * dy + dz * dz).sqrt();
        if n < f64::EPSILON {
            [0.0, 0.0, 0.0]
        } else {
            [dx / n, dy / n, dz / n]
        }
    }

    /// Principal normal vector N̂ = component of D2 perpendicular to D1,
    /// then normalised.
    ///
    /// Uses the Frenet-Serret formula: N̂ = (D2 − (D2·T̂)·T̂) / |…|.
    ///
    /// Returns `[0.0, 0.0, 0.0]` for straight curves (zero curvature) where the
    /// normal is not defined.
    // occt: GeomLProp_CLProps::Normal
    pub fn normal(&self) -> [f64; 3] {
        let t = self.tangent();
        let tn = t[0] * t[0] + t[1] * t[1] + t[2] * t[2];
        if tn < f64::EPSILON {
            return [0.0, 0.0, 0.0];
        }
        let [d2x, d2y, d2z] = self.d2;
        // D2 · T̂
        let dot = d2x * t[0] + d2y * t[1] + d2z * t[2];
        // D2 − (D2·T̂)·T̂
        let px = d2x - dot * t[0];
        let py = d2y - dot * t[1];
        let pz = d2z - dot * t[2];
        let pn = (px * px + py * py + pz * pz).sqrt();
        if pn < f64::EPSILON {
            [0.0, 0.0, 0.0]
        } else {
            [px / pn, py / pn, pz / pn]
        }
    }

    /// The evaluated point P(t).
    // occt: GeomLProp_CLProps::Value
    pub fn point(&self) -> [f64; 3] {
        self.point
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn near(a: f64, b: f64, tol: f64, label: &str) {
        assert!(
            (a - b).abs() <= tol,
            "{label}: got {a}, expected {b} (tol {tol})"
        );
    }

    // -----------------------------------------------------------------------
    // SurfaceLocalProps
    // -----------------------------------------------------------------------

    #[test]
    fn surface_new_defaults() {
        let p = SurfaceLocalProps::new(1.0, 2.0);
        assert_eq!(p.u, 1.0);
        assert_eq!(p.v, 2.0);
        assert!(!p.is_normal_defined());
        assert!(!p.is_curvature_defined());
    }

    #[test]
    fn surface_set_point_and_normal() {
        let mut p = SurfaceLocalProps::new(0.0, 0.0);
        p.set_point([1.0, 2.0, 3.0]);
        p.set_normal([0.0, 0.0, 1.0]);
        assert_eq!(p.point(), [1.0, 2.0, 3.0]);
        assert_eq!(p.normal(), [0.0, 0.0, 1.0]);
        assert!(p.is_normal_defined());
    }

    #[test]
    fn surface_curvatures_reordered() {
        let mut p = SurfaceLocalProps::new(0.0, 0.0);
        // Pass max first, min second — should be reordered.
        p.set_curvatures(5.0, 2.0);
        assert_eq!(p.min_curvature(), 2.0);
        assert_eq!(p.max_curvature(), 5.0);
        assert!(p.is_curvature_defined());
    }

    #[test]
    fn surface_mean_curvature() {
        let mut p = SurfaceLocalProps::new(0.0, 0.0);
        p.set_curvatures(1.0, 3.0);
        near(p.mean_curvature(), 2.0, 1e-14, "mean curvature");
    }

    #[test]
    fn surface_gaussian_curvature() {
        let mut p = SurfaceLocalProps::new(0.0, 0.0);
        p.set_curvatures(2.0, 4.0);
        near(p.gaussian_curvature(), 8.0, 1e-14, "gaussian curvature");
    }

    #[test]
    fn surface_gaussian_curvature_cylinder() {
        // A cylinder has one zero principal curvature -> K = 0.
        let mut p = SurfaceLocalProps::new(0.0, 0.0);
        p.set_curvatures(0.0, 1.0);
        near(p.gaussian_curvature(), 0.0, 1e-14, "cylinder K = 0");
    }

    #[test]
    fn surface_shape_index_sphere() {
        // For a sphere kmax == kmin (umbilical) -> shape_index = 0.
        let mut p = SurfaceLocalProps::new(0.0, 0.0);
        p.set_curvatures(1.0, 1.0);
        near(p.shape_index(), 0.0, 1e-14, "sphere shape index = 0");
    }

    #[test]
    fn surface_shape_index_cylinder() {
        // kmax = 1, kmin = 0 -> (2/pi)*atan((1+0)/(1-0)) = (2/pi)*atan(1) = 0.5.
        let mut p = SurfaceLocalProps::new(0.0, 0.0);
        p.set_curvatures(0.0, 1.0);
        near(p.shape_index(), 0.5, 1e-12, "cylinder shape index = 0.5");
    }

    #[test]
    fn surface_shape_index_saddle() {
        // kmax = 1, kmin = -1 -> sum = 0 -> atan(0) = 0.
        let mut p = SurfaceLocalProps::new(0.0, 0.0);
        p.set_curvatures(-1.0, 1.0);
        near(p.shape_index(), 0.0, 1e-14, "saddle shape index = 0");
    }

    #[test]
    fn surface_shape_index_formula() {
        let mut p = SurfaceLocalProps::new(0.0, 0.0);
        p.set_curvatures(1.0, 3.0);
        // (2/pi)*atan((3+1)/(3-1)) = (2/pi)*atan(2).
        let expected = (2.0 / PI) * (2.0_f64.atan());
        near(p.shape_index(), expected, 1e-12, "shape index formula");
    }

    // -----------------------------------------------------------------------
    // CurveLocalProps
    // -----------------------------------------------------------------------

    #[test]
    fn curve_new_defaults() {
        let c = CurveLocalProps::new(3.14);
        assert_eq!(c.parameter, 3.14);
        near(c.curvature(), 0.0, 1e-14, "default curvature");
        near(c.torsion(), 0.0, 1e-14, "default torsion");
    }

    #[test]
    fn curve_set_point() {
        let mut c = CurveLocalProps::new(0.0);
        c.set_point([1.0, 2.0, 3.0]);
        assert_eq!(c.point(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn curve_tangent_unit() {
        let mut c = CurveLocalProps::new(0.0);
        c.set_derivatives([3.0, 4.0, 0.0], [0.0, 0.0, 0.0]);
        let t = c.tangent();
        let n = (t[0] * t[0] + t[1] * t[1] + t[2] * t[2]).sqrt();
        near(n, 1.0, 1e-14, "|tangent| = 1");
        near(t[0], 3.0 / 5.0, 1e-14, "tx");
        near(t[1], 4.0 / 5.0, 1e-14, "ty");
    }

    #[test]
    fn curve_curvature_circle() {
        // For a unit circle: D1 = (0,1,0), D2 = (-1,0,0) at t=0.
        // k = |D1 x D2| / |D1|^3 = |(0,0,1)| / 1 = 1.
        let mut c = CurveLocalProps::new(0.0);
        c.set_derivatives([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]);
        near(c.curvature(), 1.0, 1e-14, "unit circle curvature = 1");
    }

    #[test]
    fn curve_curvature_line() {
        // A line has D2 = 0, so D1 x D2 = 0 -> k = 0.
        let mut c = CurveLocalProps::new(0.0);
        c.set_derivatives([1.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        near(c.curvature(), 0.0, 1e-14, "line curvature = 0");
    }

    #[test]
    fn curve_curvature_radius_r() {
        // Circle of radius r: D1 = (0, r, 0), D2 = (-r, 0, 0).
        // k = r / r^3 = 1/r.
        let r = 3.0_f64;
        let mut c = CurveLocalProps::new(0.0);
        c.set_derivatives([0.0, r, 0.0], [-r, 0.0, 0.0]);
        near(c.curvature(), 1.0 / r, 1e-14, "circle r=3 curvature = 1/3");
    }

    #[test]
    fn curve_normal_circle() {
        // Unit circle at t=0: D1 = (0,1,0), D2 = (-1,0,0).
        // T = (0,1,0). Component of D2 orthogonal to T: (-1,0,0) - 0*(0,1,0) = (-1,0,0).
        // N = (-1,0,0) (points toward center).
        let mut c = CurveLocalProps::new(0.0);
        c.set_derivatives([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]);
        let n = c.normal();
        near(n[0], -1.0, 1e-14, "normal x = -1");
        near(n[1], 0.0, 1e-14, "normal y = 0");
        near(n[2], 0.0, 1e-14, "normal z = 0");
    }

    #[test]
    fn curve_normal_unit_for_nonzero_curvature() {
        let mut c = CurveLocalProps::new(0.0);
        c.set_derivatives([0.0, 2.0, 0.0], [-2.0, 0.0, 0.0]);
        let n = c.normal();
        let nn = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        near(nn, 1.0, 1e-14, "|normal| = 1");
    }

    #[test]
    fn curve_normal_zero_for_line() {
        let mut c = CurveLocalProps::new(0.0);
        c.set_derivatives([1.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        let n = c.normal();
        // D2 = 0 -> projection is zero -> normal returns [0,0,0].
        near(n[0], 0.0, 1e-14, "line normal x = 0");
        near(n[1], 0.0, 1e-14, "line normal y = 0");
        near(n[2], 0.0, 1e-14, "line normal z = 0");
    }

    #[test]
    fn curve_torsion_set_get() {
        let mut c = CurveLocalProps::new(0.0);
        c.set_torsion(0.42);
        near(c.torsion(), 0.42, 1e-14, "torsion round-trip");
    }
}
