// FILE: t_collection_h_extended_string.rs
// occt: TCollection_HExtendedString

/// A handle to an extended (Unicode/16-bit) string
/// Maps to `class TCollection_HExtendedString : public Standard_Transient` in OCCT
#[derive(Debug, Clone)]
pub struct TCollectionHExtendedString {
    data: String, // Rust's String is already UTF-8, which covers all Unicode
}

impl TCollectionHExtendedString {
    /// Create an empty extended string
    pub fn new() -> Self {
        TCollectionHExtendedString {
            data: String::new(),
        }
    }

    /// Create from a C-style string
    pub fn from_cstring(s: &str) -> Self {
        TCollectionHExtendedString {
            data: s.to_string(),
        }
    }

    /// Create from a wide character string
    pub fn from_wide(s: &str) -> Self {
        TCollectionHExtendedString {
            data: s.to_string(),
        }
    }

    /// Create from a single character
    pub fn from_char(c: char) -> Self {
        TCollectionHExtendedString {
            data: c.to_string(),
        }
    }

    /// Create with length and filler character
    pub fn with_filled(length: usize, filler: char) -> Self {
        TCollectionHExtendedString {
            data: filler.to_string().repeat(length),
        }
    }

    /// Get the string as &str
    pub fn to_cstring(&self) -> &str {
        &self.data
    }

    /// Get the length in characters
    pub fn length(&self) -> usize {
        self.data.chars().count()
    }

    /// Check if string is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Append to this string
    pub fn assign_cat(&mut self, other: &TCollectionHExtendedString) {
        self.data.push_str(&other.data);
    }

    /// Concatenate and return new string
    pub fn cat(&self, other: &TCollectionHExtendedString) -> TCollectionHExtendedString {
        let mut result = self.clone();
        result.assign_cat(other);
        result
    }

    /// Replace character at position
    pub fn change_all(&mut self, old_char: char, new_char: char) {
        self.data = self.data.replace(old_char, &new_char.to_string());
    }

    /// Clear the string
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Insert a character at position (1-indexed)
    pub fn insert(&mut self, where_idx: usize, what: char) {
        if where_idx > 0 && where_idx <= self.data.len() + 1 {
            let mut chars: Vec<char> = self.data.chars().collect();
            chars.insert(where_idx - 1, what);
            self.data = chars.iter().collect();
        }
    }

    /// Get character at position (1-indexed)
    pub fn value(&self, where_idx: usize) -> Option<char> {
        if where_idx > 0 && where_idx <= self.length() {
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

    /// Get substring (1-indexed, inclusive range)
    pub fn sub_string(&self, from_idx: usize, to_idx: usize) -> Option<TCollectionHExtendedString> {
        let len = self.length();
        if from_idx > 0 && from_idx <= len && to_idx >= from_idx && to_idx <= len {
            let chars: Vec<char> = self.data.chars().collect();
            let sub: String = chars[from_idx - 1..to_idx].iter().collect();
            Some(TCollectionHExtendedString { data: sub })
        } else {
            None
        }
    }

    /// Convert to uppercase
    pub fn upper_case(&mut self) {
        self.data = self.data.to_uppercase();
    }

    /// Convert to lowercase
    pub fn lower_case(&mut self) {
        self.data = self.data.to_lowercase();
    }
}

impl Default for TCollectionHExtendedString {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for TCollectionHExtendedString {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl std::fmt::Display for TCollectionHExtendedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_extended_string_new() {
        let s = TCollectionHExtendedString::new();
        assert!(s.is_empty());
    }

    #[test]
    fn test_h_extended_string_from_cstring() {
        let s = TCollectionHExtendedString::from_cstring("hello");
        assert_eq!(s.to_cstring(), "hello");
    }

    #[test]
    fn test_h_extended_string_from_char() {
        let s = TCollectionHExtendedString::from_char('x');
        assert_eq!(s.to_cstring(), "x");
    }

    #[test]
    fn test_h_extended_string_length() {
        let s = TCollectionHExtendedString::from_cstring("hello");
        assert_eq!(s.length(), 5);
    }

    #[test]
    fn test_h_extended_string_assign_cat() {
        let mut s1 = TCollectionHExtendedString::from_cstring("hello");
        let s2 = TCollectionHExtendedString::from_cstring(" world");
        s1.assign_cat(&s2);
        assert_eq!(s1.to_cstring(), "hello world");
    }

    #[test]
    fn test_h_extended_string_cat() {
        let s1 = TCollectionHExtendedString::from_cstring("foo");
        let s2 = TCollectionHExtendedString::from_cstring("bar");
        let result = s1.cat(&s2);
        assert_eq!(result.to_cstring(), "foobar");
    }

    #[test]
    fn test_h_extended_string_change_all() {
        let mut s = TCollectionHExtendedString::from_cstring("hello");
        s.change_all('l', 'x');
        assert_eq!(s.to_cstring(), "hexxo");
    }

    #[test]
    fn test_h_extended_string_search() {
        let s = TCollectionHExtendedString::from_cstring("hello world");
        assert_eq!(s.search("world"), 7);
        assert_eq!(s.search("xyz"), -1);
    }

    #[test]
    fn test_h_extended_string_remove_all() {
        let mut s = TCollectionHExtendedString::from_cstring("hello");
        s.remove_all('l');
        assert_eq!(s.to_cstring(), "heo");
    }

    #[test]
    fn test_h_extended_string_sub_string() {
        let s = TCollectionHExtendedString::from_cstring("abcdef");
        let sub = s.sub_string(2, 4);
        assert_eq!(sub.map(|x| x.to_cstring().to_string()), Some("bcd".to_string()));
    }

    #[test]
    fn test_h_extended_string_upper_case() {
        let mut s = TCollectionHExtendedString::from_cstring("hello");
        s.upper_case();
        assert_eq!(s.to_cstring(), "HELLO");
    }

    #[test]
    fn test_h_extended_string_lower_case() {
        let mut s = TCollectionHExtendedString::from_cstring("HELLO");
        s.lower_case();
        assert_eq!(s.to_cstring(), "hello");
    }
}
