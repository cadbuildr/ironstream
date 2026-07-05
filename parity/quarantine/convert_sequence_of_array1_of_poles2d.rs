// FILE: convert_sequence_of_array1_of_poles2d.rs
// occt: Convert_SequenceOfArray1OfPoles2d

//! Deprecated typedef alias for backward compatibility.
//! This was `NCollection_Sequence<opencascade::handle<TColgp_HArray1OfPnt2d>>` in OCCT.
//!
//! In Rust, we model this as a newtype over `Vec` for sequence semantics.
//! Each element represents a handle (reference) to an array of 2D points (poles).

use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// TColgp_HArray1OfPnt2d placeholder: a 1D array of 2D points.
/// In OCCT, this is a handle to a managed array of gp_Pnt2d.
#[derive(Clone, Debug)]
pub struct TcolgpHArray1OfPnt2d {
    points: Vec<(f64, f64)>,
}

impl TcolgpHArray1OfPnt2d {
    /// Create a new array of 2D points.
    pub fn new() -> Self {
        TcolgpHArray1OfPnt2d {
            points: Vec::new(),
        }
    }

    /// Create an array with initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        TcolgpHArray1OfPnt2d {
            points: Vec::with_capacity(capacity),
        }
    }

    /// Add a 2D point to the array.
    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push((x, y));
    }

    /// Get the length of the array.
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Check if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Get a reference to a point by index (0-based).
    pub fn point(&self, index: usize) -> Option<(f64, f64)> {
        self.points.get(index).copied()
    }
}

impl Default for TcolgpHArray1OfPnt2d {
    fn default() -> Self {
        Self::new()
    }
}

/// A sequence (ordered collection) of handle-like references to TcolgpHArray1OfPnt2d.
/// Models `NCollection_Sequence<opencascade::handle<TColgp_HArray1OfPnt2d>>` from OCCT.
#[derive(Clone, Debug)]
pub struct ConvertSequenceOfArray1OfPoles2d {
    items: Vec<Arc<TcolgpHArray1OfPnt2d>>,
}

impl ConvertSequenceOfArray1OfPoles2d {
    /// Create an empty sequence.
    pub fn new() -> Self {
        ConvertSequenceOfArray1OfPoles2d {
            items: Vec::new(),
        }
    }

    /// Append a handle (Arc) to the sequence.
    pub fn append(&mut self, item: Arc<TcolgpHArray1OfPnt2d>) {
        self.items.push(item);
    }

    /// Remove and return the handle at the given index (1-based for OCCT compatibility).
    pub fn remove(&mut self, index: usize) -> Option<Arc<TcolgpHArray1OfPnt2d>> {
        if index > 0 && index <= self.items.len() {
            Some(self.items.remove(index - 1))
        } else {
            None
        }
    }

    /// Get the number of items in the sequence.
    pub fn length(&self) -> usize {
        self.items.len()
    }

    /// Get a reference to a handle at the given 1-based index.
    pub fn value(&self, index: usize) -> Option<&Arc<TcolgpHArray1OfPnt2d>> {
        if index > 0 && index <= self.items.len() {
            Some(&self.items[index - 1])
        } else {
            None
        }
    }

    /// Get a mutable reference to a handle at the given 1-based index.
    pub fn value_mut(&mut self, index: usize) -> Option<&mut Arc<TcolgpHArray1OfPnt2d>> {
        if index > 0 && index <= self.items.len() {
            Some(&mut self.items[index - 1])
        } else {
            None
        }
    }

    /// Clear the sequence, removing all items.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Insert a handle at the given 1-based position.
    pub fn insert_at(&mut self, index: usize, item: Arc<TcolgpHArray1OfPnt2d>) {
        if index > 0 && index <= self.items.len() + 1 {
            self.items.insert(index - 1, item);
        }
    }
}

impl Default for ConvertSequenceOfArray1OfPoles2d {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for ConvertSequenceOfArray1OfPoles2d {
    type Target = Vec<Arc<TcolgpHArray1OfPnt2d>>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for ConvertSequenceOfArray1OfPoles2d {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let seq = ConvertSequenceOfArray1OfPoles2d::new();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_sequence_append_and_length() {
        let mut seq = ConvertSequenceOfArray1OfPoles2d::new();
        let mut arr1 = TcolgpHArray1OfPnt2d::new();
        arr1.add_point(0.0, 0.0);
        arr1.add_point(1.0, 1.0);

        let mut arr2 = TcolgpHArray1OfPnt2d::new();
        arr2.add_point(2.0, 2.0);

        seq.append(Arc::new(arr1));
        seq.append(Arc::new(arr2));
        assert_eq!(seq.length(), 2);
    }

    #[test]
    fn test_sequence_value_1_based_indexing() {
        let mut seq = ConvertSequenceOfArray1OfPoles2d::new();
        let mut arr1 = TcolgpHArray1OfPnt2d::new();
        arr1.add_point(1.0, 2.0);
        let arc1 = Arc::new(arr1);

        let mut arr2 = TcolgpHArray1OfPnt2d::new();
        arr2.add_point(3.0, 4.0);
        let arc2 = Arc::new(arr2);

        seq.append(arc1.clone());
        seq.append(arc2.clone());

        assert_eq!(seq.value(1).map(|a| Arc::ptr_eq(a, &arc1)), Some(true));
        assert_eq!(seq.value(2).map(|a| Arc::ptr_eq(a, &arc2)), Some(true));
        assert_eq!(seq.value(0), None);
        assert_eq!(seq.value(3), None);
    }

    #[test]
    fn test_sequence_remove() {
        let mut seq = ConvertSequenceOfArray1OfPoles2d::new();
        let arr1 = Arc::new(TcolgpHArray1OfPnt2d::new());
        let arr2 = Arc::new(TcolgpHArray1OfPnt2d::new());
        let arr3 = Arc::new(TcolgpHArray1OfPnt2d::new());

        seq.append(arr1);
        seq.append(arr2.clone());
        seq.append(arr3);

        let removed = seq.remove(2);
        assert_eq!(removed.map(|a| Arc::ptr_eq(&a, &arr2)), Some(true));
        assert_eq!(seq.length(), 2);
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = ConvertSequenceOfArray1OfPoles2d::new();
        seq.append(Arc::new(TcolgpHArray1OfPnt2d::new()));
        seq.append(Arc::new(TcolgpHArray1OfPnt2d::new()));
        assert_eq!(seq.length(), 2);

        seq.clear();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn test_sequence_insert_at() {
        let mut seq = ConvertSequenceOfArray1OfPoles2d::new();
        let arr1 = Arc::new(TcolgpHArray1OfPnt2d::new());
        let arr2 = Arc::new(TcolgpHArray1OfPnt2d::new());
        let arr3 = Arc::new(TcolgpHArray1OfPnt2d::new());

        seq.append(arr1);
        seq.append(arr3.clone());
        seq.insert_at(2, arr2);

        assert_eq!(seq.length(), 3);
    }

    #[test]
    fn test_array_of_points() {
        let mut arr = TcolgpHArray1OfPnt2d::new();
        arr.add_point(1.5, 2.5);
        arr.add_point(3.5, 4.5);

        assert_eq!(arr.len(), 2);
        assert_eq!(arr.point(0), Some((1.5, 2.5)));
        assert_eq!(arr.point(1), Some((3.5, 4.5)));
        assert!(!arr.is_empty());
    }

    #[test]
    fn test_array_empty() {
        let arr = TcolgpHArray1OfPnt2d::new();
        assert!(arr.is_empty());
        assert_eq!(arr.len(), 0);
    }
}
