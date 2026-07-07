// FILE: b_rep_lib_make_vertex.rs
// occt: BRepLib_MakeVertex

/// Provides methods to build vertices from 3D points.
/// A vertex is a topological element representing a point in 3D space.
pub struct BRepLibMakeVertex {
    /// The 3D coordinates of the vertex [x, y, z]
    point: [f64; 3],
    /// Vertex is valid if created from a valid point
    is_done: bool,
}

impl BRepLibMakeVertex {
    /// Create a vertex from a 3D point
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        BRepLibMakeVertex {
            point: [x, y, z],
            is_done: true,
        }
    }

    /// Get the vertex coordinates as [x, y, z]
    pub fn vertex(&self) -> [f64; 3] {
        self.point
    }

    /// Get x coordinate
    pub fn x(&self) -> f64 {
        self.point[0]
    }

    /// Get y coordinate
    pub fn y(&self) -> f64 {
        self.point[1]
    }

    /// Get z coordinate
    pub fn z(&self) -> f64 {
        self.point[2]
    }

    /// Check if the vertex has been created successfully
    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vertex() {
        let vertex = BRepLibMakeVertex::new(1.0, 2.0, 3.0);
        assert!(vertex.is_done());
        assert_eq!(vertex.vertex(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_vertex_coordinates() {
        let vertex = BRepLibMakeVertex::new(5.5, 10.2, -3.1);
        assert_eq!(vertex.x(), 5.5);
        assert_eq!(vertex.y(), 10.2);
        assert_eq!(vertex.z(), -3.1);
    }

    #[test]
    fn test_vertex_zero() {
        let vertex = BRepLibMakeVertex::new(0.0, 0.0, 0.0);
        assert!(vertex.is_done());
        assert_eq!(vertex.vertex(), [0.0; 3]);
    }

    #[test]
    fn test_vertex_negative() {
        let vertex = BRepLibMakeVertex::new(-1.5, -2.5, -3.5);
        assert_eq!(vertex.x(), -1.5);
        assert_eq!(vertex.y(), -2.5);
        assert_eq!(vertex.z(), -3.5);
    }
}
