// FILE: t_collection_ascii_string.rs
// occt: TCollection_AsciiString

/// Variable-length sequence of 8-bit characters (UTF-8 compatible).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TCollectionAsciiString {
    data: String,
}

impl TCollectionAsciiString {
    /// Initializes an AsciiString to an empty string.
    pub fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    /// Initializes an AsciiString with a C string.
    pub fn from_str(s: &str) -> Self {
        Self {
            data: s.to_string(),
        }
    }

    /// Initializes an AsciiString with a C string and explicit length.
    pub fn from_slice(s: &str, len: usize) -> Self {
        let truncated = if len < s.len() {
            &s[..len]
        } else {
            s
        };
        Self {
            data: truncated.to_string(),
        }
    }

    /// Initializes an AsciiString with a single character.
    pub fn from_char(c: char) -> Self {
        Self {
            data: c.to_string(),
        }
    }

    /// Initializes an AsciiString with specified length filled with filler character.
    pub fn with_filler(len: usize, filler: char) -> Self {
        Self {
            data: filler.to_string().repeat(len),
        }
    }

    /// Initializes an AsciiString with an integer value.
    pub fn from_int(value: i32) -> Self {
        Self {
            data: value.to_string(),
        }
    }

    /// Initializes an AsciiString with a real value.
    pub fn from_double(value: f64) -> Self {
        Self {
            data: value.to_string(),
        }
    }

    /// Returns the length of the string.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the string is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the character at position (1-indexed).
    pub fn value_at(&self, index: usize) -> Option<char> {
        if index < 1 || index > self.data.len() {
            return None;
        }
        self.data.chars().nth(index - 1)
    }

    /// Sets the character at position (1-indexed).
    pub fn set_value(&mut self, index: usize, c: char) -> bool {
        if index < 1 || index > self.data.len() {
            return false;
        }
        let bytes = unsafe { self.data.as_bytes_mut() };
        if c.is_ascii() {
            bytes[index - 1] = c as u8;
            true
        } else {
            false
        }
    }

    /// Appends a character.
    pub fn append_char(&mut self, c: char) {
        self.data.push(c);
    }

    /// Appends a C string.
    pub fn append_str(&mut self, s: &str) {
        self.data.push_str(s);
    }

    /// Appends another AsciiString.
    pub fn append(&mut self, other: &TCollectionAsciiString) {
        self.data.push_str(&other.data);
    }

    /// Removes characters from position with given length (1-indexed).
    pub fn remove(&mut self, pos: usize, len: usize) -> bool {
        if pos < 1 || pos > self.data.len() {
            return false;
        }
        let start = pos - 1;
        let end = std::cmp::min(start + len, self.data.len());
        self.data.drain(start..end);
        true
    }

    /// Searches for a substring, returns 1-indexed position or 0 if not found.
    pub fn search(&self, substring: &str) -> usize {
        self.data
            .find(substring)
            .map(|pos| pos + 1)
            .unwrap_or(0)
    }

    /// Truncates string to given length.
    pub fn trunc(&mut self, len: usize) {
        self.data.truncate(len);
    }

    /// Converts to lowercase.
    pub fn to_lowercase(&self) -> TCollectionAsciiString {
        TCollectionAsciiString {
            data: self.data.to_lowercase(),
        }
    }

    /// Converts to uppercase.
    pub fn to_uppercase(&self) -> TCollectionAsciiString {
        TCollectionAsciiString {
            data: self.data.to_uppercase(),
        }
    }

    /// Returns a reference to the underlying string.
    pub fn as_str(&self) -> &str {
        &self.data
    }

    /// Clears the string.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for TCollectionAsciiString {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TCollectionAsciiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl From<&str> for TCollectionAsciiString {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<String> for TCollectionAsciiString {
    fn from(s: String) -> Self {
        Self { data: s }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = TCollectionAsciiString::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_from_str() {
        let s = TCollectionAsciiString::from_str("hello");
        assert_eq!(s.len(), 5);
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_append() {
        let mut s = TCollectionAsciiString::from_str("hello");
        s.append_str(" world");
        assert_eq!(s.as_str(), "hello world");
    }

    #[test]
    fn test_from_int() {
        let s = TCollectionAsciiString::from_int(42);
        assert_eq!(s.as_str(), "42");
    }

    #[test]
    fn test_search() {
        let s = TCollectionAsciiString::from_str("hello world");
        assert_eq!(s.search("world"), 7);
        assert_eq!(s.search("xyz"), 0);
    }

    #[test]
    fn test_remove() {
        let mut s = TCollectionAsciiString::from_str("hello");
        assert!(s.remove(2, 2));
        assert_eq!(s.as_str(), "hlo");
    }

    #[test]
    fn test_to_lowercase() {
        let s = TCollectionAsciiString::from_str("HeLLo");
        let lower = s.to_lowercase();
        assert_eq!(lower.as_str(), "hello");
    }

    #[test]
    fn test_to_uppercase() {
        let s = TCollectionAsciiString::from_str("hello");
        let upper = s.to_uppercase();
        assert_eq!(upper.as_str(), "HELLO");
    }
}
