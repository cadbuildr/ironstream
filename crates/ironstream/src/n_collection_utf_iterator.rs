// FILE: n_collection_utf_iterator.rs
// occt: NCollection_UtfIterator

/// Iterator over UTF-8 code points.
pub struct UtfIterator<'a> {
    data: &'a str,
    chars: std::str::Chars<'a>,
    current: Option<char>,
}

impl<'a> UtfIterator<'a> {
    /// Create iterator from UTF-8 string.
    pub fn new(s: &'a str) -> Self {
        let mut it = UtfIterator {
            data: s,
            chars: s.chars(),
            current: None,
        };
        it.current = it.chars.next();
        it
    }

    /// Check if more characters.
    pub fn more(&self) -> bool {
        self.current.is_some()
    }

    /// Get current character code.
    pub fn value(&self) -> Option<u32> {
        self.current.map(|c| c as u32)
    }

    /// Get current character.
    pub fn char_value(&self) -> Option<char> {
        self.current
    }

    /// Move to next character.
    pub fn next(&mut self) {
        self.current = self.chars.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_iterator() {
        let it = UtfIterator::new("");
        assert!(!it.more());
    }

    #[test]
    fn test_iterate_simple() {
        let mut it = UtfIterator::new("hello");
        assert!(it.more());
        assert_eq!(it.char_value(), Some('h'));
        it.next();
        assert_eq!(it.char_value(), Some('e'));
    }

    #[test]
    fn test_char_codes() {
        let mut it = UtfIterator::new("A");
        assert_eq!(it.value(), Some(65));
        it.next();
        assert!(!it.more());
    }
}
