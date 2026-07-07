// FILE: ch_fi_ds_sec_h_array1.rs
// occt: ChFiDS_SecHArray1

//! Deprecated type alias for backward compatibility.
//! Use Arc<ChFiDsSecArray1> directly instead.

use std::sync::Arc;

/// Circular section data for fillet operations.
#[derive(Clone, Debug, PartialEq)]
pub struct ChFiDsCircSection {
    /// Center X coordinate
    pub center_x: f64,
    /// Center Y coordinate
    pub center_y: f64,
    /// Center Z coordinate
    pub center_z: f64,
    /// Radius
    pub radius: f64,
    /// Parameter
    pub param: f64,
}

impl ChFiDsCircSection {
    /// Creates a new circular section.
    pub fn new(center_x: f64, center_y: f64, center_z: f64, radius: f64, param: f64) -> Self {
        ChFiDsCircSection {
            center_x,
            center_y,
            center_z,
            radius,
            param,
        }
    }

    /// Creates a zero-initialized circular section.
    pub fn default_section() -> Self {
        ChFiDsCircSection {
            center_x: 0.0,
            center_y: 0.0,
            center_z: 0.0,
            radius: 0.0,
            param: 0.0,
        }
    }
}

impl Default for ChFiDsCircSection {
    fn default() -> Self {
        Self::default_section()
    }
}

/// Handle (reference-counted) circular section array.
/// Deprecated alias for NCollection_HArray1<ChFiDS_CircSection>.
/// Modeled as Arc-wrapped array with 1-based indexing.
pub type ChFiDsSecHArray1 = Arc<ChFiDsSecArray1Inner>;

/// Inner implementation of the circular section array.
pub struct ChFiDsSecArray1Inner {
    items: Vec<ChFiDsCircSection>,
    lower: usize,
}

impl ChFiDsSecArray1Inner {
    /// Creates a new handle-wrapped array with the given size, indexed from lower to upper (inclusive).
    pub fn new(lower: usize, upper: usize) -> ChFiDsSecHArray1 {
        let size = upper.saturating_sub(lower) + 1;
        Arc::new(ChFiDsSecArray1Inner {
            items: vec![ChFiDsCircSection::default(); size],
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
    pub fn get(&self, index: usize) -> Option<&ChFiDsCircSection> {
        if index < self.lower || index > self.upper() {
            return None;
        }
        let offset = index - self.lower;
        self.items.get(offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_sec_array1_creation() {
        let array = ChFiDsSecArray1Inner::new(1, 10);
        assert_eq!(array.lower(), 1);
        assert_eq!(array.upper(), 10);
        assert_eq!(array.len(), 10);
    }

    #[test]
    fn test_handle_sec_array1_is_arc() {
        let array1 = ChFiDsSecArray1Inner::new(1, 5);
        let array2 = Arc::clone(&array1);
        assert_eq!(Arc::strong_count(&array1), 2);
        assert_eq!(Arc::strong_count(&array2), 2);
    }

    #[test]
    fn test_handle_sec_array1_get() {
        let array = ChFiDsSecArray1Inner::new(1, 5);
        let result = array.get(1);
        assert!(result.is_some());
        assert_eq!(result.unwrap().radius, 0.0);
    }

    #[test]
    fn test_handle_sec_array1_bounds() {
        let array = ChFiDsSecArray1Inner::new(5, 10);
        assert_eq!(array.lower(), 5);
        assert_eq!(array.upper(), 10);
        assert!(array.get(5).is_some());
        assert!(array.get(4).is_none());
        assert!(array.get(11).is_none());
    }

    #[test]
    fn test_handle_sec_array1_shared_reference() {
        let array1 = ChFiDsSecArray1Inner::new(1, 3);
        let array2 = Arc::clone(&array1);

        assert_eq!(array1.len(), array2.len());
        assert_eq!(array1.lower(), array2.lower());
        assert_eq!(array1.upper(), array2.upper());
    }
}
