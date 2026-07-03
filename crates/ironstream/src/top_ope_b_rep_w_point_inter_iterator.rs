// FILE: top_ope_b_rep_w_point_inter_iterator.rs
// occt: TopOpeBRep_WPointInterIterator

/// Iterator over WPointInter objects in a LineInter.
pub struct WPointInterIterator {
    wpoint_index: usize,
    wpoint_nb: usize,
    more: bool,
}

impl WPointInterIterator {
    /// Create a new WPointInterIterator.
    pub fn new() -> Self {
        WPointInterIterator {
            wpoint_index: 0,
            wpoint_nb: 0,
            more: false,
        }
    }

    /// Initialize with a LineInter.
    pub fn init(&mut self, _li: &[u8]) {
        self.wpoint_index = 0;
        self.wpoint_nb = 0;
        self.more = false;
    }

    /// Reset the iterator.
    pub fn reset(&mut self) {
        self.wpoint_index = 0;
    }

    /// Check if there are more items.
    pub fn more(&self) -> bool {
        self.more && self.wpoint_index < self.wpoint_nb
    }

    /// Move to the next item.
    pub fn next(&mut self) {
        if self.more {
            self.wpoint_index += 1;
        }
    }
}

impl Default for WPointInterIterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let iterator = WPointInterIterator::new();
        assert!(!iterator.more());
        assert_eq!(iterator.wpoint_index, 0);
    }

    #[test]
    fn test_init() {
        let mut iterator = WPointInterIterator::new();
        iterator.init(&[]);
        assert!(!iterator.more());
    }

    #[test]
    fn test_iteration() {
        let mut iterator = WPointInterIterator::new();
        iterator.wpoint_nb = 3;
        iterator.more = true;
        assert!(iterator.more());
        iterator.next();
        assert_eq!(iterator.wpoint_index, 1);
    }
}
