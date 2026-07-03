// FILE: graphic3d_shader_flags.rs
// occt: Graphic3d_ShaderFlags

/// Standard GLSL program combination bits (flags).
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Graphic3dShaderFlags(u32);

impl Graphic3dShaderFlags {
    /// Per-vertex color
    pub const VERT_COLOR: u32 = 0x0001;
    /// Handle RGB texturing
    pub const TEXTURE_RGB: u32 = 0x0002;
    /// Handle environment map (obsolete)
    pub const TEXTURE_ENV: u32 = 0x0004;
    /// Extended texture set (with normal map)
    pub const TEXTURE_NORMAL: u32 = Self::TEXTURE_RGB | Self::TEXTURE_ENV;
    /// Point marker without sprite
    pub const POINT_SIMPLE: u32 = 0x0008;
    /// Point sprite with RGB image
    pub const POINT_SPRITE: u32 = 0x0010;
    /// Point sprite with Alpha image
    pub const POINT_SPRITE_A: u32 = Self::POINT_SIMPLE | Self::POINT_SPRITE;
    /// Stipple line
    pub const STIPPLE_LINE: u32 = 0x0020;
    /// Handle 1 clipping plane
    pub const CLIP_PLANES_1: u32 = 0x0040;
    /// Handle 2 clipping planes
    pub const CLIP_PLANES_2: u32 = 0x0080;
    /// Handle N clipping planes
    pub const CLIP_PLANES_N: u32 = Self::CLIP_PLANES_1 | Self::CLIP_PLANES_2;
    /// Handle chains of clipping planes
    pub const CLIP_CHAINS: u32 = 0x0100;
    /// Draw mesh edges (wireframe)
    pub const MESH_EDGES: u32 = 0x0200;
    /// Discard fragment by alpha test
    pub const ALPHA_TEST: u32 = 0x0400;
    /// Write coverage buffer for Blended OIT
    pub const WRITE_OIT: u32 = 0x0800;
    /// Handle Depth Peeling OIT
    pub const OIT_DEPTH_PEELING: u32 = 0x1000;
    /// Apply per-vertex color only to front-facing fragments
    pub const VERT_COLOR_FRONT_ONLY: u32 = 0x2000;
    /// Overall number of combinations
    pub const NB: u32 = 0x4000;
    /// Combined flags for point-related features
    pub const IS_POINT: u32 =
        Self::POINT_SIMPLE | Self::POINT_SPRITE | Self::POINT_SPRITE_A;
    /// Combined flags for texture features
    pub const HAS_TEXTURES: u32 = Self::TEXTURE_RGB | Self::TEXTURE_ENV;
    /// Needs geometry shader
    pub const NEEDS_GEOM_SHADER: u32 = Self::MESH_EDGES;

    /// Creates shader flags from a raw value.
    pub fn new(value: u32) -> Self {
        Graphic3dShaderFlags(value)
    }

    /// Returns the raw flag value.
    pub fn value(self) -> u32 {
        self.0
    }

    /// Checks if a specific flag is set.
    pub fn has_flag(self, flag: u32) -> bool {
        (self.0 & flag) != 0
    }

    /// Sets a specific flag.
    pub fn set_flag(&mut self, flag: u32) {
        self.0 |= flag;
    }

    /// Clears a specific flag.
    pub fn clear_flag(&mut self, flag: u32) {
        self.0 &= !flag;
    }

    /// Creates flags with vertex color enabled.
    pub fn with_vert_color() -> Self {
        Graphic3dShaderFlags(Self::VERT_COLOR)
    }

    /// Creates flags with texture RGB enabled.
    pub fn with_texture_rgb() -> Self {
        Graphic3dShaderFlags(Self::TEXTURE_RGB)
    }

    /// Creates flags with point sprite enabled.
    pub fn with_point_sprite() -> Self {
        Graphic3dShaderFlags(Self::POINT_SPRITE)
    }
}

impl Default for Graphic3dShaderFlags {
    fn default() -> Self {
        Graphic3dShaderFlags(0)
    }
}

impl std::ops::BitOr for Graphic3dShaderFlags {
    type Output = Graphic3dShaderFlags;

    fn bitor(self, other: Graphic3dShaderFlags) -> Graphic3dShaderFlags {
        Graphic3dShaderFlags(self.0 | other.0)
    }
}

impl std::ops::BitAnd for Graphic3dShaderFlags {
    type Output = Graphic3dShaderFlags;

    fn bitand(self, other: Graphic3dShaderFlags) -> Graphic3dShaderFlags {
        Graphic3dShaderFlags(self.0 & other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_flags_constants() {
        assert_eq!(Graphic3dShaderFlags::VERT_COLOR, 0x0001);
        assert_eq!(Graphic3dShaderFlags::TEXTURE_RGB, 0x0002);
        assert_eq!(Graphic3dShaderFlags::POINT_SPRITE, 0x0010);
    }

    #[test]
    fn test_shader_flags_creation() {
        let flags = Graphic3dShaderFlags::new(0x0001);
        assert_eq!(flags.value(), 0x0001);
    }

    #[test]
    fn test_shader_flags_has_flag() {
        let flags = Graphic3dShaderFlags::new(0x0003);
        assert!(flags.has_flag(0x0001));
        assert!(flags.has_flag(0x0002));
        assert!(!flags.has_flag(0x0004));
    }

    #[test]
    fn test_shader_flags_set_flag() {
        let mut flags = Graphic3dShaderFlags::new(0x0001);
        flags.set_flag(0x0002);
        assert_eq!(flags.value(), 0x0003);
    }

    #[test]
    fn test_shader_flags_clear_flag() {
        let mut flags = Graphic3dShaderFlags::new(0x0003);
        flags.clear_flag(0x0001);
        assert_eq!(flags.value(), 0x0002);
    }

    #[test]
    fn test_shader_flags_with_vert_color() {
        let flags = Graphic3dShaderFlags::with_vert_color();
        assert!(flags.has_flag(Graphic3dShaderFlags::VERT_COLOR));
    }

    #[test]
    fn test_shader_flags_with_texture_rgb() {
        let flags = Graphic3dShaderFlags::with_texture_rgb();
        assert!(flags.has_flag(Graphic3dShaderFlags::TEXTURE_RGB));
    }

    #[test]
    fn test_shader_flags_bitwise_or() {
        let flags1 = Graphic3dShaderFlags::new(0x0001);
        let flags2 = Graphic3dShaderFlags::new(0x0002);
        let combined = flags1 | flags2;
        assert_eq!(combined.value(), 0x0003);
    }

    #[test]
    fn test_shader_flags_bitwise_and() {
        let flags1 = Graphic3dShaderFlags::new(0x0003);
        let flags2 = Graphic3dShaderFlags::new(0x0001);
        let result = flags1 & flags2;
        assert_eq!(result.value(), 0x0001);
    }

    #[test]
    fn test_shader_flags_default() {
        let flags = Graphic3dShaderFlags::default();
        assert_eq!(flags.value(), 0);
    }
}
