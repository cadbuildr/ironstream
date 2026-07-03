// FILE: graphic3d_array_flags.rs
// occt: Graphic3d_ArrayFlags

/// Bitmask for primitive array creation.
pub type Graphic3dArrayFlags = i32;

// Graphic3d_ArrayFlags bitmask values.

/// no flags
pub const GRAPHIC3D_ARRAY_FLAGS_NONE: Graphic3dArrayFlags = 0x00;

/// per-vertex normal attribute
pub const GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL: Graphic3dArrayFlags = 0x01;

/// per-vertex color attribute
pub const GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR: Graphic3dArrayFlags = 0x02;

/// per-vertex texel coordinates (UV) attribute
pub const GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL: Graphic3dArrayFlags = 0x04;

/// bound color
pub const GRAPHIC3D_ARRAY_FLAGS_BOUND_COLOR: Graphic3dArrayFlags = 0x10;

/// mutable array, which can be invalidated during lifetime without re-creation
pub const GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE: Graphic3dArrayFlags = 0x20;

/// non-interleaved vertex attributes packed into single array
pub const GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_DEINTERLEAVED: Graphic3dArrayFlags = 0x40;

/// mutable index array, which can be invalidated during lifetime without re-creation
pub const GRAPHIC3D_ARRAY_FLAGS_INDEXES_MUTABLE: Graphic3dArrayFlags = 0x80;

impl Graphic3dArrayFlags {
    /// Check if flags have the VERTEX_NORMAL bit set
    pub fn has_vertex_normal(&self) -> bool {
        (self & GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL) != 0
    }

    /// Check if flags have the VERTEX_COLOR bit set
    pub fn has_vertex_color(&self) -> bool {
        (self & GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR) != 0
    }

    /// Check if flags have the VERTEX_TEXEL bit set
    pub fn has_vertex_texel(&self) -> bool {
        (self & GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL) != 0
    }

    /// Check if flags have the BOUND_COLOR bit set
    pub fn has_bound_color(&self) -> bool {
        (self & GRAPHIC3D_ARRAY_FLAGS_BOUND_COLOR) != 0
    }

    /// Check if flags have the ATTRIBS_MUTABLE bit set
    pub fn has_attribs_mutable(&self) -> bool {
        (self & GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE) != 0
    }

    /// Check if flags have the ATTRIBS_DEINTERLEAVED bit set
    pub fn has_attribs_deinterleaved(&self) -> bool {
        (self & GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_DEINTERLEAVED) != 0
    }

    /// Check if flags have the INDEXES_MUTABLE bit set
    pub fn has_indexes_mutable(&self) -> bool {
        (self & GRAPHIC3D_ARRAY_FLAGS_INDEXES_MUTABLE) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_flags_constants() {
        assert_eq!(GRAPHIC3D_ARRAY_FLAGS_NONE, 0x00);
        assert_eq!(GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL, 0x01);
        assert_eq!(GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR, 0x02);
        assert_eq!(GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL, 0x04);
        assert_eq!(GRAPHIC3D_ARRAY_FLAGS_BOUND_COLOR, 0x10);
        assert_eq!(GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE, 0x20);
        assert_eq!(GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_DEINTERLEAVED, 0x40);
        assert_eq!(GRAPHIC3D_ARRAY_FLAGS_INDEXES_MUTABLE, 0x80);
    }

    #[test]
    fn test_array_flags_has_vertex_normal() {
        let flags: Graphic3dArrayFlags = GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL;
        assert!(flags.has_vertex_normal());
        assert!(!flags.has_vertex_color());
    }

    #[test]
    fn test_array_flags_multiple_flags() {
        let flags: Graphic3dArrayFlags =
            GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL | GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR;
        assert!(flags.has_vertex_normal());
        assert!(flags.has_vertex_color());
        assert!(!flags.has_vertex_texel());
    }

    #[test]
    fn test_array_flags_none() {
        let flags: Graphic3dArrayFlags = GRAPHIC3D_ARRAY_FLAGS_NONE;
        assert!(!flags.has_vertex_normal());
        assert!(!flags.has_vertex_color());
        assert!(!flags.has_vertex_texel());
    }

    #[test]
    fn test_array_flags_bitwise_operations() {
        let mut flags: Graphic3dArrayFlags = GRAPHIC3D_ARRAY_FLAGS_NONE;
        flags |= GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL;
        assert!(flags.has_vertex_normal());

        flags |= GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE;
        assert!(flags.has_vertex_normal());
        assert!(flags.has_attribs_mutable());
    }

    #[test]
    fn test_array_flags_all_attributes() {
        let flags: Graphic3dArrayFlags = GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL
            | GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR
            | GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL
            | GRAPHIC3D_ARRAY_FLAGS_BOUND_COLOR;

        assert!(flags.has_vertex_normal());
        assert!(flags.has_vertex_color());
        assert!(flags.has_vertex_texel());
        assert!(flags.has_bound_color());
    }
}
