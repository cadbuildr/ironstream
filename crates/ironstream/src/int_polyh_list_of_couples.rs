// FILE: int_polyh_list_of_couples.rs
// occt: IntPolyh_ListOfCouples

use std::vec::Vec;

/// Deprecated alias for a list of couples from polyhedral intersection.
#[derive(Clone, Debug)]
pub struct IntPolyh_ListOfCouples {
    couples: Vec<(u32, u32)>,
}

impl IntPolyh_ListOfCouples {
    /// Create a new list of couples.
    pub fn new() -> Self {
        IntPolyh_ListOfCouples {
            couples: Vec::new(),
        }
    }

    /// Add a couple to the list.
    pub fn append(&mut self, a: u32, b: u32) {
        self.couples.push((a, b));
    }

    /// Get the number of couples.
    pub fn length(&self) -> usize {
        self.couples.len()
    }

    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.couples.is_empty()
    }

    /// Get a couple by index.
    pub fn couple(&self, index: usize) -> Option<(u32, u32)> {
        self.couples.get(index).copied()
    }

    /// Clear the list.
    pub fn clear(&mut self) {
        self.couples.clear();
    }
}

impl Default for IntPolyh_ListOfCouples {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list() {
        let list = IntPolyh_ListOfCouples::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_append() {
        let mut list = IntPolyh_ListOfCouples::new();
        list.append(1, 2);
        list.append(3, 4);
        assert_eq!(list.length(), 2);
        assert_eq!(list.couple(0), Some((1, 2)));
    }

    #[test]
    fn test_clear() {
        let mut list = IntPolyh_ListOfCouples::new();
        list.append(1, 2);
        list.clear();
        assert!(list.is_empty());
    }
}
