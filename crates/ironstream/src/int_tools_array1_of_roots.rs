// FILE: int_tools_array1_of_roots.rs
// occt: IntTools_Array1OfRoots

use std::vec::Vec;

/// Root value with multiplicity
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Root {
    pub value: f64,
    pub multiplicity: u32,
}

impl Root {
    /// Create a new root.
    pub fn new(value: f64, multiplicity: u32) -> Self {
        Root { value, multiplicity }
    }
}

/// Deprecated alias for a 1D array of roots.
#[derive(Clone, Debug)]
pub struct IntTools_Array1OfRoots {
    data: Vec<Root>,
    lower: i32,
}

impl IntTools_Array1OfRoots {
    /// Create a new array with specified bounds.
    pub fn new(lower: i32, upper: i32) -> Self {
        let size = (upper - lower + 1) as usize;
        IntTools_Array1OfRoots {
            data: vec![Root::new(0.0, 0); size],
            lower,
        }
    }

    /// Set value at given index.
    pub fn set_value(&mut self, index: i32, root: Root) {
        let pos = (index - self.lower) as usize;
        if pos < self.data.len() {
            self.data[pos] = root;
        }
    }

    /// Get value at given index.
    pub fn value(&self, index: i32) -> Option<Root> {
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
    fn test_root_new() {
        let root = Root::new(2.5, 2);
        assert_eq!(root.value, 2.5);
        assert_eq!(root.multiplicity, 2);
    }

    #[test]
    fn test_array_new() {
        let arr = IntTools_Array1OfRoots::new(0, 4);
        assert_eq!(arr.lower_bound(), 0);
        assert_eq!(arr.upper_bound(), 4);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn test_set_and_get() {
        let mut arr = IntTools_Array1OfRoots::new(0, 2);
        arr.set_value(0, Root::new(1.5, 1));
        arr.set_value(1, Root::new(2.5, 2));
        assert_eq!(arr.value(0), Some(Root::new(1.5, 1)));
        assert_eq!(arr.value(1), Some(Root::new(2.5, 2)));
    }
}
