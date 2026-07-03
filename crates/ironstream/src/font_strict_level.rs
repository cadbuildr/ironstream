// FILE: font_strict_level.rs
// occt: Font_StrictLevel

/// Enumeration defining font search restrictions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FontStrictLevel {
    /// search only for exact font
    Strict = 0,
    /// search for exact font match and for aliases (ignore global fallback)
    Aliases = 1,
    /// search for any font, including global fallback
    Any = 2,
}

impl FontStrictLevel {
    /// Convert to u8 value
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Create from u8 value
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(FontStrictLevel::Strict),
            1 => Some(FontStrictLevel::Aliases),
            2 => Some(FontStrictLevel::Any),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strict_level_values() {
        assert_eq!(FontStrictLevel::Strict.as_u8(), 0);
        assert_eq!(FontStrictLevel::Aliases.as_u8(), 1);
        assert_eq!(FontStrictLevel::Any.as_u8(), 2);
    }

    #[test]
    fn test_strict_level_from_u8() {
        assert_eq!(FontStrictLevel::from_u8(0), Some(FontStrictLevel::Strict));
        assert_eq!(FontStrictLevel::from_u8(1), Some(FontStrictLevel::Aliases));
        assert_eq!(FontStrictLevel::from_u8(2), Some(FontStrictLevel::Any));
        assert_eq!(FontStrictLevel::from_u8(3), None);
    }

    #[test]
    fn test_strict_level_ordering() {
        assert!(FontStrictLevel::Strict < FontStrictLevel::Aliases);
        assert!(FontStrictLevel::Aliases < FontStrictLevel::Any);
    }

    #[test]
    fn test_strict_level_equality() {
        let l1 = FontStrictLevel::Any;
        let l2 = FontStrictLevel::Any;
        assert_eq!(l1, l2);
    }
}
