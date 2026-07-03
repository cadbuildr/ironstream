// FILE: bop_tools_pair_selector.rs
// occt: BOPTools_PairSelector

/// Pair of element IDs from two BVH trees.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PairIds {
    pub id1: i32,
    pub id2: i32,
}

impl PairIds {
    /// Create a new pair ID.
    pub fn new(id1: i32, id2: i32) -> Self {
        PairIds { id1, id2 }
    }
}

/// Selector for pairs of elements from two BVH trees.
#[derive(Clone, Debug)]
pub struct BopToolsPairSelector {
    pairs: Vec<PairIds>,
    same_bvhs: bool,
}

impl BopToolsPairSelector {
    /// Create an empty pair selector.
    pub fn new() -> Self {
        BopToolsPairSelector {
            pairs: Vec::new(),
            same_bvhs: false,
        }
    }

    /// Clear the pairs.
    pub fn clear(&mut self) {
        self.pairs.clear();
    }

    /// Sort the pairs.
    pub fn sort(&mut self) {
        self.pairs.sort();
    }

    /// Set whether the two BVH trees are the same.
    pub fn set_same(&mut self, is_same: bool) {
        self.same_bvhs = is_same;
    }

    /// Get the pairs.
    pub fn pairs(&self) -> &[PairIds] {
        &self.pairs
    }

    /// Add a pair.
    pub fn add_pair(&mut self, id1: i32, id2: i32) {
        if self.same_bvhs && id1 >= id2 {
            return;
        }
        self.pairs.push(PairIds::new(id1, id2));
    }
}

impl Default for BopToolsPairSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_ids() {
        let p = PairIds::new(1, 2);
        assert_eq!(p.id1, 1);
        assert_eq!(p.id2, 2);
    }

    #[test]
    fn test_new() {
        let selector = BopToolsPairSelector::new();
        assert_eq!(selector.pairs().len(), 0);
        assert!(!selector.same_bvhs);
    }

    #[test]
    fn test_add_pair() {
        let mut selector = BopToolsPairSelector::new();
        selector.add_pair(1, 2);
        selector.add_pair(3, 4);
        assert_eq!(selector.pairs().len(), 2);
    }

    #[test]
    fn test_same_bvhs_filter() {
        let mut selector = BopToolsPairSelector::new();
        selector.set_same(true);
        selector.add_pair(1, 2);
        selector.add_pair(2, 1); // Should be rejected
        selector.add_pair(1, 1); // Should be rejected
        assert_eq!(selector.pairs().len(), 1);
    }

    #[test]
    fn test_sort() {
        let mut selector = BopToolsPairSelector::new();
        selector.add_pair(3, 1);
        selector.add_pair(1, 2);
        selector.add_pair(2, 3);
        selector.sort();
        assert!(selector.pairs()[0] < selector.pairs()[1]);
    }
}
