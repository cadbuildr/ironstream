// FILE: prs3d_n_list_iterator_of_list_of_sequence_of_pnt.rs
// occt: Prs3d_NListIteratorOfListOfSequenceOfPnt

//! Deprecated: Prs3d_NListIteratorOfListOfSequenceOfPnt is an iterator for nested lists.

use std::collections::LinkedList;

/// 3D Point
#[derive(Debug, Clone, Copy)]
pub struct Pnt {
    x: f32,
    y: f32,
    z: f32,
}

impl Pnt {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// Sequence of points
#[derive(Debug, Clone)]
pub struct Sequence {
    points: Vec<Pnt>,
}

impl Sequence {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn append(&mut self, p: Pnt) {
        self.points.push(p);
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Pnt> {
        self.points.iter()
    }
}

impl Default for Sequence {
    fn default() -> Self {
        Self::new()
    }
}

/// List of sequences
#[derive(Debug, Clone)]
pub struct List {
    sequences: LinkedList<Sequence>,
}

impl List {
    pub fn new() -> Self {
        Self {
            sequences: LinkedList::new(),
        }
    }

    pub fn append(&mut self, seq: Sequence) {
        self.sequences.push_back(seq);
    }

    pub fn len(&self) -> usize {
        self.sequences.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sequences.is_empty()
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator over list of sequences
pub struct Iterator {
    sequences: Vec<Sequence>,
    index: usize,
}

impl Iterator {
    pub fn new(list: &List) -> Self {
        Self {
            sequences: list.sequences.iter().cloned().collect(),
            index: 0,
        }
    }

    pub fn more(&self) -> bool {
        self.index < self.sequences.len()
    }

    pub fn next(&mut self) {
        if self.more() {
            self.index += 1;
        }
    }

    pub fn value(&self) -> Option<&Sequence> {
        self.sequences.get(self.index)
    }
}

pub type Prs3dNListIteratorOfListOfSequenceOfPnt = Iterator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_list() {
        let list = List::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_append_sequence() {
        let mut list = List::new();
        let mut seq = Sequence::new();
        seq.append(Pnt::new(1.0, 2.0, 3.0));
        list.append(seq);

        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_iterator() {
        let mut list = List::new();
        let mut seq = Sequence::new();
        seq.append(Pnt::new(1.0, 2.0, 3.0));
        list.append(seq);

        let iter = Iterator::new(&list);
        assert!(iter.more());
    }
}
