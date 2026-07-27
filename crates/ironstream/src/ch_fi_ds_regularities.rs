// FILE: ch_fi_ds_regularities.rs
// occt: ChFiDS_Regularities
// occt-ref: ChFiDS_ListIteratorOfRegularities

//! Deprecated type aliases for backward compatibility.
//! Use VecDeque<ChFiDsRegul> directly instead.

use std::collections::VecDeque;

/// Regularity structure for fillet operations.
#[derive(Clone, Debug, PartialEq)]
pub struct ChFiDsRegul {
    /// First stripe index
    pub first_stripe: usize,
    /// Second stripe index
    pub second_stripe: usize,
    /// Regularity type
    pub regularity_type: u32,
}

impl ChFiDsRegul {
    /// Creates a new regularity entry.
    pub fn new(first_stripe: usize, second_stripe: usize, regularity_type: u32) -> Self {
        ChFiDsRegul {
            first_stripe,
            second_stripe,
            regularity_type,
        }
    }
}

/// Deprecated list of regularities.
/// Maps to NCollection_List<ChFiDS_Regul>.
pub type ChFiDsRegularities = VecDeque<ChFiDsRegul>;

/// Deprecated iterator over regularities.
/// Maps to NCollection_List<ChFiDS_Regul>::Iterator.
pub struct ChFiDsListIteratorOfRegularities<'a> {
    items: &'a VecDeque<ChFiDsRegul>,
    index: usize,
}

impl<'a> ChFiDsListIteratorOfRegularities<'a> {
    /// Creates a new iterator over the list.
    pub fn new(list: &'a ChFiDsRegularities) -> Self {
        ChFiDsListIteratorOfRegularities {
            items: list,
            index: 0,
        }
    }

    /// Returns true if there are more elements to iterate.
    pub fn more(&self) -> bool {
        self.index < self.items.len()
    }

    /// Returns a reference to the current element.
    pub fn value(&self) -> Option<&ChFiDsRegul> {
        self.items.get(self.index)
    }

    /// Advances to the next element.
    pub fn next(&mut self) {
        self.index += 1;
    }
}

impl<'a> Iterator for ChFiDsListIteratorOfRegularities<'a> {
    type Item = &'a ChFiDsRegul;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.items.get(self.index);
        if current.is_some() {
            self.index += 1;
        }
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regul_creation() {
        let regul = ChFiDsRegul::new(1, 2, 0);
        assert_eq!(regul.first_stripe, 1);
        assert_eq!(regul.second_stripe, 2);
        assert_eq!(regul.regularity_type, 0);
    }

    #[test]
    fn test_regularities_list_creation() {
        let list: ChFiDsRegularities = VecDeque::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_regularities_push() {
        let mut list: ChFiDsRegularities = VecDeque::new();
        list.push_back(ChFiDsRegul::new(0, 1, 1));
        list.push_back(ChFiDsRegul::new(1, 2, 2));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_regularities_iterator() {
        let mut list: ChFiDsRegularities = VecDeque::new();
        list.push_back(ChFiDsRegul::new(0, 1, 1));
        list.push_back(ChFiDsRegul::new(1, 2, 2));
        list.push_back(ChFiDsRegul::new(2, 3, 3));

        let mut iter = ChFiDsListIteratorOfRegularities::new(&list);
        assert!(iter.more());
        assert_eq!(iter.value().unwrap().first_stripe, 0);

        iter.next();
        assert!(iter.more());
        assert_eq!(iter.value().unwrap().first_stripe, 1);

        iter.next();
        assert!(iter.more());
        assert_eq!(iter.value().unwrap().first_stripe, 2);

        iter.next();
        assert!(!iter.more());
    }

    #[test]
    fn test_regularities_iterator_as_rust_iterator() {
        let mut list: ChFiDsRegularities = VecDeque::new();
        list.push_back(ChFiDsRegul::new(0, 1, 1));
        list.push_back(ChFiDsRegul::new(1, 2, 2));

        let iter = ChFiDsListIteratorOfRegularities::new(&list);
        let count = iter.count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_regularities_operations() {
        let mut list: ChFiDsRegularities = VecDeque::new();
        list.push_back(ChFiDsRegul::new(0, 1, 1));
        list.push_back(ChFiDsRegul::new(1, 2, 2));

        assert_eq!(list.len(), 2);
        assert!(list.front().is_some());
        assert!(list.back().is_some());

        list.clear();
        assert!(list.is_empty());
    }
}
