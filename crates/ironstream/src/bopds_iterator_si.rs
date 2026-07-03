// FILE: bopds_iterator_si.rs
// occt: BOPDS_IteratorSI

/// Sub-iterator for shape interference
pub struct BodsIteratorSI {
    pairs: Vec<(i32, i32)>,
    index: usize,
}

impl BodsIteratorSI {
    pub fn new() -> Self {
        BodsIteratorSI {
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

impl Default for BodsIteratorSI {
    fn default() -> Self {
        BodsIteratorSI::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let it = BodsIteratorSI::new();
        assert_eq!(it.more(), false);
    }

    #[test]
    fn test_init() {
        let it = BodsIteratorSI::new();
        let mut it_copy = BodsIteratorSI::new();
        it_copy.init();
        assert_eq!(it.more(), it_copy.more());
    }
}
