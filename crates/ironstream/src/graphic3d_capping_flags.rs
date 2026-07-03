// FILE: graphic3d_capping_flags.rs
// occt: Graphic3d_CappingFlags

/// Enumeration of capping flags.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Graphic3dCappingFlags(pub u32);

impl Graphic3dCappingFlags {
    /// No flags
    pub const NONE: Graphic3dCappingFlags = Graphic3dCappingFlags(0x0000);
    /// Use object material
    pub const OBJECT_MATERIAL: Graphic3dCappingFlags = Graphic3dCappingFlags(0x0001);
    /// Use object texture
    pub const OBJECT_TEXTURE: Graphic3dCappingFlags = Graphic3dCappingFlags(0x0002);
    /// Use object GLSL program
    pub const OBJECT_SHADER: Graphic3dCappingFlags = Graphic3dCappingFlags(0x0008);
    /// Use entire fill area aspect from object
    pub const OBJECT_ASPECT: Graphic3dCappingFlags = Graphic3dCappingFlags(
        Self::OBJECT_MATERIAL.0 | Self::OBJECT_TEXTURE.0 | Self::OBJECT_SHADER.0
    );

    /// Check if a flag is set
    pub fn contains(&self, other: Graphic3dCappingFlags) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Set a flag
    pub fn set(&mut self, other: Graphic3dCappingFlags) {
        self.0 |= other.0;
    }

    /// Unset a flag
    pub fn unset(&mut self, other: Graphic3dCappingFlags) {
        self.0 &= !other.0;
    }

    /// Return raw value
    pub fn raw(&self) -> u32 {
        self.0
    }
}

impl Default for Graphic3dCappingFlags {
    fn default() -> Self {
        Self::NONE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none_flag() {
        let flags = Graphic3dCappingFlags::NONE;
        assert_eq!(flags.raw(), 0x0000);
    }

    #[test]
    fn test_object_material_flag() {
        let flags = Graphic3dCappingFlags::OBJECT_MATERIAL;
        assert_eq!(flags.raw(), 0x0001);
    }

    #[test]
    fn test_object_texture_flag() {
        let flags = Graphic3dCappingFlags::OBJECT_TEXTURE;
        assert_eq!(flags.raw(), 0x0002);
    }

    #[test]
    fn test_object_shader_flag() {
        let flags = Graphic3dCappingFlags::OBJECT_SHADER;
        assert_eq!(flags.raw(), 0x0008);
    }

    #[test]
    fn test_object_aspect_flag() {
        let flags = Graphic3dCappingFlags::OBJECT_ASPECT;
        let expected = 0x0001 | 0x0002 | 0x0008;
        assert_eq!(flags.raw(), expected);
    }

    #[test]
    fn test_contains_set_flag() {
        let flags = Graphic3dCappingFlags::OBJECT_MATERIAL;
        assert!(flags.contains(Graphic3dCappingFlags::OBJECT_MATERIAL));
    }

    #[test]
    fn test_contains_unset_flag() {
        let flags = Graphic3dCappingFlags::OBJECT_MATERIAL;
        assert!(!flags.contains(Graphic3dCappingFlags::OBJECT_TEXTURE));
    }

    #[test]
    fn test_set_flag() {
        let mut flags = Graphic3dCappingFlags::NONE;
        flags.set(Graphic3dCappingFlags::OBJECT_MATERIAL);
        assert!(flags.contains(Graphic3dCappingFlags::OBJECT_MATERIAL));
        assert_eq!(flags.raw(), 0x0001);
    }

    #[test]
    fn test_unset_flag() {
        let mut flags = Graphic3dCappingFlags::OBJECT_MATERIAL;
        flags.unset(Graphic3dCappingFlags::OBJECT_MATERIAL);
        assert!(!flags.contains(Graphic3dCappingFlags::OBJECT_MATERIAL));
        assert_eq!(flags.raw(), 0x0000);
    }

    #[test]
    fn test_multiple_flags() {
        let mut flags = Graphic3dCappingFlags::NONE;
        flags.set(Graphic3dCappingFlags::OBJECT_MATERIAL);
        flags.set(Graphic3dCappingFlags::OBJECT_TEXTURE);
        assert!(flags.contains(Graphic3dCappingFlags::OBJECT_MATERIAL));
        assert!(flags.contains(Graphic3dCappingFlags::OBJECT_TEXTURE));
        assert!(!flags.contains(Graphic3dCappingFlags::OBJECT_SHADER));
        assert_eq!(flags.raw(), 0x0003);
    }

    #[test]
    fn test_default_is_none() {
        let flags = Graphic3dCappingFlags::default();
        assert_eq!(flags, Graphic3dCappingFlags::NONE);
    }
}
