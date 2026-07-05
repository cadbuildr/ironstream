// FILE: app_par_curves_array1_of_multi_point.rs
// occt: AppParCurves_Array1OfMultiPoint

//! Deprecated type alias for backward compatibility.
//! Originally from OCCT: NCollection_Array1<AppParCurves_MultiPoint>

/// Deprecated typedef for backward compatibility.
/// Represents a 1D array of MultiPoint elements.
#[derive(Clone, Debug)]
pub struct AppParCurves_Array1OfMultiPoint {
    items: Vec<()>,
}

impl AppParCurves_Array1OfMultiPoint {
    /// Create a new array.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Get the length of the array.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for AppParCurves_Array1OfMultiPoint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let arr = AppParCurves_Array1OfMultiPoint::new();
        assert_eq!(arr.len(), 0);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_array_default() {
        let arr = AppParCurves_Array1OfMultiPoint::default();
        assert_eq!(arr.len(), 0);
    }
}
