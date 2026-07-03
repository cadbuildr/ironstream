// FILE: bopds_index_range.rs
// occt: BOPDS_IndexRange

/// The class BOPDS_IndexRange is to store the information about range of two indices
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BopdsIndexRange {
    my_first: i32,
    my_last: i32,
}

impl BopdsIndexRange {
    /// Empty constructor
    pub fn new() -> Self {
        BopdsIndexRange {
            my_first: 0,
            my_last: 0,
        }
    }

    /// Constructor with initial indices
    pub fn with_indices(the_i1: i32, the_i2: i32) -> Self {
        BopdsIndexRange {
            my_first: the_i1,
            my_last: the_i2,
        }
    }

    /// Modifier: Sets the first index of the range
    pub fn set_first(&mut self, the_i1: i32) {
        self.my_first = the_i1;
    }

    /// Modifier: Sets the second index of the range
    pub fn set_last(&mut self, the_i2: i32) {
        self.my_last = the_i2;
    }

    /// Selector: Returns the first index of the range
    pub fn first(&self) -> i32 {
        self.my_first
    }

    /// Selector: Returns the second index of the range
    pub fn last(&self) -> i32 {
        self.my_last
    }

    /// Modifier: Sets the first and second indices of the range
    pub fn set_indices(&mut self, the_i1: i32, the_i2: i32) {
        self.my_first = the_i1;
        self.my_last = the_i2;
    }

    /// Selector: Returns the first and second indices of the range
    pub fn indices(&self) -> (i32, i32) {
        (self.my_first, self.my_last)
    }

    /// Query: Returns true if the range contains theIndex
    pub fn contains(&self, the_index: i32) -> bool {
        the_index >= self.my_first && the_index <= self.my_last
    }

    /// Dump the range information
    pub fn dump(&self) {
        println!("IndexRange: First={}, Last={}", self.my_first, self.my_last);
    }
}

impl Default for BopdsIndexRange {
    fn default() -> Self {
        BopdsIndexRange::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_range_new() {
        let range = BopdsIndexRange::new();
        assert_eq!(range.first(), 0);
        assert_eq!(range.last(), 0);
    }

    #[test]
    fn test_index_range_with_indices() {
        let range = BopdsIndexRange::with_indices(5, 10);
        assert_eq!(range.first(), 5);
        assert_eq!(range.last(), 10);
    }

    #[test]
    fn test_set_first() {
        let mut range = BopdsIndexRange::new();
        range.set_first(3);
        assert_eq!(range.first(), 3);
    }

    #[test]
    fn test_set_last() {
        let mut range = BopdsIndexRange::new();
        range.set_last(7);
        assert_eq!(range.last(), 7);
    }

    #[test]
    fn test_set_indices() {
        let mut range = BopdsIndexRange::new();
        range.set_indices(2, 8);
        assert_eq!(range.first(), 2);
        assert_eq!(range.last(), 8);
    }

    #[test]
    fn test_indices() {
        let range = BopdsIndexRange::with_indices(1, 5);
        let (first, last) = range.indices();
        assert_eq!(first, 1);
        assert_eq!(last, 5);
    }

    #[test]
    fn test_contains() {
        let range = BopdsIndexRange::with_indices(5, 10);
        assert_eq!(range.contains(5), true);
        assert_eq!(range.contains(7), true);
        assert_eq!(range.contains(10), true);
        assert_eq!(range.contains(4), false);
        assert_eq!(range.contains(11), false);
    }

    #[test]
    fn test_contains_edge_cases() {
        let range = BopdsIndexRange::with_indices(0, 0);
        assert_eq!(range.contains(0), true);
        assert_eq!(range.contains(1), false);
        assert_eq!(range.contains(-1), false);
    }

    #[test]
    fn test_dump() {
        let range = BopdsIndexRange::with_indices(3, 6);
        range.dump();
    }

    #[test]
    fn test_default() {
        let range = BopdsIndexRange::default();
        assert_eq!(range.first(), 0);
        assert_eq!(range.last(), 0);
    }

    #[test]
    fn test_clone() {
        let range1 = BopdsIndexRange::with_indices(1, 2);
        let range2 = range1;
        assert_eq!(range1, range2);
    }
}
