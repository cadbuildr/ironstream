// FILE: graphic3d_level_of_texture_anisotropy.rs
// occt: Graphic3d_LevelOfTextureAnisotropy

/// Level of anisotropy filter.
/// Notice that actual quality depends on hardware capabilities!
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum LevelOfTextureAnisotropy {
    Off = 0,
    Fast = 1,
    Middle = 2,
    Quality = 3,
}

impl Default for LevelOfTextureAnisotropy {
    fn default() -> Self {
        LevelOfTextureAnisotropy::Off
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_of_texture_anisotropy_values() {
        assert_eq!(LevelOfTextureAnisotropy::Off as u32, 0);
        assert_eq!(LevelOfTextureAnisotropy::Fast as u32, 1);
        assert_eq!(LevelOfTextureAnisotropy::Middle as u32, 2);
        assert_eq!(LevelOfTextureAnisotropy::Quality as u32, 3);
    }

    #[test]
    fn test_level_of_texture_anisotropy_default() {
        let level = LevelOfTextureAnisotropy::default();
        assert_eq!(level, LevelOfTextureAnisotropy::Off);
    }

    #[test]
    fn test_level_of_texture_anisotropy_clone() {
        let level = LevelOfTextureAnisotropy::Quality;
        let cloned = level;
        assert_eq!(level, cloned);
    }

    #[test]
    fn test_level_of_texture_anisotropy_debug() {
        let level = LevelOfTextureAnisotropy::Fast;
        let debug_str = format!("{:?}", level);
        assert!(debug_str.contains("Fast"));
    }

    #[test]
    fn test_level_of_texture_anisotropy_ordering() {
        let off = LevelOfTextureAnisotropy::Off as u32;
        let fast = LevelOfTextureAnisotropy::Fast as u32;
        let middle = LevelOfTextureAnisotropy::Middle as u32;
        let quality = LevelOfTextureAnisotropy::Quality as u32;

        assert!(off < fast);
        assert!(fast < middle);
        assert!(middle < quality);
    }
}
