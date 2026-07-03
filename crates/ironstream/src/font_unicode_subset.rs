// FILE: font_unicode_subset.rs
// occt: Font_UnicodeSubset

/// Enumeration defining Unicode subsets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FontUnicodeSubset {
    /// western letters
    Western = 0,
    /// modern Korean letters
    Korean = 1,
    /// Chinese characters (Chinese, Japanese, Korean and Vietnam)
    CJK = 2,
    /// Arabic characters
    Arabic = 3,
}

/// Number of Unicode subsets
pub const FONT_UNICODE_SUBSET_NB: u8 = 3; // Font_UnicodeSubset_Arabic

impl FontUnicodeSubset {
    /// Convert to u8 value
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Create from u8 value
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(FontUnicodeSubset::Western),
            1 => Some(FontUnicodeSubset::Korean),
            2 => Some(FontUnicodeSubset::CJK),
            3 => Some(FontUnicodeSubset::Arabic),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicode_subset_values() {
        assert_eq!(FontUnicodeSubset::Western.as_u8(), 0);
        assert_eq!(FontUnicodeSubset::Korean.as_u8(), 1);
        assert_eq!(FontUnicodeSubset::CJK.as_u8(), 2);
        assert_eq!(FontUnicodeSubset::Arabic.as_u8(), 3);
    }

    #[test]
    fn test_unicode_subset_from_u8() {
        assert_eq!(FontUnicodeSubset::from_u8(0), Some(FontUnicodeSubset::Western));
        assert_eq!(FontUnicodeSubset::from_u8(1), Some(FontUnicodeSubset::Korean));
        assert_eq!(FontUnicodeSubset::from_u8(2), Some(FontUnicodeSubset::CJK));
        assert_eq!(FontUnicodeSubset::from_u8(3), Some(FontUnicodeSubset::Arabic));
        assert_eq!(FontUnicodeSubset::from_u8(4), None);
    }

    #[test]
    fn test_unicode_subset_ordering() {
        assert!(FontUnicodeSubset::Western < FontUnicodeSubset::Korean);
        assert!(FontUnicodeSubset::Korean < FontUnicodeSubset::CJK);
        assert!(FontUnicodeSubset::CJK < FontUnicodeSubset::Arabic);
    }

    #[test]
    fn test_unicode_subset_constant() {
        assert_eq!(FONT_UNICODE_SUBSET_NB, 3);
    }

    #[test]
    fn test_unicode_subset_equality() {
        let s1 = FontUnicodeSubset::CJK;
        let s2 = FontUnicodeSubset::CJK;
        assert_eq!(s1, s2);
    }
}
