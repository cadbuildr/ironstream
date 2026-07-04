// FILE: ldom_string.rs
// occt: LDOMString

/// LDOMString represents various object types which can be mapped to XML strings.
/// Note: LDOMString is not an independent type. You must ensure that the owner
/// LDOM_Document is never lost during the lifetime of its LDOMStrings.
#[derive(Clone, Default)]
pub struct LDOMString {
    data: String,
    doc_ptr: Option<usize>, // Pointer to LDOM_MemManager (stored as raw pointer value)
}

impl LDOMString {
    /// Empty constructor
    pub fn new() -> Self {
        LDOMString {
            data: String::new(),
            doc_ptr: None,
        }
    }

    /// Create from integer
    pub fn from_int(value: i32) -> Self {
        LDOMString {
            data: value.to_string(),
            doc_ptr: None,
        }
    }

    /// Create LDOM_AsciiFree from C string
    pub fn from_cstr(value: &str) -> Self {
        LDOMString {
            data: value.to_string(),
            doc_ptr: None,
        }
    }

    /// Get the string value
    pub fn as_str(&self) -> &str {
        &self.data
    }

    /// Get the owner document pointer
    pub fn owner_document(&self) -> Option<usize> {
        self.doc_ptr
    }

    /// Set the string to null
    pub fn set_null(&mut self) {
        self.data.clear();
        self.doc_ptr = None;
    }
}

impl PartialEq for LDOMString {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for LDOMString {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let s = LDOMString::new();
        assert_eq!(s.as_str(), "");
        assert_eq!(s.owner_document(), None);
    }

    #[test]
    fn test_from_int() {
        let s = LDOMString::from_int(42);
        assert_eq!(s.as_str(), "42");
    }

    #[test]
    fn test_from_cstr() {
        let s = LDOMString::from_cstr("hello");
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_string_equality() {
        let s1 = LDOMString::from_cstr("test");
        let s2 = LDOMString::from_cstr("test");
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_set_null() {
        let mut s = LDOMString::from_cstr("test");
        s.set_null();
        assert_eq!(s.as_str(), "");
    }
}
