// FILE: prs3d_n_list_of_sequence_of_pnt.rs
// occt: Prs3d_NListOfSequenceOfPnt

//! Deprecated: Prs3d_NListOfSequenceOfPnt is a list of sequences of points.

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

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
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

    pub fn clear(&mut self) {
        self.sequences.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &Sequence> {
        self.sequences.iter()
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

pub type Prs3dNListOfSequenceOfPnt = List;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let list = List::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_append() {
        let mut list = List::new();
        let mut seq = Sequence::new();
        seq.append(Pnt::new(1.0, 2.0, 3.0));
        list.append(seq);

        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_clear() {
        let mut list = List::new();
        let seq = Sequence::new();
        list.append(seq);
        list.clear();

        assert!(list.is_empty());
    }
}
