// FILE: t_col_std_list_of_transient.rs
// occt: TColStd_ListOfTransient

/// TColStd_ListOfTransient is a deprecated alias for a list of transient objects.
/// This is a Rust port implementing OCCT's list semantics.
pub struct TColStdListOfTransient {
    data: Vec<Option<String>>,
}

impl TColStdListOfTransient {
    /// Creates a new empty list.
    pub fn new() -> Self {
        TColStdListOfTransient {
            data: Vec::new(),
        }
    }

    /// Appends an element to the list.
    pub fn append(&mut self, value: Option<String>) {
        self.data.push(value);
    }

    /// Prepends an element to the list.
    pub fn prepend(&mut self, value: Option<String>) {
        self.data.insert(0, value);
    }

    /// Returns the length of the list.
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Gets a reference to an element at 0-based index.
    pub fn at(&self, idx: usize) -> Option<&Option<String>> {
        self.data.get(idx)
    }

    /// Gets a mutable reference to an element at 0-based index.
    pub fn at_mut(&mut self, idx: usize) -> Option<&mut Option<String>> {
        self.data.get_mut(idx)
    }

    /// Removes the first element.
    pub fn remove_first(&mut self) -> Option<Option<String>> {
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

impl Default for TColStdListOfTransient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_and_length() {
        let mut list = TColStdListOfTransient::new();
        assert_eq!(list.length(), 0);

        list.append(Some("obj1".to_string()));
        list.append(None);
        assert_eq!(list.length(), 2);
    }

    #[test]
    fn test_at() {
        let mut list = TColStdListOfTransient::new();
        list.append(Some("first".to_string()));
        list.append(None);

        assert_eq!(list.at(0), Some(&Some("first".to_string())));
        assert_eq!(list.at(1), Some(&None));
        assert_eq!(list.at(2), None);
    }

    #[test]
    fn test_prepend() {
        let mut list = TColStdListOfTransient::new();
        list.append(Some("b".to_string()));
        list.prepend(Some("a".to_string()));

        assert_eq!(list.at(0), Some(&Some("a".to_string())));
    }

    #[test]
    fn test_remove_first() {
        let mut list = TColStdListOfTransient::new();
        list.append(Some("first".to_string()));
        list.append(None);

        let removed = list.remove_first();
        assert_eq!(removed, Some(Some("first".to_string())));
        assert_eq!(list.length(), 1);
    }
}
