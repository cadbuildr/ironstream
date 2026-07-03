// FILE: graphic3d_array_of_triangle_fans.rs
// occt: Graphic3d_ArrayOfTriangleFans

//! Contains triangles fan array definition.
//!
//! Triangle fans are used to represent a set of triangles that share a common vertex.
//! For example, a fan with 7 vertices creates 5 triangles (7 - 2).

#[derive(Debug, Clone)]
pub struct ArrayOfTriangleFans {
    max_vertices: usize,
    max_fans: usize,
    has_vertex_normals: bool,
    has_vertex_colors: bool,
    has_bound_colors: bool,
    has_vertex_texels: bool,
}

impl ArrayOfTriangleFans {
    /// Creates an array of triangle fans.
    ///
    /// # Arguments
    ///
    /// * `max_vertices` - defines the maximum allowed vertex number in the array
    /// * `max_fans` - defines the maximum allowed fan number in the array (default: 0)
    /// * `has_vertex_normals` - when true, vertex normals should be specified
    /// * `has_vertex_colors` - when true, vertex colors should be specified
    /// * `has_bound_colors` - when true, bound colors should be specified
    /// * `has_vertex_texels` - when true, vertex UV coordinates should be specified
    pub fn new(
        max_vertices: usize,
        max_fans: usize,
        has_vertex_normals: bool,
        has_vertex_colors: bool,
        has_bound_colors: bool,
        has_vertex_texels: bool,
    ) -> Self {
        ArrayOfTriangleFans {
            max_vertices,
            max_fans,
            has_vertex_normals,
            has_vertex_colors,
            has_bound_colors,
            has_vertex_texels,
        }
    }

    /// Returns the maximum number of vertices in the array.
    pub fn max_vertices(&self) -> usize {
        self.max_vertices
    }

    /// Returns the maximum number of fans in the array.
    pub fn max_fans(&self) -> usize {
        self.max_fans
    }

    /// Returns true if vertex normals are enabled.
    pub fn has_vertex_normals(&self) -> bool {
        self.has_vertex_normals
    }

    /// Returns true if vertex colors are enabled.
    pub fn has_vertex_colors(&self) -> bool {
        self.has_vertex_colors
    }

    /// Returns true if bound colors are enabled.
    pub fn has_bound_colors(&self) -> bool {
        self.has_bound_colors
    }

    /// Returns true if vertex texels are enabled.
    pub fn has_vertex_texels(&self) -> bool {
        self.has_vertex_texels
    }

    /// Computes the number of triangles that would be drawn.
    /// Formula: num_triangles = vertex_count - 2 * min(1, bound_count)
    pub fn triangle_count(&self, vertex_count: usize, bound_count: usize) -> usize {
        let min_bound = std::cmp::min(1, bound_count);
        vertex_count.saturating_sub(2 * min_bound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_array_of_triangle_fans() {
        let array = ArrayOfTriangleFans::new(7, 1, false, false, false, false);
        assert_eq!(array.max_vertices(), 7);
        assert_eq!(array.max_fans(), 1);
        assert!(!array.has_vertex_normals());
    }

    #[test]
    fn test_create_array_with_attributes() {
        let array = ArrayOfTriangleFans::new(8, 2, true, true, true, true);
        assert_eq!(array.max_vertices(), 8);
        assert_eq!(array.max_fans(), 2);
        assert!(array.has_vertex_normals());
        assert!(array.has_vertex_colors());
        assert!(array.has_bound_colors());
        assert!(array.has_vertex_texels());
    }

    #[test]
    fn test_triangle_count_single_fan() {
        let array = ArrayOfTriangleFans::new(7, 1, false, false, false, false);
        // Single fan: 7 vertices with bound_count = 0 => 7 - 2*0 = 7 triangles
        assert_eq!(array.triangle_count(7, 0), 7);
    }

    #[test]
    fn test_triangle_count_with_bounds() {
        let array = ArrayOfTriangleFans::new(8, 2, false, false, false, false);
        // With bound_count = 1: 8 - 2*1 = 6 triangles
        assert_eq!(array.triangle_count(8, 1), 6);
    }

    #[test]
    fn test_triangle_count_with_multiple_bounds() {
        let array = ArrayOfTriangleFans::new(8, 2, false, false, false, false);
        // With bound_count = 2 (but min(1,2)=1): 8 - 2*1 = 6 triangles
        assert_eq!(array.triangle_count(8, 2), 6);
    }
}
