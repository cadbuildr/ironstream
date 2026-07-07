// FILE: mesh_vs_h_array1_of_sequence_of_integer.rs
// occt: MeshVS_HArray1OfSequenceOfInteger

use std::rc::Rc;
use std::cell::RefCell;

/// TColStd_SequenceOfInteger represents a sequence (ordered list) of integers.
pub type TColstdSequenceOfInteger = Vec<i32>;

/// NCollection_HArray1 is a handle-based 1D array container.
/// MeshVS_HArray1OfSequenceOfInteger is a 1D array of sequences of integers.
pub struct NcollectionHArray1OfSequenceOfInteger {
    data: Vec<TColstdSequenceOfInteger>,
    lower: i32,
    upper: i32,
}

impl NcollectionHArray1OfSequenceOfInteger {
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = (upper - lower + 1) as usize;
        NcollectionHArray1OfSequenceOfInteger {
            data: vec![Vec::new(); size],
            lower,
            upper,
        }
    }

    pub fn set_value(&mut self, index: i32, value: TColstdSequenceOfInteger) {
        if index >= self.lower && index <= self.upper {
            let idx = (index - self.lower) as usize;
            if idx < self.data.len() {
                self.data[idx] = value;
            }
        }
    }

    pub fn value(&self, index: i32) -> Option<&TColstdSequenceOfInteger> {
        if index >= self.lower && index <= self.upper {
            let idx = (index - self.lower) as usize;
            if idx < self.data.len() {
                return Some(&self.data[idx]);
            }
        }
        None
    }

    pub fn value_mut(&mut self, index: i32) -> Option<&mut TColstdSequenceOfInteger> {
        if index >= self.lower && index <= self.upper {
            let idx = (index - self.lower) as usize;
            if idx < self.data.len() {
                return Some(&mut self.data[idx]);
            }
        }
        None
    }

    pub fn lower(&self) -> i32 {
        self.lower
    }

    pub fn upper(&self) -> i32 {
        self.upper
    }

    pub fn length(&self) -> i32 {
        self.upper - self.lower + 1
    }
}

/// A handle/reference-counted wrapper for NCollection_HArray1.
pub type MeshVsHArray1OfSequenceOfInteger =
    Rc<RefCell<NcollectionHArray1OfSequenceOfInteger>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let array = NcollectionHArray1OfSequenceOfInteger::new(1, 5);
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 5);
        assert_eq!(array.length(), 5);
    }

    #[test]
    fn test_array_set_and_get_value() {
        let mut array = NcollectionHArray1OfSequenceOfInteger::new(1, 3);

        let mut seq1 = Vec::new();
        seq1.extend_from_slice(&[1, 2, 3]);

        let mut seq2 = Vec::new();
        seq2.extend_from_slice(&[4, 5, 6]);

        array.set_value(1, seq1.clone());
        array.set_value(2, seq2.clone());

        assert_eq!(array.value(1), Some(&seq1));
        assert_eq!(array.value(2), Some(&seq2));
        assert_eq!(array.value(3), Some(&Vec::new()));
    }

    #[test]
    fn test_array_value_mut() {
        let mut array = NcollectionHArray1OfSequenceOfInteger::new(1, 2);

        if let Some(seq) = array.value_mut(1) {
            seq.push(10);
            seq.push(20);
        }

        if let Some(seq) = array.value(1) {
            assert_eq!(seq.len(), 2);
            assert_eq!(seq[0], 10);
            assert_eq!(seq[1], 20);
        }
    }

    #[test]
    fn test_array_out_of_bounds() {
        let array = NcollectionHArray1OfSequenceOfInteger::new(1, 3);
        assert_eq!(array.value(0), None);
        assert_eq!(array.value(4), None);
        assert_eq!(array.value(10), None);
    }

    #[test]
    fn test_sequence_of_integer() {
        let mut seq: TColstdSequenceOfInteger = Vec::new();
        assert_eq!(seq.len(), 0);

        seq.push(1);
        seq.push(2);
        seq.push(3);
        assert_eq!(seq.len(), 3);

        assert_eq!(seq[0], 1);
        assert_eq!(seq[1], 2);
        assert_eq!(seq[2], 3);
    }

    #[test]
    fn test_handle_array() {
        let handle = Rc::new(RefCell::new(NcollectionHArray1OfSequenceOfInteger::new(1, 3)));

        {
            let mut array = handle.borrow_mut();
            let mut seq = Vec::new();
            seq.extend_from_slice(&[10, 20, 30]);
            array.set_value(1, seq);
        }

        {
            let array = handle.borrow();
            let value = array.value(1);
            assert!(value.is_some());
            let seq = value.unwrap();
            assert_eq!(seq.len(), 3);
        }
    }
}
