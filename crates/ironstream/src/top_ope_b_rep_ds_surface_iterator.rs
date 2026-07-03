// FILE: top_ope_b_rep_ds_surface_iterator.rs
// occt: TopOpeBRepDS_SurfaceIterator

/// Iterator over surfaces
#[derive(Debug, Clone)]
pub struct SurfaceIterator {
    /// Current position
    current: usize,
    /// End position
    end: usize,
}

impl SurfaceIterator {
    /// Create new iterator
    pub fn new(start: usize, end: usize) -> Self {
        SurfaceIterator {
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
    fn test_surface_iterator_new() {
        let iter = SurfaceIterator::new(0, 5);
        assert!(iter.more());
        assert_eq!(iter.current_index(), 0);
    }

    #[test]
    fn test_surface_iterator_iteration() {
        let mut iter = SurfaceIterator::new(0, 2);
        assert!(iter.more());
        iter.next();
        assert_eq!(iter.current_index(), 1);
        iter.next();
        assert!(!iter.more());
    }
}
