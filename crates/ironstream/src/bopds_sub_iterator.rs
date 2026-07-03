// FILE: bopds_sub_iterator.rs
// occt: BOPDS_SubIterator

/// Sub-iterator for shape interference
pub struct BodsSubIterator {
    pairs: Vec<(i32, i32)>,
    index: usize,
}

impl BodsSubIterator {
    pub fn new() -> Self {
        BodsSubIterator {
            pairs: Vec::new(),
            index: 0,
        }
    }

    pub fn more(&self) -> bool {
        self.index < self.pairs.len()
    }

    pub fn next(&mut self) {
        if self.index < self.pairs.len() {
            self.index += 1;
        }
    }

    pub fn value(&self) -> (i32, i32) {
        if self.index < self.pairs.len() {
            self.pairs[self.index]
        } else {
            (-1, -1)
        }
    }

    pub fn init(&mut self) {
        self.index = 0;
    }
}

impl Default for BodsSubIterator {
    fn default() -> Self {
        BodsSubIterator::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let it = BodsSubIterator::new();
        assert_eq!(it.more(), false);
    }

    #[test]
    fn test_init() {
        let mut it = BodsSubIterator::new();
        it.init();
        assert_eq!(it.more(), false);
    }
}
