// FILE: i_vtk_vtk_shape_data.rs
// occt: IVtkVTK_ShapeData

/// VTK shape data structure.
#[derive(Clone, Debug)]
pub struct IVtkVTK_ShapeData {
    vertices: Vec<[f64; 3]>,
    triangles: Vec<[u32; 3]>,
}

impl IVtkVTK_ShapeData {
    /// Create a new shape data structure.
    pub fn new() -> Self {
        IVtkVTK_ShapeData {
            vertices: Vec::new(),
            triangles: Vec::new(),
        }
    }

    /// Add a vertex.
    pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) -> u32 {
        self.vertices.push([x, y, z]);
        (self.vertices.len() - 1) as u32
    }

    /// Add a triangle.
    pub fn add_triangle(&mut self, v0: u32, v1: u32, v2: u32) {
        self.triangles.push([v0, v1, v2]);
    }

    /// Get the number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Get the number of triangles.
    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    /// Get a vertex by index.
    pub fn vertex(&self, index: u32) -> Option<[f64; 3]> {
        self.vertices.get(index as usize).copied()
    }

    /// Get a triangle by index.
    pub fn triangle(&self, index: u32) -> Option<[u32; 3]> {
        self.triangles.get(index as usize).copied()
    }
}

impl Default for IVtkVTK_ShapeData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_shape_data() {
        let data = IVtkVTK_ShapeData::new();
        assert_eq!(data.vertex_count(), 0);
        assert_eq!(data.triangle_count(), 0);
    }

    #[test]
    fn test_add_vertices() {
        let mut data = IVtkVTK_ShapeData::new();
        let v0 = data.add_vertex(0.0, 0.0, 0.0);
        let v1 = data.add_vertex(1.0, 0.0, 0.0);
        let v2 = data.add_vertex(0.0, 1.0, 0.0);
        assert_eq!(data.vertex_count(), 3);
        assert_eq!(v0, 0);
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }

    #[test]
    fn test_add_triangle() {
        let mut data = IVtkVTK_ShapeData::new();
        data.add_vertex(0.0, 0.0, 0.0);
        data.add_vertex(1.0, 0.0, 0.0);
        data.add_vertex(0.0, 1.0, 0.0);
        data.add_triangle(0, 1, 2);
        assert_eq!(data.triangle_count(), 1);
    }

    #[test]
    fn test_get_vertex() {
        let mut data = IVtkVTK_ShapeData::new();
        data.add_vertex(1.5, 2.5, 3.5);
        assert_eq!(data.vertex(0), Some([1.5, 2.5, 3.5]));
    }

    #[test]
    fn test_get_triangle() {
        let mut data = IVtkVTK_ShapeData::new();
        data.add_vertex(0.0, 0.0, 0.0);
        data.add_vertex(1.0, 0.0, 0.0);
        data.add_vertex(0.0, 1.0, 0.0);
        data.add_triangle(0, 1, 2);
        assert_eq!(data.triangle(0), Some([0, 1, 2]));
    }
}
