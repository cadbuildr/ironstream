// FILE: b_rep_offset_api_sequence_of_sequence_of_shape.rs
// occt: BRepOffsetAPI_SequenceOfSequenceOfShape

//! Deprecated type alias for backward compatibility.
//! A sequence of sequences of shapes.

/// A sequence of sequences of shape identifiers.
pub type BRepOffsetAPISequenceOfSequenceOfShape = Vec<Vec<usize>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_sequence_creation() {
        let mut seq: BRepOffsetAPISequenceOfSequenceOfShape = Vec::new();
        seq.push(vec![1, 2, 3]);
        seq.push(vec![4, 5]);
        seq.push(vec![6]);

        assert_eq!(seq.len(), 3);
        assert_eq!(seq[0], vec![1, 2, 3]);
        assert_eq!(seq[1].len(), 2);
    }

    #[test]
    fn test_nested_sequence_iteration() {
        let mut seq: BRepOffsetAPISequenceOfSequenceOfShape = Vec::new();

        for i in 0..5 {
            let mut inner = Vec::new();
            for j in 0..i + 1 {
                inner.push(i * 10 + j);
            }
            seq.push(inner);
        }

        assert_eq!(seq.len(), 5);
        assert_eq!(seq[4].len(), 5);
        assert_eq!(seq[3][2], 32);
    }

    #[test]
    fn test_nested_sequence_count() {
        let seq: BRepOffsetAPISequenceOfSequenceOfShape = vec![
            vec![1, 2],
            vec![3, 4, 5],
            vec![6],
        ];

        let total: usize = seq.iter().map(|inner| inner.len()).sum();
        assert_eq!(total, 6);
    }
}
