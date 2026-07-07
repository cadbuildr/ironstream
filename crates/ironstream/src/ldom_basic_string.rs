// FILE: ldom_basic_string.rs
// occt: LDOMBasicString

/// String type enumeration for LDOM strings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StringType {
    Null = 0,
    Integer = 1,
    AsciiFree = 2,      // String not connected to any container
    AsciiDoc = 3,       // String connected to LDOM_Document (container)
    AsciiDocClear = 4,  // --"--"--, consists of only XML-valid chars
    AsciiHashed = 5,    // String connected to hash table
}

/// Basic string class for LDOM (Lightweight DOM) representation.
/// Can hold integers or ASCII strings with different storage types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LDOMBasicString {
    string_type: StringType,
    value: StringValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StringValue {
    Null,
    Integer(i64),
    String(String),
}

impl LDOMBasicString {
    /// Creates a null string (empty)
    pub fn new() -> Self {
        LDOMBasicString {
            string_type: StringType::Null,
            value: StringValue::Null,
        }
    }

    /// Creates a string from an integer value
    pub fn from_integer(value: i64) -> Self {
        LDOMBasicString {
            string_type: StringType::Integer,
            value: StringValue::Integer(value),
        }
    }

    /// Creates a free ASCII string
    pub fn from_ascii_free(value: String) -> Self {
        LDOMBasicString {
            string_type: StringType::AsciiFree,
            value: StringValue::String(value),
        }
    }

    /// Creates a documented ASCII string
    pub fn from_ascii_doc(value: String) -> Self {
        LDOMBasicString {
            string_type: StringType::AsciiDoc,
            value: StringValue::String(value),
        }
    }

    /// Returns the type of this string
    pub fn string_type(&self) -> StringType {
        self.string_type
    }

    /// Attempts to get the value as an integer
    pub fn get_integer(&self) -> Option<i64> {
        match &self.value {
            StringValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Gets the string value (empty string for non-string types)
    pub fn get_string(&self) -> &str {
        match &self.value {
            StringValue::String(s) => s,
            _ => "",
        }
    }

    /// Checks if this string is null
    pub fn is_null(&self) -> bool {
        self.string_type == StringType::Null
    }

    /// Compares two strings by content
    pub fn equals(&self, other: &LDOMBasicString) -> bool {
        self == other
    }
}

impl Default for LDOMBasicString {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_string() {
        let s = LDOMBasicString::new();
        assert!(s.is_null());
        assert_eq!(s.get_string(), "");
        assert_eq!(s.string_type(), StringType::Null);
    }

    #[test]
    fn test_integer_string() {
        let s = LDOMBasicString::from_integer(42);
        assert_eq!(s.get_integer(), Some(42));
        assert_eq!(s.string_type(), StringType::Integer);
        assert_eq!(s.get_string(), "");
    }

    #[test]
    fn test_ascii_free_string() {
        let s = LDOMBasicString::from_ascii_free("hello".to_string());
        assert_eq!(s.get_string(), "hello");
        assert_eq!(s.string_type(), StringType::AsciiFree);
        assert_eq!(s.get_integer(), None);
    }

    #[test]
    fn test_ascii_doc_string() {
        let s = LDOMBasicString::from_ascii_doc("world".to_string());
        assert_eq!(s.get_string(), "world");
        assert_eq!(s.string_type(), StringType::AsciiDoc);
    }

    #[test]
    fn test_string_equality() {
        let s1 = LDOMBasicString::from_ascii_free("test".to_string());
        let s2 = LDOMBasicString::from_ascii_free("test".to_string());
        let s3 = LDOMBasicString::from_ascii_free("other".to_string());

        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
    }

    #[test]
    fn test_equals_method() {
        let s1 = LDOMBasicString::from_integer(100);
        let s2 = LDOMBasicString::from_integer(100);
        assert!(s1.equals(&s2));
    }

    #[test]
    fn test_default() {
        let s = LDOMBasicString::default();
        assert!(s.is_null());
    }
}
