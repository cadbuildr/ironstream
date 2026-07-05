// FILE: t_col_std_h_array1_of_list_of_integer.rs
// occt: TColStd_HArray1OfListOfInteger

use std::ops::Index;

/// TColStd_HArray1OfListOfInteger is a deprecated alias for a handle (heap-allocated) 1-based array of integer lists.
/// This is a Rust port implementing OCCT's 1-based indexing semantics.
pub struct TColStdHArray1OfListOfInteger {
    data: Vec<Vec<i32>>,
    lower: i32,
    upper: i32,
}

impl TColStdHArray1OfListOfInteger {
    /// Creates a new 1-based array with the given bounds.
    pub fn new(lower: i32, upper: i32) -> Self {
        if lower > upper {
            panic!("Lower bound {} must be <= upper bound {}", lower, upper);
        }
        let size = (upper - lower + 1) as usize;
        TColStdHArray1OfListOfInteger {
            data: vec![Vec::new(); size],
            lower,
            upper,
        }
    }

    /// Returns the lower bound of the array (1-based indexing).
    pub fn lower(&self) -> i32 {
        self.lower
    }

    /// Returns the upper bound of the array (1-based indexing).
    pub fn upper(&self) -> i32 {
        self.upper
    }

    /// Returns the length of the array.
    pub fn length(&self) -> i32 {
        self.upper - self.lower + 1
    }

    /// Gets a reference to the list at the given 1-based index.
    pub fn at(&self, idx: i32) -> Option<&Vec<i32>> {
        if idx < self.lower || idx > self.upper {
            return None;
        }
        let pos = (idx - self.lower) as usize;
        Some(&self.data[pos])
    }

    /// Gets a mutable reference to the list at the given 1-based index.
    pub fn at_mut(&mut self, idx: i32) -> Option<&mut Vec<i32>> {
        if idx < self.lower || idx > self.upper {
            return None;
        }
        let pos = (idx - self.lower) as usize;
        Some(&mut self.data[pos])
    }

    /// Appends a value to the list at the given 1-based index.
    pub fn append_to_list(&mut self, idx: i32, value: i32) {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        let pos = (idx - self.lower) as usize;
        self.data[pos].push(value);
    }
}

impl Index<i32> for TColStdHArray1OfListOfInteger {
    type Output = Vec<i32>;

    fn index(&self, idx: i32) -> &Self::Output {
        if idx < self.lower || idx > self.upper {
            panic!("Index {} out of bounds [{}, {}]", idx, self.lower, self.upper);
        }
        let pos = (idx - self.lower) as usize;
        &self.data[pos]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_and_bounds() {
        let arr = TColStdHArray1OfListOfInteger::new(1, 5);
        assert_eq!(arr.lower(), 1);
        assert_eq!(arr.upper(), 5);
        assert_eq!(arr.length(), 5);
    }

    #[test]
    fn test_append_to_list() {
        let mut arr = TColStdHArray1OfListOfInteger::new(1, 2);
        arr.append_to_list(1, 10);
        arr.append_to_list(1, 20);
        arr.append_to_list(2, 30);

        assert_eq!(arr.at(1), Some(&vec![10, 20]));
        assert_eq!(arr.at(2), Some(&vec![30]));
    }

    #[test]
    fn test_at_mut() {
        let mut arr = TColStdHArray1OfListOfInteger::new(1, 2);
        arr.at_mut(1).map(|list| list.push(42));

        assert_eq!(arr.at(1), Some(&vec![42]));
    }

    #[test]
    fn test_index_operator() {
        let mut arr = TColStdHArray1OfListOfInteger::new(1, 1);
        arr.append_to_list(1, 100);
        assert_eq!(arr[1], vec![100]);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        let arr = TColStdHArray1OfListOfInteger::new(1, 3);
        let _ = arr[5];
    }
}
