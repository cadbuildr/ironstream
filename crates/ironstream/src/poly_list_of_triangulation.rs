// FILE: poly_list_of_triangulation.rs
// occt: Poly_ListOfTriangulation

//! Deprecated: Poly_ListOfTriangulation is a list type alias for NCollection_List.

use std::collections::LinkedList;

/// Triangulation placeholder
#[derive(Debug, Clone)]
pub struct Triangulation {
    id: u32,
}

impl Triangulation {
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

/// List of triangulations
#[derive(Debug, Clone)]
pub struct List {
    triangulations: LinkedList<Triangulation>,
}

impl List {
    pub fn new() -> Self {
        Self {
            triangulations: LinkedList::new(),
        }
    }

    pub fn append(&mut self, t: Triangulation) {
        self.triangulations.push_back(t);
    }

    pub fn is_empty(&self) -> bool {
        self.triangulations.is_empty()
    }

    pub fn len(&self) -> usize {
        self.triangulations.len()
    }

    pub fn clear(&mut self) {
        self.triangulations.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &Triangulation> {
        self.triangulations.iter()
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

pub type PolyListOfTriangulation = List;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        let mut list = List::new();
        list.append(Triangulation::new(1));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_is_empty() {
        let list = List::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut list = List::new();
        list.append(Triangulation::new(1));
        list.clear();
        assert!(list.is_empty());
    }
}
