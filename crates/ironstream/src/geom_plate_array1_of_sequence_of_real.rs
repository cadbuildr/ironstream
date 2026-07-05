// FILE: geom_plate_array1_of_sequence_of_real.rs
// occt: GeomPlate_Array1OfSequenceOfReal

//! Deprecated: Use Vec<Vec<f64>> directly.
//! Alias for backward compatibility with OCCT.

pub type GeomPlateArray1OfSequenceOfReal = Vec<Vec<f64>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_of_sequences() {
        let mut array: GeomPlateArray1OfSequenceOfReal = Vec::new();
        array.push(vec![1.0, 2.0, 3.0]);
        array.push(vec![4.0, 5.0]);

        assert_eq!(array.len(), 2);
        assert_eq!(array[0].len(), 3);
        assert_eq!(array[1].len(), 2);
    }

    #[test]
    fn test_array_operations() {
        let mut array: GeomPlateArray1OfSequenceOfReal = vec![vec![1.0, 2.0], vec![3.0, 4.0, 5.0]];

        assert_eq!(array.len(), 2);
        assert_eq!(array[0][0], 1.0);
        assert_eq!(array[1][2], 5.0);

        array[0].push(6.0);
        assert_eq!(array[0].len(), 3);
    }

    #[test]
    fn test_nested_iteration() {
        let array: GeomPlateArray1OfSequenceOfReal = vec![vec![1.0, 2.0], vec![3.0, 4.0]];

        let sum: f64 = array.iter().flat_map(|seq| seq.iter()).sum();
        assert_eq!(sum, 10.0);
    }
}
