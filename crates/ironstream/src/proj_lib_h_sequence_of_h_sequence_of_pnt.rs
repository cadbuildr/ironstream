// FILE: proj_lib_h_sequence_of_h_sequence_of_pnt.rs
// occt: ProjLib_HSequenceOfHSequenceOfPnt

//! Deprecated: ProjLib_HSequenceOfHSequenceOfPnt is a handle wrapper for nested sequences.

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
pub struct HSequence {
    points: VecDeque<Pnt>,
}

impl HSequence {
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

impl Default for HSequence {
    fn default() -> Self {
        Self::new()
    }
}

/// Sequence of HSequences
#[derive(Debug, Clone)]
pub struct HSequenceOfHSequence {
    sequences: VecDeque<HSequence>,
}

impl HSequenceOfHSequence {
    pub fn new() -> Self {
        Self {
            sequences: VecDeque::new(),
        }
    }

    pub fn append(&mut self, seq: HSequence) {
        self.sequences.push_back(seq);
    }

    pub fn len(&self) -> usize {
        self.sequences.len()
    }

    pub fn value(&self, index: usize) -> Option<&HSequence> {
        self.sequences.get(index)
    }

    pub fn change_value(&mut self, index: usize) -> Option<&mut HSequence> {
        self.sequences.get_mut(index)
    }
}

impl Default for HSequenceOfHSequence {
    fn default() -> Self {
        Self::new()
    }
}

pub type ProjLibHSequenceOfHSequenceOfPnt = HSequenceOfHSequence;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let seq = HSequenceOfHSequence::new();
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_append_sequence() {
        let mut outer = HSequenceOfHSequence::new();
        let mut inner = HSequence::new();
        inner.append(Pnt::new(1.0, 2.0, 3.0));
        outer.append(inner);

        assert_eq!(outer.len(), 1);
    }
}
