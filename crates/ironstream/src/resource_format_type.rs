// FILE: resource_format_type.rs
// occt: Resource_FormatType

/// List of non ASCII format types which may be converted into the Unicode 16 bits format type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceFormatType {
    /// SJIS (Shift Japanese Industrial Standards) encoding
    Sjis = 0,
    /// EUC (Extended Unix Code) multi-byte encoding
    Euc = 1,
    /// Format type indicating non-conversion behavior
    NoConversion = 2,
    /// GB (Guobiao) encoding for Simplified Chinese
    Gb = 3,
    /// Multi-byte UTF-8 encoding
    Utf8 = 4,
    /// Active system-defined locale
    SystemLocale = 5,
    /// CP1250 (Central European) encoding
    Cp1250 = 6,
    /// CP1251 (Cyrillic) encoding
    Cp1251 = 7,
    /// CP1252 (Western European) encoding
    Cp1252 = 8,
    /// CP1253 (Greek) encoding
    Cp1253 = 9,
    /// CP1254 (Turkish) encoding
    Cp1254 = 10,
    /// CP1255 (Hebrew) encoding
    Cp1255 = 11,
    /// CP1256 (Arabic) encoding
    Cp1256 = 12,
    /// CP1257 (Baltic) encoding
    Cp1257 = 13,
    /// CP1258 (Vietnamese) encoding
    Cp1258 = 14,
    /// ISO 8859-1 (Western European) encoding
    Iso88591 = 15,
    /// ISO 8859-2 (Central European) encoding
    Iso88592 = 16,
    /// ISO 8859-3 (Turkish) encoding
    Iso88593 = 17,
    /// ISO 8859-4 (Northern European) encoding
    Iso88594 = 18,
    /// ISO 8859-5 (Cyrillic) encoding
    Iso88595 = 19,
    /// ISO 8859-6 (Arabic) encoding
    Iso88596 = 20,
    /// ISO 8859-7 (Greek) encoding
    Iso88597 = 21,
    /// ISO 8859-8 (Hebrew) encoding
    Iso88598 = 22,
    /// ISO 8859-9 (Turkish) encoding
    Iso88599 = 23,
    /// ISO 850 (Western European) encoding
    Cp850 = 24,
    /// GBK (Unified Chinese) encoding
    Gbk = 25,
    /// Big5 (Traditional Chinese) encoding
    Big5 = 26,
}

impl ResourceFormatType {
    /// Creates ResourceFormatType from integer value
    pub fn from_int(val: i32) -> Option<Self> {
        match val {
            0 => Some(ResourceFormatType::Sjis),
            1 => Some(ResourceFormatType::Euc),
            2 => Some(ResourceFormatType::NoConversion),
            3 => Some(ResourceFormatType::Gb),
            4 => Some(ResourceFormatType::Utf8),
            5 => Some(ResourceFormatType::SystemLocale),
            6 => Some(ResourceFormatType::Cp1250),
            7 => Some(ResourceFormatType::Cp1251),
            8 => Some(ResourceFormatType::Cp1252),
            9 => Some(ResourceFormatType::Cp1253),
            10 => Some(ResourceFormatType::Cp1254),
            11 => Some(ResourceFormatType::Cp1255),
            12 => Some(ResourceFormatType::Cp1256),
            13 => Some(ResourceFormatType::Cp1257),
            14 => Some(ResourceFormatType::Cp1258),
            15 => Some(ResourceFormatType::Iso88591),
            16 => Some(ResourceFormatType::Iso88592),
            17 => Some(ResourceFormatType::Iso88593),
            18 => Some(ResourceFormatType::Iso88594),
            19 => Some(ResourceFormatType::Iso88595),
            20 => Some(ResourceFormatType::Iso88596),
            21 => Some(ResourceFormatType::Iso88597),
            22 => Some(ResourceFormatType::Iso88598),
            23 => Some(ResourceFormatType::Iso88599),
            24 => Some(ResourceFormatType::Cp850),
            25 => Some(ResourceFormatType::Gbk),
            26 => Some(ResourceFormatType::Big5),
            _ => None,
        }
    }

    /// Gets numeric value
    pub fn as_int(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_format_type_values() {
        assert_eq!(ResourceFormatType::Sjis.as_int(), 0);
        assert_eq!(ResourceFormatType::Utf8.as_int(), 4);
        assert_eq!(ResourceFormatType::Big5.as_int(), 26);
    }

    #[test]
    fn test_resource_format_type_from_int() {
        assert_eq!(ResourceFormatType::from_int(0), Some(ResourceFormatType::Sjis));
        assert_eq!(ResourceFormatType::from_int(4), Some(ResourceFormatType::Utf8));
        assert_eq!(ResourceFormatType::from_int(27), None);
    }
}
