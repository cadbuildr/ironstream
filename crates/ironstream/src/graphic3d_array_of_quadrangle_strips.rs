// FILE: graphic3d_array_of_quadrangle_strips.rs
// occt: Graphic3d_ArrayOfQuadrangleStrips

// Local model of the Graphic3d_ArrayFlags bitmask (self-contained).
pub type Graphic3dArrayFlags = i32;
pub const GRAPHIC3D_ARRAY_FLAGS_NONE: Graphic3dArrayFlags = 0x00;
pub const GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL: Graphic3dArrayFlags = 0x01;
pub const GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR: Graphic3dArrayFlags = 0x02;
pub const GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL: Graphic3dArrayFlags = 0x04;
pub const GRAPHIC3D_ARRAY_FLAGS_BOUND_COLOR: Graphic3dArrayFlags = 0x10;

/// Contains quadrangles strip array definition.
/// WARNING! Quadrangle primitives might be unsupported by graphics library.
/// Triangulation should be used instead of quads for better compatibility.
#[derive(Debug, Clone)]
pub struct Graphic3dArrayOfQuadrangleStrips {
    max_vertexes: i32,
    max_strips: i32,
    array_flags: Graphic3dArrayFlags,
}

impl Graphic3dArrayOfQuadrangleStrips {
    /// Creates an array of quadrangle strips (Graphic3d_TOPA_QUADRANGLESTRIPS).
    ///
    /// # Arguments
    ///
    /// * `max_vertexes` - defines the maximum allowed vertex number in the array
    /// * `max_strips` - defines the maximum allowed strip number in the array
    /// * `array_flags` - array flags
    pub fn with_flags(
        max_vertexes: i32,
        max_strips: i32,
        array_flags: Graphic3dArrayFlags,
    ) -> Self {
        Graphic3dArrayOfQuadrangleStrips {
            max_vertexes,
            max_strips,
            array_flags,
        }
    }

    /// Creates an array of quadrangle strips (Graphic3d_TOPA_QUADRANGLESTRIPS).
    ///
    /// # Arguments
    ///
    /// * `max_vertexes` - defines the maximum allowed vertex number in the array
    /// * `max_strips` - defines the maximum allowed strip number in the array
    /// * `has_v_normals` - if true, per-vertex normal attribute is enabled
    /// * `has_v_colors` - if true, per-vertex color attribute is enabled
    /// * `has_s_colors` - if true, per-strip (bound) color attribute is enabled
    /// * `has_v_texels` - if true, per-vertex texel (UV) coordinates are enabled
    pub fn new(
        max_vertexes: i32,
        max_strips: i32,
        has_v_normals: bool,
        has_v_colors: bool,
        has_s_colors: bool,
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
        if has_s_colors {
            flags |= GRAPHIC3D_ARRAY_FLAGS_BOUND_COLOR;
        }

        Graphic3dArrayOfQuadrangleStrips {
            max_vertexes,
            max_strips,
            array_flags: flags,
        }
    }

    /// Creates an array of quadrangle strips with minimal parameters.
    ///
    /// # Arguments
    ///
    /// * `max_vertexes` - defines the maximum allowed vertex number in the array
    pub fn simple(max_vertexes: i32) -> Self {
        Graphic3dArrayOfQuadrangleStrips {
            max_vertexes,
            max_strips: 0,
            array_flags: GRAPHIC3D_ARRAY_FLAGS_NONE,
        }
    }

    /// Returns the maximum number of vertexes in this array
    pub fn max_vertexes(&self) -> i32 {
        self.max_vertexes
    }

    /// Returns the maximum number of strips in this array
    pub fn max_strips(&self) -> i32 {
        self.max_strips
    }

    /// Returns the array flags
    pub fn array_flags(&self) -> Graphic3dArrayFlags {
        self.array_flags
    }

    /// Returns the number of quadrangles in the array.
    /// The number is calculated as: VertexNumber()/2 - min(1, BoundNumber())
    pub fn quadrangle_count(&self, vertex_count: i32, bound_count: i32) -> i32 {
        (vertex_count / 2) - std::cmp::min(1, bound_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_flags() {
        let array = Graphic3dArrayOfQuadrangleStrips::with_flags(100, 10, 0x03);
        assert_eq!(array.max_vertexes(), 100);
        assert_eq!(array.max_strips(), 10);
        assert_eq!(array.array_flags(), 0x03);
    }

    #[test]
    fn test_new_with_attributes() {
        let array = Graphic3dArrayOfQuadrangleStrips::new(100, 10, true, true, false, true);
        assert_eq!(array.max_vertexes(), 100);
        assert_eq!(array.max_strips(), 10);

        // Check that the flags contain vertex normal, vertex color, and vertex texel
        let flags = array.array_flags();
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL != 0);
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR != 0);
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL != 0);
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_BOUND_COLOR == 0);
    }

    #[test]
    fn test_new_simple() {
        let array = Graphic3dArrayOfQuadrangleStrips::simple(50);
        assert_eq!(array.max_vertexes(), 50);
        assert_eq!(array.max_strips(), 0);
        assert_eq!(array.array_flags(), GRAPHIC3D_ARRAY_FLAGS_NONE);
    }

    #[test]
    fn test_quadrangle_count() {
        let array = Graphic3dArrayOfQuadrangleStrips::simple(100);

        // With 8 vertices and 0 bounds: 8/2 - 0 = 4 quads
        assert_eq!(array.quadrangle_count(8, 0), 4);

        // With 8 vertices and 1 bound: 8/2 - 1 = 3 quads
        assert_eq!(array.quadrangle_count(8, 1), 3);

        // With 10 vertices and 2 bounds: 10/2 - 1 = 4 quads
        assert_eq!(array.quadrangle_count(10, 2), 4);
    }

    #[test]
    fn test_all_flags_enabled() {
        let array = Graphic3dArrayOfQuadrangleStrips::new(100, 10, true, true, true, true);
        let flags = array.array_flags();

        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL != 0);
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR != 0);
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL != 0);
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_BOUND_COLOR != 0);
    }

    #[test]
    fn test_no_flags_enabled() {
        let array = Graphic3dArrayOfQuadrangleStrips::new(100, 10, false, false, false, false);
        assert_eq!(array.array_flags(), GRAPHIC3D_ARRAY_FLAGS_NONE);
    }

    #[test]
    fn test_partial_flags() {
        let array = Graphic3dArrayOfQuadrangleStrips::new(100, 10, true, false, true, false);
        let flags = array.array_flags();

        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL != 0);
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR == 0);
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL == 0);
        assert!(flags & GRAPHIC3D_ARRAY_FLAGS_BOUND_COLOR != 0);
    }
}
