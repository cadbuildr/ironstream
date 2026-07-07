// FILE: graphic3d_sequence_of_structure.rs
// occt: Graphic3d_SequenceOfStructure

//! Deprecated: Use Vec<Structure> directly.
//! Sequence of graphic structures.

#[derive(Clone, Debug)]
pub struct Structure {
    pub id: usize,
}

impl Structure {
    pub fn new(id: usize) -> Self {
        Structure { id }
    }
}

pub type Graphic3dSequenceOfStructure = Vec<Structure>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let mut seq: Graphic3dSequenceOfStructure = Vec::new();
        seq.push(Structure::new(1));

        assert_eq!(seq.len(), 1);
        assert_eq!(seq[0].id, 1);
    }

    #[test]
    fn test_sequence_operations() {
        let seq = vec![Structure::new(10), Structure::new(20), Structure::new(30)];

        assert_eq!(seq.len(), 3);
        assert_eq!(seq[1].id, 20);
    }

    #[test]
    fn test_sequence_iteration() {
        let seq = vec![Structure::new(1), Structure::new(2), Structure::new(3)];

        let ids: Vec<usize> = seq.iter().map(|s| s.id).collect();
        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[test]
    fn test_sequence_append() {
        let mut seq: Graphic3dSequenceOfStructure = Vec::new();
        for i in 0..5 {
            seq.push(Structure::new(i));
        }

        assert_eq!(seq.len(), 5);
        assert_eq!(seq[4].id, 4);
    }
}
