// FILE: top_ope_b_rep_ds_interference_iterator.rs
// occt: TopOpeBRepDS_InterferenceIterator

/// Iterator over interferences
#[derive(Debug, Clone)]
pub struct InterferenceIterator {
    /// List of interference indices
    interferences: Vec<usize>,
    /// Current position
    current: usize,
}

impl InterferenceIterator {
    /// Create new iterator
    pub fn new() -> Self {
        InterferenceIterator {
            interferences: Vec::new(),
            current: 0,
        }
    }

    /// Create iterator from list
    pub fn from_list(interferences: Vec<usize>) -> Self {
        InterferenceIterator {
            interferences,
            current: 0,
        }
    }

    /// Add interference
    pub fn add_interference(&mut self, i: usize) {
        self.interferences.push(i);
    }

    /// Check if there are more elements
    pub fn more(&self) -> bool {
        self.current < self.interferences.len()
    }

    /// Move to next element
    pub fn next(&mut self) {
        if self.current < self.interferences.len() {
            self.current += 1;
        }
    }

    /// Get current interference index
    pub fn current_index(&self) -> Option<usize> {
        if self.current < self.interferences.len() {
            Some(self.interferences[self.current])
        } else {
            None
        }
    }

    /// Reset iterator
    pub fn reset(&mut self) {
        self.current = 0;
    }

    /// Clear all interferences
    pub fn clear(&mut self) {
        self.interferences.clear();
        self.current = 0;
    }
}

impl Default for InterferenceIterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interference_iterator_new() {
        let iter = InterferenceIterator::new();
        assert!(!iter.more());
    }

    #[test]
    fn test_interference_iterator_from_list() {
        let list = vec![1, 2, 3];
        let iter = InterferenceIterator::from_list(list);
        assert!(iter.more());
        assert_eq!(iter.current_index(), Some(1));
    }

    #[test]
    fn test_interference_iterator_add() {
        let mut iter = InterferenceIterator::new();
        iter.add_interference(5);
        iter.add_interference(10);
        assert!(iter.more());
        assert_eq!(iter.current_index(), Some(5));
        iter.next();
        assert_eq!(iter.current_index(), Some(10));
        iter.next();
        assert!(!iter.more());
    }

    #[test]
    fn test_interference_iterator_reset() {
        let mut iter = InterferenceIterator::new();
        iter.add_interference(1);
        iter.add_interference(2);
        iter.next();
        iter.next();
        assert!(!iter.more());
        iter.reset();
        assert!(iter.more());
        assert_eq!(iter.current_index(), Some(1));
    }
}
