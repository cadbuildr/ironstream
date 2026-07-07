// FILE: int_patch_polyhedron.rs
// occt: IntPatch_Polyhedron

//! Polyhedron for surface-surface intersection patch detection.

/// Polyhedron for surface intersection
pub struct IntPatchPolyhedron {
    vertices: Vec<(f64, f64, f64)>,
    triangles: Vec<(usize, usize, usize)>,
    closed: bool,
}

impl IntPatchPolyhedron {
    /// Creates an empty polyhedron
    pub fn new() -> Self {
        IntPatchPolyhedron {
            vertices: Vec::new(),
            triangles: Vec::new(),
            closed: false,
        }
    }

    /// Creates polyhedron from surface
    pub fn from_surface(_surface: &Surface) -> Self {
        IntPatchPolyhedron {
            vertices: Vec::new(),
            triangles: Vec::new(),
            closed: false,
        }
    }

    /// Returns number of vertices
    pub fn nb_vertices(&self) -> i32 {
        self.vertices.len() as i32
    }

    /// Returns number of triangles
    pub fn nb_triangles(&self) -> i32 {
        self.triangles.len() as i32
    }

    /// Returns vertex at index
    pub fn vertex(&self, index: i32) -> Option<(f64, f64, f64)> {
        self.vertices.get(index as usize).copied()
    }

    /// Adds a vertex
    pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) {
        self.vertices.push((x, y, z));
    }

    /// Adds a triangle
    pub fn add_triangle(&mut self, i1: usize, i2: usize, i3: usize) {
        self.triangles.push((i1, i2, i3));
    }

    /// Sets closed flag
    pub fn set_closed(&mut self, closed: bool) {
        self.closed = closed;
    }

    /// Returns closed flag
    pub fn is_closed(&self) -> bool {
        self.closed
    }
}

impl Default for IntPatchPolyhedron {
    fn default() -> Self {
        Self::new()
    }
}

/// Placeholder for surface
#[derive(Clone)]
pub struct Surface;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polyhedron_new() {
        let poly = IntPatchPolyhedron::new();
        assert_eq!(poly.nb_vertices(), 0);
        assert_eq!(poly.nb_triangles(), 0);
        assert!(!poly.is_closed());
    }

    #[test]
    fn test_polyhedron_add_vertex() {
        let mut poly = IntPatchPolyhedron::new();
        poly.add_vertex(0.0, 0.0, 0.0);
        poly.add_vertex(1.0, 0.0, 0.0);
        assert_eq!(poly.nb_vertices(), 2);
    }

    #[test]
    fn test_polyhedron_add_triangle() {
        let mut poly = IntPatchPolyhedron::new();
        poly.add_vertex(0.0, 0.0, 0.0);
        poly.add_vertex(1.0, 0.0, 0.0);
        poly.add_vertex(0.0, 1.0, 0.0);
        poly.add_triangle(0, 1, 2);
        assert_eq!(poly.nb_triangles(), 1);
    }
}
