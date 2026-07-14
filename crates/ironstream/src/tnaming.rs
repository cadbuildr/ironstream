// FILE: tnaming.rs
// occt: TNaming_Builder, TNaming_NamedShape, TNaming_Evolution
//       TNaming_Iterator, TNaming_Selector

/// Evolution status of a named shape.
/// occt: TNaming_Evolution
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TNamingEvolution {
    #[default]
    Primitive,      // newly created shape
    Generated,      // generated from another
    Modify,         // modified version of another
    Delete,         // shape was deleted
    Replace,        // replaces another shape
    Selected,       // selection-based reference
}

impl TNamingEvolution {
    pub fn is_alive(&self) -> bool {
        !matches!(self, Self::Delete)
    }
}

/// A single shape record (old → new) for one evolution step.
#[derive(Clone, Debug)]
pub struct TNamingShapeRecord {
    pub old_shape_id: Option<u32>,
    pub new_shape_id: Option<u32>,
    pub evolution: TNamingEvolution,
}

impl TNamingShapeRecord {
    pub fn primitive(shape_id: u32) -> Self {
        Self { old_shape_id: None, new_shape_id: Some(shape_id), evolution: TNamingEvolution::Primitive }
    }

    pub fn modified(old_id: u32, new_id: u32) -> Self {
        Self { old_shape_id: Some(old_id), new_shape_id: Some(new_id), evolution: TNamingEvolution::Modify }
    }

    pub fn deleted(shape_id: u32) -> Self {
        Self { old_shape_id: Some(shape_id), new_shape_id: None, evolution: TNamingEvolution::Delete }
    }

    pub fn generated(from_id: u32, new_id: u32) -> Self {
        Self { old_shape_id: Some(from_id), new_shape_id: Some(new_id), evolution: TNamingEvolution::Generated }
    }
}

/// Named shape attribute on an OCAF label.
/// occt: TNaming_NamedShape
#[derive(Clone, Debug, Default)]
pub struct TNamingNamedShape {
    pub label_id: u32,
    pub records: Vec<TNamingShapeRecord>,
    pub evolution: TNamingEvolution,
}

impl TNamingNamedShape {
    pub fn new(label_id: u32) -> Self {
        Self { label_id, records: Vec::new(), evolution: TNamingEvolution::Primitive }
    }

    pub fn set_evolution(&mut self, e: TNamingEvolution) { self.evolution = e; }

    pub fn nb_shapes(&self) -> usize { self.records.len() }

    /// Latest shape (new_shape_id of the last record).
    pub fn current_shape(&self) -> Option<u32> {
        self.records.last().and_then(|r| r.new_shape_id)
    }

    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}

/// Builder for named-shape records.
/// occt: TNaming_Builder
#[derive(Clone, Debug)]
pub struct TNamingBuilder {
    pub named_shape: TNamingNamedShape,
}

impl TNamingBuilder {
    pub fn new(label_id: u32) -> Self {
        Self { named_shape: TNamingNamedShape::new(label_id) }
    }

    /// Record a new generated primitive shape.
    pub fn generated(&mut self, from_shape: u32, new_shape: u32) {
        self.named_shape.records.push(TNamingShapeRecord::generated(from_shape, new_shape));
        self.named_shape.evolution = TNamingEvolution::Generated;
    }

    /// Record a modification (old → new).
    pub fn modify(&mut self, old_shape: u32, new_shape: u32) {
        self.named_shape.records.push(TNamingShapeRecord::modified(old_shape, new_shape));
        self.named_shape.evolution = TNamingEvolution::Modify;
    }

    /// Record a deletion.
    pub fn delete(&mut self, shape_id: u32) {
        self.named_shape.records.push(TNamingShapeRecord::deleted(shape_id));
        self.named_shape.evolution = TNamingEvolution::Delete;
    }

    /// Record a new primitive shape (no predecessor).
    pub fn primitive(&mut self, shape_id: u32) {
        self.named_shape.records.push(TNamingShapeRecord::primitive(shape_id));
        self.named_shape.evolution = TNamingEvolution::Primitive;
    }

    /// Record a selected shape.
    pub fn select(&mut self, shape_id: u32, in_context_id: u32) {
        self.named_shape.records.push(TNamingShapeRecord {
            old_shape_id: Some(in_context_id),
            new_shape_id: Some(shape_id),
            evolution: TNamingEvolution::Selected,
        });
        self.named_shape.evolution = TNamingEvolution::Selected;
    }

    pub fn named_shape(&self) -> &TNamingNamedShape { &self.named_shape }
    pub fn into_named_shape(self) -> TNamingNamedShape { self.named_shape }
}

/// Iterator over (old, new) shape pairs in a named shape.
/// occt: TNaming_Iterator
#[derive(Clone, Debug)]
pub struct TNamingIterator<'a> {
    records: &'a [TNamingShapeRecord],
    cursor: usize,
}

impl<'a> TNamingIterator<'a> {
    pub fn new(ns: &'a TNamingNamedShape) -> Self {
        Self { records: &ns.records, cursor: 0 }
    }

    pub fn more(&self) -> bool { self.cursor < self.records.len() }
    pub fn next(&mut self) { self.cursor += 1; }

    pub fn old_shape(&self) -> Option<u32> {
        self.records.get(self.cursor).and_then(|r| r.old_shape_id)
    }

    pub fn new_shape(&self) -> Option<u32> {
        self.records.get(self.cursor).and_then(|r| r.new_shape_id)
    }

    pub fn evolution(&self) -> Option<TNamingEvolution> {
        self.records.get(self.cursor).map(|r| r.evolution)
    }
}

/// Registry: maps label_id → TNamingNamedShape.
#[derive(Clone, Debug, Default)]
pub struct TNamingRegistry {
    pub entries: Vec<TNamingNamedShape>,
}

impl TNamingRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn insert(&mut self, ns: TNamingNamedShape) {
        if let Some(e) = self.entries.iter_mut().find(|e| e.label_id == ns.label_id) {
            *e = ns;
        } else {
            self.entries.push(ns);
        }
    }

    pub fn find(&self, label_id: u32) -> Option<&TNamingNamedShape> {
        self.entries.iter().find(|e| e.label_id == label_id)
    }

    pub fn nb_named_shapes(&self) -> usize { self.entries.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_primitive() {
        let mut b = TNamingBuilder::new(1);
        b.primitive(42);
        let ns = b.into_named_shape();
        assert_eq!(ns.current_shape(), Some(42));
        assert_eq!(ns.evolution, TNamingEvolution::Primitive);
    }

    #[test]
    fn builder_modify() {
        let mut b = TNamingBuilder::new(2);
        b.modify(10, 20);
        let ns = b.named_shape();
        assert_eq!(ns.records[0].old_shape_id, Some(10));
        assert_eq!(ns.records[0].new_shape_id, Some(20));
        assert_eq!(ns.evolution, TNamingEvolution::Modify);
    }

    #[test]
    fn builder_delete() {
        let mut b = TNamingBuilder::new(3);
        b.delete(5);
        let ns = b.into_named_shape();
        assert_eq!(ns.evolution, TNamingEvolution::Delete);
        assert!(!ns.evolution.is_alive());
    }

    #[test]
    fn iterator_over_records() {
        let mut b = TNamingBuilder::new(4);
        b.generated(1, 10);
        b.generated(2, 11);
        let ns = b.into_named_shape();
        let mut it = TNamingIterator::new(&ns);
        let mut count = 0;
        while it.more() {
            assert!(it.new_shape().is_some());
            it.next();
            count += 1;
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn registry_insert_find() {
        let mut reg = TNamingRegistry::new();
        let mut b = TNamingBuilder::new(10);
        b.primitive(99);
        reg.insert(b.into_named_shape());
        assert_eq!(reg.nb_named_shapes(), 1);
        let ns = reg.find(10).unwrap();
        assert_eq!(ns.current_shape(), Some(99));
    }

    #[test]
    fn evolution_is_alive() {
        assert!(TNamingEvolution::Primitive.is_alive());
        assert!(TNamingEvolution::Generated.is_alive());
        assert!(!TNamingEvolution::Delete.is_alive());
    }
}
