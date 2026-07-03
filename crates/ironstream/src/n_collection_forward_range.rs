// FILE: n_collection_forward_range.rs
// occt: NCollection_ForwardRange

/// Forward range for iteration.
pub struct ForwardRange<T> {
    data: Vec<T>,
    index: usize,
}

impl<T: Clone> ForwardRange<T> {
    pub fn new(data: Vec<T>) -> Self {
        ForwardRange { data, index: 0 }
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn more(&self) -> bool {
        self.index < self.data.len()
    }

    pub fn current(&self) -> Option<&T> {
        if self.more() {
            Some(&self.data[self.index])
        } else {
            None
        }
    }

    pub fn next(&mut self) {
        if self.more() {
            self.index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_range() {
        let mut range = ForwardRange::new(vec![1, 2, 3]);
        assert!(range.more());
        assert_eq!(range.current(), Some(&1));
        range.next();
        assert_eq!(range.current(), Some(&2));
    }
}
