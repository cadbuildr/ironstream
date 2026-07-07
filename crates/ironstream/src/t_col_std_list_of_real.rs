// FILE: t_col_std_list_of_real.rs
// occt: TColStd_ListOfReal

/// TColStd_ListOfReal is a deprecated alias for a list of real numbers.
/// This is a Rust port implementing OCCT's list semantics.
pub struct TColStdListOfReal {
    data: Vec<f64>,
}

impl TColStdListOfReal {
    /// Creates a new empty list.
    pub fn new() -> Self {
        TColStdListOfReal {
            data: Vec::new(),
        }
    }

    /// Appends an element to the list.
    pub fn append(&mut self, value: f64) {
        self.data.push(value);
    }

    /// Prepends an element to the list.
    pub fn prepend(&mut self, value: f64) {
        self.data.insert(0, value);
    }

    /// Returns the length of the list.
    pub fn length(&self) -> usize {
        self.data.len()
    }

    /// Gets a reference to an element at 0-based index.
    pub fn at(&self, idx: usize) -> Option<&f64> {
        self.data.get(idx)
    }

    /// Gets a mutable reference to an element at 0-based index.
    pub fn at_mut(&mut self, idx: usize) -> Option<&mut f64> {
        self.data.get_mut(idx)
    }

    /// Removes the first element.
    pub fn remove_first(&mut self) -> Option<f64> {
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

impl Default for TColStdListOfReal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_and_length() {
        let mut list = TColStdListOfReal::new();
        assert_eq!(list.length(), 0);

        list.append(1.5);
        list.append(2.5);
        list.append(3.5);
        assert_eq!(list.length(), 3);
    }

    #[test]
    fn test_at() {
        let mut list = TColStdListOfReal::new();
        list.append(1.1);
        list.append(2.2);

        assert_eq!(list.at(0), Some(&1.1));
        assert_eq!(list.at(1), Some(&2.2));
        assert_eq!(list.at(2), None);
    }

    #[test]
    fn test_prepend() {
        let mut list = TColStdListOfReal::new();
        list.append(2.0);
        list.prepend(1.0);

        assert_eq!(list.at(0), Some(&1.0));
        assert_eq!(list.at(1), Some(&2.0));
    }

    #[test]
    fn test_remove_first() {
        let mut list = TColStdListOfReal::new();
        list.append(1.0);
        list.append(2.0);

        let removed = list.remove_first();
        assert_eq!(removed, Some(1.0));
        assert_eq!(list.length(), 1);
    }
}
