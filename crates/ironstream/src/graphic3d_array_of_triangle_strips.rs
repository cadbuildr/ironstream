// FILE: graphic3d_array_of_triangle_strips.rs
// occt: Graphic3d_ArrayOfTriangleStrips

//! Contains triangle strips array definition.
//!
//! Triangle strips are used to represent a set of triangles connected in a strip topology.
//! For example, a strip with 7 vertices creates 5 triangles (7 - 2).

#[derive(Debug, Clone)]
pub struct ArrayOfTriangleStrips {
    max_vertices: usize,
    max_strips: usize,
    has_vertex_normals: bool,
    has_vertex_colors: bool,
    has_bound_colors: bool,
    has_vertex_texels: bool,
}

impl ArrayOfTriangleStrips {
    /// Creates an array of triangle strips.
    ///
    /// # Arguments
    ///
    /// * `max_vertices` - defines the maximum allowed vertex number in the array
    /// * `max_strips` - defines the maximum allowed strip number in the array (default: 0)
    /// * `has_vertex_normals` - when true, AddVertex(Point,Normal), AddVertex(Point,Normal,Color)
    ///   or AddVertex(Point,Normal,Texel) should be used to specify vertex normal
    /// * `has_vertex_colors` - when true, AddVertex(Point,Color) or AddVertex(Point,Normal,Color)
    ///   should be used to specify vertex color
    /// * `has_bound_colors` - when true, AddBound(number,Color) should be used to specify sub-group color
    /// * `has_vertex_texels` - when true, AddVertex(Point,Texel) or AddVertex(Point,Normal,Texel)
    ///   should be used to specify vertex UV coordinates
    pub fn new(
        max_vertices: usize,
        max_strips: usize,
        has_vertex_normals: bool,
        has_vertex_colors: bool,
        has_bound_colors: bool,
        has_vertex_texels: bool,
    ) -> Self {
        ArrayOfTriangleStrips {
            max_vertices,
            max_strips,
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

    /// Returns the maximum number of strips in the array.
    pub fn max_strips(&self) -> usize {
        self.max_strips
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
    fn test_create_array_of_triangle_strips() {
        let array = ArrayOfTriangleStrips::new(7, 1, false, false, false, false);
        assert_eq!(array.max_vertices(), 7);
        assert_eq!(array.max_strips(), 1);
        assert!(!array.has_vertex_normals());
    }

    #[test]
    fn test_create_array_with_attributes() {
        let array = ArrayOfTriangleStrips::new(8, 2, true, true, true, true);
        assert_eq!(array.max_vertices(), 8);
        assert_eq!(array.max_strips(), 2);
        assert!(array.has_vertex_normals());
        assert!(array.has_vertex_colors());
        assert!(array.has_bound_colors());
        assert!(array.has_vertex_texels());
    }

    #[test]
    fn test_triangle_count_single_strip() {
        let array = ArrayOfTriangleStrips::new(7, 1, false, false, false, false);
        // Single strip: 7 vertices with bound_count = 0 => 7 - 2*0 = 7 triangles
        assert_eq!(array.triangle_count(7, 0), 7);
    }

    #[test]
    fn test_triangle_count_with_bounds() {
        let array = ArrayOfTriangleStrips::new(8, 2, false, false, false, false);
        // With bound_count = 1: 8 - 2*1 = 6 triangles
        assert_eq!(array.triangle_count(8, 1), 6);
    }

    #[test]
    fn test_triangle_count_with_multiple_bounds() {
        let array = ArrayOfTriangleStrips::new(8, 2, false, false, false, false);
        // With bound_count = 2 (but min(1,2)=1): 8 - 2*1 = 6 triangles
        assert_eq!(array.triangle_count(8, 2), 6);
    }
}
