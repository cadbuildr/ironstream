// FILE: graphic3d_array_of_quadrangles.rs
// occt: Graphic3d_ArrayOfQuadrangles

// Local model of the Graphic3d_ArrayFlags bitmask (self-contained).
pub type Graphic3dArrayFlags = i32;
pub const GRAPHIC3D_ARRAY_FLAGS_NONE: Graphic3dArrayFlags = 0x00;
pub const GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL: Graphic3dArrayFlags = 0x01;
pub const GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR: Graphic3dArrayFlags = 0x02;
pub const GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL: Graphic3dArrayFlags = 0x04;
pub const GRAPHIC3D_ARRAY_FLAGS_BOUND_COLOR: Graphic3dArrayFlags = 0x10;

/// Contains quadrangles array definition.
/// WARNING! Quadrangle primitives might be unsupported by graphics library.
/// Triangulation should be used instead of quads for better compatibility.
#[derive(Debug, Clone)]
pub struct Graphic3dArrayOfQuadrangles {
    max_vertexes: i32,
    max_edges: i32,
    array_flags: Graphic3dArrayFlags,
}

impl Graphic3dArrayOfQuadrangles {
    /// Creates an array of quadrangles (Graphic3d_TOPA_QUADRANGLES).
    ///
    /// # Arguments
    ///
    /// * `max_vertexes` - defines the maximum allowed vertex number in the array
    /// * `max_edges` - defines the maximum allowed edge number in the array (for indexed array)
    /// * `array_flags` - array flags
    pub fn with_flags(
        max_vertexes: i32,
        max_edges: i32,
        array_flags: Graphic3dArrayFlags,
    ) -> Self {
        Graphic3dArrayOfQuadrangles {
            max_vertexes,
            max_edges,
            array_flags,
        }
    }

    /// Creates an array of quadrangles (Graphic3d_TOPA_QUADRANGLES).
    ///
    /// # Arguments
    ///
    /// * `max_vertexes` - defines the maximum allowed vertex number in the array
    /// * `max_edges` - defines the maximum allowed edge number in the array (for indexed array)
    /// * `has_v_normals` - if true, per-vertex normal attribute is enabled
    /// * `has_v_colors` - if true, per-vertex color attribute is enabled
    /// * `has_v_texels` - if true, per-vertex texel (UV) coordinates are enabled
    pub fn new(
        max_vertexes: i32,
        max_edges: i32,
        has_v_normals: bool,
        has_v_colors: bool,
        has_v_texels: bool,
    ) -> Self {
        let mut flags = GRAPHIC3D_ARRAY_FLAGS_NONE;
        if has_v_normals {
            flags |= GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL;
        }
        if has_v_colors {
            flags |= GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR;
        }
        if has_v_texels {
            flags |= GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL;
        }

        Graphic3dArrayOfQuadrangles {
            max_vertexes,
            max_edges,
            array_flags: flags,
        }
    }

    /// Creates an array of quadrangles with minimal parameters.
    ///
    /// # Arguments
    ///
    /// * `max_vertexes` - defines the maximum allowed vertex number in the array
    pub fn simple(max_vertexes: i32) -> Self {
        Graphic3dArrayOfQuadrangles {
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

    /// Returns the number of quadrangles that can be drawn.
    /// For non-indexed arrays: vertex_count / 4
    /// For indexed arrays: edge_count / 4
    pub fn quadrangle_count(&self, vertex_count: i32, edge_count: i32) -> i32 {
        if self.max_edges > 0 {
            // Indexed array
            edge_count / 4
        } else {
            // Non-indexed array
            vertex_count / 4
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_flags() {
        let array = Graphic3dArrayOfQuadrangles::with_flags(100, 50, 0x03);
        assert_eq!(array.max_vertexes(), 100);
        assert_eq!(array.max_edges(), 50);
        assert_eq!(array.array_flags(), 0x03);
    }

    #[test]
    fn test_new_with_attributes() {
        let array = Graphic3dArrayOfQuadrangles::new(100, 50, true, true, true);
        assert_eq!(array.max_vertexes(), 100);
        assert_eq!(array.max_edges(), 50);

        // Check that the flags contain all attributes
        let flags = array.array_flags();
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL != 0);
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR != 0);
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL != 0);
    }

    #[test]
    fn test_new_simple() {
        let array = Graphic3dArrayOfQuadrangles::simple(50);
        assert_eq!(array.max_vertexes(), 50);
        assert_eq!(array.max_edges(), 0);
        assert_eq!(array.array_flags(), GRAPHIC3D_ARRAY_FLAGS_NONE);
    }

    #[test]
    fn test_quadrangle_count_non_indexed() {
        let array = Graphic3dArrayOfQuadrangles::simple(100);

        // Non-indexed array: vertex_count / 4
        assert_eq!(array.quadrangle_count(8, 0), 2);
        assert_eq!(array.quadrangle_count(12, 0), 3);
        assert_eq!(array.quadrangle_count(16, 0), 4);
    }

    #[test]
    fn test_quadrangle_count_indexed() {
        let array = Graphic3dArrayOfQuadrangles::with_flags(100, 50, GRAPHIC3D_ARRAY_FLAGS_NONE);

        // Indexed array: edge_count / 4
        assert_eq!(array.quadrangle_count(0, 8), 2);
        assert_eq!(array.quadrangle_count(0, 12), 3);
        assert_eq!(array.quadrangle_count(0, 16), 4);
    }

    #[test]
    fn test_all_flags_enabled() {
        let array = Graphic3dArrayOfQuadrangles::new(100, 50, true, true, true);
        let flags = array.array_flags();

        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL != 0);
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR != 0);
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL != 0);
    }

    #[test]
    fn test_no_flags_enabled() {
        let array = Graphic3dArrayOfQuadrangles::new(100, 50, false, false, false);
        assert_eq!(array.array_flags(), GRAPHIC3D_ARRAY_FLAGS_NONE);
    }

    #[test]
    fn test_partial_flags() {
        let array = Graphic3dArrayOfQuadrangles::new(100, 50, true, false, true);
        let flags = array.array_flags();

        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL != 0);
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR == 0);
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL != 0);
    }

    #[test]
    fn test_indexed_vs_non_indexed_logic() {
        let non_indexed = Graphic3dArrayOfQuadrangles::simple(100);
        let indexed = Graphic3dArrayOfQuadrangles::with_flags(100, 50, GRAPHIC3D_ARRAY_FLAGS_NONE);

        // Non-indexed uses vertex count
        assert_eq!(non_indexed.quadrangle_count(8, 100), 2);
        // Indexed uses edge count
        assert_eq!(indexed.quadrangle_count(100, 8), 2);
    }
}
