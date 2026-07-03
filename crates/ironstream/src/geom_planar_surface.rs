//! `Geom_Plane` — pure-Rust zero-dependency planar surface stub, modelled after
//! OpenCascade's `Geom_Plane` (extended surface API).
//!
//! A planar surface is defined by:
//! - An **origin** point in 3-D space.
//! - A **unit normal** direction (the plane's Z axis).
//! - A **unit X axis** tangent to the plane, used as the U parametric direction.
//!
//! The surface is parameterized as:
//! ```text
//! P(U, V) = origin + U * x_axis + V * y_axis
//! ```
//! where `y_axis = cross(normal, x_axis)` is derived automatically.
//!
//! The plane is unbounded in both U and V.

// occt: Geom_Plane

/// Compute the Y axis of the plane frame as `normal × x_axis`.
///
/// Both `normal` and `x_axis` are assumed to be unit vectors that are mutually
/// orthogonal. The result is a unit vector completing the right-handed frame.
pub fn y_axis(normal: [f64; 3], x_axis: [f64; 3]) -> [f64; 3] {
    cross(normal, x_axis)
}

// occt: Geom_Plane
/// `Geom_Plane` — an infinite analytic plane in 3D space, parameterized as
/// `P(U, V) = origin + U * x_axis + V * y_axis`.
///
/// `y_axis` is derived as `cross(normal, x_axis)` and is stored explicitly for
/// performance. All surface API methods are constant-time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlanarSurface {
    /// The location (origin) of the plane.
    pub origin: [f64; 3],
    /// The unit normal of the plane (the plane's Z axis / W direction).
    pub normal: [f64; 3],
    /// The unit U-parametric axis tangent to the plane.
    pub x_axis: [f64; 3],
}

impl PlanarSurface {
    // ─────────────────────────── constructors ───────────────────────────────

    /// Construct a plane from an origin and a unit normal.
    ///
    /// An arbitrary X axis perpendicular to `normal` is chosen automatically
    /// using the standard "least-parallel axis" method so the result is always
    /// well-defined.
    ///
    /// # Example
    /// ```
    /// use ironstream::geom_planar_surface::PlanarSurface;
    /// let pl = PlanarSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    /// assert_eq!(pl.normal_at(0.0, 0.0), [0.0, 0.0, 1.0]);
    /// ```
    pub fn new(origin: [f64; 3], normal: [f64; 3]) -> Self {
        let n = normalize(normal);
        let x = arbitrary_perpendicular(n);
        Self {
            origin,
            normal: n,
            x_axis: x,
        }
    }

    /// Return a copy of this plane with the X axis replaced by `xaxis`.
    ///
    /// `xaxis` is projected onto the plane and normalized, so it is safe to
    /// pass an approximate direction.
    pub fn with_x_axis(self, xaxis: [f64; 3]) -> Self {
        // Project xaxis onto the plane: subtract the component along normal.
        let d = dot(xaxis, self.normal);
        let projected = [
            xaxis[0] - d * self.normal[0],
            xaxis[1] - d * self.normal[1],
            xaxis[2] - d * self.normal[2],
        ];
        let x = normalize(projected);
        Self {
            origin: self.origin,
            normal: self.normal,
            x_axis: x,
        }
    }

    // ─────────────────────────── evaluation ─────────────────────────────────

    /// `D0(U, V)` / `Value(U, V)` — the point on the plane at parameter `(U, V)`.
    ///
    /// `P(U, V) = origin + U * x_axis + V * y_axis`
    pub fn evaluate(&self, u: f64, v: f64) -> [f64; 3] {
        let yax = y_axis(self.normal, self.x_axis);
        [
            self.origin[0] + u * self.x_axis[0] + v * yax[0],
            self.origin[1] + u * self.x_axis[1] + v * yax[1],
            self.origin[2] + u * self.x_axis[2] + v * yax[2],
        ]
    }

    /// `Normal(U, V)` — the unit normal at parameter `(U, V)`.
    ///
    /// For a plane the normal is constant everywhere; the parameters are
    /// accepted for API symmetry with curved surfaces.
    pub fn normal_at(&self, _u: f64, _v: f64) -> [f64; 3] {
        self.normal
    }

    /// `∂P/∂U` — first partial derivative with respect to U.
    ///
    /// Equals `x_axis` (constant for a plane; parameters accepted for API
    /// symmetry).
    pub fn d1u(&self, _u: f64, _v: f64) -> [f64; 3] {
        self.x_axis
    }

    /// `∂P/∂V` — first partial derivative with respect to V.
    ///
    /// Equals `y_axis = cross(normal, x_axis)` (constant for a plane).
    pub fn d1v(&self, _u: f64, _v: f64) -> [f64; 3] {
        y_axis(self.normal, self.x_axis)
    }

    // ─────────────────────────── projection ─────────────────────────────────

    /// Project a 3-D point onto the plane and return its `(U, V)` parameters.
    ///
    /// The projection is the orthogonal foot of the perpendicular from `pt` to
    /// the plane:
    /// ```text
    /// delta = pt - origin
    /// U = dot(delta, x_axis)
    /// V = dot(delta, y_axis)
    /// ```
    pub fn project(&self, pt: [f64; 3]) -> [f64; 2] {
        let yax = y_axis(self.normal, self.x_axis);
        let delta = [
            pt[0] - self.origin[0],
            pt[1] - self.origin[1],
            pt[2] - self.origin[2],
        ];
        [dot(delta, self.x_axis), dot(delta, yax)]
    }

    /// Signed distance from a 3-D point to the plane.
    ///
    /// Positive when the point is on the side the normal points toward,
    /// negative on the opposite side.
    /// ```text
    /// distance = dot(pt - origin, normal)
    /// ```
    pub fn distance(&self, pt: [f64; 3]) -> f64 {
        let delta = [
            pt[0] - self.origin[0],
            pt[1] - self.origin[1],
            pt[2] - self.origin[2],
        ];
        dot(delta, self.normal)
    }

    // ──────────────────── topology / periodicity queries ────────────────────

    /// `IsUClosed()` — `false`; a plane is unbounded in U.
    pub fn is_u_closed(&self) -> bool {
        false
    }

    /// `IsVClosed()` — `false`; a plane is unbounded in V.
    pub fn is_v_closed(&self) -> bool {
        false
    }
}

// ─────────────────────────── private helpers ────────────────────────────────

/// Dot product of two 3-vectors.
#[inline]
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Cross product `a × b`.
#[inline]
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Normalize a 3-vector. Returns the zero vector unchanged if the magnitude is
/// (near) zero to avoid a panic; callers should supply non-degenerate inputs.
#[inline]
fn normalize(v: [f64; 3]) -> [f64; 3] {
    let len = f64::sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    if len == 0.0 {
        return v;
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

/// Choose an arbitrary unit vector perpendicular to `n`.
///
/// Picks the axis (X, Y, or Z) that is least parallel to `n`, then
/// cross-multiplies to get a perpendicular direction.
#[inline]
fn arbitrary_perpendicular(n: [f64; 3]) -> [f64; 3] {
    // Find the component of n with the smallest absolute value — the
    // corresponding world axis is "least parallel" to n.
    let ax = n[0].abs();
    let ay = n[1].abs();
    let az = n[2].abs();
    let candidate: [f64; 3] = if ax <= ay && ax <= az {
        [1.0, 0.0, 0.0]
    } else if ay <= ax && ay <= az {
        [0.0, 1.0, 0.0]
    } else {
        [0.0, 0.0, 1.0]
    };
    normalize(cross(candidate, n))
}

// ─────────────────────────────── tests ──────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: [f64; 3], b: [f64; 3]) -> bool {
        let eps = 1e-12_f64;
        (a[0] - b[0]).abs() < eps && (a[1] - b[1]).abs() < eps && (a[2] - b[2]).abs() < eps
    }

    #[test]
    fn test_new_xy_plane() {
        let pl = PlanarSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert_eq!(pl.is_u_closed(), false);
        assert_eq!(pl.is_v_closed(), false);
        assert!(approx_eq(pl.normal_at(0.0, 0.0), [0.0, 0.0, 1.0]));
    }

    #[test]
    fn test_evaluate() {
        let pl = PlanarSurface::new([1.0, 2.0, 3.0], [0.0, 0.0, 1.0]);
        // evaluate at (0,0) should be origin
        let p = pl.evaluate(0.0, 0.0);
        assert!(approx_eq(p, [1.0, 2.0, 3.0]));
    }

    #[test]
    fn test_project_origin() {
        let pl = PlanarSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let uv = pl.project([0.0, 0.0, 5.0]);
        assert!((uv[0]).abs() < 1e-12);
        assert!((uv[1]).abs() < 1e-12);
    }

    #[test]
    fn test_distance() {
        let pl = PlanarSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert!((pl.distance([0.0, 0.0, 7.0]) - 7.0).abs() < 1e-12);
        assert!((pl.distance([0.0, 0.0, -3.0]) + 3.0).abs() < 1e-12);
    }

    #[test]
    fn test_d1u_d1v_orthogonal() {
        let pl = PlanarSurface::new([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let du = pl.d1u(0.0, 0.0);
        let dv = pl.d1v(0.0, 0.0);
        // du and dv must be orthogonal to each other and to the normal
        assert!(dot(du, dv).abs() < 1e-12);
        assert!(dot(du, pl.normal).abs() < 1e-12);
        assert!(dot(dv, pl.normal).abs() < 1e-12);
    }

    #[test]
    fn test_with_x_axis() {
        let pl = PlanarSurface::new([0.0, 0.0, 0.0], [0.0, 0.0, 1.0])
            .with_x_axis([1.0, 0.0, 0.0]);
        assert!(approx_eq(pl.x_axis, [1.0, 0.0, 0.0]));
        // y_axis should be [0,1,0]
        assert!(approx_eq(y_axis(pl.normal, pl.x_axis), [0.0, 1.0, 0.0]));
    }

    #[test]
    fn test_y_axis_fn() {
        // For XY plane: normal=[0,0,1], x=[1,0,0] → y=[0,1,0]
        let y = y_axis([0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
        assert!(approx_eq(y, [0.0, 1.0, 0.0]));
    }
}
