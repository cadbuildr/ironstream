// FILE: b_rep_mat2d_data_map_of_shape_sequence_of_basic_elt.rs
// occt: BRepMAT2d_DataMapOfShapeSequenceOfBasicElt

//! Deprecated type alias for backward compatibility.
//! Maps shapes to sequences of basic elements.

use std::collections::HashMap;

/// A data map from shape identifiers to sequences of basic element identifiers.
pub type BRepMAT2dDataMapOfShapeSequenceOfBasicElt = HashMap<usize, Vec<usize>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_creation() {
        let mut map: BRepMAT2dDataMapOfShapeSequenceOfBasicElt = HashMap::new();
        map.insert(1, vec![10, 20, 30]);

        assert_eq!(map.len(), 1);
        assert_eq!(map[&1], vec![10, 20, 30]);
    }

    #[test]
    fn test_map_multiple_entries() {
        let mut map: BRepMAT2dDataMapOfShapeSequenceOfBasicElt = HashMap::new();

        for i in 0..5 {
            let mut sequence = Vec::new();
            for j in 0..3 {
                sequence.push(i * 10 + j);
            }
            map.insert(i, sequence);
        }

        assert_eq!(map.len(), 5);
        assert_eq!(map[&3].len(), 3);
        assert_eq!(map[&3][0], 30);
    }

    #[test]
    fn test_map_append() {
        let mut map: BRepMAT2dDataMapOfShapeSequenceOfBasicElt = HashMap::new();
        map.insert(1, vec![10, 20]);

        if let Some(seq) = map.get_mut(&1) {
            seq.push(30);
        }

        assert_eq!(map[&1], vec![10, 20, 30]);
    }
}
