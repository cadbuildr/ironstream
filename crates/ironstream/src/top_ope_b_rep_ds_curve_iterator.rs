// FILE: top_ope_b_rep_ds_curve_iterator.rs
// occt: TopOpeBRepDS_CurveIterator

/// Iterator over curves
#[derive(Debug, Clone)]
pub struct CurveIterator {
    /// Current position
    current: usize,
    /// End position
    end: usize,
}

impl CurveIterator {
    /// Create new iterator
    pub fn new(start: usize, end: usize) -> Self {
        CurveIterator {
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
    fn test_curve_iterator_new() {
        let iter = CurveIterator::new(0, 5);
        assert!(iter.more());
        assert_eq!(iter.current_index(), 0);
    }

    #[test]
    fn test_curve_iterator_iteration() {
        let mut iter = CurveIterator::new(0, 3);
        assert!(iter.more());
        assert_eq!(iter.current_index(), 0);
        iter.next();
        assert_eq!(iter.current_index(), 1);
        iter.next();
        assert_eq!(iter.current_index(), 2);
        iter.next();
        assert!(!iter.more());
    }

    #[test]
    fn test_curve_iterator_empty() {
        let iter = CurveIterator::new(5, 5);
        assert!(!iter.more());
    }
}
