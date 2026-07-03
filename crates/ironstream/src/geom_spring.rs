//! Helical spring / helix surface primitive for IronStream.
//!
//! OpenCASCADE does not ship a dedicated `Geom_Spring` class; instead, a spring
//! (helical pipe) surface is constructed by sweeping a circular profile along a
//! helix spine using `GeomFill_Pipe`.  This module provides a pure-Rust, zero-
//! dependency stub that captures the same geometric semantics:
//!
//! * [`Spring`] — a full spring (helix + radius metadata + turn count).
//! * [`Helix`] — the bare spine curve parameterised by angle `t` (radians).
//! * [`spring_points`] — convenience function that samples the spine uniformly.
//!
//! All arithmetic uses only `std` (`f64::sin`, `f64::cos`, `f64::sqrt`,
//! `std::f64::consts::PI`).  No external crates are used.
//!
//! The parameter `t` is the **total rotation angle in radians** measured from
//! the start of the helix.  One full turn corresponds to `t = 2*PI`.
//!
//! Coordinate convention (matches OCCT / OpenGL right-hand rule):
//!
//! ```text
//! X(t) = radius * cos(t)
//! Y(t) = radius * sin(t)
//! Z(t) = (pitch / (2*PI)) * t
//! ```

use std::f64::consts::PI;

// ─────────────────────────────────────────────────────────────────────────────
// Spring
// ─────────────────────────────────────────────────────────────────────────────

/// A helical spring primitive.
///
/// The spring is modelled as a helix spine that could be swept with a circular
/// cross-section to produce a solid spring surface.
///
/// `axis` is the unit direction vector along which the helix advances (the
/// "Z axis" of the spring); for a standard upright spring this is `[0, 0, 1]`.
/// The helix is parameterised by the total angle `t ∈ [0, turns * 2*PI]`.
///
/// ```text
/// P(t) = radius*cos(t)*X + radius*sin(t)*Y + (pitch/(2*PI))*t * axis
/// ```
// occt: GeomFill_Pipe
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spring {
    /// Unit direction vector of the helix axis (advance direction).
    pub axis: [f64; 3],
    /// Rise per full turn (distance along the axis per revolution).
    pub pitch: f64,
    /// Helix radius (distance from the axis to the spine).
    pub radius: f64,
    /// Total number of turns.
    pub turns: f64,
}

impl Spring {
    /// Construct a standard upright spring with `axis = [0, 0, 1]`.
    ///
    /// # Arguments
    /// * `radius` — distance from the axis to the helix spine (≥ 0).
    /// * `pitch`  — axial rise per full turn.
    /// * `turns`  — total number of revolutions.
    pub fn new(radius: f64, pitch: f64, turns: f64) -> Self {
        Self {
            axis: [0.0, 0.0, 1.0],
            pitch,
            radius,
            turns,
        }
    }

    /// Evaluate the 3D position on the helix spine at parameter `t` (radians).
    ///
    /// `t = 0` gives the start point; `t = turns * 2*PI` gives the end point.
    ///
    /// Returns `[x, y, z]` in world coordinates using the stored `axis`.
    pub fn point_at(&self, t: f64) -> [f64; 3] {
        helix_point(self.radius, self.pitch, &self.axis, t)
    }

    /// Unit tangent vector of the helix spine at parameter `t`.
    ///
    /// The tangent is the normalised first derivative `dP/dt`.
    pub fn tangent_at(&self, t: f64) -> [f64; 3] {
        helix_tangent(self.radius, self.pitch, &self.axis, t)
    }

    /// Approximate total arc length of the spring spine.
    ///
    /// For a helix the arc-length element is constant:
    ///
    /// ```text
    /// ds/dt = sqrt(radius^2 + (pitch / (2*PI))^2)
    /// ```
    ///
    /// so the total length is `ds/dt * turns * 2*PI`.
    pub fn total_length(&self) -> f64 {
        helix_arc_length(self.radius, self.pitch, self.turns * 2.0 * PI)
    }

    /// Sample `n` evenly-spaced points along the spring spine.
    ///
    /// `t` ranges from `0` to `turns * 2*PI` inclusive, so `n` points are
    /// returned with `n - 1` equal intervals (or a single point at `t = 0`
    /// when `n == 1`).
    ///
    /// # Panics
    /// Panics if `n == 0`.
    pub fn num_points(&self, n: usize) -> Vec<[f64; 3]> {
        assert!(n > 0, "Spring::num_points: n must be > 0");
        spring_points(self.radius, self.pitch, self.turns, n)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Helix
// ─────────────────────────────────────────────────────────────────────────────

/// A bare helical spine curve parameterised by angle `t` (radians).
///
/// The helix is always aligned with the Z axis:
///
/// ```text
/// X(t) = radius * cos(t)
/// Y(t) = radius * sin(t)
/// Z(t) = (pitch / (2*PI)) * t
/// ```
///
/// Use [`Spring`] when you also need turn count, axis direction, and tangent /
/// arc-length helpers.
// occt: GeomFill_Pipe
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Helix {
    /// Distance from the Z axis to the helix spine.
    pub radius: f64,
    /// Axial rise per full revolution.
    pub pitch: f64,
}

impl Helix {
    /// Construct a helix with the given `radius` and `pitch`.
    pub fn new(r: f64, p: f64) -> Self {
        Self { radius: r, pitch: p }
    }

    /// Evaluate the helix position at angle `t` (radians).
    ///
    /// Returns `[radius*cos(t), radius*sin(t), (pitch/(2*PI))*t]`.
    pub fn at(&self, t: f64) -> [f64; 3] {
        let rise = self.pitch / (2.0 * PI);
        [self.radius * t.cos(), self.radius * t.sin(), rise * t]
    }

    /// Approximate arc length of the helix between angles `t0` and `t1`
    /// (radians).
    ///
    /// The arc-length element of a helix is constant:
    ///
    /// ```text
    /// ds/dt = sqrt(radius^2 + (pitch / (2*PI))^2)
    /// ```
    ///
    /// so `arc_length(t0, t1) = ds/dt * |t1 - t0|`.
    pub fn arc_length(&self, t0: f64, t1: f64) -> f64 {
        helix_arc_length(self.radius, self.pitch, (t1 - t0).abs())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Free function
// ─────────────────────────────────────────────────────────────────────────────

/// Sample `n` evenly-spaced points along a standard upright spring spine
/// (axis = `[0, 0, 1]`).
///
/// `t` ranges from `0` to `turns * 2*PI` inclusive.
///
/// # Arguments
/// * `radius` — helix radius.
/// * `pitch`  — axial rise per revolution.
/// * `turns`  — total number of turns.
/// * `n`      — number of sample points (must be > 0).
///
/// # Panics
/// Panics if `n == 0`.
pub fn spring_points(radius: f64, pitch: f64, turns: f64, n: usize) -> Vec<[f64; 3]> {
    assert!(n > 0, "spring_points: n must be > 0");
    let axis = [0.0f64, 0.0, 1.0];
    let t_max = turns * 2.0 * PI;
    (0..n)
        .map(|i| {
            let t = if n == 1 {
                0.0
            } else {
                t_max * (i as f64) / ((n - 1) as f64)
            };
            helix_point(radius, pitch, &axis, t)
        })
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Evaluate the helix position for a given axis direction at angle `t`.
///
/// The "X" direction of the plane perpendicular to `axis` is found by
/// constructing an arbitrary perpendicular.  For the default axis `[0, 0, 1]`
/// the result reduces to `[r*cos(t), r*sin(t), rise*t]`.
fn helix_point(radius: f64, pitch: f64, axis: &[f64; 3], t: f64) -> [f64; 3] {
    let rise = pitch / (2.0 * PI);
    let (xd, yd) = perpendicular_frame(axis);
    let (ct, st) = (t.cos(), t.sin());
    [
        radius * ct * xd[0] + radius * st * yd[0] + rise * t * axis[0],
        radius * ct * xd[1] + radius * st * yd[1] + rise * t * axis[1],
        radius * ct * xd[2] + radius * st * yd[2] + rise * t * axis[2],
    ]
}

/// Normalised tangent of the helix at angle `t` (`dP/dt` normalised).
fn helix_tangent(radius: f64, pitch: f64, axis: &[f64; 3], t: f64) -> [f64; 3] {
    let rise = pitch / (2.0 * PI);
    let (xd, yd) = perpendicular_frame(axis);
    let (ct, st) = (t.cos(), t.sin());
    // dP/dt = radius*(-sin(t))*xd + radius*cos(t)*yd + rise*axis
    let dx = -radius * st * xd[0] + radius * ct * yd[0] + rise * axis[0];
    let dy = -radius * st * xd[1] + radius * ct * yd[1] + rise * axis[1];
    let dz = -radius * st * xd[2] + radius * ct * yd[2] + rise * axis[2];
    let len = (dx * dx + dy * dy + dz * dz).sqrt();
    if len == 0.0 {
        [0.0, 0.0, 0.0]
    } else {
        [dx / len, dy / len, dz / len]
    }
}

/// Arc length of a helix over an angular span of `delta_t` radians.
#[inline]
fn helix_arc_length(radius: f64, pitch: f64, delta_t: f64) -> f64 {
    let rise = pitch / (2.0 * PI);
    (radius * radius + rise * rise).sqrt() * delta_t
}

/// Build an orthonormal pair `(x_dir, y_dir)` that is perpendicular to `axis`.
///
/// `axis` is assumed to be a unit vector.  The X direction is chosen as the
/// projection of a global reference vector onto the plane perpendicular to
/// `axis`, so that the result is well-defined and continuous.  For the default
/// `axis = [0, 0, 1]` this yields `xd = [1, 0, 0]`, `yd = [0, 1, 0]`.
fn perpendicular_frame(axis: &[f64; 3]) -> ([f64; 3], [f64; 3]) {
    let [ax, ay, az] = *axis;
    // Pick the global axis whose component along `axis` is smallest in
    // magnitude; projecting it onto the perpendicular plane gives the largest
    // (most numerically stable) result.
    let candidate = if ax.abs() <= ay.abs() && ax.abs() <= az.abs() {
        [1.0f64, 0.0, 0.0]
    } else if ay.abs() <= az.abs() {
        [0.0f64, 1.0, 0.0]
    } else {
        [0.0f64, 0.0, 1.0]
    };
    // Project `candidate` onto the plane perpendicular to `axis`:
    //   v = candidate - (candidate · axis) * axis
    let dot = candidate[0] * ax + candidate[1] * ay + candidate[2] * az;
    let v = [
        candidate[0] - dot * ax,
        candidate[1] - dot * ay,
        candidate[2] - dot * az,
    ];
    let xd = normalise(v);
    // yd completes the right-handed frame: yd = axis × xd
    let yd = cross(axis, &xd);
    (xd, yd)
}

#[inline]
fn cross(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
fn normalise(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len == 0.0 {
        v
    } else {
        [v[0] / len, v[1] / len, v[2] / len]
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    fn vec_approx_eq(a: [f64; 3], b: [f64; 3]) -> bool {
        approx_eq(a[0], b[0]) && approx_eq(a[1], b[1]) && approx_eq(a[2], b[2])
    }

    #[test]
    fn helix_at_zero() {
        let h = Helix::new(1.0, 1.0);
        let p = h.at(0.0);
        assert!(vec_approx_eq(p, [1.0, 0.0, 0.0]));
    }

    #[test]
    fn helix_at_half_turn() {
        let h = Helix::new(1.0, 2.0);
        let p = h.at(PI);
        // cos(PI) = -1, sin(PI) = 0, z = (2/(2*PI))*PI = 1
        assert!(approx_eq(p[0], -1.0));
        assert!(approx_eq(p[1], 0.0));
        assert!(approx_eq(p[2], 1.0));
    }

    #[test]
    fn helix_arc_length_one_turn() {
        let h = Helix::new(1.0, 2.0 * PI);
        // rise = 1.0, so ds/dt = sqrt(1+1), total = sqrt(2)*2*PI
        let expected = (2.0f64).sqrt() * 2.0 * PI;
        let got = h.arc_length(0.0, 2.0 * PI);
        assert!(approx_eq(got, expected));
    }

    #[test]
    fn spring_new_defaults() {
        let s = Spring::new(2.0, 5.0, 3.0);
        assert_eq!(s.axis, [0.0, 0.0, 1.0]);
        assert!(approx_eq(s.radius, 2.0));
        assert!(approx_eq(s.pitch, 5.0));
        assert!(approx_eq(s.turns, 3.0));
    }

    #[test]
    fn spring_point_at_zero() {
        let s = Spring::new(3.0, 1.0, 2.0);
        let p = s.point_at(0.0);
        assert!(approx_eq(p[0], 3.0));
        assert!(approx_eq(p[1], 0.0));
        assert!(approx_eq(p[2], 0.0));
    }

    #[test]
    fn spring_total_length() {
        let r = 1.0f64;
        let pitch = 2.0 * PI; // rise per turn = 1
        let turns = 1.0;
        let s = Spring::new(r, pitch, turns);
        // ds/dt = sqrt(r^2 + 1^2) = sqrt(2), total_angle = 2*PI
        let expected = (2.0f64).sqrt() * 2.0 * PI;
        assert!(approx_eq(s.total_length(), expected));
    }

    #[test]
    fn spring_num_points_count() {
        let s = Spring::new(1.0, 1.0, 2.0);
        let pts = s.num_points(11);
        assert_eq!(pts.len(), 11);
    }

    #[test]
    fn spring_num_points_endpoints() {
        let s = Spring::new(1.0, 1.0, 1.0);
        let pts = s.num_points(2);
        // first point: t=0 → [1, 0, 0]
        assert!(approx_eq(pts[0][0], 1.0));
        assert!(approx_eq(pts[0][1], 0.0));
        assert!(approx_eq(pts[0][2], 0.0));
        // last point: t=2*PI → [1, 0, pitch/1]
        assert!(approx_eq(pts[1][0], 1.0));
        assert!(approx_eq(pts[1][1], 0.0));
        assert!(approx_eq(pts[1][2], 1.0));
    }

    #[test]
    fn spring_points_free_fn() {
        let pts = spring_points(1.0, 1.0, 1.0, 5);
        assert_eq!(pts.len(), 5);
    }

    #[test]
    fn spring_tangent_unit_length() {
        let s = Spring::new(2.0, 1.0, 3.0);
        for i in 0..8 {
            let t = i as f64 * PI / 4.0;
            let tan = s.tangent_at(t);
            let len = (tan[0] * tan[0] + tan[1] * tan[1] + tan[2] * tan[2]).sqrt();
            assert!(approx_eq(len, 1.0), "tangent not unit at t={t}: len={len}");
        }
    }

    #[test]
    #[should_panic]
    fn spring_num_points_zero_panics() {
        let s = Spring::new(1.0, 1.0, 1.0);
        let _ = s.num_points(0);
    }

    #[test]
    #[should_panic]
    fn spring_points_zero_panics() {
        let _ = spring_points(1.0, 1.0, 1.0, 0);
    }
}
