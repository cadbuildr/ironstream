// FILE: iges_select_spline_to_b_spline.rs
// occt: IGESSelect_SplineToBSpline

//! Transformer that converts IGES Spline Curves (type 112) and Surfaces (type 126)
//! to B-Spline Curves (type 114) and Surfaces (type 128).
//!
//! The transformer provides an option to upgrade continuity to C1 or C2 during conversion.

use std::collections::HashMap;

/// A handle-like wrapper for transient objects
#[derive(Clone)]
struct TransientHandle {
    id: usize,
}

/// Interface protocol type stub
pub struct InterfaceProtocol;

/// Copy control for entity mapping during transformation
pub struct InterfaceCopyControl {
    mapping: HashMap<usize, usize>,
}

impl InterfaceCopyControl {
    fn new() -> Self {
        InterfaceCopyControl {
            mapping: HashMap::new(),
        }
    }

    fn insert(&mut self, from_id: usize, to_id: usize) {
        self.mapping.insert(from_id, to_id);
    }

    fn search(&self, from_id: usize) -> Option<usize> {
        self.mapping.get(&from_id).copied()
    }

    fn nullify(&mut self) {
        self.mapping.clear();
    }

    fn is_null(&self) -> bool {
        self.mapping.is_empty()
    }
}

/// Represents an entity in the IGES model
pub struct IGESEntity {
    type_number: i32,
    id: usize,
}

impl IGESEntity {
    pub fn new(type_number: i32, id: usize) -> Self {
        IGESEntity { type_number, id }
    }

    /// Returns the IGES type number (e.g., 112 for Spline, 114 for B-Spline Curve)
    pub fn type_number(&self) -> i32 {
        self.type_number
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Interface graph representing all entities and their relationships
pub struct InterfaceGraph {
    entities: Vec<IGESEntity>,
}

impl InterfaceGraph {
    pub fn new(entities: Vec<IGESEntity>) -> Self {
        InterfaceGraph { entities }
    }

    pub fn size(&self) -> usize {
        self.entities.len()
    }

    pub fn entity(&self, index: usize) -> Option<&IGESEntity> {
        if index > 0 && index <= self.entities.len() {
            Some(&self.entities[index - 1])
        } else {
            None
        }
    }
}

/// Interface check iterator for collecting transformation errors
pub struct InterfaceCheckIterator {
    failures: Vec<String>,
}

impl InterfaceCheckIterator {
    pub fn new() -> Self {
        InterfaceCheckIterator {
            failures: Vec::new(),
        }
    }

    pub fn add_fail(&mut self, message: &str) {
        self.failures.push(message.to_string());
    }

    pub fn failures(&self) -> &[String] {
        &self.failures
    }
}

/// The transformed interface model
pub struct InterfaceModel {
    entities: Vec<IGESEntity>,
}

impl InterfaceModel {
    pub fn new(entities: Vec<IGESEntity>) -> Self {
        InterfaceModel { entities }
    }

    pub fn nullify(&mut self) {
        self.entities.clear();
    }

    pub fn is_null(&self) -> bool {
        self.entities.is_empty()
    }
}

/// Transformer that converts Spline curves/surfaces to B-Spline forms
pub struct IGESSelectSplineToBSpline {
    try_c2: bool,
    found: bool,
    map: Option<InterfaceCopyControl>,
}

impl IGESSelectSplineToBSpline {
    /// Creates a transformer with option to upgrade to C2 continuity
    pub fn new(try_c2: bool) -> Self {
        IGESSelectSplineToBSpline {
            try_c2,
            found: false,
            map: None,
        }
    }

    /// Returns the option for upgrading to C2 continuity
    pub fn option_try_c2(&self) -> bool {
        self.try_c2
    }

    /// Performs the transformation of Spline entities to B-Spline entities
    ///
    /// Returns true if transformation succeeded (or if no splines found),
    /// false if errors occurred.
    pub fn perform(
        &mut self,
        graph: &InterfaceGraph,
        _protocol: &InterfaceProtocol,
        checks: &mut InterfaceCheckIterator,
        new_mod: &mut InterfaceModel,
    ) -> bool {
        let nbe = graph.size();
        self.found = false;
        self.map = Some(InterfaceCopyControl::new());

        // First pass: scan for spline entities (type 112 or 126)
        for i in 1..=nbe {
            if let Some(ent) = graph.entity(i) {
                let it = ent.type_number();
                if it == 112 || it == 126 {
                    self.found = true;
                    // Debug output
                    #[cfg(test)]
                    {
                        let entity_type = if it == 112 { "Curve" } else { "Surface" };
                        eprintln!(
                            "IGESSelect_SplineToBSpline : n0.{}, {} to convert",
                            i, entity_type
                        );
                    }
                }
            }
        }

        new_mod.nullify();
        if !self.found {
            return true;
        }

        // Conversion not yet fully implemented in OCCT source
        checks.add_fail("IGESSelect_SplineToBSpline : not yet implemented");
        false
    }

    /// Returns the transformed entity corresponding to an input entity
    ///
    /// If no splines were found, returns the original entity unchanged.
    /// Otherwise returns the B-Spline counterpart from the mapping.
    pub fn updated(&self, entfrom: usize, checks: &mut InterfaceCheckIterator) -> Option<usize> {
        if !self.found {
            return Some(entfrom);
        }

        match &self.map {
            Some(map) => map.search(entfrom).or_else(|| {
                checks.add_fail("Entity not found in transformation mapping");
                None
            }),
            None => {
                checks.add_fail("No mapping available after transformation");
                None
            }
        }
    }

    /// Returns a description label for the transformation
    pub fn label(&self) -> String {
        if self.try_c2 {
            "Convert Spline Forms to BSpline, trying to recover C1-C2 continuity".to_string()
        } else {
            "Convert Spline Forms to BSpline".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let transformer = IGESSelectSplineToBSpline::new(true);
        assert!(transformer.option_try_c2());
        assert!(!transformer.found);
    }

    #[test]
    fn test_creation_no_c2() {
        let transformer = IGESSelectSplineToBSpline::new(false);
        assert!(!transformer.option_try_c2());
    }

    #[test]
    fn test_label_with_c2() {
        let transformer = IGESSelectSplineToBSpline::new(true);
        let label = transformer.label();
        assert!(label.contains("C1-C2"));
    }

    #[test]
    fn test_label_without_c2() {
        let transformer = IGESSelectSplineToBSpline::new(false);
        let label = transformer.label();
        assert_eq!(label, "Convert Spline Forms to BSpline");
    }

    #[test]
    fn test_perform_empty_graph() {
        let mut transformer = IGESSelectSplineToBSpline::new(false);
        let graph = InterfaceGraph::new(vec![]);
        let mut checks = InterfaceCheckIterator::new();
        let mut new_mod = InterfaceModel::new(vec![]);
        let protocol = InterfaceProtocol;

        let result = transformer.perform(&graph, &protocol, &mut checks, &mut new_mod);

        assert!(result);
        assert!(!transformer.found);
        assert!(checks.failures().is_empty());
    }

    #[test]
    fn test_perform_with_non_spline_entities() {
        let mut transformer = IGESSelectSplineToBSpline::new(false);
        let entities = vec![
            IGESEntity::new(100, 1),
            IGESEntity::new(200, 2),
            IGESEntity::new(300, 3),
        ];
        let graph = InterfaceGraph::new(entities);
        let mut checks = InterfaceCheckIterator::new();
        let mut new_mod = InterfaceModel::new(vec![]);
        let protocol = InterfaceProtocol;

        let result = transformer.perform(&graph, &protocol, &mut checks, &mut new_mod);

        assert!(result);
        assert!(!transformer.found);
    }

    #[test]
    fn test_perform_with_spline_curve() {
        let mut transformer = IGESSelectSplineToBSpline::new(false);
        let entities = vec![
            IGESEntity::new(100, 1),
            IGESEntity::new(112, 2), // Spline curve
            IGESEntity::new(200, 3),
        ];
        let graph = InterfaceGraph::new(entities);
        let mut checks = InterfaceCheckIterator::new();
        let mut new_mod = InterfaceModel::new(vec![]);
        let protocol = InterfaceProtocol;

        let result = transformer.perform(&graph, &protocol, &mut checks, &mut new_mod);

        assert!(!result);
        assert!(transformer.found);
        assert_eq!(checks.failures().len(), 1);
        assert!(checks.failures()[0].contains("not yet implemented"));
    }

    #[test]
    fn test_perform_with_spline_surface() {
        let mut transformer = IGESSelectSplineToBSpline::new(true);
        let entities = vec![IGESEntity::new(126, 1)]; // Spline surface
        let graph = InterfaceGraph::new(entities);
        let mut checks = InterfaceCheckIterator::new();
        let mut new_mod = InterfaceModel::new(vec![]);
        let protocol = InterfaceProtocol;

        let result = transformer.perform(&graph, &protocol, &mut checks, &mut new_mod);

        assert!(!result);
        assert!(transformer.found);
    }

    #[test]
    fn test_updated_no_spline_found() {
        let transformer = IGESSelectSplineToBSpline::new(false);
        let mut checks = InterfaceCheckIterator::new();

        let result = transformer.updated(42, &mut checks);

        assert_eq!(result, Some(42));
        assert!(checks.failures().is_empty());
    }

    #[test]
    fn test_updated_with_spline_no_mapping() {
        let mut transformer = IGESSelectSplineToBSpline::new(false);
        transformer.found = true;
        transformer.map = Some(InterfaceCopyControl::new());

        let mut checks = InterfaceCheckIterator::new();
        let result = transformer.updated(42, &mut checks);

        assert_eq!(result, None);
        assert!(!checks.failures().is_empty());
    }

    #[test]
    fn test_interface_graph_iteration() {
        let entities = vec![
            IGESEntity::new(100, 1),
            IGESEntity::new(112, 2),
            IGESEntity::new(126, 3),
        ];
        let graph = InterfaceGraph::new(entities);

        assert_eq!(graph.size(), 3);
        assert_eq!(graph.entity(1).map(|e| e.type_number()), Some(100));
        assert_eq!(graph.entity(2).map(|e| e.type_number()), Some(112));
        assert_eq!(graph.entity(3).map(|e| e.type_number()), Some(126));
        assert!(graph.entity(4).is_none());
    }

    #[test]
    fn test_copy_control_mapping() {
        let mut map = InterfaceCopyControl::new();
        assert!(map.is_null());

        map.insert(10, 20);
        map.insert(30, 40);
        assert!(!map.is_null());

        assert_eq!(map.search(10), Some(20));
        assert_eq!(map.search(30), Some(40));
        assert_eq!(map.search(50), None);

        map.nullify();
        assert!(map.is_null());
    }
}
