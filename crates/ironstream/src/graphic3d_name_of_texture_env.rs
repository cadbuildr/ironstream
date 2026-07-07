// FILE: graphic3d_name_of_texture_env.rs
// occt: Graphic3d_NameOfTextureEnv

/// Types of standard environment textures.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameOfTextureEnv {
    Clouds = 0,
    Cv = 1,
    Medit = 2,
    Pearl = 3,
    Sky1 = 4,
    Sky2 = 5,
    Lines = 6,
    Road = 7,
    Unknown = 8,
}

impl NameOfTextureEnv {
    /// Returns the numeric value of the texture type.
    pub fn as_u32(self) -> u32 {
        self as u32
    }

    /// Converts from u32 to NameOfTextureEnv, returning Unknown for unknown values.
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => NameOfTextureEnv::Clouds,
            1 => NameOfTextureEnv::Cv,
            2 => NameOfTextureEnv::Medit,
            3 => NameOfTextureEnv::Pearl,
            4 => NameOfTextureEnv::Sky1,
            5 => NameOfTextureEnv::Sky2,
            6 => NameOfTextureEnv::Lines,
            7 => NameOfTextureEnv::Road,
            _ => NameOfTextureEnv::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_env_enum_values() {
        assert_eq!(NameOfTextureEnv::Clouds.as_u32(), 0);
        assert_eq!(NameOfTextureEnv::Cv.as_u32(), 1);
        assert_eq!(NameOfTextureEnv::Sky1.as_u32(), 4);
        assert_eq!(NameOfTextureEnv::Road.as_u32(), 7);
        assert_eq!(NameOfTextureEnv::Unknown.as_u32(), 8);
    }

    #[test]
    fn test_from_u32_conversion() {
        assert_eq!(NameOfTextureEnv::from_u32(0), NameOfTextureEnv::Clouds);
        assert_eq!(NameOfTextureEnv::from_u32(3), NameOfTextureEnv::Pearl);
        assert_eq!(NameOfTextureEnv::from_u32(8), NameOfTextureEnv::Unknown);
        assert_eq!(NameOfTextureEnv::from_u32(99), NameOfTextureEnv::Unknown);
    }

    #[test]
    fn test_roundtrip_conversion() {
        let original = NameOfTextureEnv::Sky2;
        let value = original.as_u32();
        let converted = NameOfTextureEnv::from_u32(value);
        assert_eq!(original, converted);
    }
}
