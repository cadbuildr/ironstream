// FILE: resource_convert_unicode.rs
// occt: Resource_ConvertUnicode

/// Unicode conversion utilities.
pub struct ConvertUnicode;

impl ConvertUnicode {
    pub fn utf8_to_utf16(s: &str) -> Vec<u16> {
        s.encode_utf16(&mut vec![]).into()
    }

    pub fn utf16_to_utf8(v: &[u16]) -> String {
        String::from_utf16_lossy(v).into_owned()
    }
}
