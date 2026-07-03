// FILE: t_collection_extended_string.rs
// occt: TCollection_ExtendedString

/// Variable-length sequence of wide characters (Unicode).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TCollectionExtendedString {
    data: String,
}

impl TCollectionExtendedString {
    /// Initializes an ExtendedString to an empty string.
    pub fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    /// Initializes an ExtendedString with a wide string.
    pub fn from_str(s: &str) -> Self {
        Self {
            data: s.to_string(),
        }
    }

    /// Initializes an ExtendedString with a single wide character.
    pub fn from_char(c: char) -> Self {
        Self {
            data: c.to_string(),
        }
    }

    /// Returns the length of the string (character count).
    pub fn len(&self) -> usize {
        self.data.chars().count()
    }

    /// Returns true if the string is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the character at position (1-indexed).
    pub fn value_at(&self, index: usize) -> Option<char> {
        if index < 1 || index > self.len() {
            return None;
        }
        self.data.chars().nth(index - 1)
    }

    /// Sets the character at position (1-indexed).
    pub fn set_value(&mut self, index: usize, c: char) -> bool {
        let len = self.len();
        if index < 1 || index > len {
            return false;
        }

        let chars: Vec<char> = self.data.chars().collect();
        if index - 1 < chars.len() {
            let mut result = String::new();
            for (i, ch) in chars.iter().enumerate() {
                if i == index - 1 {
                    result.push(c);
                } else {
                    result.push(*ch);
                }
            }
            self.data = result;
            true
        } else {
            false
        }
    }

    /// Appends a character.
    pub fn append_char(&mut self, c: char) {
        self.data.push(c);
    }

    /// Appends a string.
    pub fn append_str(&mut self, s: &str) {
        self.data.push_str(s);
    }

    /// Appends another ExtendedString.
    pub fn append(&mut self, other: &TCollectionExtendedString) {
        self.data.push_str(&other.data);
    }

    /// Searches for a substring, returns 1-indexed position or 0 if not found.
    pub fn search(&self, substring: &str) -> usize {
        self.data
            .find(substring)
            .map(|pos| {
                self.data[..pos]
                    .chars()
                    .count() + 1
            })
            .unwrap_or(0)
    }

    /// Removes characters from position with given length (1-indexed).
    pub fn remove(&mut self, pos: usize, len: usize) -> bool {
        let total_len = self.len();
        if pos < 1 || pos > total_len {
            return false;
        }

        let chars: Vec<char> = self.data.chars().collect();
        let start = pos - 1;
        let end = std::cmp::min(start + len, chars.len());

        let result: String = chars[..start]
            .iter()
            .chain(chars[end..].iter())
            .collect();
        self.data = result;
        true
    }

    /// Truncates string to given length.
    pub fn trunc(&mut self, len: usize) {
        let chars: Vec<char> = self.data.chars().take(len).collect();
        self.data = chars.iter().collect();
    }

    /// Converts to lowercase.
    pub fn to_lowercase(&self) -> TCollectionExtendedString {
        TCollectionExtendedString {
            data: self.data.to_lowercase(),
        }
    }

    /// Converts to uppercase.
    pub fn to_uppercase(&self) -> TCollectionExtendedString {
        TCollectionExtendedString {
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

impl Default for TCollectionExtendedString {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TCollectionExtendedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl From<&str> for TCollectionExtendedString {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<String> for TCollectionExtendedString {
    fn from(s: String) -> Self {
        Self { data: s }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = TCollectionExtendedString::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_from_str() {
        let s = TCollectionExtendedString::from_str("hello");
        assert_eq!(s.len(), 5);
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_unicode() {
        let s = TCollectionExtendedString::from_str("hello世界");
        assert_eq!(s.len(), 7);
    }

    #[test]
    fn test_append() {
        let mut s = TCollectionExtendedString::from_str("hello");
        s.append_str(" world");
        assert_eq!(s.as_str(), "hello world");
    }

    #[test]
    fn test_search() {
        let s = TCollectionExtendedString::from_str("hello world");
        assert_eq!(s.search("world"), 7);
        assert_eq!(s.search("xyz"), 0);
    }

    #[test]
    fn test_to_lowercase() {
        let s = TCollectionExtendedString::from_str("HeLLo");
        let lower = s.to_lowercase();
        assert_eq!(lower.as_str(), "hello");
    }

    #[test]
    fn test_remove() {
        let mut s = TCollectionExtendedString::from_str("hello");
        assert!(s.remove(2, 2));
        assert_eq!(s.as_str(), "hlo");
    }
}
