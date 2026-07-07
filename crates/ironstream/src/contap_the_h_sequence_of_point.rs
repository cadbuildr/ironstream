// FILE: contap_the_h_sequence_of_point.rs
// occt: Contap_TheHSequenceOfPoint

//! Deprecated type alias for backward compatibility.
//! This type represents a heap-allocated sequence of Contap_Point objects.
//!
//! This is a deprecated typedef from OCCT. Use a standard Vec or VecDeque instead.
//! Provided for compatibility when porting OCCT code.

/// Deprecated: Heap-allocated sequence of Contap_Point.
/// In modern Rust code, use `Vec<ContapPoint>` instead.
pub type ContapTheHSequenceOfPoint = Vec<ContapPoint>;

/// Placeholder for Contap_Point (would be defined in contap_point module).
#[derive(Debug, Clone, Copy)]
pub struct ContapPoint {
    // Fields would go here in a full implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contap_the_h_sequence_of_point_creation() {
        let seq: ContapTheHSequenceOfPoint = Vec::new();
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_contap_the_h_sequence_of_point_push() {
        let mut seq: ContapTheHSequenceOfPoint = Vec::new();
        let point = ContapPoint {};
        seq.push(point);
        assert_eq!(seq.len(), 1);
    }
}
