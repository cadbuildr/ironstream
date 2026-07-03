// FILE: n_collection_utf_string.rs
// occt: NCollection_UtfString

/// UTF-8 string container.
#[derive(Clone, Debug)]
pub struct UtfString {
    data: String,
}

impl UtfString {
    /// Create empty UTF string.
    pub fn new() -> Self {
        UtfString {
            data: String::new(),
        }
    }

    /// Create from str.
    pub fn from_str(s: &str) -> Self {
        UtfString {
            data: s.to_string(),
        }
    }

    /// Create from bytes (UTF-8).
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::string::FromUtf8Error> {
        Ok(UtfString {
            data: String::from_utf8(bytes.to_vec())?,
        })
    }

    /// Get as str.
    pub fn as_str(&self) -> &str {
        &self.data
    }

    /// Get length in characters.
    pub fn len_chars(&self) -> usize {
        self.data.chars().count()
    }

    /// Get length in bytes.
    pub fn len_bytes(&self) -> usize {
        self.data.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Append UTF string.
    pub fn append(&mut self, s: &str) {
        self.data.push_str(s);
    }
}

impl Default for UtfString {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_utf_string() {
        let s = UtfString::new();
        assert!(s.is_empty());
    }

    #[test]
    fn test_from_str() {
        let s = UtfString::from_str("hello");
        assert_eq!(s.as_str(), "hello");
        assert_eq!(s.len_bytes(), 5);
    }

    #[test]
    fn test_utf_chars() {
        let s = UtfString::from_str("hello world");
        assert_eq!(s.len_chars(), 11);
    }

    #[test]
    fn test_append() {
        let mut s = UtfString::from_str("hello");
        s.append(" world");
        assert_eq!(s.as_str(), "hello world");
    }
}
