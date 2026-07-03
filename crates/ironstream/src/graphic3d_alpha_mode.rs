// FILE: graphic3d_alpha_mode.rs
// occt: Graphic3d_AlphaMode

/// Defines how alpha value of base color / texture should be treated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Graphic3dAlphaMode {
    /// rendered output is fully opaque and alpha value is ignored
    Opaque = 0,
    /// rendered output is either fully opaque or fully transparent
    /// depending on the alpha value and the alpha cutoff value
    Mask = 1,
    /// rendered output is combined with the background
    Blend = 2,
    /// performs in-place blending (without implicit reordering of opaque objects)
    /// with alpha-test
    MaskBlend = 3,
}

/// Special value for backward compatibility
/// Equal to Graphic3d_AlphaMode_Blend when Material transparency is not zero
/// and Graphic3d_AlphaMode_Opaque otherwise.
/// Since this check ignores possible transparency defined by per-vertex colors
/// and textures - NOT recommended to use!
pub const GRAPHIC3D_ALPHA_MODE_BLEND_AUTO: i32 = -1;

impl Graphic3dAlphaMode {
    /// Convert to i32 value
    pub fn as_i32(self) -> i32 {
        self as i32
    }

    /// Create from i32 value
    pub fn from_i32(val: i32) -> Option<Self> {
        match val {
            0 => Some(Graphic3dAlphaMode::Opaque),
            1 => Some(Graphic3dAlphaMode::Mask),
            2 => Some(Graphic3dAlphaMode::Blend),
            3 => Some(Graphic3dAlphaMode::MaskBlend),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alpha_mode_values() {
        assert_eq!(Graphic3dAlphaMode::Opaque.as_i32(), 0);
        assert_eq!(Graphic3dAlphaMode::Mask.as_i32(), 1);
        assert_eq!(Graphic3dAlphaMode::Blend.as_i32(), 2);
        assert_eq!(Graphic3dAlphaMode::MaskBlend.as_i32(), 3);
    }

    #[test]
    fn test_alpha_mode_from_i32() {
        assert_eq!(Graphic3dAlphaMode::from_i32(0), Some(Graphic3dAlphaMode::Opaque));
        assert_eq!(Graphic3dAlphaMode::from_i32(1), Some(Graphic3dAlphaMode::Mask));
        assert_eq!(Graphic3dAlphaMode::from_i32(2), Some(Graphic3dAlphaMode::Blend));
        assert_eq!(Graphic3dAlphaMode::from_i32(3), Some(Graphic3dAlphaMode::MaskBlend));
        assert_eq!(Graphic3dAlphaMode::from_i32(4), None);
    }

    #[test]
    fn test_alpha_mode_blend_auto_constant() {
        assert_eq!(GRAPHIC3D_ALPHA_MODE_BLEND_AUTO, -1);
    }

    #[test]
    fn test_alpha_mode_equality() {
        let m1 = Graphic3dAlphaMode::Blend;
        let m2 = Graphic3dAlphaMode::Blend;
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_alpha_mode_copy() {
        let m = Graphic3dAlphaMode::MaskBlend;
        let m_copy = m;
        assert_eq!(m, m_copy);
    }
}
