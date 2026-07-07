// FILE: poly_h_array1_of_triangle.rs
// occt: Poly_HArray1OfTriangle

//! Deprecated: Poly_HArray1OfTriangle is a handle wrapper for array of triangles.

/// Triangle representation
#[derive(Debug, Clone)]
pub struct Triangle {
    vertices: [u32; 3],
}

impl Triangle {
    pub fn new(v1: u32, v2: u32, v3: u32) -> Self {
        Self {
            vertices: [v1, v2, v3],
        }
    }

    pub fn vertex(&self, i: usize) -> Option<u32> {
        if i < 3 {
            Some(self.vertices[i])
        } else {
            None
        }
    }
}

/// Handle array of triangles
#[derive(Debug, Clone)]
pub struct HArray1 {
    triangles: Vec<Triangle>,
}

impl HArray1 {
    pub fn new(size: usize) -> Self {
        Self {
            triangles: vec![Triangle::new(0, 0, 0); size],
        }
    }

    pub fn len(&self) -> usize {
        self.triangles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.triangles.is_empty()
    }

    pub fn value(&self, index: usize) -> Option<&Triangle> {
        self.triangles.get(index)
    }

    pub fn change_value(&mut self, index: usize) -> Option<&mut Triangle> {
        self.triangles.get_mut(index)
    }
}

pub type PolyHArray1OfTriangle = HArray1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_create() {
        let tri = Triangle::new(1, 2, 3);
        assert_eq!(tri.vertex(0), Some(1));
        assert_eq!(tri.vertex(1), Some(2));
        assert_eq!(tri.vertex(2), Some(3));
    }

    #[test]
    fn test_h_array1_create() {
        let arr = HArray1::new(10);
        assert_eq!(arr.len(), 10);
    }

    #[test]
    fn test_value() {
        let arr = HArray1::new(3);
        assert!(arr.value(0).is_some());
        assert!(arr.value(3).is_none());
    }
}
