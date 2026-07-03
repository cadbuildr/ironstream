// FILE: top_ope_b_rep_v_point_inter_iterator.rs
// occt: TopOpeBRep_VPointInterIterator

/// Iterator over VPointInter objects in a LineInter.
pub struct VPointInterIterator {
    vpoint_index: usize,
    vpoint_nb: usize,
    checkkeep: bool,
    more: bool,
}

impl VPointInterIterator {
    /// Create a new VPointInterIterator.
    pub fn new() -> Self {
        VPointInterIterator {
            vpoint_index: 0,
            vpoint_nb: 0,
            checkkeep: false,
            more: false,
        }
    }

    /// Initialize with a LineInter.
    pub fn init(&mut self, _li: &[u8], checkkeep: bool) {
        self.vpoint_index = 0;
        self.vpoint_nb = 0;
        self.checkkeep = checkkeep;
        self.more = false;
    }

    /// Reset the iterator.
    pub fn reset(&mut self) {
        self.vpoint_index = 0;
    }

    /// Check if there are more items.
    pub fn more(&self) -> bool {
        self.more && self.vpoint_index < self.vpoint_nb
    }

    /// Move to the next item.
    pub fn next(&mut self) {
        if self.more {
            self.vpoint_index += 1;
        }
    }

    /// Get the current VPointInter index.
    pub fn current_vp_index(&self) -> usize {
        self.vpoint_index
    }
}

impl Default for VPointInterIterator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let iterator = VPointInterIterator::new();
        assert!(!iterator.more());
        assert_eq!(iterator.current_vp_index(), 0);
    }

    #[test]
    fn test_init() {
        let mut iterator = VPointInterIterator::new();
        iterator.init(&[], false);
        assert!(!iterator.more());
    }

    #[test]
    fn test_iteration() {
        let mut iterator = VPointInterIterator::new();
        iterator.vpoint_nb = 3;
        iterator.more = true;
        assert!(iterator.more());
        iterator.next();
        assert_eq!(iterator.current_vp_index(), 1);
    }
}
