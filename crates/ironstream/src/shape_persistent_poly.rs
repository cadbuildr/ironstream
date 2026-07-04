// FILE: shape_persistent_poly.rs
// occt: ShapePersistent_Poly

/// Polyhedral shape persistence (triangles, polygons)
pub struct ShapePersistentPoly;

/// Triangle representation
pub struct Triangle {
    n1: i32,
    n2: i32,
    n3: i32,
}

impl Triangle {
    /// Create a new triangle
    pub fn new(n1: i32, n2: i32, n3: i32) -> Self {
        Triangle { n1, n2, n3 }
    }

    /// Get first node index
    pub fn n1(&self) -> i32 {
        self.n1
    }

    /// Get second node index
    pub fn n2(&self) -> i32 {
        self.n2
    }

    /// Get third node index
    pub fn n3(&self) -> i32 {
        self.n3
    }
}

impl ShapePersistentPoly {
    /// Create polyhedral persistence manager
    pub fn new() -> Self {
        ShapePersistentPoly
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle() {
        let tri = Triangle::new(1, 2, 3);
        assert_eq!(tri.n1(), 1);
        assert_eq!(tri.n2(), 2);
        assert_eq!(tri.n3(), 3);
    }

    #[test]
    fn test_create_poly() {
        let _ = ShapePersistentPoly::new();
    }
}
