// FILE: top_ope_b_rep_ds_point_iterator.rs
// occt: TopOpeBRepDS_PointIterator

/// Iterator over points
#[derive(Debug, Clone)]
pub struct PointIterator {
    /// Current position
    current: usize,
    /// End position
    end: usize,
}

impl PointIterator {
    /// Create new iterator
    pub fn new(start: usize, end: usize) -> Self {
        PointIterator {
            current: start,
            end,
        }
    }

    /// Check if there are more elements
    pub fn more(&self) -> bool {
        self.current < self.end
    }

    /// Move to next element
    pub fn next(&mut self) {
        if self.current < self.end {
            self.current += 1;
        }
    }

    /// Get current index
    pub fn current_index(&self) -> usize {
        self.current
    }

    /// Reset iterator
    pub fn reset(&mut self, start: usize, end: usize) {
        self.current = start;
        self.end = end;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_iterator_new() {
        let iter = PointIterator::new(0, 5);
        assert!(iter.more());
        assert_eq!(iter.current_index(), 0);
    }

    #[test]
    fn test_point_iterator_iteration() {
        let mut iter = PointIterator::new(0, 3);
        assert!(iter.more());
        assert_eq!(iter.current_index(), 0);
        iter.next();
        assert_eq!(iter.current_index(), 1);
        iter.next();
        assert_eq!(iter.current_index(), 2);
        iter.next();
        assert!(!iter.more());
    }
}
