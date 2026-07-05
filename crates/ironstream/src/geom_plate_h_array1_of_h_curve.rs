// FILE: geom_plate_h_array1_of_h_curve.rs
// occt: GeomPlate_HArray1OfHCurve

//! Deprecated: Use Arc<Vec<HCurve>> directly.
//! Alias for backward compatibility with OCCT.

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct HCurve {
    pub handle_id: usize,
}

impl HCurve {
    pub fn new(id: usize) -> Self {
        HCurve { handle_id: id }
    }
}

pub type GeomPlateHArray1OfHCurve = Arc<Vec<HCurve>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_array_creation() {
        let vec = vec![HCurve::new(1), HCurve::new(2)];
        let h_array: GeomPlateHArray1OfHCurve = Arc::new(vec);

        assert_eq!(h_array.len(), 2);
        assert_eq!(h_array[0].handle_id, 1);
    }

    #[test]
    fn test_h_array_shared() {
        let vec = vec![HCurve::new(10), HCurve::new(20)];
        let h_array1 = Arc::new(vec);
        let h_array2 = Arc::clone(&h_array1);

        assert_eq!(Arc::strong_count(&h_array1), 2);
        assert_eq!(h_array1[1].handle_id, 20);
        assert_eq!(h_array2[0].handle_id, 10);
    }

    #[test]
    fn test_h_array_immutable() {
        let vec = vec![HCurve::new(5)];
        let h_array: GeomPlateHArray1OfHCurve = Arc::new(vec);

        assert_eq!(h_array[0].handle_id, 5);
        assert_eq!(h_array.len(), 1);
    }
}
