// FILE: n_collection_string.rs
// occt: NCollection_String

/// String container for NCollection.
#[derive(Clone, Debug)]
pub struct NString {
    data: String,
}

impl NString {
    /// Create empty string.
    pub fn new() -> Self {
        NString {
            data: String::new(),
        }
    }

    /// Create string from str.
    pub fn from_str(s: &str) -> Self {
        NString {
            data: s.to_string(),
        }
    }

    /// Get length.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get as str slice.
    pub fn as_str(&self) -> &str {
        &self.data
    }

    /// Clear string.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Append to string.
    pub fn append(&mut self, s: &str) {
        self.data.push_str(s);
    }

    /// Get character at index.
    pub fn char_at(&self, idx: usize) -> Option<char> {
        self.data.chars().nth(idx)
    }
}

impl Default for NString {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for NString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_string() {
        let s = NString::new();
        assert!(s.is_empty());
    }

    #[test]
    fn test_from_str() {
        let s = NString::from_str("hello");
        assert_eq!(s.as_str(), "hello");
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn test_append() {
        let mut s = NString::from_str("hello");
        s.append(" world");
        assert_eq!(s.as_str(), "hello world");
    }

    #[test]
    fn test_clear() {
        let mut s = NString::from_str("text");
        s.clear();
        assert!(s.is_empty());
    }
}
