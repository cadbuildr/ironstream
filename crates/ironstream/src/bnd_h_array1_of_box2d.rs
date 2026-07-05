// FILE: bnd_h_array1_of_box2d.rs
// occt: Bnd_HArray1OfBox2d

//! Deprecated type alias for backward compatibility.
//! Use Arc<BndArray1OfBox2d> directly instead.

use std::sync::Arc;

/// Handle (reference-counted) 2D bounding box array.
/// Deprecated alias for NCollection_HArray1<Bnd_Box2d>.
/// Modeled as Arc-wrapped array with 1-based indexing.
pub type BndHArray1OfBox2d = Arc<BndArray1OfBox2dInner>;

/// Inner implementation of the 2D bounding box array.
pub struct BndArray1OfBox2dInner {
    items: Vec<BndBox2dItem>,
    lower: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct BndBox2dItem {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

impl BndArray1OfBox2dInner {
    /// Creates a new handle-wrapped 2D box array with the given size, indexed from lower to upper (inclusive).
    pub fn new(lower: usize, upper: usize) -> BndHArray1OfBox2d {
        let size = upper.saturating_sub(lower) + 1;
        Arc::new(BndArray1OfBox2dInner {
            items: vec![BndBox2dItem::default(); size],
            lower,
        })
    }

    /// Returns the lower index bound.
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Returns the upper index bound.
    pub fn upper(&self) -> usize {
        if self.items.is_empty() {
            self.lower
        } else {
            self.lower + self.items.len() - 1
        }
    }

    /// Returns the length of the array.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Gets a reference to an element at the given index (1-based).
    pub fn get(&self, index: usize) -> Option<(f64, f64, f64, f64)> {
        if index < self.lower || index > self.upper() {
            return None;
        }
        let offset = index - self.lower;
        self.items.get(offset).map(|item| {
            (item.x_min, item.x_max, item.y_min, item.y_max)
        })
    }
}

impl Default for BndBox2dItem {
    fn default() -> Self {
        BndBox2dItem {
            x_min: f64::INFINITY,
            x_max: f64::NEG_INFINITY,
            y_min: f64::INFINITY,
            y_max: f64::NEG_INFINITY,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_array2d_creation() {
        let array = BndArray1OfBox2dInner::new(1, 10);
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 10);
        assert_eq!(array.len(), 10);
    }

    #[test]
    fn test_handle_array2d_is_arc() {
        let array1 = BndArray1OfBox2dInner::new(1, 5);
        let array2 = Arc::clone(&array1);
        assert_eq!(Arc::strong_count(&array1), 2);
        assert_eq!(Arc::strong_count(&array2), 2);
    }

    #[test]
    fn test_handle_array2d_get() {
        let array = BndArray1OfBox2dInner::new(1, 5);
        let result = array.get(1);
        assert!(result.is_some());
    }

    #[test]
    fn test_handle_array2d_bounds() {
        let array = BndArray1OfBox2dInner::new(5, 10);
        assert_eq!(array.lower(), 5);
        assert_eq!(array.upper(), 10);
        assert!(array.get(5).is_some());
        assert!(array.get(4).is_none());
        assert!(array.get(11).is_none());
    }
}
