// FILE: poly_occt.rs
//
// 3D triangular polyhedrons, 3D polygons, and 2D polygon tools.
// Faithful port of OCCT's Poly package geometric algorithms.

// Simple 2D point type (replaces gp_Pnt2d)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }

    pub fn xy(&self) -> [f64; 2] {
        [self.x, self.y]
    }
}

impl std::ops::Add for Point2D {
    type Output = Point2D;
    fn add(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point2D {
    type Output = Point2D;
    fn sub(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f64> for Point2D {
    type Output = Point2D;
    fn mul(self, scalar: f64) -> Point2D {
        Point2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl std::ops::Mul<Point2D> for f64 {
    type Output = Point2D;
    fn mul(self, point: Point2D) -> Point2D {
        Point2D {
            x: self * point.x,
            y: self * point.y,
        }
    }
}

// Simple 3D point type (replaces gp_Pnt and gp_XYZ)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    pub fn xyz(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn distance_squared(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }

    pub fn distance(&self, other: &Point3D) -> f64 {
        self.distance_squared(other).sqrt()
    }
}

impl std::ops::Sub for Point3D {
    type Output = Point3D;
    fn sub(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Add for Point3D {
    type Output = Point3D;
    fn add(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Mul<f64> for Point3D {
    type Output = Point3D;
    fn mul(self, scalar: f64) -> Point3D {
        Point3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

// Direction in 3D (normalized vector)
#[derive(Debug, Clone, Copy)]
pub struct Direction3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Direction3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let len = (x * x + y * y + z * z).sqrt();
        if len > 1e-15 {
            Direction3D {
                x: x / len,
                y: y / len,
                z: z / len,
            }
        } else {
            Direction3D { x: 0.0, y: 0.0, z: 0.0 }
        }
    }

    pub fn dot(&self, other: &Direction3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Direction3D) -> Direction3D {
        Direction3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

// occt: Poly
/// Tools for handling 3D triangular polyhedrons, 3D polygons, and 2D polygons.
pub struct Poly;

impl Poly {
    /// Computes parameters of the point P on triangle
    /// defined by points P1, P2, and P3, in 2d.
    /// The parameters U and V are defined so that
    /// P = P1 + U * (P2 - P1) + V * (P3 - P1),
    /// with U >= 0, V >= 0, U + V <= 1.
    /// If P is located outside of triangle, or triangle
    /// is degenerated, the returned parameters correspond
    /// to closest point, and returned value is square of
    /// the distance from original point to triangle (0 if
    /// point is inside).
    pub fn point_on_triangle(
        p1: Point2D,
        p2: Point2D,
        p3: Point2D,
        p: Point2D,
    ) -> (f64, f64, f64) {
        let ap = p - p1;
        let du = p2 - p1;
        let dv = p3 - p1;

        // Cross product in 2D: du ^ dv = du.x * dv.y - du.y * dv.x
        let det = du.x * dv.y - du.y * dv.x;

        const RESOLUTION: f64 = 1e-15;

        // Case of non-degenerated triangle
        if det.abs() > RESOLUTION {
            let u = (ap.x * dv.y - ap.y * dv.x) / det;
            let v = -(ap.x * du.y - ap.y * du.x) / det;

            // If point is inside triangle, return parameters and zero distance
            if u > -RESOLUTION && v > -RESOLUTION && (1.0 - u - v) > -RESOLUTION {
                return (u, v, 0.0);
            }

            // Point is outside triangle; find closest point on triangle sides
            // Project on side U=0 (edge P1-P3)
            let mut best_u = 0.0;
            let mut best_v =
                (ap.x * dv.x + ap.y * dv.y) / (dv.x * dv.x + dv.y * dv.y);
            best_v = best_v.max(0.0).min(1.0);
            let offset_v = best_v * dv;
            let mut best_d = (offset_v.x - ap.x).powi(2) + (offset_v.y - ap.y).powi(2);

            // Project on side V=0 (edge P1-P2)
            let u_param = (ap.x * du.x + ap.y * du.y) / (du.x * du.x + du.y * du.y);
            let u_clamped = u_param.max(0.0).min(1.0);
            let offset_u = u_clamped * du;
            let d = (offset_u.x - ap.x).powi(2) + (offset_u.y - ap.y).powi(2);
            if d < best_d {
                best_u = u_clamped;
                best_v = 0.0;
                best_d = d;
            }

            // Project on side U+V=1 (edge P2-P3)
            let duv = dv - du;
            let v_param = ((ap.x - du.x) * duv.x + (ap.y - du.y) * duv.y)
                / (duv.x * duv.x + duv.y * duv.y);
            let v_clamped = v_param.max(0.0).min(1.0);
            let p2_offset = du + v_clamped * duv;
            let d = (p2_offset.x - ap.x).powi(2) + (p2_offset.y - ap.y).powi(2);
            if d < best_d {
                best_u = 1.0 - v_clamped;
                best_v = v_clamped;
                best_d = d;
            }

            return (best_u, best_v, best_d);
        }

        // Degenerated triangle
        let l2u = du.x * du.x + du.y * du.y;
        let l2v = dv.x * dv.x + dv.y * dv.y;

        if l2u < RESOLUTION {
            // Side 1-2 is degenerated
            if l2v < RESOLUTION {
                // Whole triangle is degenerated to point
                let d = ap.x * ap.x + ap.y * ap.y;
                return (0.0, 0.0, d);
            } else {
                let v = (ap.x * dv.x + ap.y * dv.y) / l2v;
                let offset = dv * v;
                let d = (offset.x - ap.x).powi(2) + (offset.y - ap.y).powi(2);
                return (0.0, v, d);
            }
        } else if l2v < RESOLUTION {
            // Side 1-3 is degenerated
            let u = (ap.x * du.x + ap.y * du.y) / l2u;
            let offset = du * u;
            let d = (offset.x - ap.x).powi(2) + (offset.y - ap.y).powi(2);
            return (u, 0.0, d);
        } else {
            // Sides 1-2 and 1-3 are collinear
            let u = (ap.x * du.x + ap.y * du.y) / l2u;
            let v = (ap.x * dv.x + ap.y * dv.y) / l2v;

            let u_clamped = u.max(0.0).min(1.0);
            let v_clamped = v.max(0.0).min(1.0);

            let offset_u = du * u_clamped;
            let d1 = (ap.x - offset_u.x).powi(2) + (ap.y - offset_u.y).powi(2);

            let offset_v = dv * v_clamped;
            let d2 = (ap.x - offset_v.x).powi(2) + (ap.y - offset_v.y).powi(2);

            if d1 < d2 {
                return (u_clamped, 0.0, d1);
            } else {
                return (0.0, v_clamped, d2);
            }
        }
    }

    /// Computes the intersection between a triangle defined by three vertexes and a line.
    /// Returns (intersects, parameter_on_line)
    /// where parameter_on_line is the distance along the ray to the intersection point.
    pub fn intersect_tri_line(
        start: Point3D,
        dir: Direction3D,
        v0: Point3D,
        v1: Point3D,
        v2: Point3D,
    ) -> (bool, f64) {
        const CONF: f64 = 1e-15;

        // Build 3x4 matrix:
        // [-dir.x,  v1.x - v0.x,  v2.x - v0.x,  start.x - v0.x]
        // [-dir.y,  v1.y - v0.y,  v2.y - v0.y,  start.y - v0.y]
        // [-dir.z,  v1.z - v0.z,  v2.z - v0.z,  start.z - v0.z]

        let mat = [
            [
                -dir.x,
                v1.x - v0.x,
                v2.x - v0.x,
                start.x - v0.x,
            ],
            [
                -dir.y,
                v1.y - v0.y,
                v2.y - v0.y,
                start.y - v0.y,
            ],
            [
                -dir.z,
                v1.z - v0.z,
                v2.z - v0.z,
                start.z - v0.z,
            ],
        ];

        // Calculate determinants
        let d = determinant_3x3(&mat, 0, 1, 2);
        let dt = determinant_3x3(&mat, 3, 1, 2);

        let mut found = false;

        if d.abs() > CONF {
            if d > 0.0 {
                let da = determinant_3x3(&mat, 0, 3, 2);
                if da > -CONF {
                    let db = determinant_3x3(&mat, 0, 1, 3);
                    found = (db > -CONF) && (da + db <= d + CONF);
                }
            } else {
                let da = determinant_3x3(&mat, 0, 3, 2);
                if da < CONF {
                    let db = determinant_3x3(&mat, 0, 1, 3);
                    found = (db < CONF) && (da + db >= d - CONF);
                }
            }
        }

        if found {
            (true, dt / d)
        } else {
            (false, 0.0)
        }
    }

    /// Returns area and perimeter of 2D-polygon given by its vertices.
    /// area will be negative if the polygon is bypassed clockwise
    /// and will be positive, otherwise. perimeter will always be positive.
    pub fn polygon_properties(points: &[Point2D]) -> (f64, f64) {
        if points.len() < 2 {
            return (0.0, 0.0);
        }

        let ref_pnt = points[0];
        let mut prev_pt = points[1] - ref_pnt;

        let mut area = 0.0;
        let mut perimeter = (prev_pt.x.powi(2) + prev_pt.y.powi(2)).sqrt();

        for i in 2..points.len() {
            let curr_pt = points[i] - ref_pnt;

            // Cross product in 2D
            let delta = prev_pt.x * curr_pt.y - prev_pt.y * curr_pt.x;
            area += delta;

            // Distance between previous and current
            let dx = prev_pt.x - curr_pt.x;
            let dy = prev_pt.y - curr_pt.y;
            perimeter += (dx * dx + dy * dy).sqrt();

            prev_pt = curr_pt;
        }

        perimeter += (prev_pt.x.powi(2) + prev_pt.y.powi(2)).sqrt();
        area *= 0.5;

        (area, perimeter)
    }
}

/// Calculate the minor (3x3 determinant) of the given 3x4 matrix,
/// defined by the columns specified by c1, c2, c3.
fn determinant_3x3(mat: &[[f64; 4]; 3], c1: usize, c2: usize, c3: usize) -> f64 {
    let a = &mat;
    a[0][c1] * a[1][c2] * a[2][c3]
        + a[0][c2] * a[1][c3] * a[2][c1]
        + a[0][c3] * a[1][c1] * a[2][c2]
        - a[0][c3] * a[1][c2] * a[2][c1]
        - a[0][c2] * a[1][c1] * a[2][c3]
        - a[0][c1] * a[1][c3] * a[2][c2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_on_triangle_inside() {
        // Triangle with vertices at (0,0), (1,0), (0,1)
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(1.0, 0.0);
        let p3 = Point2D::new(0.0, 1.0);

        // Point inside: (0.25, 0.25)
        let p = Point2D::new(0.25, 0.25);
        let (u, v, dist_sq) = Poly::point_on_triangle(p1, p2, p3, p);

        // Inside point should give u ≈ 0.25, v ≈ 0.25, dist ≈ 0
        assert!(u > 0.2 && u < 0.3, "u={}", u);
        assert!(v > 0.2 && v < 0.3, "v={}", v);
        assert!(dist_sq < 0.01, "dist_sq={}", dist_sq);
    }

    #[test]
    fn test_point_on_triangle_vertex() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(1.0, 0.0);
        let p3 = Point2D::new(0.0, 1.0);

        // Point at vertex P1
        let p = Point2D::new(0.0, 0.0);
        let (u, v, dist_sq) = Poly::point_on_triangle(p1, p2, p3, p);

        assert!(u.abs() < 0.01, "u={}", u);
        assert!(v.abs() < 0.01, "v={}", v);
        assert!(dist_sq < 0.01, "dist_sq={}", dist_sq);
    }

    #[test]
    fn test_point_on_triangle_outside() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(1.0, 0.0);
        let p3 = Point2D::new(0.0, 1.0);

        // Point outside triangle
        let p = Point2D::new(2.0, 2.0);
        let (u, v, dist_sq) = Poly::point_on_triangle(p1, p2, p3, p);

        // Should give positive distance
        assert!(dist_sq > 1.0, "dist_sq={}", dist_sq);
    }

    #[test]
    fn test_point_on_degenerate_triangle_point() {
        // Degenerate triangle: all points at same location
        let p1 = Point2D::new(1.0, 1.0);
        let p2 = Point2D::new(1.0, 1.0);
        let p3 = Point2D::new(1.0, 1.0);

        let p = Point2D::new(2.0, 2.0);
        let (u, v, dist_sq) = Poly::point_on_triangle(p1, p2, p3, p);

        // Distance should be sqrt(2)^2 = 2
        assert!(dist_sq > 1.9 && dist_sq < 2.1, "dist_sq={}", dist_sq);
    }

    #[test]
    fn test_intersect_tri_line_hit() {
        // Triangle vertices
        let v0 = Point3D::new(0.0, 0.0, 0.0);
        let v1 = Point3D::new(1.0, 0.0, 0.0);
        let v2 = Point3D::new(0.0, 1.0, 0.0);

        // Ray starting at z=-1, pointing in +z direction
        let start = Point3D::new(0.3, 0.3, -1.0);
        let dir = Direction3D::new(0.0, 0.0, 1.0);

        let (intersects, param) = Poly::intersect_tri_line(start, dir, v0, v1, v2);

        assert!(intersects, "should intersect triangle");
        assert!((param - 1.0).abs() < 0.1, "param={}", param);
    }

    #[test]
    fn test_intersect_tri_line_miss() {
        // Triangle in XY plane at z=0
        let v0 = Point3D::new(0.0, 0.0, 0.0);
        let v1 = Point3D::new(1.0, 0.0, 0.0);
        let v2 = Point3D::new(0.0, 1.0, 0.0);

        // Ray outside triangle footprint
        let start = Point3D::new(2.0, 2.0, -1.0);
        let dir = Direction3D::new(0.0, 0.0, 1.0);

        let (intersects, _param) = Poly::intersect_tri_line(start, dir, v0, v1, v2);

        assert!(!intersects, "should not intersect");
    }

    #[test]
    fn test_polygon_properties_square() {
        // Unit square: (0,0), (1,0), (1,1), (0,1)
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(0.0, 1.0),
        ];

        let (area, perimeter) = Poly::polygon_properties(&points);

        // Area should be 1.0 (positive for CCW)
        assert!((area - 1.0).abs() < 0.01, "area={}", area);
        // Perimeter should be 4.0
        assert!((perimeter - 4.0).abs() < 0.01, "perimeter={}", perimeter);
    }

    #[test]
    fn test_polygon_properties_triangle() {
        // Right triangle: (0,0), (2,0), (0,2)
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(2.0, 0.0),
            Point2D::new(0.0, 2.0),
        ];

        let (area, perimeter) = Poly::polygon_properties(&points);

        // Area should be 2.0 (0.5 * base * height = 0.5 * 2 * 2)
        assert!((area - 2.0).abs() < 0.01, "area={}", area);
        // Perimeter: 2 + 2 + 2*sqrt(2) ≈ 6.828
        assert!(perimeter > 6.8 && perimeter < 6.9, "perimeter={}", perimeter);
    }

    #[test]
    fn test_polygon_properties_degenerate() {
        // Single point repeated (degenerate)
        let points = vec![Point2D::new(1.0, 1.0)];

        let (area, perimeter) = Poly::polygon_properties(&points);

        assert_eq!(area, 0.0);
        assert_eq!(perimeter, 0.0);
    }

    #[test]
    fn test_point_2d_operations() {
        let p1 = Point2D::new(1.0, 2.0);
        let p2 = Point2D::new(3.0, 4.0);

        let sum = p1 + p2;
        assert_eq!(sum.x, 4.0);
        assert_eq!(sum.y, 6.0);

        let diff = p2 - p1;
        assert_eq!(diff.x, 2.0);
        assert_eq!(diff.y, 2.0);

        let scaled = p1 * 2.0;
        assert_eq!(scaled.x, 2.0);
        assert_eq!(scaled.y, 4.0);
    }

    #[test]
    fn test_point_3d_distance() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(3.0, 4.0, 0.0);

        let dist = p1.distance(&p2);
        assert!((dist - 5.0).abs() < 0.01);

        let dist_sq = p1.distance_squared(&p2);
        assert!((dist_sq - 25.0).abs() < 0.01);
    }

    #[test]
    fn test_direction_normalization() {
        let dir = Direction3D::new(3.0, 4.0, 0.0);
        let len = (dir.x * dir.x + dir.y * dir.y + dir.z * dir.z).sqrt();
        assert!((len - 1.0).abs() < 1e-10, "direction should be normalized");
    }
}
