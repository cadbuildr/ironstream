// FILE: top_loc_s_list_node_of_item_location.rs
// occt: TopLoc_SListNodeOfItemLocation

use crate::top_loc_item_location::ItemLocation;

/// Singly-linked list node for ItemLocation.
#[derive(Debug, Clone)]
pub struct SListNodeOfItemLocation {
    pub data: ItemLocation,
    pub next: Option<Box<SListNodeOfItemLocation>>,
}

impl SListNodeOfItemLocation {
    /// Create new node with data.
    pub fn new(data: ItemLocation) -> Self {
        SListNodeOfItemLocation { data, next: None }
    }

    /// Get reference to data.
    pub fn get_data(&self) -> &ItemLocation {
        &self.data
    }

    /// Get mutable reference to data.
    pub fn get_data_mut(&mut self) -> &mut ItemLocation {
        &mut self.data
    }

    /// Get reference to next node.
    pub fn next_node(&self) -> Option<&SListNodeOfItemLocation> {
        self.next.as_ref().map(|boxed| boxed.as_ref())
    }

    /// Set next node.
    pub fn set_next(&mut self, next: Option<Box<SListNodeOfItemLocation>>) {
        self.next = next;
    }

    /// Check if this is terminal node.
    pub fn is_terminal(&self) -> bool {
        self.next.is_none()
    }
}

/// Simple linked list for ItemLocation.
#[derive(Debug, Clone)]
pub struct ItemLocationList {
    head: Option<Box<SListNodeOfItemLocation>>,
}

impl ItemLocationList {
    /// Create empty list.
    pub fn new() -> Self {
        ItemLocationList { head: None }
    }

    /// Prepend item to front of list.
    pub fn prepend(&mut self, item: ItemLocation) {
        let mut new_node = Box::new(SListNodeOfItemLocation::new(item));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    /// Get first node.
    pub fn first(&self) -> Option<&SListNodeOfItemLocation> {
        self.head.as_ref().map(|boxed| boxed.as_ref())
    }

    /// Check if list is empty.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Get length of list.
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            count += 1;
            current = node.next.as_ref();
        }
        count
    }
}

impl Default for ItemLocationList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_node_new() {
        let item = ItemLocation::new();
        let node = SListNodeOfItemLocation::new(item);
        assert!(node.is_terminal());
    }

    #[test]
    fn test_item_location_list_new() {
        let list = ItemLocationList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_item_location_list_prepend() {
        let mut list = ItemLocationList::new();
        let item = ItemLocation::with_values(1, 1, -1);
        list.prepend(item);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_item_location_list_multiple() {
        let mut list = ItemLocationList::new();
        list.prepend(ItemLocation::with_values(1, 1, -1));
        list.prepend(ItemLocation::with_values(2, 2, -1));
        list.prepend(ItemLocation::with_values(3, 3, -1));
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_list_node_data() {
        let item = ItemLocation::with_values(5, 2, -1);
        let node = SListNodeOfItemLocation::new(item);
        assert_eq!(node.get_data().get_trsf_index(), 5);
    }
}
