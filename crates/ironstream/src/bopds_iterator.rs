// FILE: bopds_iterator.rs
// occt: BOPDS_Iterator

/// Iterator over pairs of interfering shapes
pub struct BodsIterator {
    pairs: Vec<(i32, i32)>,
    index: usize,
}

impl BodsIterator {
    pub fn new() -> Self {
        BodsIterator {
            pairs: Vec::new(),
            index: 0,
        }
    }

    pub fn add_pair(&mut self, i1: i32, i2: i32) {
        self.pairs.push((i1, i2));
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

    pub fn clear(&mut self) {
        self.pairs.clear();
        self.index = 0;
    }
}

impl Default for BodsIterator {
    fn default() -> Self {
        BodsIterator::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let it = BodsIterator::new();
        assert_eq!(it.more(), false);
    }

    #[test]
    fn test_add_and_iterate() {
        let mut it = BodsIterator::new();
        it.add_pair(0, 1);
        it.add_pair(1, 2);
        assert_eq!(it.more(), true);
        assert_eq!(it.value(), (0, 1));
        it.next();
        assert_eq!(it.value(), (1, 2));
        it.next();
        assert_eq!(it.more(), false);
    }

    #[test]
    fn test_init() {
        let mut it = BodsIterator::new();
        it.add_pair(0, 1);
        it.next();
        assert_eq!(it.more(), false);
        it.init();
        assert_eq!(it.more(), true);
    }

    #[test]
    fn test_clear() {
        let mut it = BodsIterator::new();
        it.add_pair(0, 1);
        it.clear();
        assert_eq!(it.more(), false);
    }
}
