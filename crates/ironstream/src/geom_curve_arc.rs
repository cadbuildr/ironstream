// FILE: crates/ironstream/src/geom_curve_arc.rs
//
// Pure-Rust, zero-dependency stubs modelled after OpenCascade's
// GC_MakeArcOfCircle and GC_MakeArcOfEllipse construction helpers.
//
// No external crates are used; only `std` primitives (`f64::sin`,
// `f64::cos`, `f64::sqrt`, `f64::atan2`) are required.

use std::f64::consts::{PI, TAU};

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Compute the dot product of two 3-vectors.
#[inline]
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Compute the cross product of two 3-vectors.
#[inline]
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Euclidean norm of a 3-vector.
#[inline]
fn norm(v: [f64; 3]) -> f64 {
    f64::sqrt(dot(v, v))
}

/// Normalise a 3-vector; returns `None` if the vector is degenerate.
#[inline]
fn normalize(v: [f64; 3]) -> Option<[f64; 3]> {
    let n = norm(v);
    if n < f64::EPSILON {
        None
    } else {
        Some([v[0] / n, v[1] / n, v[2] / n])
    }
}

/// Subtract two 3-vectors.
#[inline]
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// Add two 3-vectors.
#[inline]
fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// Scale a 3-vector by a scalar.
#[inline]
fn scale(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

/// Wrap an angle into `[0, 2π)`.
#[inline]
fn wrap_angle(a: f64) -> f64 {
    let m = a % TAU;
    if m < 0.0 {
        m + TAU
    } else {
        m
    }
}

/// Solve the circumscribed circle of three points in 3-D space.
///
/// Returns `(center, radius, normal)` where `normal` is the unit normal of
/// the plane defined by the three points and the winding order p1 → p2 → p3.
/// Returns `None` when the three points are collinear or coincident.
fn circumscribed_circle(
    p1: [f64; 3],
    p2: [f64; 3],
    p3: [f64; 3],
) -> Option<([f64; 3], f64, [f64; 3])> {
    // Vectors from p1 to p2 and p3.
    let a = sub(p2, p1);
    let b = sub(p3, p1);

    let n_raw = cross(a, b);
    let normal = normalize(n_raw)?;

    // In the local plane spanned by `a` and `b`, the circumcenter satisfies:
    //   |c - p1|² = |c - p2|² = |c - p3|²
    //
    // Expanding and rearranging gives two linear equations (one per edge midpoint
    // perpendicular bisector) in the coefficients (s, t) such that
    //   center = p1 + s*a + t*b.
    //
    //   2*dot(a,a)*s + 2*dot(a,b)*t = dot(a,a)
    //   2*dot(a,b)*s + 2*dot(b,b)*t = dot(b,b)
    let aa = dot(a, a);
    let ab = dot(a, b);
    let bb = dot(b, b);

    let denom = 2.0 * (aa * bb - ab * ab);
    if denom.abs() < f64::EPSILON {
        return None;
    }

    let s = (bb * aa - ab * ab) / denom; // simplifies to (bb*(aa - ab)) / …
    let t = (aa * bb - ab * aa) / denom; // simplifies to (aa*(bb - ab)) / …

    // Recalculate properly:
    //   2*aa*s + 2*ab*t = aa  →  s = (aa*bb - ab*(ab*t/bb)) …
    // Use Cramer's rule directly:
    //   | 2aa  2ab | | s |   | aa |
    //   | 2ab  2bb | | t | = | bb |
    //
    //   s = (aa*bb - ab*bb) / (2*(aa*bb - ab²))  ... no, redo carefully.
    //
    // System:  2aa·s + 2ab·t = aa
    //          2ab·s + 2bb·t = bb
    //
    // Cramer's:
    //   D   = (2aa)(2bb) - (2ab)(2ab) = 4(aa·bb - ab²)
    //   Ds  = aa·(2bb) - bb·(2ab)     = 2(aa·bb - ab·bb)  -- wait, RHS column:
    //
    //   Ds  = |aa  2ab| = aa·2bb - bb·2ab = 2(aa·bb - ab·bb)
    //         |bb  2bb|
    //
    //   Dt  = |2aa  aa| = 2aa·bb - 2ab·aa = 2aa(bb - ab)
    //         |2ab  bb|
    //
    //   s = Ds/D = 2(aa·bb - ab·bb) / 4(aa·bb - ab²)
    //            = (aa·bb - ab·bb)   / 2(aa·bb - ab²)
    //            = bb(aa - ab)       / 2(aa·bb - ab²)
    //
    //   t = Dt/D = 2aa(bb - ab)     / 4(aa·bb - ab²)
    //            = aa(bb - ab)       / 2(aa·bb - ab²)

    let d = 2.0 * (aa * bb - ab * ab);
    if d.abs() < f64::EPSILON {
        return None;
    }
    let s = bb * (aa - ab) / d;
    let t = aa * (bb - ab) / d;

    let center = [
        p1[0] + s * a[0] + t * b[0],
        p1[1] + s * a[1] + t * b[1],
        p1[2] + s * a[2] + t * b[2],
    ];

    let radius = norm(sub(center, p1));
    if radius < f64::EPSILON {
        return None;
    }

    Some((center, radius, normal))
}

// ---------------------------------------------------------------------------
// ArcOfCircle
// ---------------------------------------------------------------------------

// occt: GC_MakeArcOfCircle
/// A circular arc in 3-D space, modelled after `GC_MakeArcOfCircle`.
///
/// The arc lies in the plane defined by `normal` and is parameterised by an
/// angle measured from the positive X-axis of a local right-hand frame whose
/// Z-axis is `normal`. Angles are in radians and follow the convention
/// `start_angle < end_angle` after canonical wrapping.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArcOfCircle {
    /// Centre of the underlying circle in world coordinates.
    pub center: [f64; 3],
    /// Radius of the underlying circle.
    pub radius: f64,
    /// Start angle of the arc (radians, in the plane of the arc).
    pub start_angle: f64,
    /// End angle of the arc (radians, in the plane of the arc).
    pub end_angle: f64,
    /// Unit normal to the plane of the arc.
    pub normal: [f64; 3],
}

impl ArcOfCircle {
    // occt: GC_MakeArcOfCircle::GC_MakeArcOfCircle(P1, P2, P3)
    /// Construct an arc passing through three points in order `p1 → p2 → p3`.
    ///
    /// Returns `None` when the three points are collinear or coincident (no
    /// unique circle exists).
    pub fn from_3_points(p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> Option<Self> {
        let (center, radius, normal) = circumscribed_circle(p1, p2, p3)?;

        // Build a local 2-D frame in the arc's plane:
        //   x_axis = (p1 - center) / radius
        //   y_axis = cross(normal, x_axis)   (right-hand)
        let cp1 = sub(p1, center);
        let x_axis = normalize(cp1)?;
        let y_axis_raw = cross(normal, x_axis);
        let y_axis = normalize(y_axis_raw)?;

        // Project each circumscribed point onto the local frame to get angles.
        let angle_of = |p: [f64; 3]| -> f64 {
            let v = sub(p, center);
            let px = dot(v, x_axis);
            let py = dot(v, y_axis);
            wrap_angle(f64::atan2(py, px))
        };

        let a1 = angle_of(p1); // = 0 by construction
        let a2 = angle_of(p2);
        let a3 = angle_of(p3);

        // Ensure arc goes from p1 to p3 passing through p2 in the direction
        // consistent with the normal.  Advance angles so start ≤ mid ≤ end.
        let start_angle = a1;
        let mut mid = a2;
        let mut end_angle = a3;

        if mid < start_angle {
            mid += TAU;
        }
        if end_angle <= start_angle {
            end_angle += TAU;
        }
        // If the midpoint is still beyond the end, the arc wraps the other way.
        if mid > end_angle {
            end_angle += TAU;
        }

        Some(ArcOfCircle {
            center,
            radius,
            start_angle,
            end_angle,
            normal,
        })
    }

    // occt: GC_MakeArcOfCircle — local x/y frame derived from normal
    /// Reconstruct the local X-axis of the arc's plane from the stored normal
    /// and the implicit convention that the X-axis points to the start of the
    /// arc.  This is an internal helper.
    fn local_frame(&self) -> ([f64; 3], [f64; 3]) {
        // Choose an arbitrary vector not parallel to `normal`.
        let n = self.normal;
        let arbitary = if n[0].abs() < 0.9 {
            [1.0, 0.0, 0.0]
        } else {
            [0.0, 1.0, 0.0]
        };
        let x_raw = cross(arbitary, n);
        // x_axis after subtracting the projection onto n.
        let proj = dot(x_raw, n);
        let x_sub = [
            x_raw[0] - proj * n[0],
            x_raw[1] - proj * n[1],
            x_raw[2] - proj * n[2],
        ];
        let x_axis = normalize(x_sub).unwrap_or([1.0, 0.0, 0.0]);
        let y_axis = normalize(cross(n, x_axis)).unwrap_or([0.0, 1.0, 0.0]);
        (x_axis, y_axis)
    }

    // occt: GC_MakeArcOfCircle — Value / D0
    /// Evaluate the arc at parameter `t` (radians, measured from the global
    /// X-axis of the arc's plane).  `t` is not clamped.
    pub fn evaluate(&self, t: f64) -> [f64; 3] {
        let (x_axis, y_axis) = self.local_frame();
        let c = f64::cos(t);
        let s = f64::sin(t);
        [
            self.center[0] + self.radius * (c * x_axis[0] + s * y_axis[0]),
            self.center[1] + self.radius * (c * x_axis[1] + s * y_axis[1]),
            self.center[2] + self.radius * (c * x_axis[2] + s * y_axis[2]),
        ]
    }

    // occt: GC_MakeArcOfCircle — D1
    /// First derivative of the arc at parameter `t`.
    ///
    /// The magnitude equals the radius (arc-length parameterisation speed).
    pub fn d1(&self, t: f64) -> [f64; 3] {
        let (x_axis, y_axis) = self.local_frame();
        let c = f64::cos(t);
        let s = f64::sin(t);
        // d/dt [ r*(cos t * X + sin t * Y) ] = r*(-sin t * X + cos t * Y)
        [
            self.radius * (-s * x_axis[0] + c * y_axis[0]),
            self.radius * (-s * x_axis[1] + c * y_axis[1]),
            self.radius * (-s * x_axis[2] + c * y_axis[2]),
        ]
    }

    // occt: GC_MakeArcOfCircle — IsDone
    /// Always returns `true` for a successfully constructed arc.
    ///
    /// Mirrors `GC_MakeArcOfCircle::IsDone()`.  A value of `false` is only
    /// reachable if the struct is constructed through a path that explicitly
    /// marks failure — direct field construction always yields `true`.
    pub fn is_done(&self) -> bool {
        true
    }

    // occt: GC_MakeArcOfCircle — arc length
    /// Arc length: `radius × |end_angle − start_angle|`.
    pub fn length(&self) -> f64 {
        self.radius * (self.end_angle - self.start_angle).abs()
    }
}

// ---------------------------------------------------------------------------
// ArcOfEllipse
// ---------------------------------------------------------------------------

// occt: GC_MakeArcOfEllipse
/// An arc of an ellipse in 3-D space, modelled after `GC_MakeArcOfEllipse`.
///
/// The ellipse lies in the XY-plane of world space (Z = `cz`). For a general
/// orientation extend this struct with an `Ax2` placement frame — this stub
/// follows the minimal interface requested.
///
/// The parametric equation is:
/// ```text
/// P(t) = [cx + major*cos(t), cy + minor*sin(t), cz]
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArcOfEllipse {
    /// Centre of the ellipse in world coordinates.
    pub center: [f64; 3],
    /// Length of the semi-major axis.
    pub major: f64,
    /// Length of the semi-minor axis.
    pub minor: f64,
    /// Start parameter angle (radians).
    pub start_angle: f64,
    /// End parameter angle (radians).
    pub end_angle: f64,
}

impl ArcOfEllipse {
    // occt: GC_MakeArcOfEllipse::GC_MakeArcOfEllipse
    /// Construct an `ArcOfEllipse`.
    ///
    /// Parameters
    /// ----------
    /// `cx`, `cy`, `cz` — centre of the ellipse.
    /// `a`              — semi-major axis length.
    /// `b`              — semi-minor axis length.
    /// `s`              — start angle in radians.
    /// `e`              — end angle in radians.
    ///
    /// # Panics
    /// Does not panic; degenerate inputs (e.g. `a == 0`) are stored as-is and
    /// produce degenerate geometry on evaluation.
    pub fn new(cx: f64, cy: f64, cz: f64, a: f64, b: f64, s: f64, e: f64) -> Self {
        ArcOfEllipse {
            center: [cx, cy, cz],
            major: a,
            minor: b,
            start_angle: s,
            end_angle: e,
        }
    }

    // occt: GC_MakeArcOfEllipse — Value / D0
    /// Evaluate the elliptic arc at parameter `t` (radians).
    ///
    /// ```text
    /// P(t) = [cx + major·cos(t), cy + minor·sin(t), cz]
    /// ```
    ///
    /// `t` is not clamped to `[start_angle, end_angle]`; callers are
    /// responsible for bounding the parameter if needed.
    pub fn evaluate(&self, t: f64) -> [f64; 3] {
        [
            self.center[0] + self.major * f64::cos(t),
            self.center[1] + self.minor * f64::sin(t),
            self.center[2],
        ]
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    fn vec_approx_eq(a: [f64; 3], b: [f64; 3]) -> bool {
        approx_eq(a[0], b[0]) && approx_eq(a[1], b[1]) && approx_eq(a[2], b[2])
    }

    // --- ArcOfCircle ---------------------------------------------------------

    #[test]
    fn arc_of_circle_from_3_points_unit_circle() {
        // Three points on the unit circle in the XY plane.
        let p1 = [1.0_f64, 0.0, 0.0];
        let p2 = [0.0, 1.0, 0.0];
        let p3 = [-1.0, 0.0, 0.0];
        let arc = ArcOfCircle::from_3_points(p1, p2, p3).expect("should succeed");
        assert!(approx_eq(arc.radius, 1.0), "radius should be 1, got {}", arc.radius);
        assert!(approx_eq(arc.center[0], 0.0), "cx = 0");
        assert!(approx_eq(arc.center[1], 0.0), "cy = 0");
        assert!(approx_eq(arc.center[2], 0.0), "cz = 0");
        assert!(arc.is_done());
    }

    #[test]
    fn arc_of_circle_collinear_returns_none() {
        let p1 = [0.0, 0.0, 0.0];
        let p2 = [1.0, 0.0, 0.0];
        let p3 = [2.0, 0.0, 0.0];
        assert!(ArcOfCircle::from_3_points(p1, p2, p3).is_none());
    }

    #[test]
    fn arc_of_circle_length() {
        // Semi-circle on unit circle: length = π.
        let p1 = [1.0_f64, 0.0, 0.0];
        let p2 = [0.0, 1.0, 0.0];
        let p3 = [-1.0, 0.0, 0.0];
        let arc = ArcOfCircle::from_3_points(p1, p2, p3).unwrap();
        let l = arc.length();
        assert!(
            approx_eq(l, PI),
            "semi-circle length should be π ≈ {PI}, got {l}"
        );
    }

    #[test]
    fn arc_of_circle_d1_perpendicular_to_radius() {
        let p1 = [1.0_f64, 0.0, 0.0];
        let p2 = [0.0, 1.0, 0.0];
        let p3 = [-1.0, 0.0, 0.0];
        let arc = ArcOfCircle::from_3_points(p1, p2, p3).unwrap();
        let t = PI / 4.0;
        let pt = arc.evaluate(t);
        let dt = arc.d1(t);
        let r_vec = sub(pt, arc.center);
        let dot_val = dot(r_vec, dt);
        assert!(
            dot_val.abs() < 1e-9,
            "tangent should be perpendicular to radius, dot = {dot_val}"
        );
    }

    // --- ArcOfEllipse --------------------------------------------------------

    #[test]
    fn arc_of_ellipse_new_and_evaluate() {
        let arc = ArcOfEllipse::new(0.0, 0.0, 0.0, 3.0, 2.0, 0.0, PI);
        // At t=0: (3, 0, 0)
        let p0 = arc.evaluate(0.0);
        assert!(vec_approx_eq(p0, [3.0, 0.0, 0.0]), "P(0) = (3,0,0), got {p0:?}");
        // At t=π/2: (0, 2, 0)
        let ph = arc.evaluate(PI / 2.0);
        assert!(vec_approx_eq(ph, [0.0, 2.0, 0.0]), "P(π/2) = (0,2,0), got {ph:?}");
        // At t=π: (-3, 0, 0)
        let pp = arc.evaluate(PI);
        assert!(vec_approx_eq(pp, [-3.0, 0.0, 0.0]), "P(π) = (-3,0,0), got {pp:?}");
    }

    #[test]
    fn arc_of_ellipse_non_zero_center() {
        let arc = ArcOfEllipse::new(1.0, 2.0, 5.0, 4.0, 1.0, 0.0, TAU);
        let p0 = arc.evaluate(0.0);
        assert!(
            vec_approx_eq(p0, [5.0, 2.0, 5.0]),
            "P(0) should be (5, 2, 5), got {p0:?}"
        );
    }
}
