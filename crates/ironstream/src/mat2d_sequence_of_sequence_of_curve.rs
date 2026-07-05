// FILE: mat2d_sequence_of_sequence_of_curve.rs
// occt: MAT2d_SequenceOfSequenceOfCurve

pub struct MAT2dSequenceOfSequenceOfCurve {
    items: Vec<Vec<u32>>,
}

impl MAT2dSequenceOfSequenceOfCurve {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn append(&mut self, item: Vec<u32>) {
        self.items.push(item);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for MAT2dSequenceOfSequenceOfCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        let mut seq = MAT2dSequenceOfSequenceOfCurve::new();
        seq.append(vec![1, 2, 3]);
        seq.append(vec![4, 5]);
        assert_eq!(seq.len(), 2);
    }
}
