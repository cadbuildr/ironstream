// FILE: prs_mgr_list_of_presentations.rs
// occt: PrsMgr_ListOfPresentations

//! Deprecated: PrsMgr_ListOfPresentations is a list type alias for presentations.

use std::collections::LinkedList;

/// Presentation placeholder
#[derive(Debug, Clone)]
pub struct Presentation {
    id: u32,
}

impl Presentation {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// List of presentations
#[derive(Debug, Clone)]
pub struct List {
    presentations: LinkedList<Presentation>,
}

impl List {
    pub fn new() -> Self {
        Self {
            presentations: LinkedList::new(),
        }
    }

    pub fn append(&mut self, prs: Presentation) {
        self.presentations.push_back(prs);
    }

    pub fn prepend(&mut self, prs: Presentation) {
        self.presentations.push_front(prs);
    }

    pub fn len(&self) -> usize {
        self.presentations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.presentations.is_empty()
    }

    pub fn clear(&mut self) {
        self.presentations.clear();
    }

    pub fn first(&self) -> Option<&Presentation> {
        self.presentations.front()
    }

    pub fn last(&self) -> Option<&Presentation> {
        self.presentations.back()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Presentation> {
        self.presentations.iter()
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

pub type PrsMgrListOfPresentations = List;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        let mut list = List::new();
        list.append(Presentation::new(1));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_first_last() {
        let mut list = List::new();
        list.append(Presentation::new(1));
        list.append(Presentation::new(2));

        assert_eq!(list.first().unwrap().id(), 1);
        assert_eq!(list.last().unwrap().id(), 2);
    }

    #[test]
    fn test_clear() {
        let mut list = List::new();
        list.append(Presentation::new(1));
        list.clear();
        assert!(list.is_empty());
    }
}
