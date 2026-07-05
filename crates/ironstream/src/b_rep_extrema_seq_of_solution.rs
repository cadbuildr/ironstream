// FILE: b_rep_extrema_seq_of_solution.rs
// occt: BRepExtrema_SeqOfSolution

//! Deprecated type alias for backward compatibility.
//! A sequence of BRepExtrema solution elements.

/// A sequence of extrema solution elements.
/// Deprecated since OCCT 8.0.0; use Vec<BRepExtremaSolutionElem> directly.
pub type BRepExtremaSeqOfSolution = Vec<BRepExtremaSolutionElem>;

/// Represents a single extrema solution element.
#[derive(Clone, Debug, PartialEq)]
pub struct BRepExtremaSolutionElem {
    pub value: f64,
    pub index1: i32,
    pub index2: i32,
}

impl BRepExtremaSolutionElem {
    pub fn new(value: f64, index1: i32, index2: i32) -> Self {
        Self { value, index1, index2 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seq_of_solution_creation() {
        let mut seq: BRepExtremaSeqOfSolution = Vec::new();
        assert!(seq.is_empty());

        let elem = BRepExtremaSolutionElem::new(1.5, 0, 1);
        seq.push(elem.clone());

        assert_eq!(seq.len(), 1);
        assert_eq!(seq[0].value, 1.5);
        assert_eq!(seq[0].index1, 0);
        assert_eq!(seq[0].index2, 1);
    }

    #[test]
    fn test_seq_of_solution_multiple() {
        let mut seq: BRepExtremaSeqOfSolution = Vec::new();

        for i in 0..10 {
            let elem = BRepExtremaSolutionElem::new(i as f64 * 0.5, i as i32, i as i32 + 1);
            seq.push(elem);
        }

        assert_eq!(seq.len(), 10);
        assert_eq!(seq[5].value, 2.5);
        assert_eq!(seq[5].index1, 5);
    }

    #[test]
    fn test_solution_elem_equality() {
        let elem1 = BRepExtremaSolutionElem::new(1.0, 0, 1);
        let elem2 = BRepExtremaSolutionElem::new(1.0, 0, 1);
        let elem3 = BRepExtremaSolutionElem::new(2.0, 0, 1);

        assert_eq!(elem1, elem2);
        assert_ne!(elem1, elem3);
    }
}
