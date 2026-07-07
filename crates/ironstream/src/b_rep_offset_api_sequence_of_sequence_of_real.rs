// FILE: b_rep_offset_api_sequence_of_sequence_of_real.rs
// occt: BRepOffsetAPI_SequenceOfSequenceOfReal

//! Deprecated type alias for backward compatibility.
//! A sequence of sequences of floating-point numbers.

/// A sequence of sequences of real numbers.
pub type BRepOffsetAPISequenceOfSequenceOfReal = Vec<Vec<f64>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_sequence_creation() {
        let mut seq: BRepOffsetAPISequenceOfSequenceOfReal = Vec::new();
        seq.push(vec![1.0, 2.0, 3.0]);
        seq.push(vec![4.0, 5.0]);
        seq.push(vec![6.0]);

        assert_eq!(seq.len(), 3);
        assert_eq!(seq[0], vec![1.0, 2.0, 3.0]);
        assert_eq!(seq[1].len(), 2);
    }

    #[test]
    fn test_nested_sequence_iteration() {
        let mut seq: BRepOffsetAPISequenceOfSequenceOfReal = Vec::new();

        for i in 0..5 {
            let mut inner = Vec::new();
            for j in 0..i + 1 {
                inner.push((i * 10 + j) as f64);
            }
            seq.push(inner);
        }

        assert_eq!(seq.len(), 5);
        assert_eq!(seq[4].len(), 5);
        assert!((seq[3][2] - 32.0).abs() < 1e-10);
    }

    #[test]
    fn test_nested_sequence_sum() {
        let seq: BRepOffsetAPISequenceOfSequenceOfReal = vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0, 5.0],
            vec![6.0],
        ];

        let total: f64 = seq.iter().flat_map(|inner| inner.iter()).sum();
        assert!((total - 21.0).abs() < 1e-10);
    }
}
