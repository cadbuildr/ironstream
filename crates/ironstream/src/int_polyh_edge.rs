// FILE: int_polyh_edge.rs
// occt: IntPolyh_Edge

/// Represents an edge in a polyhedron, connecting two points and referenced by two triangles.
#[derive(Clone, Debug, Copy)]
pub struct Edge {
    point1: i32,
    point2: i32,
    triangle1: i32,
    triangle2: i32,
}

impl Edge {
    /// Create a default edge
    pub fn new() -> Self {
        Edge {
            point1: -1,
            point2: -1,
            triangle1: -1,
            triangle2: -1,
        }
    }

    /// Create an edge with points and triangles
    pub fn with_values(p1: i32, p2: i32, t1: i32, t2: i32) -> Self {
        Edge {
            point1: p1,
            point2: p2,
            triangle1: t1,
            triangle2: t2,
        }
    }

    /// Get first point
    pub fn first_point(&self) -> i32 {
        self.point1
    }

    /// Get second point
    pub fn second_point(&self) -> i32 {
        self.point2
    }

    /// Get first triangle
    pub fn first_triangle(&self) -> i32 {
        self.triangle1
    }

    /// Get second triangle
    pub fn second_triangle(&self) -> i32 {
        self.triangle2
    }

    /// Set first point
    pub fn set_first_point(&mut self, p: i32) {
        self.point1 = p;
    }

    /// Set second point
    pub fn set_second_point(&mut self, p: i32) {
        self.point2 = p;
    }

    /// Set first triangle
    pub fn set_first_triangle(&mut self, t: i32) {
        self.triangle1 = t;
    }

    /// Set second triangle
    pub fn set_second_triangle(&mut self, t: i32) {
        self.triangle2 = t;
    }
}

impl Default for Edge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let edge = Edge::new();
        assert_eq!(edge.first_point(), -1);
        assert_eq!(edge.second_point(), -1);
        assert_eq!(edge.first_triangle(), -1);
        assert_eq!(edge.second_triangle(), -1);
    }

    #[test]
    fn test_with_values() {
        let edge = Edge::with_values(0, 1, 5, 10);
        assert_eq!(edge.first_point(), 0);
        assert_eq!(edge.second_point(), 1);
        assert_eq!(edge.first_triangle(), 5);
        assert_eq!(edge.second_triangle(), 10);
    }

    #[test]
    fn test_setters() {
        let mut edge = Edge::new();
        edge.set_first_point(3);
        edge.set_second_point(4);
        edge.set_first_triangle(7);
        edge.set_second_triangle(8);

        assert_eq!(edge.first_point(), 3);
        assert_eq!(edge.second_point(), 4);
        assert_eq!(edge.first_triangle(), 7);
        assert_eq!(edge.second_triangle(), 8);
    }
}
