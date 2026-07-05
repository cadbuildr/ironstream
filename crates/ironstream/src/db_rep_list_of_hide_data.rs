// FILE: db_rep_list_of_hide_data.rs
// occt: DBRep_ListOfHideData

//! Deprecated typedef alias for backward compatibility.
//! This was `NCollection_List<DBRep_HideData>` in OCCT.
//!
//! In Rust, we model this as a newtype over `Vec` for list semantics.

use std::ops::{Deref, DerefMut};

/// DBRep_HideData: placeholder for hidden line removal data.
#[derive(Clone, Debug, PartialEq)]
pub struct DbrepHideData {
    id: u32,
    line_type: u32,
}

impl DbrepHideData {
    pub fn new(id: u32, line_type: u32) -> Self {
        DbrepHideData { id, line_type }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn line_type(&self) -> u32 {
        self.line_type
    }
}

/// A list (ordered collection) of DBRep_HideData items.
/// Models `NCollection_List<DBRep_HideData>` from OCCT.
#[derive(Clone, Debug)]
pub struct DbrepListOfHideData {
    items: Vec<DbrepHideData>,
}

impl DbrepListOfHideData {
    /// Create an empty list.
    pub fn new() -> Self {
        DbrepListOfHideData {
            items: Vec::new(),
        }
    }

    /// Append an item to the end of the list.
    pub fn append(&mut self, item: DbrepHideData) {
        self.items.push(item);
    }

    /// Prepend an item to the beginning of the list.
    pub fn prepend(&mut self, item: DbrepHideData) {
        self.items.insert(0, item);
    }

    /// Remove and return the first item from the list.
    pub fn remove_first(&mut self) -> Option<DbrepHideData> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    /// Remove and return the last item from the list.
    pub fn remove_last(&mut self) -> Option<DbrepHideData> {
        self.items.pop()
    }

    /// Get the number of items in the list.
    pub fn length(&self) -> usize {
        self.items.len()
    }

    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get a reference to the first item.
    pub fn first(&self) -> Option<&DbrepHideData> {
        self.items.first()
    }

    /// Get a reference to the last item.
    pub fn last(&self) -> Option<&DbrepHideData> {
        self.items.last()
    }

    /// Clear the list, removing all items.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Reverse the order of items in the list.
    pub fn reverse(&mut self) {
        self.items.reverse();
    }
}

impl Default for DbrepListOfHideData {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for DbrepListOfHideData {
    type Target = Vec<DbrepHideData>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for DbrepListOfHideData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

/// Iterator for DBRep_ListOfHideData.
/// Models `NCollection_List<DBRep_HideData>::Iterator` from OCCT.
pub struct DbrepListIteratorOfListOfHideData {
    items: Vec<DbrepHideData>,
    index: usize,
}

impl DbrepListIteratorOfListOfHideData {
    /// Create an iterator from a list.
    pub fn new(list: &DbrepListOfHideData) -> Self {
        DbrepListIteratorOfListOfHideData {
            items: list.items.clone(),
            index: 0,
        }
    }

    /// Check if there are more items.
    pub fn more(&self) -> bool {
        self.index < self.items.len()
    }

    /// Get the current item and move to the next.
    pub fn next(&mut self) -> Option<&DbrepHideData> {
        if self.more() {
            let item = &self.items[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }

    /// Move to the next item without returning it.
    pub fn step_forward(&mut self) {
        if self.more() {
            self.index += 1;
        }
    }

    /// Get the current item without advancing.
    pub fn current(&self) -> Option<&DbrepHideData> {
        if self.more() {
            Some(&self.items[self.index])
        } else {
            None
        }
    }
}

impl IntoIterator for DbrepListOfHideData {
    type Item = DbrepHideData;
    type IntoIter = std::vec::IntoIter<DbrepHideData>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_creation() {
        let list = DbrepListOfHideData::new();
        assert!(list.is_empty());
        assert_eq!(list.length(), 0);
    }

    #[test]
    fn test_list_append() {
        let mut list = DbrepListOfHideData::new();
        list.append(DbrepHideData::new(1, 10));
        list.append(DbrepHideData::new(2, 20));

        assert_eq!(list.length(), 2);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_list_prepend() {
        let mut list = DbrepListOfHideData::new();
        list.append(DbrepHideData::new(2, 20));
        list.prepend(DbrepHideData::new(1, 10));

        assert_eq!(list.first().unwrap().id(), 1);
        assert_eq!(list.last().unwrap().id(), 2);
    }

    #[test]
    fn test_list_first_last() {
        let mut list = DbrepListOfHideData::new();
        list.append(DbrepHideData::new(1, 10));
        list.append(DbrepHideData::new(2, 20));
        list.append(DbrepHideData::new(3, 30));

        assert_eq!(list.first().unwrap().id(), 1);
        assert_eq!(list.last().unwrap().id(), 3);
    }

    #[test]
    fn test_list_remove_first() {
        let mut list = DbrepListOfHideData::new();
        list.append(DbrepHideData::new(1, 10));
        list.append(DbrepHideData::new(2, 20));

        let removed = list.remove_first();
        assert_eq!(removed.unwrap().id(), 1);
        assert_eq!(list.length(), 1);
    }

    #[test]
    fn test_list_remove_last() {
        let mut list = DbrepListOfHideData::new();
        list.append(DbrepHideData::new(1, 10));
        list.append(DbrepHideData::new(2, 20));

        let removed = list.remove_last();
        assert_eq!(removed.unwrap().id(), 2);
        assert_eq!(list.length(), 1);
    }

    #[test]
    fn test_list_clear() {
        let mut list = DbrepListOfHideData::new();
        list.append(DbrepHideData::new(1, 10));
        list.append(DbrepHideData::new(2, 20));

        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.length(), 0);
    }

    #[test]
    fn test_list_reverse() {
        let mut list = DbrepListOfHideData::new();
        list.append(DbrepHideData::new(1, 10));
        list.append(DbrepHideData::new(2, 20));
        list.append(DbrepHideData::new(3, 30));

        list.reverse();
        assert_eq!(list.first().unwrap().id(), 3);
        assert_eq!(list.last().unwrap().id(), 1);
    }

    #[test]
    fn test_iterator() {
        let mut list = DbrepListOfHideData::new();
        list.append(DbrepHideData::new(1, 10));
        list.append(DbrepHideData::new(2, 20));
        list.append(DbrepHideData::new(3, 30));

        let mut iter = DbrepListIteratorOfListOfHideData::new(&list);

        assert!(iter.more());
        assert_eq!(iter.current().unwrap().id(), 1);
        iter.step_forward();

        assert!(iter.more());
        assert_eq!(iter.current().unwrap().id(), 2);
        iter.step_forward();

        assert!(iter.more());
        assert_eq!(iter.current().unwrap().id(), 3);
        iter.step_forward();

        assert!(!iter.more());
    }

    #[test]
    fn test_iterator_next() {
        let mut list = DbrepListOfHideData::new();
        list.append(DbrepHideData::new(1, 10));
        list.append(DbrepHideData::new(2, 20));

        let mut iter = DbrepListIteratorOfListOfHideData::new(&list);

        assert_eq!(iter.next().unwrap().id(), 1);
        assert_eq!(iter.next().unwrap().id(), 2);
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_into_iter() {
        let mut list = DbrepListOfHideData::new();
        list.append(DbrepHideData::new(1, 10));
        list.append(DbrepHideData::new(2, 20));

        let count = list.into_iter().count();
        assert_eq!(count, 2);
    }
}
