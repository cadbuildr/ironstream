// FILE: bnd_h_array1_of_sphere.rs
// occt: Bnd_HArray1OfSphere

//! Deprecated type alias for backward compatibility.
//! Use Arc<BndArray1OfSphere> directly instead.

use std::sync::Arc;

/// Handle (reference-counted) bounding sphere array.
/// Deprecated alias for NCollection_HArray1<Bnd_Sphere>.
/// Modeled as Arc-wrapped array with 1-based indexing.
pub type BndHArray1OfSphere = Arc<BndArray1OfSphereInner>;

/// Inner implementation of the bounding sphere array.
pub struct BndArray1OfSphereInner {
    items: Vec<BndSphereItem>,
    lower: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct BndSphereItem {
    center_x: f64,
    center_y: f64,
    center_z: f64,
    radius: f64,
}

impl BndArray1OfSphereInner {
    /// Creates a new handle-wrapped sphere array with the given size, indexed from lower to upper (inclusive).
    pub fn new(lower: usize, upper: usize) -> BndHArray1OfSphere {
        let size = upper.saturating_sub(lower) + 1;
        Arc::new(BndArray1OfSphereInner {
            items: vec![BndSphereItem::default(); size],
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
            (item.center_x, item.center_y, item.center_z, item.radius)
        })
    }
}

impl Default for BndSphereItem {
    fn default() -> Self {
        BndSphereItem {
            center_x: 0.0,
            center_y: 0.0,
            center_z: 0.0,
            radius: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_sphere_array_creation() {
        let array = BndArray1OfSphereInner::new(1, 10);
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 10);
        assert_eq!(array.len(), 10);
    }

    #[test]
    fn test_handle_sphere_array_is_arc() {
        let array1 = BndArray1OfSphereInner::new(1, 5);
        let array2 = Arc::clone(&array1);
        assert_eq!(Arc::strong_count(&array1), 2);
        assert_eq!(Arc::strong_count(&array2), 2);
    }

    #[test]
    fn test_handle_sphere_array_get() {
        let array = BndArray1OfSphereInner::new(1, 5);
        let result = array.get(1);
        assert!(result.is_some());
    }

    #[test]
    fn test_handle_sphere_array_bounds() {
        let array = BndArray1OfSphereInner::new(5, 10);
        assert_eq!(array.lower(), 5);
        assert_eq!(array.upper(), 10);
        assert!(array.get(5).is_some());
        assert!(array.get(4).is_none());
        assert!(array.get(11).is_none());
    }
}
