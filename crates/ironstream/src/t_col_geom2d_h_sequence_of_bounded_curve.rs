// FILE: t_col_geom2d_h_sequence_of_bounded_curve.rs
// occt: TColGeom2d_HSequenceOfBoundedCurve

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TColGeom2dSeqBoundedCurve {
    elements: Vec<u64>,
}

impl TColGeom2dSeqBoundedCurve {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn append(&mut self, value: u64) {
        self.elements.push(value);
    }

    pub fn prepend(&mut self, value: u64) {
        self.elements.insert(0, value);
    }

    pub fn length(&self) -> usize {
        self.elements.len()
    }

    pub fn first(&self) -> Option<u64> {
        self.elements.first().copied()
    }

    pub fn last(&self) -> Option<u64> {
        self.elements.last().copied()
    }

    pub fn value_at(&self, idx: usize) -> Option<u64> {
        if idx == 0 || idx > self.elements.len() {
            None
        } else {
            self.elements.get(idx - 1).copied()
        }
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }
}

pub type TColGeom2d_HSequenceOfBoundedCurve = Arc<TColGeom2dSeqBoundedCurve>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsequence_append() {
        let mut seq = TColGeom2dSeqBoundedCurve::new();
        seq.append(10);
        seq.append(20);

        assert_eq!(seq.length(), 2);
        assert_eq!(seq.first(), Some(10));
        assert_eq!(seq.last(), Some(20));
    }

    #[test]
    fn test_hsequence_prepend() {
        let mut seq = TColGeom2dSeqBoundedCurve::new();
        seq.append(20);
        seq.prepend(10);

        assert_eq!(seq.value_at(1), Some(10));
        assert_eq!(seq.value_at(2), Some(20));
    }

    #[test]
    fn test_hsequence_shared() {
        let mut seq = TColGeom2dSeqBoundedCurve::new();
        seq.append(42);
        let seq_arc = Arc::new(seq);
        let seq_arc2 = Arc::clone(&seq_arc);

        assert_eq!(Arc::strong_count(&seq_arc), 2);
    }
}
