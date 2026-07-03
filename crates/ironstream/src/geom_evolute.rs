//! Evolute, involute, and curve-offset constructions.
//!
//! Models the OCCT `Geom_OffsetCurve` / axis-symmetric-surface evolute
//! constructions (`src/ModelingData/TKG3d/Geom/Geom_OffsetCurve.hxx`).
//!
//! An **evolute** of a plane curve is the locus of its centres of curvature.
//! An **involute** is the curve traced by the end of a taut string unwound from
//! a given curve.  A **curve offset** displaces every point by a fixed signed
//! distance along the curve normal.
//!
//! All three types are stub implementations that synthesise geometry from the
//! curve name via a small set of closed-form parametric formulae — sufficient
//! for testing and for driving downstream consumers without requiring a full
//! OCCT kernel.
//!
//! Only `std` is used; no external crates.

use std::f64::consts::PI;

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers: parametric geometry for named curves
// ─────────────────────────────────────────────────────────────────────────────

/// Evaluate the position, first derivative, and curvature normal of a named
/// parametric curve at parameter `t`.
///
/// Supported curve names (case-insensitive):
/// - `"circle"` — unit circle in the XY plane.
/// - `"ellipse"` — ellipse with semi-axes a=2, b=1 in the XY plane.
/// - `"parabola"` — parabola y = t², z = 0.
/// - `"line"` / `"segment"` — line along the X axis.
///
/// Unknown names fall back to `"circle"`.
///
/// Returns `(position, tangent, unit_normal)` where `unit_normal` points
/// toward the centre of curvature (or the Y axis for degenerate cases).
fn curve_geometry(name: &str, t: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
    match name.to_ascii_lowercase().as_str() {
        "ellipse" => {
            let (a, b) = (2.0_f64, 1.0_f64);
            let (s, c) = t.sin_cos();
            let pos = [a * c, b * s, 0.0];
            // tangent (unnormalised): (-a·sin t, b·cos t, 0)
            let tx = -a * s;
            let ty = b * c;
            let tlen = (tx * tx + ty * ty).sqrt().max(1e-12);
            let tan = [tx / tlen, ty / tlen, 0.0];
            // inward normal (rotate tangent 90° left): (-ty/tlen, tx/tlen, 0)
            let n = [-ty / tlen, tx / tlen, 0.0];
            // but conventional curvature normal for ellipse points inward
            let nx = -a * c;
            let ny = -b * s;
            let nlen = (nx * nx + ny * ny).sqrt().max(1e-12);
            let norm = [nx / nlen, ny / nlen, 0.0];
            (pos, tan, norm)
        }
        "parabola" => {
            // C(t) = (t, t², 0),  C'(t) = (1, 2t, 0)
            let pos = [t, t * t, 0.0];
            let tx = 1.0_f64;
            let ty = 2.0 * t;
            let tlen = (tx * tx + ty * ty).sqrt().max(1e-12);
            let tan = [tx / tlen, ty / tlen, 0.0];
            // curvature: κ = 2 / (1 + 4t²)^(3/2); normal points in +Y direction
            let nx = -ty / tlen;
            let ny = tx / tlen;
            let norm = [nx, ny, 0.0];
            (pos, tan, norm)
        }
        "line" | "segment" => {
            // C(t) = (t, 0, 0) — straight line along X; curvature = 0 so
            // normal is defined as the Y unit vector (curvature radius = ∞).
            let pos = [t, 0.0, 0.0];
            let tan = [1.0, 0.0, 0.0];
            let norm = [0.0, 1.0, 0.0];
            (pos, tan, norm)
        }
        // Default: unit circle in XY plane
        _ => {
            let (s, c) = t.sin_cos();
            let pos = [c, s, 0.0];
            let tan = [-s, c, 0.0];
            // inward normal for a unit circle: (-cos t, -sin t, 0)
            let norm = [-c, -s, 0.0];
            (pos, tan, norm)
        }
    }
}

/// Radius of curvature at parameter `t` for a named curve.
fn curvature_radius(name: &str, t: f64) -> f64 {
    match name.to_ascii_lowercase().as_str() {
        "circle" => 1.0,
        "ellipse" => {
            let (a, b) = (2.0_f64, 1.0_f64);
            let (s, c) = t.sin_cos();
            // ρ = (a²sin²t + b²cos²t)^(3/2) / (a·b)
            let top = (a * a * s * s + b * b * c * c).powf(1.5);
            top / (a * b)
        }
        "parabola" => {
            // ρ = (1 + 4t²)^(3/2) / 2
            (1.0 + 4.0 * t * t).powf(1.5) / 2.0
        }
        _ => f64::INFINITY,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Evolute
// ─────────────────────────────────────────────────────────────────────────────

/// The evolute of a named parametric curve: the locus of its centres of
/// curvature.
///
/// For a curve `C(t)` with unit tangent `T(t)`, unit principal normal `N(t)`,
/// and radius of curvature `ρ(t)`, the evolute is:
///
/// ```text
/// E(t) = C(t) + ρ(t) · N(t)
/// ```
///
/// Pre-computed samples are stored so that downstream consumers do not need to
/// re-evaluate the geometry repeatedly.
// occt: Geom_OffsetCurve
#[derive(Clone, Debug)]
pub struct Evolute {
    /// Name of the source parametric curve.
    pub curve_name: String,
    /// Pre-sampled evolute points `[x, y, z]`.
    pub samples: Vec<[f64; 3]>,
}

impl Evolute {
    /// Build an evolute from a named curve, sampling it at `n` uniformly
    /// spaced parameter values over `[0, 2π]`.
    ///
    /// # Parameters
    /// - `curve` — name of the source curve (e.g. `"circle"`, `"ellipse"`).
    /// - `n` — number of sample points (must be ≥ 1).
    ///
    /// # Panics
    /// Panics if `n == 0`.
    pub fn from_curve(curve: &str, n: usize) -> Self {
        assert!(n >= 1, "Evolute::from_curve: n must be >= 1");
        let mut samples = Vec::with_capacity(n);
        for i in 0..n {
            let t = if n == 1 {
                0.0
            } else {
                2.0 * PI * (i as f64) / (n as f64)
            };
            let pt = Self::compute_at(curve, t);
            samples.push(pt);
        }
        Evolute {
            curve_name: curve.to_owned(),
            samples,
        }
    }

    /// Evaluate the evolute at parameter `t` (radians / curve parameter).
    ///
    /// `E(t) = C(t) + ρ(t) · N(t)`
    pub fn at(&self, t: f64) -> [f64; 3] {
        Self::compute_at(&self.curve_name, t)
    }

    /// Number of pre-sampled points stored in [`Evolute::samples`].
    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }

    // ── private ──────────────────────────────────────────────────────────────

    fn compute_at(name: &str, t: f64) -> [f64; 3] {
        let (pos, _tan, norm) = curve_geometry(name, t);
        let rho = curvature_radius(name, t);
        if rho.is_infinite() || rho.is_nan() {
            // Zero curvature: centre of curvature at infinity; return the
            // curve point itself (matches OCCT's degenerate handling).
            pos
        } else {
            [
                pos[0] + rho * norm[0],
                pos[1] + rho * norm[1],
                pos[2] + rho * norm[2],
            ]
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Involute
// ─────────────────────────────────────────────────────────────────────────────

/// The involute of a named parametric curve starting at parameter `start_param`.
///
/// The involute is the path traced by the end of a taut string of length
/// `L(t) − L(t₀)` (accumulated arc-length from the starting parameter `t₀`)
/// unwound from the curve:
///
/// ```text
/// I(t) = C(t) − (arc_length(t₀ → t)) · T(t)
/// ```
///
/// Arc-length is approximated by Gaussian quadrature over small sub-intervals.
#[derive(Clone, Debug)]
pub struct Involute {
    /// Name of the source parametric curve.
    pub curve_name: String,
    /// Parameter at which string unwinding begins (`t₀`).
    pub start_param: f64,
}

impl Involute {
    /// Construct an involute of `curve` starting at parameter `start`.
    pub fn new(curve: &str, start: f64) -> Self {
        Involute {
            curve_name: curve.to_owned(),
            start_param: start,
        }
    }

    /// Evaluate the involute at parameter `t`.
    ///
    /// Returns the `[x, y, z]` position.
    pub fn at(&self, t: f64) -> [f64; 3] {
        let arc = self.arc_length(self.start_param, t);
        let (pos, tan, _norm) = curve_geometry(&self.curve_name, t);
        // Subtract the unwound string length along the current tangent.
        // When t < start_param the string is being wound rather than unwound;
        // arc is negative in that case, so the formula still holds.
        [
            pos[0] - arc * tan[0],
            pos[1] - arc * tan[1],
            pos[2] - arc * tan[2],
        ]
    }

    // ── private ──────────────────────────────────────────────────────────────

    /// Approximate arc-length of `curve_name` from `a` to `b` using an
    /// 8-point Gauss-Legendre quadrature.
    fn arc_length(&self, a: f64, b: f64) -> f64 {
        // 8-point Gauss-Legendre nodes and weights on [-1, 1].
        const NODES: [f64; 8] = [
            -0.960289856497536,
            -0.796666477413627,
            -0.525532409916329,
            -0.183434642495650,
            0.183434642495650,
            0.525532409916329,
            0.796666477413627,
            0.960289856497536,
        ];
        const WEIGHTS: [f64; 8] = [
            0.101228536290376,
            0.222381034453374,
            0.313706645877887,
            0.362683783378362,
            0.362683783378362,
            0.313706645877887,
            0.222381034453374,
            0.101228536290376,
        ];

        let mid = (b + a) * 0.5;
        let half = (b - a) * 0.5;
        let mut sum = 0.0_f64;
        for (xi, wi) in NODES.iter().zip(WEIGHTS.iter()) {
            let t = mid + half * xi;
            let (_pos, tan, _n) = curve_geometry(&self.curve_name, t);
            let speed = (tan[0] * tan[0] + tan[1] * tan[1] + tan[2] * tan[2]).sqrt();
            sum += wi * speed;
        }
        half * sum
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CurveOffset
// ─────────────────────────────────────────────────────────────────────────────

/// An offset of a named parametric curve by a signed distance.
///
/// At each parameter value `t` the offset point is displaced from the curve
/// by `distance` along the (inward) principal normal:
///
/// ```text
/// Q(t) = C(t) + distance · N(t)
/// ```
///
/// First derivatives are computed analytically using finite differences of the
/// position formula (a central-difference step of `h = 1 × 10⁻⁶`).
// occt: Geom_OffsetCurve
#[derive(Clone, Debug)]
pub struct CurveOffset {
    /// Name of the source parametric curve.
    pub curve_name: String,
    /// Signed offset distance.
    pub distance: f64,
}

impl CurveOffset {
    /// Construct an offset of `curve` by `dist`.
    pub fn new(curve: &str, dist: f64) -> Self {
        CurveOffset {
            curve_name: curve.to_owned(),
            distance: dist,
        }
    }

    /// Evaluate the offset curve at parameter `t`.
    ///
    /// `Q(t) = C(t) + distance · N(t)`
    pub fn evaluate(&self, t: f64) -> [f64; 3] {
        let (pos, _tan, norm) = curve_geometry(&self.curve_name, t);
        [
            pos[0] + self.distance * norm[0],
            pos[1] + self.distance * norm[1],
            pos[2] + self.distance * norm[2],
        ]
    }

    /// First derivative of the offset curve at parameter `t`.
    ///
    /// Computed via central finite differences:
    ///
    /// `Q'(t) ≈ (Q(t + h) − Q(t − h)) / (2h)`
    ///
    /// with `h = 1 × 10⁻⁶`.
    pub fn d1(&self, t: f64) -> [f64; 3] {
        const H: f64 = 1e-6;
        let p_plus = self.evaluate(t + H);
        let p_minus = self.evaluate(t - H);
        let inv2h = 1.0 / (2.0 * H);
        [
            (p_plus[0] - p_minus[0]) * inv2h,
            (p_plus[1] - p_minus[1]) * inv2h,
            (p_plus[2] - p_minus[2]) * inv2h,
        ]
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// offset_curve_points
// ─────────────────────────────────────────────────────────────────────────────

/// Sample an offset of `curve` at `n` uniformly spaced parameter values over
/// `[0, 2π]` with offset distance `dist`.
///
/// Returns a `Vec` of `n` offset positions `[x, y, z]`.
///
/// # Parameters
/// - `curve` — name of the source curve.
/// - `dist`  — signed offset distance.
/// - `n`     — number of sample points (must be ≥ 1).
///
/// # Panics
/// Panics if `n == 0`.
pub fn offset_curve_points(curve: &str, dist: f64, n: usize) -> Vec<[f64; 3]> {
    assert!(n >= 1, "offset_curve_points: n must be >= 1");
    let off = CurveOffset::new(curve, dist);
    (0..n)
        .map(|i| {
            let t = if n == 1 {
                0.0
            } else {
                2.0 * PI * (i as f64) / (n as f64)
            };
            off.evaluate(t)
        })
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evolute_unit_circle_centre() {
        // The evolute of a unit circle is its centre (0, 0, 0).
        let ev = Evolute::from_curve("circle", 8);
        assert_eq!(ev.sample_count(), 8);
        for pt in &ev.samples {
            assert!(pt[0].abs() < 1e-10, "x not near 0: {}", pt[0]);
            assert!(pt[1].abs() < 1e-10, "y not near 0: {}", pt[1]);
            assert!(pt[2].abs() < 1e-10, "z not near 0: {}", pt[2]);
        }
    }

    #[test]
    fn evolute_at_matches_samples() {
        let ev = Evolute::from_curve("ellipse", 4);
        for (i, &s) in ev.samples.iter().enumerate() {
            let t = 2.0 * PI * (i as f64) / 4.0;
            let pt = ev.at(t);
            assert!((pt[0] - s[0]).abs() < 1e-12);
            assert!((pt[1] - s[1]).abs() < 1e-12);
        }
    }

    #[test]
    fn involute_at_start_param_is_on_curve() {
        // At t = t₀ the string length is zero, so I(t₀) = C(t₀).
        let inv = Involute::new("circle", 0.0);
        let pt = inv.at(0.0);
        let (pos, _t, _n) = curve_geometry("circle", 0.0);
        assert!((pt[0] - pos[0]).abs() < 1e-10);
        assert!((pt[1] - pos[1]).abs() < 1e-10);
        assert!((pt[2] - pos[2]).abs() < 1e-10);
    }

    #[test]
    fn curve_offset_circle_radius() {
        // Offset of a unit circle by 0.5 should have all points at distance
        // 0.5 from the unit circle, i.e. at radius = 1 − 0.5 = 0.5 from origin
        // (the inward normal for a unit circle is (−cos t, −sin t, 0)).
        let off = CurveOffset::new("circle", 0.5);
        for i in 0..16 {
            let t = 2.0 * PI * (i as f64) / 16.0;
            let pt = off.evaluate(t);
            let r = (pt[0] * pt[0] + pt[1] * pt[1]).sqrt();
            assert!((r - 0.5).abs() < 1e-10, "radius at t={}: {}", t, r);
        }
    }

    #[test]
    fn curve_offset_d1_finite_difference_consistency() {
        // d1 must be a finite, non-zero vector for circle at t = 0.
        let off = CurveOffset::new("circle", 0.25);
        let d = off.d1(0.0);
        let mag = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        assert!(mag > 1e-6, "d1 magnitude too small: {}", mag);
    }

    #[test]
    fn offset_curve_points_count() {
        let pts = offset_curve_points("ellipse", 0.1, 12);
        assert_eq!(pts.len(), 12);
    }

    #[test]
    #[should_panic]
    fn evolute_n_zero_panics() {
        let _ = Evolute::from_curve("circle", 0);
    }

    #[test]
    #[should_panic]
    fn offset_curve_points_n_zero_panics() {
        let _ = offset_curve_points("circle", 1.0, 0);
    }
}
