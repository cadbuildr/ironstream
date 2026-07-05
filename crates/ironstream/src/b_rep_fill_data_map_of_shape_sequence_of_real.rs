// FILE: b_rep_fill_data_map_of_shape_sequence_of_real.rs
// occt: BRepFill_DataMapOfShapeSequenceOfReal

//! Deprecated type alias for backward compatibility.
//! Maps shapes to sequences of floating-point numbers.

use std::collections::HashMap;

/// A data map from shape identifiers to sequences of real numbers.
pub type BRepFillDataMapOfShapeSequenceOfReal = HashMap<usize, Vec<f64>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let mut map: BRepFillDataMapOfShapeSequenceOfReal = HashMap::new();
        let sequence = vec![1.0, 2.5, 3.7, 4.2];
        map.insert(1, sequence);

        assert_eq!(map.len(), 1);
        assert_eq!(map[&1], vec![1.0, 2.5, 3.7, 4.2]);
    }

    #[test]
    fn test_map_multiple_shapes() {
        let mut map: BRepFillDataMapOfShapeSequenceOfReal = HashMap::new();

        for i in 0..5 {
            let mut sequence = Vec::new();
            for j in 0..3 {
                sequence.push((i as f64) + (j as f64) * 0.5);
            }
            map.insert(i, sequence);
        }

        assert_eq!(map.len(), 5);
        assert_eq!(map[&2].len(), 3);
        assert!((map[&2][1] - 2.5).abs() < 1e-10);
    }

    #[test]
    fn test_map_append() {
        let mut map: BRepFillDataMapOfShapeSequenceOfReal = HashMap::new();
        map.insert(1, vec![1.0, 2.0]);

        if let Some(seq) = map.get_mut(&1) {
            seq.push(3.0);
            seq.push(4.0);
        }

        assert_eq!(map[&1], vec![1.0, 2.0, 3.0, 4.0]);
    }
}
