// FILE: osd_error_list.rs
// occt: OSD_ErrorList

/// List of OS errors.
pub struct ErrorList {
    errors: Vec<(i32, String)>,
}

impl ErrorList {
    /// Create empty error list.
    pub fn new() -> Self {
        ErrorList {
            errors: Vec::new(),
        }
    }

    /// Add error to list.
    pub fn add_error(&mut self, code: i32, msg: &str) {
        self.errors.push((code, msg.to_string()));
    }

    /// Get number of errors.
    pub fn count(&self) -> usize {
        self.errors.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get error at index.
    pub fn get(&self, idx: usize) -> Option<(i32, &str)> {
        self.errors.get(idx).map(|(code, msg)| (*code, msg.as_str()))
    }

    /// Clear all errors.
    pub fn clear(&mut self) {
        self.errors.clear();
    }
}

impl Default for ErrorList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list = ErrorList::new();
        assert!(list.is_empty());
        assert_eq!(list.count(), 0);
    }

    #[test]
    fn test_add_error() {
        let mut list = ErrorList::new();
        list.add_error(1, "Error 1");
        assert_eq!(list.count(), 1);
        assert_eq!(list.get(0).unwrap().0, 1);
    }
}
