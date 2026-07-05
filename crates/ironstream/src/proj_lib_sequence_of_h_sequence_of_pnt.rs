// FILE: proj_lib_sequence_of_h_sequence_of_pnt.rs
// occt: ProjLib_SequenceOfHSequenceOfPnt

//! Deprecated: ProjLib_SequenceOfHSequenceOfPnt is a sequence of nested sequences.

use std::collections::VecDeque;

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
    points: VecDeque<Pnt>,
}

impl Sequence {
    pub fn new() -> Self {
        Self {
            points: VecDeque::new(),
        }
    }

    pub fn append(&mut self, p: Pnt) {
        self.points.push_back(p);
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn value(&self, index: usize) -> Option<Pnt> {
        self.points.get(index).copied()
    }
}

impl Default for Sequence {
    fn default() -> Self {
        Self::new()
    }
}

/// Sequence of sequences
#[derive(Debug, Clone)]
pub struct SequenceOfSequence {
    sequences: VecDeque<Sequence>,
}

impl SequenceOfSequence {
    pub fn new() -> Self {
        Self {
            sequences: VecDeque::new(),
        }
    }

    pub fn append(&mut self, seq: Sequence) {
        self.sequences.push_back(seq);
    }

    pub fn len(&self) -> usize {
        self.sequences.len()
    }

    pub fn value(&self, index: usize) -> Option<&Sequence> {
        self.sequences.get(index)
    }

    pub fn change_value(&mut self, index: usize) -> Option<&mut Sequence> {
        self.sequences.get_mut(index)
    }
}

impl Default for SequenceOfSequence {
    fn default() -> Self {
        Self::new()
    }
}

pub type ProjLibSequenceOfHSequenceOfPnt = SequenceOfSequence;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let seq = SequenceOfSequence::new();
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_append() {
        let mut outer = SequenceOfSequence::new();
        let mut inner = Sequence::new();
        inner.append(Pnt::new(1.0, 2.0, 3.0));
        outer.append(inner);

        assert_eq!(outer.len(), 1);
    }
}
