// FILE: geom2d_conic.rs

//! 2D and 3D geometric abstract trait hierarchy porting OCCT Geom2d / Geom classes.

// ---------------------------------------------------------------------------
// 2D TRAIT HIERARCHY
// ---------------------------------------------------------------------------

// occt: Geom2d_Geometry
/// Base trait for all 2D geometric objects.
pub trait Geom2dGeometry: Send + Sync {
    fn clone_box(&self) -> Box<dyn Geom2dGeometry>;
}

// occt-ref: Geom2d_Curve
/// Trait for parametric 2D curves.
pub trait Geom2dCurve: Geom2dGeometry {
    /// Point on the curve at parameter u.
    fn value(&self, u: f64) -> [f64; 2];
    /// Point and first derivative at parameter u.
    fn d1(&self, u: f64) -> ([f64; 2], [f64; 2]);
    /// First (lower) parameter value.
    fn first_param(&self) -> f64;
    /// Last (upper) parameter value.
    fn last_param(&self) -> f64;
    /// Whether the curve is periodic.
    fn is_periodic(&self) -> bool;
    /// Period of the curve. Only meaningful when `is_periodic()` is true.
    fn period(&self) -> f64;
    /// Whether the curve is closed (start == end).
    fn is_closed(&self) -> bool;
}

// occt: Geom2d_BoundedCurve
/// Trait for 2D curves with explicit start and end points.
pub trait Geom2dBoundedCurve: Geom2dCurve {
    /// Point at the start of the curve.
    fn start_point(&self) -> [f64; 2];
    /// Point at the end of the curve.
    fn end_point(&self) -> [f64; 2];
}

// occt: Geom2d_Conic
/// Trait for 2D conic sections (circle, ellipse, hyperbola, parabola).
pub trait Geom2dConic: Geom2dCurve {
    /// Location (center/vertex) of the conic in 2D.
    fn location(&self) -> [f64; 2];
    /// Axis as [origin_x, origin_y, dir_x, dir_y].
    fn axis(&self) -> [f64; 4];
    /// Eccentricity: 0 = circle, 0<e<1 = ellipse, 1 = parabola, >1 = hyperbola.
    fn eccentricity(&self) -> f64;
}

// ---------------------------------------------------------------------------
// 3D TRAIT HIERARCHY
// ---------------------------------------------------------------------------

// occt-ref: Geom_Geometry
/// Base trait for all 3D geometric objects.
pub trait GeomGeometry: Send + Sync {
    fn clone_box(&self) -> Box<dyn GeomGeometry>;
}

// occt-ref: Geom_Curve
/// Trait for parametric 3D curves.
pub trait GeomCurve: GeomGeometry {
    /// Point on the curve at parameter u.
    fn value(&self, u: f64) -> [f64; 3];
    /// Point and first derivative at parameter u.
    fn d1(&self, u: f64) -> ([f64; 3], [f64; 3]);
    /// First (lower) parameter value.
    fn first_param(&self) -> f64;
    /// Last (upper) parameter value.
    fn last_param(&self) -> f64;
    /// Whether the curve is periodic.
    fn is_periodic(&self) -> bool;
    /// Period of the curve. Only meaningful when `is_periodic()` is true.
    fn period(&self) -> f64;
    /// Whether the curve is closed (start == end).
    fn is_closed(&self) -> bool;
}

// occt-ref: Geom_BoundedCurve
/// Trait for 3D curves with explicit start and end points.
pub trait GeomBoundedCurve: GeomCurve {
    /// Point at the start of the curve.
    fn start_point(&self) -> [f64; 3];
    /// Point at the end of the curve.
    fn end_point(&self) -> [f64; 3];
}

// occt-ref: Geom_Conic
/// Trait for 3D conic sections.
pub trait GeomConic: GeomCurve {
    /// Location (center/vertex) of the conic in 3D.
    fn location(&self) -> [f64; 3];
    /// Axis as [origin_x, origin_y, origin_z, dir_x, dir_y, dir_z].
    fn axis(&self) -> [f64; 6];
    /// Eccentricity.
    fn eccentricity(&self) -> f64;
}

// ---------------------------------------------------------------------------
// CONCRETE: trivial 2D line segment for trait dispatch testing
// ---------------------------------------------------------------------------

/// A line segment in 2D parameterised by arc-length from `start` to `end`.
///
/// The parameter range is [0, length]. At u=0 the point is `start`; at
/// u=length the point is `end`. The direction is constant.
///
/// OCCT analogue: Geom2d_TrimmedCurve wrapping Geom2d_Line, trimmed to the
/// two endpoints.
// occt-ref: Geom2d_TrimmedCurve
#[derive(Clone)]
pub struct Line2d {
    pub start: [f64; 2],
    pub end: [f64; 2],
    length: f64,
    dir: [f64; 2],
}

impl Line2d {
    /// Construct a line segment from `start` to `end`.
    /// Panics if the two points are identical (zero-length segment).
    pub fn new(start: [f64; 2], end: [f64; 2]) -> Self {
        let dx = end[0] - start[0];
        let dy = end[1] - start[1];
        let length = (dx * dx + dy * dy).sqrt();
        assert!(length > 1e-14, "Line2d: degenerate segment (zero length)");
        let dir = [dx / length, dy / length];
        Line2d { start, end, length, dir }
    }

    /// Length of the segment (equals `last_param()`).
    pub fn length(&self) -> f64 {
        self.length
    }
}

impl Geom2dGeometry for Line2d {
    fn clone_box(&self) -> Box<dyn Geom2dGeometry> {
        Box::new(self.clone())
    }
}

impl Geom2dCurve for Line2d {
    fn value(&self, u: f64) -> [f64; 2] {
        [
            self.start[0] + u * self.dir[0],
            self.start[1] + u * self.dir[1],
        ]
    }

    fn d1(&self, _u: f64) -> ([f64; 2], [f64; 2]) {
        let pt = self.value(_u);
        (pt, self.dir)
    }

    fn first_param(&self) -> f64 {
        0.0
    }

    fn last_param(&self) -> f64 {
        self.length
    }

    fn is_periodic(&self) -> bool {
        false
    }

    /// A non-periodic curve has no meaningful period; return 0.0.
    fn period(&self) -> f64 {
        0.0
    }

    fn is_closed(&self) -> bool {
        let dx = self.end[0] - self.start[0];
        let dy = self.end[1] - self.start[1];
        (dx * dx + dy * dy).sqrt() < 1e-7
    }
}

impl Geom2dBoundedCurve for Line2d {
    fn start_point(&self) -> [f64; 2] {
        self.start
    }

    fn end_point(&self) -> [f64; 2] {
        self.end
    }
}

// ---------------------------------------------------------------------------
// CONCRETE: 2D circle for Geom2dConic trait dispatch testing
// ---------------------------------------------------------------------------

/// A full 2D circle parameterised by angle u in [0, 2π).
// occt-ref: Geom2d_Circle
#[derive(Clone)]
pub struct Circle2d {
    /// Centre of the circle.
    pub center: [f64; 2],
    /// Radius of the circle (must be > 0).
    pub radius: f64,
}

impl Circle2d {
    const TWO_PI: f64 = 2.0 * std::f64::consts::PI;

    /// Construct a circle.
    pub fn new(center: [f64; 2], radius: f64) -> Self {
        assert!(radius > 0.0, "Circle2d: radius must be positive");
        Circle2d { center, radius }
    }
}

impl Geom2dGeometry for Circle2d {
    fn clone_box(&self) -> Box<dyn Geom2dGeometry> {
        Box::new(self.clone())
    }
}

impl Geom2dCurve for Circle2d {
    fn value(&self, u: f64) -> [f64; 2] {
        [
            self.center[0] + self.radius * u.cos(),
            self.center[1] + self.radius * u.sin(),
        ]
    }

    fn d1(&self, u: f64) -> ([f64; 2], [f64; 2]) {
        let pt = self.value(u);
        let d = [-self.radius * u.sin(), self.radius * u.cos()];
        (pt, d)
    }

    fn first_param(&self) -> f64 {
        0.0
    }

    fn last_param(&self) -> f64 {
        Self::TWO_PI
    }

    fn is_periodic(&self) -> bool {
        true
    }

    fn period(&self) -> f64 {
        Self::TWO_PI
    }

    fn is_closed(&self) -> bool {
        true
    }
}

impl Geom2dConic for Circle2d {
    fn location(&self) -> [f64; 2] {
        self.center
    }

    /// Axis: [cx, cy, 1.0, 0.0] — X-axis direction.
    fn axis(&self) -> [f64; 4] {
        [self.center[0], self.center[1], 1.0, 0.0]
    }

    /// A circle has eccentricity 0.
    fn eccentricity(&self) -> f64 {
        0.0
    }
}

// ---------------------------------------------------------------------------
// CONCRETE: 2D ellipse for Geom2dConic trait dispatch testing
// ---------------------------------------------------------------------------

/// A full 2D ellipse parameterised by eccentric anomaly u in [0, 2π).
///
/// The semi-major axis lies along the local X direction, semi-minor along Y.
/// The major axis length must be >= the minor axis length > 0.
// occt-ref: Geom2d_Ellipse
#[derive(Clone)]
pub struct Ellipse2d {
    /// Centre of the ellipse.
    pub center: [f64; 2],
    /// Semi-major axis (along local X).
    pub major_radius: f64,
    /// Semi-minor axis (along local Y).
    pub minor_radius: f64,
    /// Orientation angle of the major axis with respect to the global X axis (radians).
    pub angle: f64,
}

impl Ellipse2d {
    const TWO_PI: f64 = 2.0 * std::f64::consts::PI;

    /// Construct an ellipse.
    pub fn new(center: [f64; 2], major_radius: f64, minor_radius: f64, angle: f64) -> Self {
        assert!(
            major_radius >= minor_radius && minor_radius > 0.0,
            "Ellipse2d: need major_radius >= minor_radius > 0"
        );
        Ellipse2d { center, major_radius, minor_radius, angle }
    }

    fn point_at(&self, u: f64) -> [f64; 2] {
        let ca = self.angle.cos();
        let sa = self.angle.sin();
        let x_local = self.major_radius * u.cos();
        let y_local = self.minor_radius * u.sin();
        [
            self.center[0] + ca * x_local - sa * y_local,
            self.center[1] + sa * x_local + ca * y_local,
        ]
    }
}

impl Geom2dGeometry for Ellipse2d {
    fn clone_box(&self) -> Box<dyn Geom2dGeometry> {
        Box::new(self.clone())
    }
}

impl Geom2dCurve for Ellipse2d {
    fn value(&self, u: f64) -> [f64; 2] {
        self.point_at(u)
    }

    fn d1(&self, u: f64) -> ([f64; 2], [f64; 2]) {
        let pt = self.point_at(u);
        let ca = self.angle.cos();
        let sa = self.angle.sin();
        let dx_local = -self.major_radius * u.sin();
        let dy_local = self.minor_radius * u.cos();
        let d = [
            ca * dx_local - sa * dy_local,
            sa * dx_local + ca * dy_local,
        ];
        (pt, d)
    }

    fn first_param(&self) -> f64 {
        0.0
    }

    fn last_param(&self) -> f64 {
        Self::TWO_PI
    }

    fn is_periodic(&self) -> bool {
        true
    }

    fn period(&self) -> f64 {
        Self::TWO_PI
    }

    fn is_closed(&self) -> bool {
        true
    }
}

impl Geom2dConic for Ellipse2d {
    fn location(&self) -> [f64; 2] {
        self.center
    }

    fn axis(&self) -> [f64; 4] {
        let ca = self.angle.cos();
        let sa = self.angle.sin();
        [self.center[0], self.center[1], ca, sa]
    }

    fn eccentricity(&self) -> f64 {
        let a = self.major_radius;
        let b = self.minor_radius;
        // e = sqrt(1 - (b/a)^2)
        let ratio = b / a;
        (1.0 - ratio * ratio).sqrt()
    }
}

// ---------------------------------------------------------------------------
// CONCRETE: 3D line segment for GeomBoundedCurve testing
// ---------------------------------------------------------------------------

/// A line segment in 3D parameterised by arc-length.
// occt-ref: Geom_TrimmedCurve
#[derive(Clone)]
pub struct Line3d {
    pub start: [f64; 3],
    pub end: [f64; 3],
    length: f64,
    dir: [f64; 3],
}

impl Line3d {
    /// Construct a 3D line segment.
    pub fn new(start: [f64; 3], end: [f64; 3]) -> Self {
        let dx = end[0] - start[0];
        let dy = end[1] - start[1];
        let dz = end[2] - start[2];
        let length = (dx * dx + dy * dy + dz * dz).sqrt();
        assert!(length > 1e-14, "Line3d: degenerate segment (zero length)");
        let dir = [dx / length, dy / length, dz / length];
        Line3d { start, end, length, dir }
    }

    pub fn length(&self) -> f64 {
        self.length
    }
}

impl GeomGeometry for Line3d {
    fn clone_box(&self) -> Box<dyn GeomGeometry> {
        Box::new(self.clone())
    }
}

impl GeomCurve for Line3d {
    fn value(&self, u: f64) -> [f64; 3] {
        [
            self.start[0] + u * self.dir[0],
            self.start[1] + u * self.dir[1],
            self.start[2] + u * self.dir[2],
        ]
    }

    fn d1(&self, u: f64) -> ([f64; 3], [f64; 3]) {
        let pt = self.value(u);
        (pt, self.dir)
    }

    fn first_param(&self) -> f64 {
        0.0
    }

    fn last_param(&self) -> f64 {
        self.length
    }

    fn is_periodic(&self) -> bool {
        false
    }

    fn period(&self) -> f64 {
        0.0
    }

    fn is_closed(&self) -> bool {
        let dx = self.end[0] - self.start[0];
        let dy = self.end[1] - self.start[1];
        let dz = self.end[2] - self.start[2];
        (dx * dx + dy * dy + dz * dz).sqrt() < 1e-7
    }
}

impl GeomBoundedCurve for Line3d {
    fn start_point(&self) -> [f64; 3] {
        self.start
    }

    fn end_point(&self) -> [f64; 3] {
        self.end
    }
}

// ---------------------------------------------------------------------------
// CONCRETE: 3D circle for GeomConic testing
// ---------------------------------------------------------------------------

/// A full 3D circle in the XY plane (z=plane_z) with given centre and radius.
// occt-ref: Geom_Circle
#[derive(Clone)]
pub struct Circle3d {
    pub center: [f64; 3],
    pub radius: f64,
    /// Normal direction [nx, ny, nz] (unit vector).
    pub normal: [f64; 3],
}

impl Circle3d {
    const TWO_PI: f64 = 2.0 * std::f64::consts::PI;

    pub fn new(center: [f64; 3], radius: f64, normal: [f64; 3]) -> Self {
        assert!(radius > 0.0, "Circle3d: radius must be positive");
        let n = normal;
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        assert!(len > 1e-14, "Circle3d: normal must be non-zero");
        let normal = [n[0] / len, n[1] / len, n[2] / len];
        Circle3d { center, radius, normal }
    }

    /// Compute an orthonormal basis (u_vec, v_vec) in the plane with given normal.
    fn basis(&self) -> ([f64; 3], [f64; 3]) {
        let n = self.normal;
        // Choose a reference vector not parallel to n
        let ref_vec: [f64; 3] = if n[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        let u = cross3(ref_vec, n);
        let u_len = (u[0] * u[0] + u[1] * u[1] + u[2] * u[2]).sqrt();
        let u = [u[0] / u_len, u[1] / u_len, u[2] / u_len];
        let v = cross3(n, u);
        (u, v)
    }
}

fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

impl GeomGeometry for Circle3d {
    fn clone_box(&self) -> Box<dyn GeomGeometry> {
        Box::new(self.clone())
    }
}

impl GeomCurve for Circle3d {
    fn value(&self, u: f64) -> [f64; 3] {
        let (ux, vx) = self.basis();
        let cu = u.cos();
        let su = u.sin();
        [
            self.center[0] + self.radius * (cu * ux[0] + su * vx[0]),
            self.center[1] + self.radius * (cu * ux[1] + su * vx[1]),
            self.center[2] + self.radius * (cu * ux[2] + su * vx[2]),
        ]
    }

    fn d1(&self, u: f64) -> ([f64; 3], [f64; 3]) {
        let pt = self.value(u);
        let (ux, vx) = self.basis();
        let cu = u.cos();
        let su = u.sin();
        let d = [
            self.radius * (-su * ux[0] + cu * vx[0]),
            self.radius * (-su * ux[1] + cu * vx[1]),
            self.radius * (-su * ux[2] + cu * vx[2]),
        ];
        (pt, d)
    }

    fn first_param(&self) -> f64 {
        0.0
    }

    fn last_param(&self) -> f64 {
        Self::TWO_PI
    }

    fn is_periodic(&self) -> bool {
        true
    }

    fn period(&self) -> f64 {
        Self::TWO_PI
    }

    fn is_closed(&self) -> bool {
        true
    }
}

impl GeomConic for Circle3d {
    fn location(&self) -> [f64; 3] {
        self.center
    }

    fn axis(&self) -> [f64; 6] {
        let n = self.normal;
        [self.center[0], self.center[1], self.center[2], n[0], n[1], n[2]]
    }

    fn eccentricity(&self) -> f64 {
        0.0
    }
}

// ---------------------------------------------------------------------------
// UNIT TESTS
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    fn approx_eq2(a: [f64; 2], b: [f64; 2], eps: f64) -> bool {
        approx_eq(a[0], b[0], eps) && approx_eq(a[1], b[1], eps)
    }

    fn approx_eq3(a: [f64; 3], b: [f64; 3], eps: f64) -> bool {
        approx_eq(a[0], b[0], eps) && approx_eq(a[1], b[1], eps) && approx_eq(a[2], b[2], eps)
    }

    // -----------------------------------------------------------------------
    // Line2d
    // -----------------------------------------------------------------------

    #[test]
    fn line2d_value_at_endpoints() {
        let l = Line2d::new([1.0, 2.0], [4.0, 6.0]);
        let p0 = l.value(0.0);
        let p1 = l.value(l.last_param());
        assert!(approx_eq2(p0, [1.0, 2.0], EPS));
        assert!(approx_eq2(p1, [4.0, 6.0], EPS));
    }

    #[test]
    fn line2d_midpoint() {
        let l = Line2d::new([0.0, 0.0], [2.0, 0.0]);
        let mid = l.value(1.0);
        assert!(approx_eq2(mid, [1.0, 0.0], EPS));
    }

    #[test]
    fn line2d_length() {
        let l = Line2d::new([0.0, 0.0], [3.0, 4.0]);
        assert!(approx_eq(l.length(), 5.0, EPS));
        assert!(approx_eq(l.first_param(), 0.0, EPS));
        assert!(approx_eq(l.last_param(), 5.0, EPS));
    }

    #[test]
    fn line2d_d1_constant_direction() {
        let l = Line2d::new([1.0, 1.0], [4.0, 5.0]);
        let (_, d0) = l.d1(0.0);
        let (_, d1) = l.d1(l.last_param() / 2.0);
        assert!(approx_eq2(d0, d1, EPS));
        // derivative should be unit vector
        let mag = (d0[0] * d0[0] + d0[1] * d0[1]).sqrt();
        assert!(approx_eq(mag, 1.0, EPS));
    }

    #[test]
    fn line2d_not_periodic() {
        let l = Line2d::new([0.0, 0.0], [1.0, 0.0]);
        assert!(!l.is_periodic());
        assert!(approx_eq(l.period(), 0.0, EPS));
    }

    #[test]
    fn line2d_not_closed() {
        let l = Line2d::new([0.0, 0.0], [1.0, 0.0]);
        assert!(!l.is_closed());
    }

    #[test]
    fn line2d_bounded_curve_start_end() {
        let l = Line2d::new([3.0, -1.0], [-1.0, 2.0]);
        assert!(approx_eq2(l.start_point(), [3.0, -1.0], EPS));
        assert!(approx_eq2(l.end_point(), [-1.0, 2.0], EPS));
    }

    #[test]
    fn line2d_trait_dispatch_via_dyn() {
        let l = Line2d::new([0.0, 0.0], [1.0, 0.0]);
        let curve: &dyn Geom2dCurve = &l;
        let pt = curve.value(0.5);
        assert!(approx_eq2(pt, [0.5, 0.0], EPS));
    }

    #[test]
    fn line2d_clone_box() {
        let l = Line2d::new([0.0, 0.0], [1.0, 1.0]);
        let geom: &dyn Geom2dGeometry = &l;
        let _cloned = geom.clone_box();
    }

    // -----------------------------------------------------------------------
    // Circle2d
    // -----------------------------------------------------------------------

    #[test]
    fn circle2d_value_at_zero() {
        let c = Circle2d::new([0.0, 0.0], 3.0);
        let p = c.value(0.0);
        assert!(approx_eq2(p, [3.0, 0.0], EPS));
    }

    #[test]
    fn circle2d_value_at_pi_over_2() {
        let c = Circle2d::new([0.0, 0.0], 3.0);
        let p = c.value(std::f64::consts::PI / 2.0);
        assert!(approx_eq2(p, [0.0, 3.0], EPS));
    }

    #[test]
    fn circle2d_value_at_pi() {
        let c = Circle2d::new([0.0, 0.0], 5.0);
        let p = c.value(std::f64::consts::PI);
        assert!(approx_eq2(p, [-5.0, 0.0], EPS));
    }

    #[test]
    fn circle2d_d1_tangent_orthogonal_to_radius() {
        let c = Circle2d::new([0.0, 0.0], 2.0);
        for i in 0..8 {
            let u = i as f64 * std::f64::consts::PI / 4.0;
            let (pt, d) = c.d1(u);
            // radius vector
            let rx = pt[0];
            let ry = pt[1];
            // dot(radius, tangent) should be 0
            let dot = rx * d[0] + ry * d[1];
            assert!(approx_eq(dot, 0.0, EPS), "u={}: dot={}", u, dot);
        }
    }

    #[test]
    fn circle2d_d1_tangent_magnitude_equals_radius() {
        let r = 4.0;
        let c = Circle2d::new([1.0, -1.0], r);
        let u = 1.23;
        let (_, d) = c.d1(u);
        let mag = (d[0] * d[0] + d[1] * d[1]).sqrt();
        assert!(approx_eq(mag, r, EPS));
    }

    #[test]
    fn circle2d_is_periodic_and_closed() {
        let c = Circle2d::new([0.0, 0.0], 1.0);
        assert!(c.is_periodic());
        assert!(c.is_closed());
        assert!(approx_eq(c.period(), 2.0 * std::f64::consts::PI, EPS));
    }

    #[test]
    fn circle2d_param_range() {
        let c = Circle2d::new([0.0, 0.0], 1.0);
        assert!(approx_eq(c.first_param(), 0.0, EPS));
        assert!(approx_eq(c.last_param(), 2.0 * std::f64::consts::PI, EPS));
    }

    #[test]
    fn circle2d_conic_location_and_eccentricity() {
        let c = Circle2d::new([3.0, -2.0], 5.0);
        assert!(approx_eq2(c.location(), [3.0, -2.0], EPS));
        assert!(approx_eq(c.eccentricity(), 0.0, EPS));
    }

    #[test]
    fn circle2d_conic_axis() {
        let c = Circle2d::new([1.0, 2.0], 3.0);
        let ax = c.axis();
        assert!(approx_eq(ax[0], 1.0, EPS));
        assert!(approx_eq(ax[1], 2.0, EPS));
        // direction should be unit length
        let len = (ax[2] * ax[2] + ax[3] * ax[3]).sqrt();
        assert!(approx_eq(len, 1.0, EPS));
    }

    #[test]
    fn circle2d_trait_dispatch_via_dyn_conic() {
        let c = Circle2d::new([0.0, 0.0], 2.0);
        let conic: &dyn Geom2dConic = &c;
        assert!(approx_eq(conic.eccentricity(), 0.0, EPS));
        let pt = conic.value(0.0);
        assert!(approx_eq2(pt, [2.0, 0.0], EPS));
    }

    // -----------------------------------------------------------------------
    // Ellipse2d
    // -----------------------------------------------------------------------

    #[test]
    fn ellipse2d_value_at_zero_angle() {
        let e = Ellipse2d::new([0.0, 0.0], 5.0, 3.0, 0.0);
        let p = e.value(0.0);
        assert!(approx_eq2(p, [5.0, 0.0], EPS));
    }

    #[test]
    fn ellipse2d_value_at_pi_over_2() {
        let e = Ellipse2d::new([0.0, 0.0], 5.0, 3.0, 0.0);
        let p = e.value(std::f64::consts::PI / 2.0);
        assert!(approx_eq2(p, [0.0, 3.0], EPS));
    }

    #[test]
    fn ellipse2d_center_offset() {
        let e = Ellipse2d::new([1.0, 2.0], 4.0, 2.0, 0.0);
        let p = e.value(0.0);
        assert!(approx_eq2(p, [5.0, 2.0], EPS));
    }

    #[test]
    fn ellipse2d_eccentricity_circle_case() {
        // When major == minor, eccentricity should be 0
        let e = Ellipse2d::new([0.0, 0.0], 3.0, 3.0, 0.0);
        assert!(approx_eq(e.eccentricity(), 0.0, EPS));
    }

    #[test]
    fn ellipse2d_eccentricity_3_4_5() {
        // a=5, b=3 => e = sqrt(1-9/25) = sqrt(16/25) = 4/5 = 0.8
        let e = Ellipse2d::new([0.0, 0.0], 5.0, 3.0, 0.0);
        assert!(approx_eq(e.eccentricity(), 0.8, EPS));
    }

    #[test]
    fn ellipse2d_is_periodic() {
        let e = Ellipse2d::new([0.0, 0.0], 4.0, 2.0, 0.0);
        assert!(e.is_periodic());
        assert!(e.is_closed());
    }

    #[test]
    fn ellipse2d_d1_numerical_check() {
        let e = Ellipse2d::new([0.0, 0.0], 5.0, 3.0, 0.0);
        let u = 0.7;
        let h = 1e-7;
        let (_, d) = e.d1(u);
        let p_plus = e.value(u + h);
        let p_minus = e.value(u - h);
        let fd = [(p_plus[0] - p_minus[0]) / (2.0 * h), (p_plus[1] - p_minus[1]) / (2.0 * h)];
        assert!(approx_eq(d[0], fd[0], 1e-5), "d[0]={} fd[0]={}", d[0], fd[0]);
        assert!(approx_eq(d[1], fd[1], 1e-5), "d[1]={} fd[1]={}", d[1], fd[1]);
    }

    #[test]
    fn ellipse2d_rotated_axis() {
        let angle = std::f64::consts::PI / 4.0;
        let e = Ellipse2d::new([0.0, 0.0], 3.0, 1.0, angle);
        let ax = e.axis();
        let ca = angle.cos();
        let sa = angle.sin();
        assert!(approx_eq(ax[2], ca, EPS));
        assert!(approx_eq(ax[3], sa, EPS));
    }

    #[test]
    fn ellipse2d_rotated_value_at_zero() {
        let angle = std::f64::consts::PI / 2.0;
        let e = Ellipse2d::new([0.0, 0.0], 5.0, 2.0, angle);
        // At u=0 the point should be in the direction of the rotated major axis
        let p = e.value(0.0);
        // major axis is now pointing in Y direction after pi/2 rotation
        assert!(approx_eq2(p, [0.0, 5.0], EPS));
    }

    // -----------------------------------------------------------------------
    // Line3d
    // -----------------------------------------------------------------------

    #[test]
    fn line3d_value_at_endpoints() {
        let l = Line3d::new([1.0, 2.0, 3.0], [4.0, 6.0, 3.0]);
        let p0 = l.value(0.0);
        let p1 = l.value(l.last_param());
        assert!(approx_eq3(p0, [1.0, 2.0, 3.0], EPS));
        assert!(approx_eq3(p1, [4.0, 6.0, 3.0], EPS));
    }

    #[test]
    fn line3d_length() {
        let l = Line3d::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert!(approx_eq(l.length(), 1.0, EPS));
    }

    #[test]
    fn line3d_bounded_curve() {
        let l = Line3d::new([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
        assert!(approx_eq3(l.start_point(), [1.0, 2.0, 3.0], EPS));
        assert!(approx_eq3(l.end_point(), [4.0, 5.0, 6.0], EPS));
    }

    #[test]
    fn line3d_not_periodic() {
        let l = Line3d::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        assert!(!l.is_periodic());
        assert!(approx_eq(l.period(), 0.0, EPS));
    }

    // -----------------------------------------------------------------------
    // Circle3d
    // -----------------------------------------------------------------------

    #[test]
    fn circle3d_value_at_zero_xy_plane() {
        let c = Circle3d::new([0.0, 0.0, 0.0], 2.0, [0.0, 0.0, 1.0]);
        let p = c.value(0.0);
        // Should lie in the XY plane at radius 2
        let r = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
        assert!(approx_eq(r, 2.0, EPS));
        assert!(approx_eq(p[2], 0.0, EPS));
    }

    #[test]
    fn circle3d_points_on_circle() {
        let r = 3.0;
        let c = Circle3d::new([1.0, 0.0, 0.0], r, [0.0, 0.0, 1.0]);
        for i in 0..16 {
            let u = i as f64 * std::f64::consts::PI / 8.0;
            let p = c.value(u);
            // distance from center should equal radius
            let dx = p[0] - 1.0;
            let dy = p[1];
            let dz = p[2];
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();
            assert!(approx_eq(dist, r, EPS), "u={}: dist={}", u, dist);
        }
    }

    #[test]
    fn circle3d_d1_orthogonal_to_radius() {
        let c = Circle3d::new([0.0, 0.0, 2.0], 4.0, [0.0, 0.0, 1.0]);
        let u = 1.1;
        let (pt, d) = c.d1(u);
        // radius vector from center
        let rx = pt[0] - c.center[0];
        let ry = pt[1] - c.center[1];
        let rz = pt[2] - c.center[2];
        let dot = rx * d[0] + ry * d[1] + rz * d[2];
        assert!(approx_eq(dot, 0.0, EPS));
    }

    #[test]
    fn circle3d_is_periodic_and_closed() {
        let c = Circle3d::new([0.0, 0.0, 0.0], 1.0, [0.0, 0.0, 1.0]);
        assert!(c.is_periodic());
        assert!(c.is_closed());
        assert!(approx_eq(c.period(), 2.0 * std::f64::consts::PI, EPS));
    }

    #[test]
    fn circle3d_conic_eccentricity() {
        let c = Circle3d::new([0.0, 0.0, 0.0], 5.0, [0.0, 0.0, 1.0]);
        assert!(approx_eq(c.eccentricity(), 0.0, EPS));
    }

    #[test]
    fn circle3d_conic_axis_has_unit_direction() {
        let n = [1.0_f64, 1.0, 0.0];
        let n_len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        let nn = [n[0] / n_len, n[1] / n_len, n[2] / n_len];
        let c = Circle3d::new([0.0, 0.0, 0.0], 2.0, nn);
        let ax = c.axis();
        let dir_len = (ax[3] * ax[3] + ax[4] * ax[4] + ax[5] * ax[5]).sqrt();
        assert!(approx_eq(dir_len, 1.0, EPS));
    }

    #[test]
    fn circle3d_clone_box() {
        let c = Circle3d::new([0.0, 0.0, 0.0], 1.0, [0.0, 0.0, 1.0]);
        let geom: &dyn GeomGeometry = &c;
        let _cloned = geom.clone_box();
    }

    // -----------------------------------------------------------------------
    // Trait object upcasting / dynamic dispatch chain
    // -----------------------------------------------------------------------

    #[test]
    fn trait_dispatch_geom2d_geometry_via_boxed_curve() {
        let line = Line2d::new([0.0, 0.0], [1.0, 0.0]);
        let boxed: Box<dyn Geom2dCurve> = Box::new(line);
        let pt = boxed.value(0.5);
        assert!(approx_eq2(pt, [0.5, 0.0], EPS));
        let (p2, d2) = boxed.d1(0.5);
        assert!(approx_eq2(p2, [0.5, 0.0], EPS));
        let _ = d2;
    }

    #[test]
    fn trait_dispatch_geom2d_conic_via_boxed() {
        let circle = Circle2d::new([1.0, 2.0], 3.0);
        let boxed: Box<dyn Geom2dConic> = Box::new(circle);
        assert!(approx_eq2(boxed.location(), [1.0, 2.0], EPS));
        assert!(approx_eq(boxed.eccentricity(), 0.0, EPS));
        assert!(boxed.is_periodic());
    }

    #[test]
    fn trait_dispatch_geom_3d_conic_via_boxed() {
        let circle = Circle3d::new([0.0, 0.0, 0.0], 1.0, [0.0, 0.0, 1.0]);
        let boxed: Box<dyn GeomConic> = Box::new(circle);
        assert!(approx_eq(boxed.eccentricity(), 0.0, EPS));
        let pt = boxed.value(0.0);
        let dist = (pt[0] * pt[0] + pt[1] * pt[1] + pt[2] * pt[2]).sqrt();
        assert!(approx_eq(dist, 1.0, EPS));
    }

    // -----------------------------------------------------------------------
    // Edge cases
    // -----------------------------------------------------------------------

    #[test]
    fn line2d_diagonal_midpoint_numerical() {
        // 3-4-5 triangle: from (0,0) to (3,4), length=5
        let l = Line2d::new([0.0, 0.0], [3.0, 4.0]);
        let mid = l.value(2.5);
        assert!(approx_eq2(mid, [1.5, 2.0], EPS));
    }

    #[test]
    fn circle2d_value_at_two_pi_equals_zero() {
        let c = Circle2d::new([0.0, 0.0], 1.0);
        let p0 = c.value(0.0);
        let p2pi = c.value(2.0 * std::f64::consts::PI);
        assert!(approx_eq2(p0, p2pi, EPS));
    }

    #[test]
    fn ellipse2d_full_orbit_returns_to_start() {
        let e = Ellipse2d::new([0.0, 0.0], 7.0, 3.0, 0.5);
        let p0 = e.value(0.0);
        let p2pi = e.value(2.0 * std::f64::consts::PI);
        assert!(approx_eq2(p0, p2pi, EPS));
    }

    #[test]
    fn circle3d_full_orbit_returns_to_start() {
        let c = Circle3d::new([2.0, 3.0, 4.0], 5.0, [0.0, 0.0, 1.0]);
        let p0 = c.value(0.0);
        let p2pi = c.value(2.0 * std::f64::consts::PI);
        assert!(approx_eq3(p0, p2pi, EPS));
    }
}
