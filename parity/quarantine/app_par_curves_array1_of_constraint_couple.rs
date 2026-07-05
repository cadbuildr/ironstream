// FILE: app_par_curves_array1_of_constraint_couple.rs
// occt: AppParCurves_Array1OfConstraintCouple

//! Deprecated type alias for backward compatibility.
//! Originally from OCCT: NCollection_Array1<AppParCurves_ConstraintCouple>

/// Deprecated typedef for backward compatibility.
/// Represents a 1D array of ConstraintCouple elements.
#[derive(Clone, Debug)]
pub struct AppParCurves_Array1OfConstraintCouple {
    items: Vec<()>,
}

impl AppParCurves_Array1OfConstraintCouple {
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

impl Default for AppParCurves_Array1OfConstraintCouple {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_creation() {
        let arr = AppParCurves_Array1OfConstraintCouple::new();
        assert_eq!(arr.len(), 0);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_array_default() {
        let arr = AppParCurves_Array1OfConstraintCouple::default();
        assert_eq!(arr.len(), 0);
    }
}
