// FILE: t_collection_h_ascii_string.rs
// occt: TCollection_HAsciiString

/// A handle to an ASCII (8-bit) string
/// Maps to `class TCollection_HAsciiString : public Standard_Transient` in OCCT
#[derive(Debug, Clone)]
pub struct TCollectionHAsciiString {
    data: String,
}

impl TCollectionHAsciiString {
    /// Create an empty ASCII string
    pub fn new() -> Self {
        TCollectionHAsciiString {
            data: String::new(),
        }
    }

    /// Create from a C-style string
    pub fn from_cstring(s: &str) -> Self {
        TCollectionHAsciiString {
            data: s.to_string(),
        }
    }

    /// Create from a single character
    pub fn from_char(c: char) -> Self {
        TCollectionHAsciiString {
            data: c.to_string(),
        }
    }

    /// Create with length and filler character
    pub fn with_filled(length: usize, filler: char) -> Self {
        TCollectionHAsciiString {
            data: filler.to_string().repeat(length),
        }
    }

    /// Create from an integer
    pub fn from_int(value: i32) -> Self {
        TCollectionHAsciiString {
            data: value.to_string(),
        }
    }

    /// Create from a double
    pub fn from_double(value: f64) -> Self {
        TCollectionHAsciiString {
            data: value.to_string(),
        }
    }

    /// Get the string as &str
    pub fn to_cstring(&self) -> &str {
        &self.data
    }

    /// Get the length
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Check if string is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Append a string
    pub fn assign_cat(&mut self, other: &str) {
        self.data.push_str(other);
    }

    /// Append another HAsciiString
    pub fn assign_cat_h(&mut self, other: &TCollectionHAsciiString) {
        self.data.push_str(&other.data);
    }

    /// Concatenate and return new string
    pub fn cat(&self, other: &str) -> TCollectionHAsciiString {
        let mut result = self.clone();
        result.assign_cat(other);
        result
    }

    /// Convert to uppercase
    pub fn upper_case(&mut self) {
        self.data = self.data.to_uppercase();
    }

    /// Convert to lowercase
    pub fn lower_case(&mut self) {
        self.data = self.data.to_lowercase();
    }

    /// Capitalize first letter
    pub fn capitalize(&mut self) {
        if !self.data.is_empty() {
            let mut chars = self.data.chars();
            let first = chars.next().unwrap().to_uppercase().to_string();
            let rest: String = chars.map(|c| c.to_lowercase().to_string()).collect();
            self.data = first + &rest;
        }
    }

    /// Replace character
    pub fn set_value(&mut self, where_idx: usize, what: char) {
        if where_idx < self.data.len() {
            let mut chars: Vec<char> = self.data.chars().collect();
            chars[where_idx] = what;
            self.data = chars.iter().collect();
        }
    }

    /// Get character at position (1-indexed like OCCT)
    pub fn value(&self, where_idx: usize) -> Option<char> {
        if where_idx > 0 && where_idx <= self.data.len() {
            self.data.chars().nth(where_idx - 1)
        } else {
            None
        }
    }

    /// Search for substring
    pub fn search(&self, what: &str) -> i32 {
        self.data
            .find(what)
            .map(|idx| (idx + 1) as i32)
            .unwrap_or(-1)
    }

    /// Remove all occurrences of character
    pub fn remove_all(&mut self, c: char) {
        self.data = self.data.replace(c, "");
    }

    /// Clear the string
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Parse as integer
    pub fn integer_value(&self) -> Option<i32> {
        self.data.parse().ok()
    }

    /// Parse as float
    pub fn real_value(&self) -> Option<f64> {
        self.data.parse().ok()
    }

    /// Check if string is pure ASCII
    pub fn is_ascii(&self) -> bool {
        self.data.chars().all(|c| c.is_ascii())
    }
}

impl Default for TCollectionHAsciiString {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for TCollectionHAsciiString {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl std::fmt::Display for TCollectionHAsciiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_ascii_string_new() {
        let s = TCollectionHAsciiString::new();
        assert!(s.is_empty());
    }

    #[test]
    fn test_h_ascii_string_from_cstring() {
        let s = TCollectionHAsciiString::from_cstring("hello");
        assert_eq!(s.to_cstring(), "hello");
        assert_eq!(s.length(), 5);
    }

    #[test]
    fn test_h_ascii_string_from_char() {
        let s = TCollectionHAsciiString::from_char('x');
        assert_eq!(s.to_cstring(), "x");
    }

    #[test]
    fn test_h_ascii_string_from_int() {
        let s = TCollectionHAsciiString::from_int(42);
        assert_eq!(s.to_cstring(), "42");
    }

    #[test]
    fn test_h_ascii_string_assign_cat() {
        let mut s = TCollectionHAsciiString::from_cstring("hello");
        s.assign_cat(" world");
        assert_eq!(s.to_cstring(), "hello world");
    }

    #[test]
    fn test_h_ascii_string_cat() {
        let s = TCollectionHAsciiString::from_cstring("foo");
        let result = s.cat("bar");
        assert_eq!(result.to_cstring(), "foobar");
    }

    #[test]
    fn test_h_ascii_string_upper_case() {
        let mut s = TCollectionHAsciiString::from_cstring("hello");
        s.upper_case();
        assert_eq!(s.to_cstring(), "HELLO");
    }

    #[test]
    fn test_h_ascii_string_lower_case() {
        let mut s = TCollectionHAsciiString::from_cstring("HELLO");
        s.lower_case();
        assert_eq!(s.to_cstring(), "hello");
    }

    #[test]
    fn test_h_ascii_string_capitalize() {
        let mut s = TCollectionHAsciiString::from_cstring("hELLO");
        s.capitalize();
        assert_eq!(s.to_cstring(), "Hello");
    }

    #[test]
    fn test_h_ascii_string_search() {
        let s = TCollectionHAsciiString::from_cstring("hello world");
        assert_eq!(s.search("world"), 7);
        assert_eq!(s.search("xyz"), -1);
    }

    #[test]
    fn test_h_ascii_string_remove_all() {
        let mut s = TCollectionHAsciiString::from_cstring("hello");
        s.remove_all('l');
        assert_eq!(s.to_cstring(), "heo");
    }

    #[test]
    fn test_h_ascii_string_integer_value() {
        let s = TCollectionHAsciiString::from_cstring("123");
        assert_eq!(s.integer_value(), Some(123));
    }

    #[test]
    fn test_h_ascii_string_real_value() {
        let s = TCollectionHAsciiString::from_cstring("3.14");
        assert!(s.real_value().is_some());
    }

    #[test]
    fn test_h_ascii_string_is_ascii() {
        let s = TCollectionHAsciiString::from_cstring("hello");
        assert!(s.is_ascii());
    }
}
