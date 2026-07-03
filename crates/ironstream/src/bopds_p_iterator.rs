// FILE: bopds_p_iterator.rs
// occt: BOPDS_PIterator

/// Parallel Iterator for shape pairs
pub struct BodsPIterator {
    pairs: Vec<(i32, i32)>,
}

impl BodsPIterator {
    pub fn new() -> Self {
        BodsPIterator {
            pairs: Vec::new(),
        }
    }

    pub fn add_pair(&mut self, i1: i32, i2: i32) {
        self.pairs.push((i1, i2));
    }

    pub fn pairs(&self) -> &[(i32, i32)] {
        &self.pairs
    }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn clear(&mut self) {
        self.pairs.clear();
    }
}

impl Default for BodsPIterator {
    fn default() -> Self {
        BodsPIterator::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let it = BodsPIterator::new();
        assert_eq!(it.len(), 0);
    }

    #[test]
    fn test_add_pair() {
        let mut it = BodsPIterator::new();
        it.add_pair(0, 1);
        assert_eq!(it.len(), 1);
    }

    #[test]
    fn test_pairs() {
        let mut it = BodsPIterator::new();
        it.add_pair(1, 2);
        it.add_pair(3, 4);
        let pairs = it.pairs();
        assert_eq!(pairs.len(), 2);
        assert_eq!(pairs[0], (1, 2));
    }
}
