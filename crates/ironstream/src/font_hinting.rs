// FILE: font_hinting.rs
// occt: Font_Hinting

/// Enumeration defining font hinting options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontHinting {
    /// no hinting (FT_LOAD_NO_HINTING)
    Off = 0x00,
    /// default hinting (FT_LOAD_TARGET_NORMAL)
    Normal = 0x01,
    /// light hinting (FT_LOAD_TARGET_LIGHT)
    Light = 0x02,
    /// prefer autohinting over native hinting (FT_LOAD_FORCE_AUTOHINT)
    ForceAutohint = 0x10,
    /// disallow autohinting (FT_LOAD_NO_AUTOHINT)
    NoAutohint = 0x20,
}

impl FontHinting {
    /// Convert to u8 value
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Create from u8 value
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0x00 => Some(FontHinting::Off),
            0x01 => Some(FontHinting::Normal),
            0x02 => Some(FontHinting::Light),
            0x10 => Some(FontHinting::ForceAutohint),
            0x20 => Some(FontHinting::NoAutohint),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hinting_values() {
        assert_eq!(FontHinting::Off.as_u8(), 0x00);
        assert_eq!(FontHinting::Normal.as_u8(), 0x01);
        assert_eq!(FontHinting::Light.as_u8(), 0x02);
        assert_eq!(FontHinting::ForceAutohint.as_u8(), 0x10);
        assert_eq!(FontHinting::NoAutohint.as_u8(), 0x20);
    }

    #[test]
    fn test_hinting_from_u8() {
        assert_eq!(FontHinting::from_u8(0x00), Some(FontHinting::Off));
        assert_eq!(FontHinting::from_u8(0x01), Some(FontHinting::Normal));
        assert_eq!(FontHinting::from_u8(0x02), Some(FontHinting::Light));
        assert_eq!(FontHinting::from_u8(0x10), Some(FontHinting::ForceAutohint));
        assert_eq!(FontHinting::from_u8(0x20), Some(FontHinting::NoAutohint));
        assert_eq!(FontHinting::from_u8(0xFF), None);
    }

    #[test]
    fn test_hinting_equality() {
        let h1 = FontHinting::Normal;
        let h2 = FontHinting::Normal;
        assert_eq!(h1, h2);
    }
}
