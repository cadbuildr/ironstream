// FILE: bnd_h_array1_of_box.rs
// occt: Bnd_HArray1OfBox

//! Deprecated type alias for backward compatibility.
//! Use Arc<BndArray1OfBox> directly instead.

use std::sync::Arc;

/// Handle (reference-counted) bounding box array.
/// Deprecated alias for NCollection_HArray1<Bnd_Box>.
/// Modeled as Arc-wrapped array with 1-based indexing.
pub type BndHArray1OfBox = Arc<BndArray1OfBoxInner>;

/// Inner implementation of the bounding box array.
pub struct BndArray1OfBoxInner {
    items: Vec<BndBoxItem>,
    lower: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct BndBoxItem {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    z_min: f64,
    z_max: f64,
}

impl BndArray1OfBoxInner {
    /// Creates a new handle-wrapped array with the given size, indexed from lower to upper (inclusive).
    pub fn new(lower: usize, upper: usize) -> BndHArray1OfBox {
        let size = upper.saturating_sub(lower) + 1;
        Arc::new(BndArray1OfBoxInner {
            items: vec![BndBoxItem::default(); size],
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
    pub fn get(&self, index: usize) -> Option<(f64, f64, f64, f64, f64, f64)> {
        if index < self.lower || index > self.upper() {
            return None;
        }
        let offset = index - self.lower;
        self.items.get(offset).map(|item| {
            (item.x_min, item.x_max, item.y_min, item.y_max, item.z_min, item.z_max)
        })
    }
}

impl Default for BndBoxItem {
    fn default() -> Self {
        BndBoxItem {
            x_min: f64::INFINITY,
            x_max: f64::NEG_INFINITY,
            y_min: f64::INFINITY,
            y_max: f64::NEG_INFINITY,
            z_min: f64::INFINITY,
            z_max: f64::NEG_INFINITY,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_array_creation() {
        let array = BndArray1OfBoxInner::new(1, 10);
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 10);
        assert_eq!(array.len(), 10);
    }

    #[test]
    fn test_handle_array_is_arc() {
        let array1 = BndArray1OfBoxInner::new(1, 5);
        let array2 = Arc::clone(&array1);
        assert_eq!(Arc::strong_count(&array1), 2);
        assert_eq!(Arc::strong_count(&array2), 2);
    }

    #[test]
    fn test_handle_array_get() {
        let array = BndArray1OfBoxInner::new(1, 5);
        // Default values are set to infinity/neg_infinity
        let result = array.get(1);
        assert!(result.is_some());
    }

    #[test]
    fn test_handle_array_bounds() {
        let array = BndArray1OfBoxInner::new(5, 10);
        assert_eq!(array.lower(), 5);
        assert_eq!(array.upper(), 10);
        assert!(array.get(5).is_some());
        assert!(array.get(4).is_none());
        assert!(array.get(11).is_none());
    }
}
