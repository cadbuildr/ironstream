// FILE: top_ope_b_rep_build_block_iterator.rs
// occt: TopOpeBRepBuild_BlockIterator

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildBlockIterator {
    lower: i32,
    upper: i32,
    current: i32,
}

impl TopOpeBRepBuildBlockIterator {
    pub fn new() -> Self {
        TopOpeBRepBuildBlockIterator {
            lower: 0,
            upper: 0,
            current: 0,
        }
    }

    pub fn with_range(lower: i32, upper: i32) -> Self {
        TopOpeBRepBuildBlockIterator {
            lower,
            upper,
            current: lower,
        }
    }

    pub fn initialize(&mut self) {
        self.current = self.lower;
    }

    pub fn more(&self) -> bool {
        self.current <= self.upper
    }

    pub fn next(&mut self) {
        self.current += 1;
    }

    pub fn value(&self) -> i32 {
        self.current
    }

    pub fn extent(&self) -> i32 {
        if self.lower <= self.upper {
            self.upper - self.lower + 1
        } else {
            0
        }
    }
}

impl Default for TopOpeBRepBuildBlockIterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_iterator_new() {
        let iter = TopOpeBRepBuildBlockIterator::new();
        assert_eq!(iter.lower, 0);
        assert_eq!(iter.upper, 0);
    }

    #[test]
    fn test_block_iterator_with_range() {
        let mut iter = TopOpeBRepBuildBlockIterator::with_range(1, 5);
        assert_eq!(iter.value(), 1);
        assert!(iter.more());

        iter.next();
        assert_eq!(iter.value(), 2);

        iter.next();
        iter.next();
        iter.next();
        assert_eq!(iter.value(), 5);

        iter.next();
        assert!(!iter.more());
    }

    #[test]
    fn test_block_iterator_extent() {
        let iter = TopOpeBRepBuildBlockIterator::with_range(1, 5);
        assert_eq!(iter.extent(), 5);

        let iter_empty = TopOpeBRepBuildBlockIterator::with_range(5, 1);
        assert_eq!(iter_empty.extent(), 0);
    }

    #[test]
    fn test_block_iterator_initialize() {
        let mut iter = TopOpeBRepBuildBlockIterator::with_range(2, 8);
        iter.next();
        iter.next();
        assert_eq!(iter.value(), 4);

        iter.initialize();
        assert_eq!(iter.value(), 2);
    }
}
