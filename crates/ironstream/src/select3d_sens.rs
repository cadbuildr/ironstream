// FILE: select3d_sens.rs
//
// Zero-dependency pure-Rust stubs modelled after OCCT Select3D sensitive primitives.
// Each type carries an owner_id that corresponds to SelectMgr_EntityOwner in OCCT.

// ---------------------------------------------------------------------------
// occt: Select3D_SensitiveBox
// ---------------------------------------------------------------------------

/// Axis-aligned bounding-box sensitive entity.
/// Mirrors Select3D_SensitiveBox from OCCT.
// occt: Select3D_SensitiveBox
#[derive(Clone, Copy, Debug)]
pub struct SensitiveBox {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub owner_id: u32,
}

impl SensitiveBox {
    /// Construct a new box sensitive with the given axis-aligned extents and owner.
    pub fn new(min: [f64; 3], max: [f64; 3], owner: u32) -> Self {
        Self { min, max, owner_id: owner }
    }

    /// Slab-method (Kay-Kajiya) ray–AABB intersection test.
    ///
    /// Returns `true` when the ray defined by `origin` + t*`dir` (t >= 0)
    /// intersects this box.  `dir` need not be normalised.
    pub fn matches_ray(&self, origin: [f64; 3], dir: [f64; 3]) -> bool {
        let mut t_min = f64::NEG_INFINITY;
        let mut t_max = f64::INFINITY;

        for i in 0..3 {
            let d = dir[i];
            if d.abs() < f64::EPSILON {
                // Ray is parallel to this slab; check if origin is within it.
                if origin[i] < self.min[i] || origin[i] > self.max[i] {
                    return false;
                }
            } else {
                let inv_d = 1.0 / d;
                let mut t1 = (self.min[i] - origin[i]) * inv_d;
                let mut t2 = (self.max[i] - origin[i]) * inv_d;
                if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                }
                if t1 > t_min {
                    t_min = t1;
                }
                if t2 < t_max {
                    t_max = t2;
                }
                if t_min > t_max {
                    return false;
                }
            }
        }

        // Intersection must be in the forward direction (t >= 0).
        t_max >= 0.0
    }

    /// Returns the diagonal length of the box, used as a proxy for sensitivity
    /// (analogous to the tolerance value in OCCT's Select3D_SensitiveBox).
    pub fn sensitivity(&self) -> f64 {
        let dx = self.max[0] - self.min[0];
        let dy = self.max[1] - self.min[1];
        let dz = self.max[2] - self.min[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

// ---------------------------------------------------------------------------
// occt: Select3D_SensitiveSphere
// ---------------------------------------------------------------------------

/// Sphere sensitive entity.
/// Mirrors Select3D_SensitiveSphere from OCCT.
// occt: Select3D_SensitiveSphere
#[derive(Clone, Copy, Debug)]
pub struct SensitiveSphere {
    pub center: [f64; 3],
    pub radius: f64,
    pub owner_id: u32,
}

impl SensitiveSphere {
    /// Construct a new sphere sensitive with the given centre, radius, and owner.
    pub fn new(center: [f64; 3], r: f64, owner: u32) -> Self {
        Self { center, radius: r, owner_id: owner }
    }

    /// Analytic ray–sphere intersection test.
    ///
    /// Solves ||(origin + t*dir) - center||^2 = radius^2 for real t >= 0.
    /// Returns `true` when at least one non-negative solution exists.
    /// `dir` need not be normalised.
    pub fn matches_ray(&self, origin: [f64; 3], dir: [f64; 3]) -> bool {
        // Vector from sphere centre to ray origin.
        let oc = [
            origin[0] - self.center[0],
            origin[1] - self.center[1],
            origin[2] - self.center[2],
        ];

        let a = dot(dir, dir);
        if a < f64::EPSILON {
            // Degenerate (zero-length) direction: check if origin is inside sphere.
            return dot(oc, oc) <= self.radius * self.radius;
        }

        let b = 2.0 * dot(oc, dir);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_disc = discriminant.sqrt();
        let t1 = (-b - sqrt_disc) / (2.0 * a);
        let t2 = (-b + sqrt_disc) / (2.0 * a);

        // Hit is valid when at least one intersection lies ahead of the origin.
        t1 >= 0.0 || t2 >= 0.0
    }
}

// ---------------------------------------------------------------------------
// occt: Select3D_SensitiveFace
// ---------------------------------------------------------------------------

/// Planar polygon (face) sensitive entity.
/// Mirrors Select3D_SensitiveFace from OCCT.
// occt: Select3D_SensitiveFace
#[derive(Clone, Debug)]
pub struct SensitiveFace {
    pub points: Vec<[f64; 3]>,
    pub owner_id: u32,
}

impl SensitiveFace {
    /// Construct a new empty face sensitive with the given owner.
    pub fn new(owner: u32) -> Self {
        Self { points: Vec::new(), owner_id: owner }
    }

    /// Append a vertex to the polygon.
    pub fn add_point(&mut self, p: [f64; 3]) {
        self.points.push(p);
    }

    /// Ray–polygon intersection test using the Möller–Trumbore algorithm applied
    /// to each triangle fan from the first vertex.
    ///
    /// The face must have at least three points. Sub-triangles are formed as
    /// (points[0], points[i], points[i+1]) for i in 1..n-1.
    ///
    /// Returns `true` when the ray hits any triangle in the fan (t >= 0).
    pub fn matches_ray(&self, origin: [f64; 3], dir: [f64; 3]) -> bool {
        let n = self.points.len();
        if n < 3 {
            return false;
        }

        let v0 = self.points[0];
        for i in 1..(n - 1) {
            let v1 = self.points[i];
            let v2 = self.points[i + 1];
            if moller_trumbore(origin, dir, v0, v1, v2) {
                return true;
            }
        }
        false
    }
}

// ---------------------------------------------------------------------------
// Internal geometry helpers
// ---------------------------------------------------------------------------

#[inline]
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// Möller–Trumbore ray–triangle intersection.
/// Returns `true` when the ray hits the (double-sided) triangle with t >= 0.
fn moller_trumbore(
    origin: [f64; 3],
    dir: [f64; 3],
    v0: [f64; 3],
    v1: [f64; 3],
    v2: [f64; 3],
) -> bool {
    const EPSILON: f64 = 1e-10;

    let edge1 = sub(v1, v0);
    let edge2 = sub(v2, v0);
    let h = cross(dir, edge2);
    let a = dot(edge1, h);

    if a.abs() < EPSILON {
        // Ray is parallel to the triangle.
        return false;
    }

    let f = 1.0 / a;
    let s = sub(origin, v0);
    let u = f * dot(s, h);
    if !(0.0..=1.0).contains(&u) {
        return false;
    }

    let q = cross(s, edge1);
    let v = f * dot(dir, q);
    if v < 0.0 || u + v > 1.0 {
        return false;
    }

    let t = f * dot(edge2, q);
    t >= 0.0
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- SensitiveBox ---

    #[test]
    fn test_box_new_fields() {
        let b = SensitiveBox::new([1.0, 2.0, 3.0], [4.0, 5.0, 6.0], 7);
        assert_eq!(b.min, [1.0, 2.0, 3.0]);
        assert_eq!(b.max, [4.0, 5.0, 6.0]);
        assert_eq!(b.owner_id, 7);
    }

    #[test]
    fn test_box_sensitivity_unit_cube() {
        let b = SensitiveBox::new([0.0; 3], [1.0, 1.0, 1.0], 0);
        let expected = 3_f64.sqrt();
        assert!((b.sensitivity() - expected).abs() < 1e-12);
    }

    #[test]
    fn test_box_ray_hit_along_x_axis() {
        let b = SensitiveBox::new([-1.0; 3], [1.0; 3], 0);
        assert!(b.matches_ray([-5.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_box_ray_miss() {
        let b = SensitiveBox::new([-1.0; 3], [1.0; 3], 0);
        assert!(!b.matches_ray([-5.0, 2.0, 0.0], [1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_box_ray_origin_inside() {
        let b = SensitiveBox::new([-1.0; 3], [1.0; 3], 0);
        assert!(b.matches_ray([0.0; 3], [1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_box_ray_wrong_direction() {
        let b = SensitiveBox::new([2.0; 3], [4.0; 3], 0);
        // Ray points away from the box.
        assert!(!b.matches_ray([0.0; 3], [-1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_box_ray_diagonal() {
        let b = SensitiveBox::new([0.0; 3], [2.0; 3], 0);
        assert!(b.matches_ray([-1.0; 3], [1.0, 1.0, 1.0]));
    }

    // --- SensitiveSphere ---

    #[test]
    fn test_sphere_new_fields() {
        let s = SensitiveSphere::new([1.0, 2.0, 3.0], 5.0, 42);
        assert_eq!(s.center, [1.0, 2.0, 3.0]);
        assert_eq!(s.radius, 5.0);
        assert_eq!(s.owner_id, 42);
    }

    #[test]
    fn test_sphere_ray_hit_through_centre() {
        let s = SensitiveSphere::new([0.0; 3], 1.0, 0);
        assert!(s.matches_ray([-5.0, 0.0, 0.0], [1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_sphere_ray_tangent() {
        let s = SensitiveSphere::new([0.0; 3], 1.0, 0);
        // Tangent ray along y = 1 plane.
        assert!(s.matches_ray([-5.0, 1.0, 0.0], [1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_sphere_ray_miss() {
        let s = SensitiveSphere::new([0.0; 3], 1.0, 0);
        assert!(!s.matches_ray([-5.0, 1.5, 0.0], [1.0, 0.0, 0.0]));
    }

    #[test]
    fn test_sphere_ray_origin_inside() {
        let s = SensitiveSphere::new([0.0; 3], 10.0, 0);
        assert!(s.matches_ray([0.0; 3], [0.0, 1.0, 0.0]));
    }

    #[test]
    fn test_sphere_ray_wrong_direction() {
        let s = SensitiveSphere::new([5.0, 0.0, 0.0], 1.0, 0);
        // Ray points away from the sphere.
        assert!(!s.matches_ray([0.0; 3], [-1.0, 0.0, 0.0]));
    }

    // --- SensitiveFace ---

    #[test]
    fn test_face_new_empty() {
        let f = SensitiveFace::new(99);
        assert!(f.points.is_empty());
        assert_eq!(f.owner_id, 99);
    }

    #[test]
    fn test_face_add_point() {
        let mut f = SensitiveFace::new(0);
        f.add_point([0.0, 0.0, 0.0]);
        f.add_point([1.0, 0.0, 0.0]);
        f.add_point([0.0, 1.0, 0.0]);
        assert_eq!(f.points.len(), 3);
    }

    #[test]
    fn test_face_ray_hit_triangle_xy_plane() {
        let mut f = SensitiveFace::new(0);
        f.add_point([0.0, 0.0, 0.0]);
        f.add_point([2.0, 0.0, 0.0]);
        f.add_point([0.0, 2.0, 0.0]);
        // Ray coming from above, hitting the triangle centroid.
        assert!(f.matches_ray([0.5, 0.5, 5.0], [0.0, 0.0, -1.0]));
    }

    #[test]
    fn test_face_ray_miss_triangle() {
        let mut f = SensitiveFace::new(0);
        f.add_point([0.0, 0.0, 0.0]);
        f.add_point([1.0, 0.0, 0.0]);
        f.add_point([0.0, 1.0, 0.0]);
        // Ray aimed away from the triangle.
        assert!(!f.matches_ray([5.0, 5.0, 1.0], [0.0, 0.0, -1.0]));
    }

    #[test]
    fn test_face_too_few_points() {
        let mut f = SensitiveFace::new(0);
        f.add_point([0.0, 0.0, 0.0]);
        f.add_point([1.0, 0.0, 0.0]);
        assert!(!f.matches_ray([0.5, 0.0, 1.0], [0.0, 0.0, -1.0]));
    }

    #[test]
    fn test_face_quad_hit_second_triangle() {
        // Quad: two triangles forming a unit square in the XY plane.
        let mut f = SensitiveFace::new(0);
        f.add_point([0.0, 0.0, 0.0]);
        f.add_point([1.0, 0.0, 0.0]);
        f.add_point([1.0, 1.0, 0.0]);
        f.add_point([0.0, 1.0, 0.0]);
        // Hit a point that lies in the second triangle of the fan.
        assert!(f.matches_ray([0.1, 0.9, 1.0], [0.0, 0.0, -1.0]));
    }

    #[test]
    fn test_face_ray_wrong_direction() {
        let mut f = SensitiveFace::new(0);
        f.add_point([0.0, 0.0, 0.0]);
        f.add_point([2.0, 0.0, 0.0]);
        f.add_point([0.0, 2.0, 0.0]);
        // Ray pointing upward, away from the XY-plane triangle.
        assert!(!f.matches_ray([0.5, 0.5, -5.0], [0.0, 0.0, -1.0]));
    }
}
