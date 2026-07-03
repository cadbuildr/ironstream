// FILE: t_naming_delta_on_modification.rs
// occt: TNaming_DeltaOnModification

/// Enumeration for shape evolution types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TNamingEvolution {
    Primitive = 0,
    Generated = 1,
    Modified = 2,
    Delete = 3,
}

impl TNamingEvolution {
    pub fn to_string(&self) -> &'static str {
        match self {
            TNamingEvolution::Primitive => "PRIMITIVE",
            TNamingEvolution::Generated => "GENERATED",
            TNamingEvolution::Modified => "MODIFIED",
            TNamingEvolution::Delete => "DELETE",
        }
    }
}

/// Placeholder for a topological shape.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TopodsShape {
    id: u64,
}

impl TopodsShape {
    pub fn new() -> Self {
        TopodsShape { id: 0 }
    }

    pub fn with_id(id: u64) -> Self {
        TopodsShape { id }
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

/// A named shape attribute record.
#[derive(Clone, Debug)]
pub struct ShapeRecord {
    pub old_shape: TopodsShape,
    pub new_shape: TopodsShape,
}

impl ShapeRecord {
    pub fn new(old: TopodsShape, new: TopodsShape) -> Self {
        ShapeRecord {
            old_shape: old,
            new_shape: new,
        }
    }
}

/// A named shape in the topological naming framework.
/// Mirrors OCCT's TNaming_NamedShape (simplified).
#[derive(Clone, Debug)]
pub struct TNamingNamedShape {
    evolution: TNamingEvolution,
    shapes: Vec<ShapeRecord>,
}

impl TNamingNamedShape {
    pub fn new() -> Self {
        TNamingNamedShape {
            evolution: TNamingEvolution::Primitive,
            shapes: Vec::new(),
        }
    }

    pub fn with_evolution(evolution: TNamingEvolution) -> Self {
        TNamingNamedShape {
            evolution,
            shapes: Vec::new(),
        }
    }

    pub fn add_shape(&mut self, old: TopodsShape, new: TopodsShape) {
        self.shapes.push(ShapeRecord::new(old, new));
    }

    pub fn evolution(&self) -> TNamingEvolution {
        self.evolution
    }

    pub fn shapes(&self) -> &[ShapeRecord] {
        &self.shapes
    }

    pub fn nb_shapes(&self) -> usize {
        self.shapes.len()
    }
}

impl Default for TNamingNamedShape {
    fn default() -> Self {
        Self::new()
    }
}

/// A delta on modification for undo/redo support.
/// Mirrors OCCT's TNaming_DeltaOnModification.
#[derive(Clone, Debug)]
pub struct TNamingDeltaOnModification {
    evolution: TNamingEvolution,
    old_shapes: Vec<TopodsShape>,
    new_shapes: Vec<TopodsShape>,
}

impl TNamingDeltaOnModification {
    /// Initializes a delta from a named shape attribute.
    pub fn new(named_shape: &TNamingNamedShape) -> Self {
        let evolution = named_shape.evolution();
        let mut old_shapes = Vec::new();
        let mut new_shapes = Vec::new();

        match evolution {
            TNamingEvolution::Primitive => {
                // For PRIMITIVE, only new shapes are stored
                for shape_record in named_shape.shapes() {
                    new_shapes.push(shape_record.new_shape.clone());
                }
            }
            TNamingEvolution::Delete => {
                // For DELETE, only old shapes are stored
                for shape_record in named_shape.shapes() {
                    old_shapes.push(shape_record.old_shape.clone());
                }
            }
            _ => {
                // For GENERATED, MODIFIED, etc., both old and new are stored
                for shape_record in named_shape.shapes() {
                    old_shapes.push(shape_record.old_shape.clone());
                    new_shapes.push(shape_record.new_shape.clone());
                }
            }
        }

        TNamingDeltaOnModification {
            evolution,
            old_shapes,
            new_shapes,
        }
    }

    /// Returns the evolution type.
    pub fn evolution(&self) -> TNamingEvolution {
        self.evolution
    }

    /// Returns the old shapes.
    pub fn old_shapes(&self) -> &[TopodsShape] {
        &self.old_shapes
    }

    /// Returns the new shapes.
    pub fn new_shapes(&self) -> &[TopodsShape] {
        &self.new_shapes
    }

    /// Applies the delta (placeholder for full undo/redo logic).
    /// In a full implementation, this would restore the attribute to its previous state.
    pub fn apply(&self) {
        // Placeholder: would apply the delta to reverse a modification
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evolution_to_string() {
        assert_eq!(TNamingEvolution::Primitive.to_string(), "PRIMITIVE");
        assert_eq!(TNamingEvolution::Generated.to_string(), "GENERATED");
        assert_eq!(TNamingEvolution::Modified.to_string(), "MODIFIED");
        assert_eq!(TNamingEvolution::Delete.to_string(), "DELETE");
    }

    #[test]
    fn test_topods_shape() {
        let shape = TopodsShape::with_id(10);
        assert!(!shape.is_null());
        assert_eq!(shape.id, 10);

        let null_shape = TopodsShape::new();
        assert!(null_shape.is_null());
    }

    #[test]
    fn test_shape_record() {
        let old = TopodsShape::with_id(1);
        let new = TopodsShape::with_id(2);
        let record = ShapeRecord::new(old.clone(), new.clone());
        assert_eq!(record.old_shape, old);
        assert_eq!(record.new_shape, new);
    }

    #[test]
    fn test_named_shape_creation() {
        let ns = TNamingNamedShape::new();
        assert_eq!(ns.evolution(), TNamingEvolution::Primitive);
        assert_eq!(ns.nb_shapes(), 0);
    }

    #[test]
    fn test_named_shape_add_shapes() {
        let mut ns = TNamingNamedShape::with_evolution(TNamingEvolution::Generated);
        let old = TopodsShape::with_id(5);
        let new = TopodsShape::with_id(6);
        ns.add_shape(old.clone(), new.clone());
        assert_eq!(ns.nb_shapes(), 1);
        assert_eq!(ns.shapes()[0].old_shape, old);
        assert_eq!(ns.shapes()[0].new_shape, new);
    }

    #[test]
    fn test_delta_on_modification_primitive() {
        let mut ns = TNamingNamedShape::with_evolution(TNamingEvolution::Primitive);
        ns.add_shape(TopodsShape::new(), TopodsShape::with_id(1));
        ns.add_shape(TopodsShape::new(), TopodsShape::with_id(2));

        let delta = TNamingDeltaOnModification::new(&ns);
        assert_eq!(delta.evolution(), TNamingEvolution::Primitive);
        assert_eq!(delta.old_shapes().len(), 0);
        assert_eq!(delta.new_shapes().len(), 2);
    }

    #[test]
    fn test_delta_on_modification_delete() {
        let mut ns = TNamingNamedShape::with_evolution(TNamingEvolution::Delete);
        ns.add_shape(TopodsShape::with_id(10), TopodsShape::new());
        ns.add_shape(TopodsShape::with_id(11), TopodsShape::new());

        let delta = TNamingDeltaOnModification::new(&ns);
        assert_eq!(delta.evolution(), TNamingEvolution::Delete);
        assert_eq!(delta.old_shapes().len(), 2);
        assert_eq!(delta.new_shapes().len(), 0);
    }

    #[test]
    fn test_delta_on_modification_generated() {
        let mut ns = TNamingNamedShape::with_evolution(TNamingEvolution::Generated);
        ns.add_shape(TopodsShape::with_id(3), TopodsShape::with_id(4));
        ns.add_shape(TopodsShape::with_id(5), TopodsShape::with_id(6));

        let delta = TNamingDeltaOnModification::new(&ns);
        assert_eq!(delta.evolution(), TNamingEvolution::Generated);
        assert_eq!(delta.old_shapes().len(), 2);
        assert_eq!(delta.new_shapes().len(), 2);
        assert_eq!(delta.old_shapes()[0].id, 3);
        assert_eq!(delta.new_shapes()[0].id, 4);
    }

    #[test]
    fn test_delta_apply() {
        let mut ns = TNamingNamedShape::with_evolution(TNamingEvolution::Modified);
        ns.add_shape(TopodsShape::with_id(7), TopodsShape::with_id(8));
        let delta = TNamingDeltaOnModification::new(&ns);
        delta.apply(); // Should not panic
    }
}
