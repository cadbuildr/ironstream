// FILE: geom_plate_h_array1_of_sequence_of_real.rs
// occt: GeomPlate_HArray1OfSequenceOfReal

//! Deprecated: Use Arc<Vec<Vec<f64>>> directly.
//! Alias for backward compatibility with OCCT.

use std::sync::Arc;

pub type GeomPlateHArray1OfSequenceOfReal = Arc<Vec<Vec<f64>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_array_creation() {
        let vec = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let h_array: GeomPlateHArray1OfSequenceOfReal = Arc::new(vec);

        assert_eq!(h_array.len(), 2);
        assert_eq!(h_array[0][0], 1.0);
    }

    #[test]
    fn test_h_array_shared() {
        let vec = vec![vec![1.0, 2.0, 3.0]];
        let h_array1 = Arc::new(vec);
        let h_array2 = Arc::clone(&h_array1);

        assert_eq!(Arc::strong_count(&h_array1), 2);
        assert_eq!(h_array2[0][2], 3.0);
    }

    #[test]
    fn test_h_array_immutable_access() {
        let vec = vec![vec![5.0, 6.0], vec![7.0]];
        let h_array: GeomPlateHArray1OfSequenceOfReal = Arc::new(vec);

        assert_eq!(h_array.len(), 2);
        assert_eq!(h_array[1].len(), 1);
        assert_eq!(h_array[1][0], 7.0);
    }
}
