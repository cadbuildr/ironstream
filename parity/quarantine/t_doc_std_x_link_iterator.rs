// FILE: t_doc_std_x_link_iterator.rs
// occt: TDocStd_XLinkIterator

/// An iterator for traversing XLink attributes.
#[derive(Clone, Debug)]
pub struct TDocStd_XLinkIterator {
    xlinks: Vec<String>,
    current_index: Option<usize>,
}

impl TDocStd_XLinkIterator {
    /// Create a new XLink iterator.
    pub fn new(xlinks: Vec<String>) -> Self {
        Self {
            xlinks,
            current_index: None,
        }
    }

    /// Reset the iterator to the beginning.
    pub fn init(&mut self) {
        if !self.xlinks.is_empty() {
            self.current_index = Some(0);
        } else {
            self.current_index = None;
        }
    }

    /// Check if there are more elements.
    pub fn more(&self) -> bool {
        if let Some(idx) = self.current_index {
            idx < self.xlinks.len()
        } else {
            false
        }
    }

    /// Move to the next element.
    pub fn next(&mut self) {
        if let Some(idx) = self.current_index {
            if idx + 1 < self.xlinks.len() {
                self.current_index = Some(idx + 1);
            } else {
                self.current_index = None;
            }
        }
    }

    /// Get the current element.
    pub fn value(&self) -> Option<&str> {
        self.current_index
            .and_then(|idx| self.xlinks.get(idx).map(|s| s.as_str()))
    }
}

impl Default for TDocStd_XLinkIterator {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_iterator() {
        let xlinks = vec!["link1".to_string(), "link2".to_string()];
        let iter = TDocStd_XLinkIterator::new(xlinks);
        assert!(!iter.more());
    }

    #[test]
    fn test_iterate() {
        let xlinks = vec!["link1".to_string(), "link2".to_string(), "link3".to_string()];
        let mut iter = TDocStd_XLinkIterator::new(xlinks);
        iter.init();

        assert!(iter.more());
        assert_eq!(iter.value(), Some("link1"));

        iter.next();
        assert!(iter.more());
        assert_eq!(iter.value(), Some("link2"));

        iter.next();
        assert!(iter.more());
        assert_eq!(iter.value(), Some("link3"));

        iter.next();
        assert!(!iter.more());
    }

    #[test]
    fn test_empty_iterator() {
        let iter = TDocStd_XLinkIterator::new(Vec::new());
        assert!(!iter.more());
    }

    #[test]
    fn test_default() {
        let iter = TDocStd_XLinkIterator::default();
        assert!(!iter.more());
    }
}
