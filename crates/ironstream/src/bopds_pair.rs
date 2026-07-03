// FILE: bopds_pair.rs
// occt: BOPDS_Pair

use std::cmp::Ordering;

/// The class is to provide the pair of indices of interfering shapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BodsPair {
    my_index1: i32,
    my_index2: i32,
}

impl BodsPair {
    /// Creates an empty pair
    pub fn new() -> Self {
        BodsPair {
            my_index1: -1,
            my_index2: -1,
        }
    }

    /// Creates a pair with indices
    pub fn with_indices(the_index1: i32, the_index2: i32) -> Self {
        BodsPair {
            my_index1: the_index1,
            my_index2: the_index2,
        }
    }

    /// Sets the indices
    pub fn set_indices(&mut self, the_index1: i32, the_index2: i32) {
        self.my_index1 = the_index1;
        self.my_index2 = the_index2;
    }

    /// Gets the indices
    pub fn indices(&self) -> (i32, i32) {
        (self.my_index1, self.my_index2)
    }

    /// Returns the first index
    pub fn first(&self) -> i32 {
        self.my_index1
    }

    /// Returns the second index
    pub fn second(&self) -> i32 {
        self.my_index2
    }

    /// Returns true if the Pair is equal to theOther
    pub fn is_equal(&self, other: &BodsPair) -> bool {
        (self.my_index1 == other.my_index1 && self.my_index2 == other.my_index2)
            || (self.my_index1 == other.my_index2 && self.my_index2 == other.my_index1)
    }
}

impl Default for BodsPair {
    fn default() -> Self {
        BodsPair::new()
    }
}

impl PartialOrd for BodsPair {
    fn partial_cmp(&self, other: &BodsPair) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BodsPair {
    fn cmp(&self, other: &BodsPair) -> Ordering {
        if self.my_index1 != other.my_index1 {
            self.my_index1.cmp(&other.my_index1)
        } else {
            self.my_index2.cmp(&other.my_index2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_new() {
        let pair = BodsPair::new();
        assert_eq!(pair.first(), -1);
        assert_eq!(pair.second(), -1);
    }

    #[test]
    fn test_pair_with_indices() {
        let pair = BodsPair::with_indices(3, 7);
        assert_eq!(pair.first(), 3);
        assert_eq!(pair.second(), 7);
    }

    #[test]
    fn test_set_indices() {
        let mut pair = BodsPair::new();
        pair.set_indices(2, 5);
        assert_eq!(pair.first(), 2);
        assert_eq!(pair.second(), 5);
    }

    #[test]
    fn test_indices() {
        let pair = BodsPair::with_indices(1, 4);
        let (i1, i2) = pair.indices();
        assert_eq!(i1, 1);
        assert_eq!(i2, 4);
    }

    #[test]
    fn test_is_equal_same() {
        let pair1 = BodsPair::with_indices(2, 5);
        let pair2 = BodsPair::with_indices(2, 5);
        assert_eq!(pair1.is_equal(&pair2), true);
    }

    #[test]
    fn test_is_equal_swapped() {
        let pair1 = BodsPair::with_indices(2, 5);
        let pair2 = BodsPair::with_indices(5, 2);
        assert_eq!(pair1.is_equal(&pair2), true);
    }

    #[test]
    fn test_is_equal_different() {
        let pair1 = BodsPair::with_indices(2, 5);
        let pair2 = BodsPair::with_indices(3, 6);
        assert_eq!(pair1.is_equal(&pair2), false);
    }

    #[test]
    fn test_ordering_less() {
        let pair1 = BodsPair::with_indices(2, 5);
        let pair2 = BodsPair::with_indices(3, 5);
        assert!(pair1 < pair2);
    }

    #[test]
    fn test_ordering_greater() {
        let pair1 = BodsPair::with_indices(3, 5);
        let pair2 = BodsPair::with_indices(2, 5);
        assert!(pair1 > pair2);
    }

    #[test]
    fn test_ordering_same_first_different_second() {
        let pair1 = BodsPair::with_indices(2, 3);
        let pair2 = BodsPair::with_indices(2, 5);
        assert!(pair1 < pair2);
    }

    #[test]
    fn test_default() {
        let pair = BodsPair::default();
        assert_eq!(pair.first(), -1);
        assert_eq!(pair.second(), -1);
    }
}
