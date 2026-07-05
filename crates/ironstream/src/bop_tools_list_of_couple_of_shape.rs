// FILE: bop_tools_list_of_couple_of_shape.rs
// occt: BOPTools_ListOfCoupleOfShape

use std::collections::VecDeque;

/// A couple (pair) of shapes used in Boolean operations.
/// Mirrors BOPTools_CoupleOfShape from OCCT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoupleOfShape {
    shape1: usize,             // First shape ID
    shape2: usize,             // Second shape ID
}

impl CoupleOfShape {
    /// Creates a new couple with default (0, 0) shapes.
    pub fn new() -> Self {
        CoupleOfShape {
            shape1: 0,
            shape2: 0,
        }
    }

    /// Creates a new couple from two shape IDs.
    pub fn from_shapes(shape1: usize, shape2: usize) -> Self {
        CoupleOfShape { shape1, shape2 }
    }

    /// Sets the first shape.
    pub fn set_shape1(&mut self, shape: usize) {
        self.shape1 = shape;
    }

    /// Returns the first shape ID.
    pub fn shape1(&self) -> usize {
        self.shape1
    }

    /// Sets the second shape.
    pub fn set_shape2(&mut self, shape: usize) {
        self.shape2 = shape;
    }

    /// Returns the second shape ID.
    pub fn shape2(&self) -> usize {
        self.shape2
    }
}

impl Default for CoupleOfShape {
    fn default() -> Self {
        Self::new()
    }
}

/// Deprecated type alias: list of couples of shapes.
/// This is a newtype wrapping VecDeque<CoupleOfShape> to match OCCT's NCollection_List semantics.
pub struct BoptoolsListOfCoupleOfShape {
    data: VecDeque<CoupleOfShape>,
}

impl BoptoolsListOfCoupleOfShape {
    /// Creates an empty list.
    pub fn new() -> Self {
        BoptoolsListOfCoupleOfShape {
            data: VecDeque::new(),
        }
    }

    /// Appends a couple to the list.
    pub fn push(&mut self, couple: CoupleOfShape) {
        self.data.push_back(couple);
    }

    /// Prepends a couple to the list.
    pub fn push_front(&mut self, couple: CoupleOfShape) {
        self.data.push_front(couple);
    }

    /// Returns the number of couples.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Accesses a couple by index.
    pub fn get(&self, index: usize) -> Option<&CoupleOfShape> {
        self.data.get(index)
    }

    /// Mutably accesses a couple by index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut CoupleOfShape> {
        self.data.get_mut(index)
    }

    /// Returns an iterator over the couples.
    pub fn iter(&self) -> impl Iterator<Item = &CoupleOfShape> {
        self.data.iter()
    }

    /// Returns a mutable iterator over the couples.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut CoupleOfShape> {
        self.data.iter_mut()
    }

    /// Clears all couples.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Removes and returns the first couple, if any.
    pub fn pop_front(&mut self) -> Option<CoupleOfShape> {
        self.data.pop_front()
    }

    /// Removes and returns the last couple, if any.
    pub fn pop_back(&mut self) -> Option<CoupleOfShape> {
        self.data.pop_back()
    }

    /// Checks if the list contains a couple with both shapes.
    pub fn contains(&self, couple: &CoupleOfShape) -> bool {
        self.data.contains(couple)
    }
}

impl Default for BoptoolsListOfCoupleOfShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_couple_creation() {
        let couple = CoupleOfShape::new();
        assert_eq!(couple.shape1(), 0);
        assert_eq!(couple.shape2(), 0);
    }

    #[test]
    fn test_couple_from_shapes() {
        let couple = CoupleOfShape::from_shapes(1, 2);
        assert_eq!(couple.shape1(), 1);
        assert_eq!(couple.shape2(), 2);
    }

    #[test]
    fn test_couple_set_shapes() {
        let mut couple = CoupleOfShape::new();
        couple.set_shape1(10);
        couple.set_shape2(20);
        assert_eq!(couple.shape1(), 10);
        assert_eq!(couple.shape2(), 20);
    }

    #[test]
    fn test_couple_equality() {
        let couple1 = CoupleOfShape::from_shapes(5, 6);
        let couple2 = CoupleOfShape::from_shapes(5, 6);
        let couple3 = CoupleOfShape::from_shapes(6, 5);
        assert_eq!(couple1, couple2);
        assert_ne!(couple1, couple3);
    }

    #[test]
    fn test_list_creation() {
        let list = BoptoolsListOfCoupleOfShape::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_list_push() {
        let mut list = BoptoolsListOfCoupleOfShape::new();
        let couple = CoupleOfShape::from_shapes(1, 2);
        list.push(couple);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_list_push_front() {
        let mut list = BoptoolsListOfCoupleOfShape::new();
        let couple1 = CoupleOfShape::from_shapes(1, 2);
        let couple2 = CoupleOfShape::from_shapes(3, 4);
        list.push(couple1);
        list.push_front(couple2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.get(0).unwrap().shape1(), 3);
    }

    #[test]
    fn test_list_multiple() {
        let mut list = BoptoolsListOfCoupleOfShape::new();
        for i in 0..5 {
            let couple = CoupleOfShape::from_shapes(i, i + 10);
            list.push(couple);
        }
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn test_list_get() {
        let mut list = BoptoolsListOfCoupleOfShape::new();
        let couple = CoupleOfShape::from_shapes(42, 43);
        list.push(couple);

        let retrieved = list.get(0).unwrap();
        assert_eq!(retrieved.shape1(), 42);
        assert_eq!(retrieved.shape2(), 43);
    }

    #[test]
    fn test_list_get_mut() {
        let mut list = BoptoolsListOfCoupleOfShape::new();
        let couple = CoupleOfShape::from_shapes(1, 2);
        list.push(couple);

        if let Some(c) = list.get_mut(0) {
            c.set_shape1(99);
        }
        assert_eq!(list.get(0).unwrap().shape1(), 99);
    }

    #[test]
    fn test_list_clear() {
        let mut list = BoptoolsListOfCoupleOfShape::new();
        list.push(CoupleOfShape::from_shapes(1, 2));
        list.push(CoupleOfShape::from_shapes(3, 4));
        assert_eq!(list.len(), 2);

        list.clear();
        assert!(list.is_empty());
    }

    #[test]
    fn test_list_pop_front() {
        let mut list = BoptoolsListOfCoupleOfShape::new();
        list.push(CoupleOfShape::from_shapes(1, 2));
        list.push(CoupleOfShape::from_shapes(3, 4));

        let popped = list.pop_front();
        assert!(popped.is_some());
        assert_eq!(popped.unwrap().shape1(), 1);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_list_pop_back() {
        let mut list = BoptoolsListOfCoupleOfShape::new();
        list.push(CoupleOfShape::from_shapes(1, 2));
        list.push(CoupleOfShape::from_shapes(3, 4));

        let popped = list.pop_back();
        assert!(popped.is_some());
        assert_eq!(popped.unwrap().shape1(), 3);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_list_contains() {
        let mut list = BoptoolsListOfCoupleOfShape::new();
        let couple = CoupleOfShape::from_shapes(5, 6);
        list.push(couple);

        assert!(list.contains(&couple));
        assert!(!list.contains(&CoupleOfShape::from_shapes(7, 8)));
    }

    #[test]
    fn test_list_iterator() {
        let mut list = BoptoolsListOfCoupleOfShape::new();
        for i in 0..3 {
            list.push(CoupleOfShape::from_shapes(i, i + 10));
        }

        let count = list.iter().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_list_iter_mut() {
        let mut list = BoptoolsListOfCoupleOfShape::new();
        list.push(CoupleOfShape::from_shapes(1, 2));
        list.push(CoupleOfShape::from_shapes(3, 4));

        for couple in list.iter_mut() {
            couple.set_shape1(99);
        }

        for couple in list.iter() {
            assert_eq!(couple.shape1(), 99);
        }
    }
}
