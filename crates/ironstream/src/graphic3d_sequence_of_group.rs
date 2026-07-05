// FILE: graphic3d_sequence_of_group.rs
// occt: Graphic3d_SequenceOfGroup

//! Deprecated: Use Vec<Group> directly.
//! Sequence of graphic groups.

#[derive(Clone, Debug)]
pub struct Group {
    pub id: usize,
}

impl Group {
    pub fn new(id: usize) -> Self {
        Group { id }
    }
}

pub type Graphic3dSequenceOfGroup = Vec<Group>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_creation() {
        let mut seq: Graphic3dSequenceOfGroup = Vec::new();
        seq.push(Group::new(1));

        assert_eq!(seq.len(), 1);
        assert_eq!(seq[0].id, 1);
    }

    #[test]
    fn test_sequence_operations() {
        let seq = vec![Group::new(10), Group::new(20), Group::new(30)];

        assert_eq!(seq.len(), 3);
        assert_eq!(seq[1].id, 20);
    }

    #[test]
    fn test_sequence_iteration() {
        let seq = vec![Group::new(1), Group::new(2), Group::new(3)];

        let ids: Vec<usize> = seq.iter().map(|g| g.id).collect();
        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[test]
    fn test_sequence_append() {
        let mut seq: Graphic3dSequenceOfGroup = Vec::new();
        for i in 0..5 {
            seq.push(Group::new(i));
        }

        assert_eq!(seq.len(), 5);
        assert_eq!(seq[4].id, 4);
    }
}
