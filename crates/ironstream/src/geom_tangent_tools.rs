// FILE: geom_tangent_tools.rs
//! Tangent/normal frame utilities — zero-dependency Rust stub modelled after
//! `LProp_CLProps` and `BRep_CurveRepresentation` from OpenCASCADE Technology.
//!
//! All types are plain-data structs with no external crate dependencies.
//! Parameter `t` throughout refers to the curve parameter (not arc-length).

// ---------------------------------------------------------------------------
// Private vector math helpers
// ---------------------------------------------------------------------------

#[inline]
fn vec_norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

#[inline]
fn vec_normalize(v: [f64; 3]) -> [f64; 3] {
    let mag = vec_norm(v);
    if mag < f64::EPSILON {
        [0.0, 0.0, 0.0]
    } else {
        [v[0] / mag, v[1] / mag, v[2] / mag]
    }
}

#[inline]
fn vec_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

// ---------------------------------------------------------------------------
// FrenetFrame
// ---------------------------------------------------------------------------

/// The Frenet-Serret moving frame at a point on a curve.
///
/// Stores the curve parameter, point, tangent, normal, curvature, and torsion.
/// The binormal is computed on demand as `tangent × normal`.
///
/// Setters normalise the tangent and normal vectors automatically.
///
// occt: LProp_CLProps / BRep_CurveRepresentation
#[derive(Debug, Clone, PartialEq)]
pub struct FrenetFrame {
    parameter: f64,
    point: [f64; 3],
    tangent: [f64; 3],
    normal: [f64; 3],
    curvature: f64,
    torsion: f64,
}

impl FrenetFrame {
    /// Construct a new frame at the given curve parameter with all vectors zeroed.
    // occt: LProp_CLProps
    pub fn new(parameter: f64) -> Self {
        Self {
            parameter,
            point: [0.0; 3],
            tangent: [0.0; 3],
            normal: [0.0; 3],
            curvature: 0.0,
            torsion: 0.0,
        }
    }

    // ── Getters ───────────────────────────────────────────────────────────────

    /// Return the curve parameter at which this frame was computed.
    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    /// Return the 3-D point on the curve.
    pub fn point(&self) -> [f64; 3] {
        self.point
    }

    /// Return the unit tangent vector.
    pub fn tangent(&self) -> [f64; 3] {
        self.tangent
    }

    /// Return the unit principal normal vector.
    pub fn normal(&self) -> [f64; 3] {
        self.normal
    }

    /// Return the curvature κ = |d²C/ds²|.
    pub fn curvature(&self) -> f64 {
        self.curvature
    }

    /// Return the torsion τ.
    pub fn torsion(&self) -> f64 {
        self.torsion
    }

    /// Return the unit binormal vector B = T × N (computed on demand).
    pub fn binormal(&self) -> [f64; 3] {
        vec_normalize(vec_cross(self.tangent, self.normal))
    }

    // ── Setters ───────────────────────────────────────────────────────────────

    /// Set the 3-D point.
    pub fn set_point(&mut self, p: [f64; 3]) {
        self.point = p;
    }

    /// Set the tangent vector. The vector is normalised before storing.
    pub fn set_tangent(&mut self, t: [f64; 3]) {
        self.tangent = vec_normalize(t);
    }

    /// Set the normal vector. The vector is normalised before storing.
    pub fn set_normal(&mut self, n: [f64; 3]) {
        self.normal = vec_normalize(n);
    }

    /// Set the curvature.
    pub fn set_curvature(&mut self, k: f64) {
        self.curvature = k;
    }

    /// Set the torsion.
    pub fn set_torsion(&mut self, tau: f64) {
        self.torsion = tau;
    }
}

// ---------------------------------------------------------------------------
// frenet_line — Frenet frame for a straight line
// ---------------------------------------------------------------------------

/// Construct the Frenet frame for a straight line at parameter `t`.
///
/// For a line `C(t) = origin + t * direction`:
/// - Point: `origin + t * unit_direction`
/// - Tangent: `unit_direction`
/// - Normal: zero vector (curvature = 0, so the Frenet normal is undefined)
/// - Curvature: 0
/// - Torsion: 0
///
// occt: LProp_CLProps
pub fn frenet_line(t: f64, direction: [f64; 3], origin: [f64; 3]) -> FrenetFrame {
    let unit_dir = vec_normalize(direction);
    let mut f = FrenetFrame::new(t);
    f.set_point([
        origin[0] + t * unit_dir[0],
        origin[1] + t * unit_dir[1],
        origin[2] + t * unit_dir[2],
    ]);
    // set_tangent normalises: unit_dir is already unit, so this is fine.
    f.tangent = unit_dir;
    // Normal is the zero vector for a straight line.
    f.normal = [0.0, 0.0, 0.0];
    f.curvature = 0.0;
    f.torsion = 0.0;
    f
}

// ---------------------------------------------------------------------------
// frenet_circle — Frenet frame for a circle in the XY plane
// ---------------------------------------------------------------------------

/// Construct the Frenet frame for a circle of radius `r` centred at `center`
/// in the XY plane, at parameter `t`.
///
/// Parameterisation follows OCCT `Geom_Circle`:
/// - `C(t) = center + r*(cos t, sin t, 0)`
/// - `T(t) = (-sin t, cos t, 0)`
/// - `N(t) = (-cos t, -sin t, 0)` (toward centre)
/// - `κ = 1/r` (or 0 for degenerate `r = 0`)
/// - `τ = 0`
///
// occt: LProp_CLProps / Geom_Circle
pub fn frenet_circle(t: f64, radius: f64, center: [f64; 3]) -> FrenetFrame {
    let ct = t.cos();
    let st = t.sin();

    let mut f = FrenetFrame::new(t);

    // Point on circle.
    f.set_point([
        center[0] + radius * ct,
        center[1] + radius * st,
        center[2],
    ]);

    // Tangent = (-sin t, cos t, 0) — already unit for any t.
    f.tangent = [-st, ct, 0.0];

    if radius.abs() < f64::EPSILON {
        // Degenerate circle: point at centre, curvature undefined → 0.
        f.set_point(center);
        f.tangent = [0.0, 0.0, 0.0];
        f.normal = [0.0, 0.0, 0.0];
        f.curvature = 0.0;
    } else {
        // Principal normal = (-cos t, -sin t, 0) points toward centre.
        f.normal = [-ct, -st, 0.0];
        f.curvature = 1.0 / radius;
    }

    f.torsion = 0.0;
    f
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-12;

    #[test]
    fn frenet_frame_defaults_zero() {
        let f = FrenetFrame::new(2.0);
        assert!((f.parameter() - 2.0).abs() < EPS);
        assert_eq!(f.point(), [0.0, 0.0, 0.0]);
        assert_eq!(f.tangent(), [0.0, 0.0, 0.0]);
        assert_eq!(f.normal(), [0.0, 0.0, 0.0]);
        assert!((f.curvature()).abs() < EPS);
        assert!((f.torsion()).abs() < EPS);
    }

    #[test]
    fn frenet_frame_set_tangent_normalises() {
        let mut f = FrenetFrame::new(0.0);
        f.set_tangent([3.0, 0.0, 0.0]);
        let t = f.tangent();
        assert!((t[0] - 1.0).abs() < EPS);
    }

    #[test]
    fn frenet_frame_binormal_cross_product() {
        let mut f = FrenetFrame::new(0.0);
        f.set_tangent([1.0, 0.0, 0.0]);
        f.set_normal([0.0, 1.0, 0.0]);
        let b = f.binormal();
        assert!((b[0]).abs() < EPS);
        assert!((b[1]).abs() < EPS);
        assert!((b[2] - 1.0).abs() < EPS);
    }

    #[test]
    fn frenet_line_zero_curvature() {
        let f = frenet_line(1.0, [1.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        assert!((f.curvature()).abs() < EPS);
        assert!((f.torsion()).abs() < EPS);
    }

    #[test]
    fn frenet_circle_curvature_inverse_radius() {
        let f = frenet_circle(0.0, 5.0, [0.0, 0.0, 0.0]);
        assert!((f.curvature() - 0.2).abs() < EPS);
        assert!((f.torsion()).abs() < EPS);
    }

    #[test]
    fn frenet_circle_binormal_z() {
        let f = frenet_circle(0.0, 1.0, [0.0, 0.0, 0.0]);
        let b = f.binormal();
        assert!((b[2] - 1.0).abs() < EPS);
    }
}
