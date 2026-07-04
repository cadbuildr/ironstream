// FILE: if_select_select_pointed.rs
// occt: IFSelect_SelectPointed

use std::collections::HashSet;

/// A direct selection without an explicit criterium,
/// for instance the result of picking entities on a graphic screen.
/// Can also serve as an internal alternate input.
#[derive(Clone, Debug)]
pub struct IFSelectSelectPointed {
    is_set: bool,
    // Using simplified representation without full entity handles
    items: Vec<usize>, // entity indices
}

impl IFSelectSelectPointed {
    /// Creates a SelectPointed
    pub fn new() -> Self {
        Self {
            is_set: false,
            items: Vec::new(),
        }
    }

    /// Clears the list of selected items.
    /// Also marks the list as unset.
    pub fn clear(&mut self) {
        self.is_set = false;
        self.items.clear();
    }

    /// Tells if the list has been set (even if empty)
    pub fn is_set(&self) -> bool {
        self.is_set
    }

    /// Sets a single entity as the only item in the list
    pub fn set_entity(&mut self, item: usize) {
        self.is_set = true;
        self.items.clear();
        self.items.push(item);
    }

    /// Sets a list of items as the selected entities
    pub fn set_list(&mut self, list: Vec<usize>) {
        self.is_set = true;
        self.items = list;
    }

    /// Adds an item. Returns true if added (was not already present)
    pub fn add(&mut self, item: usize) -> bool {
        self.is_set = true;
        if self.items.contains(&item) {
            false
        } else {
            self.items.push(item);
            true
        }
    }

    /// Removes an item. Returns true if removed (was present)
    pub fn remove(&mut self, item: usize) -> bool {
        if let Some(pos) = self.items.iter().position(|&x| x == item) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    /// Toggles status of an item. Returns the new status (true = present after toggle)
    pub fn toggle(&mut self, item: usize) -> bool {
        self.is_set = true;
        if let Some(pos) = self.items.iter().position(|&x| x == item) {
            self.items.remove(pos);
            false
        } else {
            self.items.push(item);
            true
        }
    }

    /// Adds all items from a list. Returns true if at least one was added
    pub fn add_list(&mut self, list: &[usize]) -> bool {
        self.is_set = true;
        let initial_len = self.items.len();
        for &item in list {
            if !self.items.contains(&item) {
                self.items.push(item);
            }
        }
        self.items.len() > initial_len
    }

    /// Removes all items from a list. Returns true if at least one was removed
    pub fn remove_list(&mut self, list: &[usize]) -> bool {
        let initial_len = self.items.len();
        for &item in list {
            self.items.retain(|&x| x != item);
        }
        self.items.len() < initial_len
    }

    /// Toggles status of all items in a list
    pub fn toggle_list(&mut self, list: &[usize]) -> bool {
        self.is_set = true;
        let mut changed = false;
        for &item in list {
            if let Some(pos) = self.items.iter().position(|&x| x == item) {
                self.items.remove(pos);
                changed = true;
            } else {
                self.items.push(item);
                changed = true;
            }
        }
        changed
    }

    /// Returns the rank (1-indexed) of an item, or 0 if not found
    pub fn rank(&self, item: usize) -> usize {
        self.items
            .iter()
            .position(|&x| x == item)
            .map(|p| p + 1)
            .unwrap_or(0)
    }

    /// Returns the count of selected items
    pub fn nb_items(&self) -> usize {
        self.items.len()
    }

    /// Returns an item by 1-indexed position, or None
    pub fn item(&self, num: usize) -> Option<usize> {
        if num >= 1 && num <= self.items.len() {
            Some(self.items[num - 1])
        } else {
            None
        }
    }

    /// Returns a text identifying this selection type: "Pointed Entities"
    pub fn label(&self) -> &'static str {
        "Pointed Entities"
    }
}

impl Default for IFSelectSelectPointed {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sel = IFSelectSelectPointed::new();
        assert!(!sel.is_set());
        assert_eq!(sel.nb_items(), 0);
    }

    #[test]
    fn test_set_entity() {
        let mut sel = IFSelectSelectPointed::new();
        sel.set_entity(42);
        assert!(sel.is_set());
        assert_eq!(sel.nb_items(), 1);
        assert_eq!(sel.item(1), Some(42));
    }

    #[test]
    fn test_add() {
        let mut sel = IFSelectSelectPointed::new();
        assert!(sel.add(1));
        assert!(!sel.add(1)); // Already present
        assert!(sel.add(2));
        assert_eq!(sel.nb_items(), 2);
        assert!(sel.is_set());
    }

    #[test]
    fn test_remove() {
        let mut sel = IFSelectSelectPointed::new();
        sel.add(1);
        sel.add(2);
        assert!(sel.remove(1));
        assert!(!sel.remove(1)); // Not present
        assert_eq!(sel.nb_items(), 1);
    }

    #[test]
    fn test_toggle() {
        let mut sel = IFSelectSelectPointed::new();
        assert!(sel.toggle(1)); // Add
        assert!(!sel.toggle(1)); // Remove
        assert!(sel.toggle(1)); // Add again
        assert_eq!(sel.nb_items(), 1);
    }

    #[test]
    fn test_rank() {
        let mut sel = IFSelectSelectPointed::new();
        sel.add(10);
        sel.add(20);
        sel.add(30);
        assert_eq!(sel.rank(10), 1);
        assert_eq!(sel.rank(20), 2);
        assert_eq!(sel.rank(30), 3);
        assert_eq!(sel.rank(99), 0);
    }

    #[test]
    fn test_clear() {
        let mut sel = IFSelectSelectPointed::new();
        sel.add(1);
        sel.add(2);
        sel.clear();
        assert!(!sel.is_set());
        assert_eq!(sel.nb_items(), 0);
    }

    #[test]
    fn test_label() {
        let sel = IFSelectSelectPointed::new();
        assert_eq!(sel.label(), "Pointed Entities");
    }

    #[test]
    fn test_add_list() {
        let mut sel = IFSelectSelectPointed::new();
        assert!(sel.add_list(&[1, 2, 3]));
        assert_eq!(sel.nb_items(), 3);
        assert!(!sel.add_list(&[1, 2, 3])); // All already present
    }

    #[test]
    fn test_remove_list() {
        let mut sel = IFSelectSelectPointed::new();
        sel.add_list(&[1, 2, 3, 4]);
        assert!(sel.remove_list(&[1, 3]));
        assert_eq!(sel.nb_items(), 2);
        assert_eq!(sel.rank(2), 1);
        assert_eq!(sel.rank(4), 2);
    }
}
