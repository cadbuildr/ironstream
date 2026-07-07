// FILE: t_col_std_list_of_ascii_string.rs
// occt: TColStd_ListOfAsciiString

/// TColStd_ListOfAsciiString is a deprecated alias for a list of ASCII strings.
/// This is a Rust port implementing OCCT's list semantics.
pub struct TColStdListOfAsciiString {
    data: Vec<String>,
}

impl TColStdListOfAsciiString {
    /// Creates a new empty list.
    pub fn new() -> Self {
        TColStdListOfAsciiString {
            data: Vec::new(),
        }
    }

    /// Appends an element to the list.
    pub fn append(&mut self, value: String) {
        self.data.push(value);
    }

    /// Prepends an element to the list.
    pub fn prepend(&mut self, value: String) {
        self.data.insert(0, value);
    }

    /// Returns the length of the list.
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Gets a reference to an element at 0-based index.
    pub fn at(&self, idx: usize) -> Option<&String> {
        self.data.get(idx)
    }

    /// Gets a mutable reference to an element at 0-based index.
    pub fn at_mut(&mut self, idx: usize) -> Option<&mut String> {
        self.data.get_mut(idx)
    }

    /// Removes the first element.
    pub fn remove_first(&mut self) -> Option<String> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    /// Clears the list.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for TColStdListOfAsciiString {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_and_length() {
        let mut list = TColStdListOfAsciiString::new();
        assert_eq!(list.length(), 0);

        list.append("hello".to_string());
        list.append("world".to_string());
        assert_eq!(list.length(), 2);
    }

    #[test]
    fn test_at() {
        let mut list = TColStdListOfAsciiString::new();
        list.append("first".to_string());
        list.append("second".to_string());

        assert_eq!(list.at(0), Some(&"first".to_string()));
        assert_eq!(list.at(1), Some(&"second".to_string()));
        assert_eq!(list.at(2), None);
    }

    #[test]
    fn test_prepend() {
        let mut list = TColStdListOfAsciiString::new();
        list.append("b".to_string());
        list.prepend("a".to_string());

        assert_eq!(list.at(0), Some(&"a".to_string()));
        assert_eq!(list.at(1), Some(&"b".to_string()));
    }

    #[test]
    fn test_remove_first() {
        let mut list = TColStdListOfAsciiString::new();
        list.append("a".to_string());
        list.append("b".to_string());

        let removed = list.remove_first();
        assert_eq!(removed, Some("a".to_string()));
        assert_eq!(list.length(), 1);
    }
}
