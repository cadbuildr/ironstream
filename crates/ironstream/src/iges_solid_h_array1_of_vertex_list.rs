// FILE: iges_solid_h_array1_of_vertex_list.rs
// occt: IGESSolid_HArray1OfVertexList

use std::vec::Vec;

/// Deprecated alias for a 1D array of IGESSolid_VertexList objects.
/// This is a legacy wrapper over a Vec for OCCT compatibility.
#[derive(Clone, Debug)]
pub struct IGESSolid_HArray1OfVertexList {
    data: Vec<i32>,
    lower: i32,
}

impl IGESSolid_HArray1OfVertexList {
    /// Create a new array with specified bounds.
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = (upper - lower + 1) as usize;
        IGESSolid_HArray1OfVertexList {
            data: vec![0; size],
            lower,
        }
    }

    /// Set value at given index.
    pub fn set_value(&mut self, index: i32, value: i32) {
        let pos = (index - self.lower) as usize;
        if pos < self.data.len() {
            self.data[pos] = value;
        }
    }

    /// Get value at given index.
    pub fn value(&self, index: i32) -> Option<i32> {
        let pos = (index - self.lower) as usize;
        self.data.get(pos).copied()
    }

    /// Get lower bound.
    pub fn lower_bound(&self) -> i32 {
        self.lower
    }

    /// Get upper bound.
    pub fn upper_bound(&self) -> i32 {
        self.lower + self.data.len() as i32 - 1
    }

    /// Get array length.
    pub fn length(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_array() {
        let arr = IGESSolid_HArray1OfVertexList::new(1, 6);
        assert_eq!(arr.lower_bound(), 1);
        assert_eq!(arr.upper_bound(), 6);
        assert_eq!(arr.length(), 6);
    }

    #[test]
    fn test_set_and_get_value() {
        let mut arr = IGESSolid_HArray1OfVertexList::new(0, 5);
        arr.set_value(2, 77);
        arr.set_value(5, 88);
        assert_eq!(arr.value(2), Some(77));
        assert_eq!(arr.value(5), Some(88));
    }
}
