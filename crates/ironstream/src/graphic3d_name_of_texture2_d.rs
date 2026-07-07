// FILE: graphic3d_name_of_texture2_d.rs
// occt: Graphic3d_NameOfTexture2D

/// Types of standard 2D textures.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameOfTexture2d {
    Matra = 0,
    AlienSkin = 1,
    BlueRock = 2,
    BluewhitePaper = 3,
    Brushed = 4,
    Bubbles = 5,
    Bump = 6,
    Cast = 7,
    Chipbd = 8,
    Clouds = 9,
    Flesh = 10,
    Floor = 11,
    Galvanised = 12,
    Grass = 13,
    Aluminum = 14,
    Rock = 15,
    Knurl = 16,
    Maple = 17,
    Marble = 18,
    Mottled = 19,
    Rain = 20,
    Chess = 21,
    Unknown = 22,
}

impl NameOfTexture2d {
    /// Returns the numeric value of the texture type.
    pub fn as_u32(self) -> u32 {
        self as u32
    }

    /// Converts from u32 to NameOfTexture2d, returning Unknown for unknown values.
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => NameOfTexture2d::Matra,
            1 => NameOfTexture2d::AlienSkin,
            2 => NameOfTexture2d::BlueRock,
            3 => NameOfTexture2d::BluewhitePaper,
            4 => NameOfTexture2d::Brushed,
            5 => NameOfTexture2d::Bubbles,
            6 => NameOfTexture2d::Bump,
            7 => NameOfTexture2d::Cast,
            8 => NameOfTexture2d::Chipbd,
            9 => NameOfTexture2d::Clouds,
            10 => NameOfTexture2d::Flesh,
            11 => NameOfTexture2d::Floor,
            12 => NameOfTexture2d::Galvanised,
            13 => NameOfTexture2d::Grass,
            14 => NameOfTexture2d::Aluminum,
            15 => NameOfTexture2d::Rock,
            16 => NameOfTexture2d::Knurl,
            17 => NameOfTexture2d::Maple,
            18 => NameOfTexture2d::Marble,
            19 => NameOfTexture2d::Mottled,
            20 => NameOfTexture2d::Rain,
            21 => NameOfTexture2d::Chess,
            _ => NameOfTexture2d::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_enum_values() {
        assert_eq!(NameOfTexture2d::Matra.as_u32(), 0);
        assert_eq!(NameOfTexture2d::AlienSkin.as_u32(), 1);
        assert_eq!(NameOfTexture2d::Chess.as_u32(), 21);
        assert_eq!(NameOfTexture2d::Unknown.as_u32(), 22);
    }

    #[test]
    fn test_from_u32_conversion() {
        assert_eq!(NameOfTexture2d::from_u32(0), NameOfTexture2d::Matra);
        assert_eq!(NameOfTexture2d::from_u32(9), NameOfTexture2d::Clouds);
        assert_eq!(NameOfTexture2d::from_u32(22), NameOfTexture2d::Unknown);
        assert_eq!(NameOfTexture2d::from_u32(99), NameOfTexture2d::Unknown);
    }

    #[test]
    fn test_roundtrip_conversion() {
        let original = NameOfTexture2d::Marble;
        let value = original.as_u32();
        let converted = NameOfTexture2d::from_u32(value);
        assert_eq!(original, converted);
    }
}
