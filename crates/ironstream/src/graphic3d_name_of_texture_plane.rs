// FILE: graphic3d_name_of_texture_plane.rs
// occt: Graphic3d_NameOfTexturePlane

/// Type of the texture projection plane for both S and T texture coordinates.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameOfTexturePlane {
    /// XY plane projection
    Xy = 0,
    /// YZ plane projection
    Yz = 1,
    /// ZX plane projection
    Zx = 2,
    /// Unknown plane
    Unknown = 3,
}

impl NameOfTexturePlane {
    /// Returns the numeric value of the plane type.
    pub fn as_u32(self) -> u32 {
        self as u32
    }

    /// Converts from u32 to NameOfTexturePlane, returning Unknown for unknown values.
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => NameOfTexturePlane::Xy,
            1 => NameOfTexturePlane::Yz,
            2 => NameOfTexturePlane::Zx,
            _ => NameOfTexturePlane::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_plane_enum_values() {
        assert_eq!(NameOfTexturePlane::Xy.as_u32(), 0);
        assert_eq!(NameOfTexturePlane::Yz.as_u32(), 1);
        assert_eq!(NameOfTexturePlane::Zx.as_u32(), 2);
        assert_eq!(NameOfTexturePlane::Unknown.as_u32(), 3);
    }

    #[test]
    fn test_from_u32_conversion() {
        assert_eq!(NameOfTexturePlane::from_u32(0), NameOfTexturePlane::Xy);
        assert_eq!(NameOfTexturePlane::from_u32(1), NameOfTexturePlane::Yz);
        assert_eq!(NameOfTexturePlane::from_u32(2), NameOfTexturePlane::Zx);
        assert_eq!(NameOfTexturePlane::from_u32(3), NameOfTexturePlane::Unknown);
        assert_eq!(NameOfTexturePlane::from_u32(99), NameOfTexturePlane::Unknown);
    }

    #[test]
    fn test_roundtrip_conversion() {
        let original = NameOfTexturePlane::Zx;
        let value = original.as_u32();
        let converted = NameOfTexturePlane::from_u32(value);
        assert_eq!(original, converted);
    }
}
