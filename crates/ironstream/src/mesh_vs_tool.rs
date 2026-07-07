// FILE: mesh_vs_tool.rs
// occt: MeshVS_Tool

/// Auxiliary structure to represent a 3D point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    /// Creates a new point with the given coordinates
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    /// Returns the distance to another point
    pub fn distance_to(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Returns a new point at the midpoint between this and another point
    pub fn midpoint(&self, other: &Point3D) -> Point3D {
        Point3D {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
            z: (self.z + other.z) / 2.0,
        }
    }
}

/// Auxiliary structure to represent a 3D vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    /// Creates a new vector with the given components
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    /// Creates a vector from one point to another
    pub fn from_points(from: &Point3D, to: &Point3D) -> Self {
        Vector3D {
            x: to.x - from.x,
            y: to.y - from.y,
            z: to.z - from.z,
        }
    }

    /// Returns the magnitude (length) of the vector
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Returns a normalized unit vector in the same direction
    pub fn normalize(&self) -> Vector3D {
        let mag = self.magnitude();
        if mag == 0.0 {
            Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            Vector3D {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        }
    }

    /// Computes the cross product with another vector
    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Computes the dot product with another vector
    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

/// This class provides auxiliary methods to work with mesh visualization.
pub struct MeshVSTool;

impl MeshVSTool {
    /// Computes the normal to a polygon described by the given points.
    /// If the polygon is planar, returns true and sets the normal.
    /// Returns false if the polygon is not planar or degenerate.
    pub fn get_normal(nodes: &[Point3D]) -> Option<Vector3D> {
        if nodes.len() < 3 {
            return None;
        }

        // Compute a normal using the first three points
        let v1 = Vector3D::from_points(&nodes[0], &nodes[1]);
        let v2 = Vector3D::from_points(&nodes[0], &nodes[2]);
        let normal = v1.cross(&v2);

        if normal.magnitude() < 1e-10 {
            // First three points are collinear
            return None;
        }

        let normalized = normal.normalize();

        // Check if all points are coplanar by verifying they lie on the same plane
        // defined by the first point and the computed normal
        let tolerance = 1e-7;
        for i in 3..nodes.len() {
            let v = Vector3D::from_points(&nodes[0], &nodes[i]);
            let distance = (v.dot(&normalized)).abs();
            if distance > tolerance {
                // Point is not coplanar with the others
                return None;
            }
        }

        Some(normalized)
    }

    /// Computes the average of normals to a potentially non-planar polygon,
    /// or computes the normal of a planar polygon.
    /// Returns the averaged/computed normal if successful, None otherwise.
    pub fn get_average_normal(nodes: &[Point3D]) -> Option<Vector3D> {
        if nodes.len() < 3 {
            return None;
        }

        // Compute average normal by summing cross products of consecutive edges
        let mut avg_normal = Vector3D::new(0.0, 0.0, 0.0);
        let num_nodes = nodes.len();

        for i in 0..num_nodes {
            let p0 = &nodes[i];
            let p1 = &nodes[(i + 1) % num_nodes];
            let p2 = &nodes[(i + 2) % num_nodes];

            let v1 = Vector3D::from_points(p0, p1);
            let v2 = Vector3D::from_points(p0, p2);
            let cross = v1.cross(&v2);

            avg_normal.x += cross.x;
            avg_normal.y += cross.y;
            avg_normal.z += cross.z;
        }

        let magnitude = avg_normal.magnitude();
        if magnitude < 1e-10 {
            return None;
        }

        Some(avg_normal.normalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point3d_creation() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    #[test]
    fn test_point3d_distance() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(3.0, 4.0, 0.0);
        assert!((p1.distance_to(&p2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_point3d_midpoint() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(2.0, 4.0, 6.0);
        let mid = p1.midpoint(&p2);
        assert!((mid.x - 1.0).abs() < 1e-10);
        assert!((mid.y - 2.0).abs() < 1e-10);
        assert!((mid.z - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector3d_magnitude() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        assert!((v.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector3d_normalize() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        let n = v.normalize();
        assert!((n.magnitude() - 1.0).abs() < 1e-10);
        assert!((n.x - 0.6).abs() < 1e-10);
        assert!((n.y - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_vector3d_cross_product() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2);
        assert!((cross.x - 0.0).abs() < 1e-10);
        assert!((cross.y - 0.0).abs() < 1e-10);
        assert!((cross.z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector3d_dot_product() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(1.0, 0.0, 0.0);
        assert!((v1.dot(&v2) - 1.0).abs() < 1e-10);

        let v3 = Vector3D::new(0.0, 1.0, 0.0);
        assert!((v1.dot(&v3) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_get_normal_planar_triangle() {
        let nodes = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(0.0, 1.0, 0.0),
        ];
        let normal = MeshVSTool::get_normal(&nodes);
        assert!(normal.is_some());
        let n = normal.unwrap();
        // Normal should be pointing in +Z or -Z direction
        assert!(n.z.abs() > 0.99);
    }

    #[test]
    fn test_get_normal_collinear() {
        let nodes = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(2.0, 0.0, 0.0),
        ];
        let normal = MeshVSTool::get_normal(&nodes);
        assert!(normal.is_none());
    }

    #[test]
    fn test_get_normal_degenerate() {
        let nodes = vec![Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 0.0, 0.0)];
        let normal = MeshVSTool::get_normal(&nodes);
        assert!(normal.is_none());
    }

    #[test]
    fn test_get_average_normal_triangle() {
        let nodes = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(0.0, 1.0, 0.0),
        ];
        let normal = MeshVSTool::get_average_normal(&nodes);
        assert!(normal.is_some());
        let n = normal.unwrap();
        assert!(n.z.abs() > 0.99);
    }

    #[test]
    fn test_get_average_normal_square() {
        let nodes = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(1.0, 1.0, 0.0),
            Point3D::new(0.0, 1.0, 0.0),
        ];
        let normal = MeshVSTool::get_average_normal(&nodes);
        assert!(normal.is_some());
        let n = normal.unwrap();
        assert!(n.z.abs() > 0.99);
    }
}
