// FILE: graphic3d_array_of_segments.rs
// occt: Graphic3d_ArrayOfSegments

use crate::graphic3d_array_flags::{
    Graphic3dArrayFlags, GRAPHIC3D_ARRAY_FLAGS_NONE, GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR,
};

/// Contains segments array definition.
#[derive(Debug, Clone)]
pub struct Graphic3dArrayOfSegments {
    max_vertexes: i32,
    max_edges: i32,
    array_flags: Graphic3dArrayFlags,
}

impl Graphic3dArrayOfSegments {
    /// Creates an array of segments (Graphic3d_TOPA_SEGMENTS).
    ///
    /// # Arguments
    ///
    /// * `max_vertexes` - defines the maximum allowed vertex number in the array
    /// * `max_edges` - defines the maximum allowed edge number in the array
    /// * `array_flags` - array flags
    pub fn with_flags(
        max_vertexes: i32,
        max_edges: i32,
        array_flags: Graphic3dArrayFlags,
    ) -> Self {
        Graphic3dArrayOfSegments {
            max_vertexes,
            max_edges,
            array_flags,
        }
    }

    /// Creates an array of segments (Graphic3d_TOPA_SEGMENTS).
    ///
    /// # Arguments
    ///
    /// * `max_vertexes` - defines the maximum allowed vertex number in the array
    /// * `max_edges` - defines the maximum allowed edge number in the array
    /// * `has_v_colors` - when TRUE, AddVertex(Point, Color) should be used for specifying
    ///   vertex color
    pub fn new(max_vertexes: i32, max_edges: i32, has_v_colors: bool) -> Self {
        let flags = if has_v_colors {
            GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR
        } else {
            GRAPHIC3D_ARRAY_FLAGS_NONE
        };

        Graphic3dArrayOfSegments {
            max_vertexes,
            max_edges,
            array_flags: flags,
        }
    }

    /// Creates an array of segments with minimal parameters.
    ///
    /// # Arguments
    ///
    /// * `max_vertexes` - defines the maximum allowed vertex number in the array
    pub fn simple(max_vertexes: i32) -> Self {
        Graphic3dArrayOfSegments {
            max_vertexes,
            max_edges: 0,
            array_flags: GRAPHIC3D_ARRAY_FLAGS_NONE,
        }
    }

    /// Returns the maximum number of vertexes in this array
    pub fn max_vertexes(&self) -> i32 {
        self.max_vertexes
    }

    /// Returns the maximum number of edges in this array
    pub fn max_edges(&self) -> i32 {
        self.max_edges
    }

    /// Returns the array flags
    pub fn array_flags(&self) -> Graphic3dArrayFlags {
        self.array_flags
    }

    /// Returns whether vertex colors are enabled
    pub fn has_vertex_colors(&self) -> bool {
        (self.array_flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR) != 0
    }

    /// Returns the number of segments that can be drawn.
    /// For non-indexed arrays: vertex_count / 2
    /// For indexed arrays: edge_count / 2
    pub fn segment_count(&self, vertex_count: i32, edge_count: i32) -> i32 {
        if self.max_edges > 0 {
            // Indexed array: edges define segments, each edge is one segment endpoint pair
            edge_count / 2
        } else {
            // Non-indexed array: pairs of consecutive vertices form segments
            vertex_count / 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_flags() {
        let array = Graphic3dArrayOfSegments::with_flags(100, 50, 0x02);
        assert_eq!(array.max_vertexes(), 100);
        assert_eq!(array.max_edges(), 50);
        assert_eq!(array.array_flags(), 0x02);
    }

    #[test]
    fn test_new_with_colors() {
        let array = Graphic3dArrayOfSegments::new(100, 50, true);
        assert_eq!(array.max_vertexes(), 100);
        assert_eq!(array.max_edges(), 50);
        assert!(array.has_vertex_colors());
    }

    #[test]
    fn test_new_without_colors() {
        let array = Graphic3dArrayOfSegments::new(100, 50, false);
        assert_eq!(array.max_vertexes(), 100);
        assert_eq!(array.max_edges(), 50);
        assert!(!array.has_vertex_colors());
    }

    #[test]
    fn test_new_simple() {
        let array = Graphic3dArrayOfSegments::simple(50);
        assert_eq!(array.max_vertexes(), 50);
        assert_eq!(array.max_edges(), 0);
        assert_eq!(array.array_flags(), GRAPHIC3D_ARRAY_FLAGS_NONE);
        assert!(!array.has_vertex_colors());
    }

    #[test]
    fn test_segment_count_non_indexed() {
        let array = Graphic3dArrayOfSegments::simple(100);

        // Non-indexed array: vertex_count / 2
        // 4 vertices -> 2 segments
        assert_eq!(array.segment_count(4, 0), 2);
        // 6 vertices -> 3 segments
        assert_eq!(array.segment_count(6, 0), 3);
        // 10 vertices -> 5 segments
        assert_eq!(array.segment_count(10, 0), 5);
    }

    #[test]
    fn test_segment_count_indexed() {
        let array = Graphic3dArrayOfSegments::with_flags(100, 50, GRAPHIC3D_ARRAY_FLAGS_NONE);

        // Indexed array: edge_count / 2
        // 4 edges -> 2 segments
        assert_eq!(array.segment_count(0, 4), 2);
        // 6 edges -> 3 segments
        assert_eq!(array.segment_count(0, 6), 3);
        // 10 edges -> 5 segments
        assert_eq!(array.segment_count(0, 10), 5);
    }

    #[test]
    fn test_has_vertex_colors() {
        let with_colors = Graphic3dArrayOfSegments::new(100, 50, true);
        let without_colors = Graphic3dArrayOfSegments::new(100, 50, false);

        assert!(with_colors.has_vertex_colors());
        assert!(!without_colors.has_vertex_colors());
    }

    #[test]
    fn test_flags_value() {
        let with_colors = Graphic3dArrayOfSegments::new(100, 50, true);
        assert_eq!(
            with_colors.array_flags(),
            GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR
        );

        let without_colors = Graphic3dArrayOfSegments::new(100, 50, false);
        assert_eq!(without_colors.array_flags(), GRAPHIC3D_ARRAY_FLAGS_NONE);
    }

    #[test]
    fn test_indexed_vs_non_indexed_logic() {
        let non_indexed = Graphic3dArrayOfSegments::simple(100);
        let indexed = Graphic3dArrayOfSegments::with_flags(100, 50, GRAPHIC3D_ARRAY_FLAGS_NONE);

        // Non-indexed uses vertex count
        assert_eq!(non_indexed.segment_count(4, 100), 2);
        // Indexed uses edge count
        assert_eq!(indexed.segment_count(100, 4), 2);
    }

    #[test]
    fn test_example_from_occt_comment() {
        // Create a set of indexed segments
        let array = Graphic3dArrayOfSegments::with_flags(4, 8, GRAPHIC3D_ARRAY_FLAGS_NONE);

        // The example adds 4 vertices and then 4 edges (8 edge endpoints / 2 = 4 segments)
        assert_eq!(array.max_vertexes(), 4);
        assert_eq!(array.max_edges(), 8);

        // With 8 edge values (4 pairs): 8 / 2 = 4 segments
        assert_eq!(array.segment_count(4, 8), 4);
    }
}
