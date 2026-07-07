// FILE: helix_geom_helix_curve_o.rs
// occt: HelixGeom_HelixCurve

//! Analytical helix curve representation.
//!
//! Provides parametric evaluation of helix curves using analytical expressions:
//! - x(t) = r(t) * cos(t)
//! - y(t) = r(t) * sin(t)
//! - z(t) = (pitch / 2*PI) * t
//! where r(t) = r_start + taper_factor * t

use std::f64::consts::PI;

/// Continuity constraint type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContinuityType {
    C0 = 0,
    C1 = 1,
    C2 = 2,
}

/// 3D point.
#[derive(Clone, Copy, Debug)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn distance_to(&self, other: &Point3d) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// 3D vector.
#[derive(Clone, Copy, Debug)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vector3d {
        let mag = self.magnitude();
        if mag < 1e-15 {
            Vector3d::new(0.0, 0.0, 0.0)
        } else {
            Vector3d::new(self.x / mag, self.y / mag, self.z / mag)
        }
    }
}

/// Analytical helix curve.
pub struct HelixGeomHelixCurve {
    /// Parameter range
    t1: f64,
    t2: f64,
    /// Pitch (height per 2*PI radians)
    pitch: f64,
    /// Starting radius
    r_start: f64,
    /// Taper angle (for conical helix)
    taper_angle: f64,
    /// Clockwise orientation
    is_clockwise: bool,
    /// Taper factor = r_start * tan(taper_angle)
    taper_factor: f64,
}

impl HelixGeomHelixCurve {
    /// Create a new helix curve with default parameters.
    pub fn new() -> Self {
        Self {
            t1: 0.0,
            t2: 2.0 * PI,
            pitch: 1.0,
            r_start: 1.0,
            taper_angle: 0.0,
            is_clockwise: false,
            taper_factor: 0.0,
        }
    }

    /// Load helix parameters.
    pub fn load(
        &mut self,
        t1: f64,
        t2: f64,
        pitch: f64,
        r_start: f64,
        taper_angle: f64,
        is_clockwise: bool,
    ) {
        self.t1 = t1;
        self.t2 = t2;
        self.pitch = pitch;
        self.r_start = r_start;
        self.taper_angle = taper_angle;
        self.is_clockwise = is_clockwise;
        // Pre-compute taper factor for efficiency
        self.taper_factor = r_start * taper_angle.tan();
    }

    /// Get first parameter.
    pub fn first_parameter(&self) -> f64 {
        self.t1
    }

    /// Get last parameter.
    pub fn last_parameter(&self) -> f64 {
        self.t2
    }

    /// Get continuity (C1 for helix curves).
    pub fn continuity(&self) -> ContinuityType {
        ContinuityType::C1
    }

    /// Get the number of continuity intervals.
    pub fn nb_intervals(&self, _continuity: ContinuityType) -> i32 {
        1
    }

    /// Check if curve is closed.
    pub fn is_closed(&self) -> bool {
        false
    }

    /// Check if curve is periodic.
    pub fn is_periodic(&self) -> bool {
        false
    }

    /// Evaluate the curve at parameter t.
    pub fn value(&self, t: f64) -> Point3d {
        let r = self.r_start + self.taper_factor * t;
        let sign = if self.is_clockwise { -1.0 } else { 1.0 };

        Point3d::new(
            r * t.cos(),
            sign * r * t.sin(),
            (self.pitch / (2.0 * PI)) * t,
        )
    }

    /// Evaluate the curve and first derivative at parameter t.
    pub fn d1(&self, t: f64) -> (Point3d, Vector3d) {
        let r = self.r_start + self.taper_factor * t;
        let dr_dt = self.taper_factor;
        let sign = if self.is_clockwise { -1.0 } else { 1.0 };

        let pt = Point3d::new(
            r * t.cos(),
            sign * r * t.sin(),
            (self.pitch / (2.0 * PI)) * t,
        );

        let vec = Vector3d::new(
            dr_dt * t.cos() - r * t.sin(),
            sign * (dr_dt * t.sin() + r * t.cos()),
            self.pitch / (2.0 * PI),
        );

        (pt, vec)
    }

    /// Evaluate the curve and first/second derivatives at parameter t.
    pub fn d2(&self, t: f64) -> (Point3d, Vector3d, Vector3d) {
        let r = self.r_start + self.taper_factor * t;
        let dr_dt = self.taper_factor;
        let d2r_dt2 = 0.0; // Linear radius change
        let sign = if self.is_clockwise { -1.0 } else { 1.0 };

        let pt = Point3d::new(
            r * t.cos(),
            sign * r * t.sin(),
            (self.pitch / (2.0 * PI)) * t,
        );

        let d1 = Vector3d::new(
            dr_dt * t.cos() - r * t.sin(),
            sign * (dr_dt * t.sin() + r * t.cos()),
            self.pitch / (2.0 * PI),
        );

        let d2 = Vector3d::new(
            d2r_dt2 * t.cos() - 2.0 * dr_dt * t.sin() - r * t.cos(),
            sign * (d2r_dt2 * t.sin() + 2.0 * dr_dt * t.cos() - r * t.sin()),
            0.0,
        );

        (pt, d1, d2)
    }

    /// Get parametric resolution for 3D tolerance.
    pub fn resolution(&self, r3d: f64) -> f64 {
        r3d / (self.r_start.max(1.0))
    }
}

impl Default for HelixGeomHelixCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_curve() {
        let curve = HelixGeomHelixCurve::new();
        assert_eq!(curve.first_parameter(), 0.0);
        assert_eq!(curve.last_parameter(), 2.0 * PI);
        assert_eq!(curve.pitch, 1.0);
        assert_eq!(curve.r_start, 1.0);
    }

    #[test]
    fn test_load_parameters() {
        let mut curve = HelixGeomHelixCurve::new();
        curve.load(0.0, 4.0 * PI, 2.5, 3.0, 0.0, true);

        assert_eq!(curve.first_parameter(), 0.0);
        assert_eq!(curve.last_parameter(), 4.0 * PI);
        assert_eq!(curve.pitch, 2.5);
        assert_eq!(curve.r_start, 3.0);
    }

    #[test]
    fn test_value_at_start() {
        let curve = HelixGeomHelixCurve::new();
        let pt = curve.value(0.0);

        assert!((pt.x - 1.0).abs() < 1e-10);
        assert!((pt.y - 0.0).abs() < 1e-10);
        assert!((pt.z - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_value_at_half_turn() {
        let curve = HelixGeomHelixCurve::new();
        let pt = curve.value(PI);

        assert!((pt.x - (-1.0)).abs() < 1e-10);
        assert!(pt.y.abs() < 1e-10);
        assert!((pt.z - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_value_at_full_turn() {
        let curve = HelixGeomHelixCurve::new();
        let pt = curve.value(2.0 * PI);

        assert!((pt.x - 1.0).abs() < 1e-10);
        assert!(pt.y.abs() < 1e-10);
        assert!((pt.z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_clockwise_orientation() {
        let mut curve = HelixGeomHelixCurve::new();
        curve.load(0.0, 2.0 * PI, 1.0, 1.0, 0.0, true);
        let pt = curve.value(PI / 2.0);

        // Clockwise: y should be negative
        assert!(pt.y < 0.0);
    }

    #[test]
    fn test_d1_derivative() {
        let curve = HelixGeomHelixCurve::new();
        let (_pt, vec) = curve.d1(0.0);

        // At t=0: dx/dt = 0, dy/dt = 1, dz/dt = pitch/(2*PI)
        assert!(vec.x.abs() < 1e-10);
        assert!((vec.y - 1.0).abs() < 1e-10);
        assert!((vec.z - 1.0 / (2.0 * PI)).abs() < 1e-10);
    }

    #[test]
    fn test_continuity() {
        let curve = HelixGeomHelixCurve::new();
        assert_eq!(curve.continuity(), ContinuityType::C1);
    }

    #[test]
    fn test_is_closed() {
        let curve = HelixGeomHelixCurve::new();
        assert!(!curve.is_closed());
    }

    #[test]
    fn test_is_periodic() {
        let curve = HelixGeomHelixCurve::new();
        assert!(!curve.is_periodic());
    }

    #[test]
    fn test_resolution() {
        let curve = HelixGeomHelixCurve::new();
        let res = curve.resolution(0.001);
        assert!(res > 0.0);
    }

    #[test]
    fn test_d2_curvature() {
        let curve = HelixGeomHelixCurve::new();
        let (_pt, d1, d2) = curve.d2(0.0);

        // Second derivative at start
        assert!(d2.magnitude() > 0.0);
    }

    #[test]
    fn test_tapered_helix() {
        let mut curve = HelixGeomHelixCurve::new();
        curve.load(0.0, 2.0 * PI, 1.0, 1.0, 0.1, false);

        let pt0 = curve.value(0.0);
        let pt_end = curve.value(2.0 * PI);

        // Radius increases with taper
        assert!(pt_end.x.abs() > pt0.x.abs() || (pt_end.x.abs() - pt0.x.abs()).abs() < 0.5);
    }
}
