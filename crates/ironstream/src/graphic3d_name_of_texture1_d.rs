// FILE: graphic3d_name_of_texture1_d.rs
// occt: Graphic3d_NameOfTexture1D

/// Standard 1D texture types.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Graphic3dNameOfTexture1d {
    /// Elevation texture (for relief mapping)
    Elevation = 0,
    /// Unknown texture type
    Unknown = 1,
}

impl Graphic3dNameOfTexture1d {
    /// Returns the numeric value of the enum variant.
    pub fn value(self) -> u32 {
        self as u32
    }

    /// Creates an enum variant from a numeric value.
    pub fn from_value(val: u32) -> Option<Self> {
        match val {
            0 => Some(Graphic3dNameOfTexture1d::Elevation),
            1 => Some(Graphic3dNameOfTexture1d::Unknown),
            _ => None,
        }
    }
}

impl Default for Graphic3dNameOfTexture1d {
    fn default() -> Self {
        Graphic3dNameOfTexture1d::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_name_elevation() {
        let tex = Graphic3dNameOfTexture1d::Elevation;
        assert_eq!(tex.value(), 0);
    }

    #[test]
    fn test_texture_name_unknown() {
        let tex = Graphic3dNameOfTexture1d::Unknown;
        assert_eq!(tex.value(), 1);
    }

    #[test]
    fn test_texture_name_from_value() {
        assert_eq!(
            Graphic3dNameOfTexture1d::from_value(0),
            Some(Graphic3dNameOfTexture1d::Elevation)
        );
        assert_eq!(
            Graphic3dNameOfTexture1d::from_value(1),
            Some(Graphic3dNameOfTexture1d::Unknown)
        );
        assert_eq!(Graphic3dNameOfTexture1d::from_value(99), None);
    }

    #[test]
    fn test_texture_name_default() {
        assert_eq!(
            Graphic3dNameOfTexture1d::default(),
            Graphic3dNameOfTexture1d::Unknown
        );
    }

    #[test]
    fn test_texture_name_equality() {
        let t1 = Graphic3dNameOfTexture1d::Elevation;
        let t2 = Graphic3dNameOfTexture1d::Elevation;
        assert_eq!(t1, t2);

        let t3 = Graphic3dNameOfTexture1d::Unknown;
        assert_ne!(t1, t3);
    }
}
