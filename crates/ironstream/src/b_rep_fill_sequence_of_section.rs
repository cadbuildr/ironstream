// FILE: b_rep_fill_sequence_of_section.rs
// occt: BRepFill_SequenceOfSection

//! Deprecated type alias for backward compatibility.
//! A sequence of section elements.

/// Represents a section element.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub id: usize,
}

impl Section {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

/// A sequence of Section elements.
pub type BRepFillSequenceOfSection = Vec<Section>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_creation() {
        let section = Section::new(42);
        assert_eq!(section.id, 42);
    }

    #[test]
    fn test_sequence_creation() {
        let mut seq: BRepFillSequenceOfSection = Vec::new();
        seq.push(Section::new(1));
        seq.push(Section::new(2));
        seq.push(Section::new(3));

        assert_eq!(seq.len(), 3);
        assert_eq!(seq[0].id, 1);
        assert_eq!(seq[2].id, 3);
    }

    #[test]
    fn test_sequence_iteration() {
        let mut seq: BRepFillSequenceOfSection = Vec::new();
        for i in 0..10 {
            seq.push(Section::new(i));
        }

        let ids: Vec<usize> = seq.iter().map(|s| s.id).collect();
        assert_eq!(ids, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
