// FILE: mesh_vs_sequence_of_prs_builder.rs
// occt: MeshVS_SequenceOfPrsBuilder

use std::rc::Rc;
use std::cell::RefCell;

/// MeshVS_PrsBuilder is a builder for presentation objects.
#[derive(Clone, Debug)]
pub struct MeshVsPrsBuilder {
    id: i32,
    name: String,
    priority: i32,
}

impl MeshVsPrsBuilder {
    pub fn new(id: i32, name: String, priority: i32) -> Self {
        MeshVsPrsBuilder { id, name, priority }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn priority(&self) -> i32 {
        self.priority
    }

    pub fn set_priority(&mut self, priority: i32) {
        self.priority = priority;
    }
}

/// A handle/reference-counted wrapper for MeshVS_PrsBuilder.
pub type MeshVsPrsBuilderHandle = Rc<RefCell<MeshVsPrsBuilder>>;

/// Deprecated typedef alias for backward compatibility.
/// Original OCCT: `NCollection_Sequence<opencascade::handle<MeshVS_PrsBuilder>>`
pub type MeshVsSequenceOfPrsBuilder = Vec<MeshVsPrsBuilderHandle>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prs_builder_creation() {
        let builder = MeshVsPrsBuilder::new(1, "TestBuilder".to_string(), 5);
        assert_eq!(builder.id(), 1);
        assert_eq!(builder.name(), "TestBuilder");
        assert_eq!(builder.priority(), 5);
    }

    #[test]
    fn test_prs_builder_set_priority() {
        let mut builder = MeshVsPrsBuilder::new(1, "TestBuilder".to_string(), 5);
        assert_eq!(builder.priority(), 5);

        builder.set_priority(10);
        assert_eq!(builder.priority(), 10);

        builder.set_priority(0);
        assert_eq!(builder.priority(), 0);
    }

    #[test]
    fn test_sequence_creation() {
        let sequence: MeshVsSequenceOfPrsBuilder = Vec::new();
        assert!(sequence.is_empty());
        assert_eq!(sequence.len(), 0);
    }

    #[test]
    fn test_sequence_push() {
        let mut sequence: MeshVsSequenceOfPrsBuilder = Vec::new();

        let builder1 = Rc::new(RefCell::new(MeshVsPrsBuilder::new(
            1,
            "Builder1".to_string(),
            5,
        )));
        let builder2 = Rc::new(RefCell::new(MeshVsPrsBuilder::new(
            2,
            "Builder2".to_string(),
            10,
        )));

        sequence.push(builder1.clone());
        sequence.push(builder2.clone());

        assert_eq!(sequence.len(), 2);
        assert_eq!(sequence[0].borrow().id(), 1);
        assert_eq!(sequence[1].borrow().id(), 2);
    }

    #[test]
    fn test_sequence_access() {
        let mut sequence: MeshVsSequenceOfPrsBuilder = Vec::new();

        let builder = Rc::new(RefCell::new(MeshVsPrsBuilder::new(
            42,
            "TestBuilder".to_string(),
            3,
        )));
        sequence.push(builder.clone());

        let retrieved = sequence.get(0).unwrap();
        assert_eq!(retrieved.borrow().id(), 42);
        assert_eq!(retrieved.borrow().name(), "TestBuilder");
    }

    #[test]
    fn test_sequence_iteration() {
        let mut sequence: MeshVsSequenceOfPrsBuilder = Vec::new();

        for i in 1..=5 {
            let builder = Rc::new(RefCell::new(MeshVsPrsBuilder::new(
                i,
                format!("Builder{}", i),
                i * 10,
            )));
            sequence.push(builder);
        }

        assert_eq!(sequence.len(), 5);

        let mut ids = Vec::new();
        for handle in &sequence {
            ids.push(handle.borrow().id());
        }
        assert_eq!(ids, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sequence_remove() {
        let mut sequence: MeshVsSequenceOfPrsBuilder = Vec::new();

        let builder1 = Rc::new(RefCell::new(MeshVsPrsBuilder::new(1, "B1".to_string(), 1)));
        let builder2 = Rc::new(RefCell::new(MeshVsPrsBuilder::new(2, "B2".to_string(), 2)));
        let builder3 = Rc::new(RefCell::new(MeshVsPrsBuilder::new(3, "B3".to_string(), 3)));

        sequence.push(builder1.clone());
        sequence.push(builder2.clone());
        sequence.push(builder3.clone());

        assert_eq!(sequence.len(), 3);
        sequence.remove(1);
        assert_eq!(sequence.len(), 2);
        assert_eq!(sequence[0].borrow().id(), 1);
        assert_eq!(sequence[1].borrow().id(), 3);
    }
}
