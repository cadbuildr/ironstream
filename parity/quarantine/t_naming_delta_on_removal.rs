// FILE: t_naming_delta_on_removal.rs
// occt: TNaming_DeltaOnRemoval

use crate::t_naming_delta_on_modification::{
    TNamingDeltaOnModification, TNamingEvolution, TNamingNamedShape, TopodsShape,
};

/// A delta on removal for undo/redo support.
/// Mirrors OCCT's TNaming_DeltaOnRemoval.
/// When an attribute is removed, this captures the delta using TNaming_DeltaOnModification.
#[derive(Clone, Debug)]
pub struct TNamingDeltaOnRemoval {
    delta: TNamingDeltaOnModification,
}

impl TNamingDeltaOnRemoval {
    /// Initializes a delta from a named shape that is being removed.
    /// The delta stores the state of the named shape at the time of removal.
    pub fn new(named_shape: &TNamingNamedShape) -> Self {
        let delta = TNamingDeltaOnModification::new(named_shape);
        TNamingDeltaOnRemoval { delta }
    }

    /// Applies the delta (reverses the removal by restoring the named shape).
    pub fn apply(&self) {
        self.delta.apply();
    }

    /// Returns the underlying delta on modification.
    pub fn delta(&self) -> &TNamingDeltaOnModification {
        &self.delta
    }

    /// Returns the evolution type of the removed attribute.
    pub fn evolution(&self) -> TNamingEvolution {
        self.delta.evolution()
    }

    /// Returns the old shapes stored in the delta.
    pub fn old_shapes(&self) -> &[TopodsShape] {
        self.delta.old_shapes()
    }

    /// Returns the new shapes stored in the delta.
    pub fn new_shapes(&self) -> &[TopodsShape] {
        self.delta.new_shapes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_on_removal_creation() {
        let mut ns = TNamingNamedShape::with_evolution(TNamingEvolution::Primitive);
        ns.add_shape(TopodsShape::new(), TopodsShape::with_id(1));

        let delta = TNamingDeltaOnRemoval::new(&ns);
        assert_eq!(delta.evolution(), TNamingEvolution::Primitive);
        assert_eq!(delta.new_shapes().len(), 1);
    }

    #[test]
    fn test_delta_on_removal_generated() {
        let mut ns = TNamingNamedShape::with_evolution(TNamingEvolution::Generated);
        ns.add_shape(TopodsShape::with_id(10), TopodsShape::with_id(20));
        ns.add_shape(TopodsShape::with_id(11), TopodsShape::with_id(21));

        let delta = TNamingDeltaOnRemoval::new(&ns);
        assert_eq!(delta.evolution(), TNamingEvolution::Generated);
        assert_eq!(delta.old_shapes().len(), 2);
        assert_eq!(delta.new_shapes().len(), 2);
        assert_eq!(delta.old_shapes()[0].id, 10);
        assert_eq!(delta.new_shapes()[0].id, 20);
    }

    #[test]
    fn test_delta_on_removal_delete() {
        let mut ns = TNamingNamedShape::with_evolution(TNamingEvolution::Delete);
        ns.add_shape(TopodsShape::with_id(5), TopodsShape::new());

        let delta = TNamingDeltaOnRemoval::new(&ns);
        assert_eq!(delta.evolution(), TNamingEvolution::Delete);
        assert_eq!(delta.old_shapes().len(), 1);
        assert_eq!(delta.new_shapes().len(), 0);
    }

    #[test]
    fn test_delta_on_removal_apply() {
        let mut ns = TNamingNamedShape::with_evolution(TNamingEvolution::Modified);
        ns.add_shape(TopodsShape::with_id(30), TopodsShape::with_id(40));

        let delta = TNamingDeltaOnRemoval::new(&ns);
        delta.apply(); // Should not panic
    }

    #[test]
    fn test_delta_on_removal_access_delta() {
        let mut ns = TNamingNamedShape::with_evolution(TNamingEvolution::Primitive);
        ns.add_shape(TopodsShape::new(), TopodsShape::with_id(100));

        let removal_delta = TNamingDeltaOnRemoval::new(&ns);
        let mod_delta = removal_delta.delta();
        assert_eq!(mod_delta.evolution(), TNamingEvolution::Primitive);
        assert_eq!(mod_delta.new_shapes()[0].id, 100);
    }

    #[test]
    fn test_delta_on_removal_multiple_shapes() {
        let mut ns = TNamingNamedShape::with_evolution(TNamingEvolution::Modified);
        for i in 0..5 {
            ns.add_shape(TopodsShape::with_id(i), TopodsShape::with_id(i + 100));
        }

        let delta = TNamingDeltaOnRemoval::new(&ns);
        assert_eq!(delta.old_shapes().len(), 5);
        assert_eq!(delta.new_shapes().len(), 5);
        for i in 0..5 {
            assert_eq!(delta.old_shapes()[i].id, i as u64);
            assert_eq!(delta.new_shapes()[i].id, (i + 100) as u64);
        }
    }
}
