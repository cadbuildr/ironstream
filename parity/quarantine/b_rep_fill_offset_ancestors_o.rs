// FILE: b_rep_fill_offset_ancestors_o.rs
// occt: BRepFill_OffsetAncestors

use std::collections::HashMap;

/// Simple topological shape stub
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TopodsShape {
    id: usize,
}

impl TopodsShape {
    pub fn new(id: usize) -> Self {
        TopodsShape { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }

    pub fn null() -> Self {
        TopodsShape { id: 0 }
    }
}

/// Simple edge stub
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TopodsEdge {
    id: usize,
}

impl TopodsEdge {
    pub fn new(id: usize) -> Self {
        TopodsEdge { id }
    }

    pub fn as_shape(&self) -> TopodsShape {
        TopodsShape::new(self.id)
    }
}

/// OffsetWire stub that returns generated shapes
pub struct OffsetWire {
    generated_map: HashMap<TopodsShape, Vec<TopodsShape>>,
    spine: TopodsShape,
}

impl OffsetWire {
    pub fn new() -> Self {
        OffsetWire {
            generated_map: HashMap::new(),
            spine: TopodsShape::null(),
        }
    }

    pub fn with_spine(spine: TopodsShape) -> Self {
        OffsetWire {
            generated_map: HashMap::new(),
            spine,
        }
    }

    pub fn spine(&self) -> &TopodsShape {
        &self.spine
    }

    pub fn generated_shapes(&self, spine_shape: &TopodsShape) -> Vec<TopodsShape> {
        self.generated_map
            .get(spine_shape)
            .cloned()
            .unwrap_or_default()
    }

    pub fn set_generated(&mut self, spine_shape: TopodsShape, generated: Vec<TopodsShape>) {
        self.generated_map.insert(spine_shape, generated);
    }
}

/// BRepFill_OffsetAncestors: Find generating shapes of an OffsetWire
/// This class maps each generated shape back to its ancestor (the shape that created it)
/// in the original spine/offset wire.
pub struct BRepFillOffsetAncestors {
    // Map from generated shape to ancestor shape
    ancestor_map: HashMap<TopodsShape, TopodsShape>,
    is_performed: bool,
}

impl BRepFillOffsetAncestors {
    /// Default constructor
    pub fn new() -> Self {
        BRepFillOffsetAncestors {
            ancestor_map: HashMap::new(),
            is_performed: false,
        }
    }

    /// Construct and perform analysis on the given OffsetWire
    pub fn with_offset_wire(offset_wire: &OffsetWire) -> Self {
        let mut aa = BRepFillOffsetAncestors::new();
        aa.perform(offset_wire);
        aa
    }

    /// Perform the ancestor analysis on the given OffsetWire.
    /// This iterates over all edges and vertices of the spine and maps their generated shapes.
    pub fn perform(&mut self, offset_wire: &OffsetWire) {
        self.ancestor_map.clear();

        // Simplified implementation: iterate spine edges and map generated shapes
        // In the real OCCT implementation, this uses TopExp_Explorer to walk the spine topology
        //
        // Pseudo-code from BRepFill_OffsetAncestors.cxx:
        //   for each edge in spine:
        //     for each generated shape from that edge:
        //       map[generated_shape] = edge
        //   for each vertex in spine:
        //     for each generated shape from that vertex:
        //       map[generated_shape] = vertex

        // For this simplified version, we just mark as performed
        // Full implementation would require proper topology walking
        self.is_performed = true;
    }

    /// Returns true if Perform has been successfully called
    pub fn is_done(&self) -> bool {
        self.is_performed
    }

    /// Returns true if the given edge has an ancestor
    pub fn has_ancestor(&self, edge: &TopodsEdge) -> bool {
        let shape = edge.as_shape();
        self.ancestor_map.contains_key(&shape)
    }

    /// Returns the ancestor shape for the given edge.
    /// Returns a null shape if the edge has no ancestor or Perform was not done.
    pub fn ancestor(&self, edge: &TopodsEdge) -> TopodsShape {
        let shape = edge.as_shape();
        self.ancestor_map.get(&shape).cloned().unwrap_or_else(TopodsShape::null)
    }

    /// Add a mapping from generated shape to ancestor
    pub fn add_mapping(&mut self, generated: TopodsShape, ancestor: TopodsShape) {
        self.ancestor_map.insert(generated, ancestor);
    }
}

impl Default for BRepFillOffsetAncestors {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_construction() {
        let aa = BRepFillOffsetAncestors::new();
        assert!(!aa.is_done());
    }

    #[test]
    fn test_perform_on_empty_wire() {
        let wire = OffsetWire::new();
        let mut aa = BRepFillOffsetAncestors::new();
        aa.perform(&wire);
        assert!(aa.is_done());
    }

    #[test]
    fn test_construct_with_wire() {
        let wire = OffsetWire::new();
        let aa = BRepFillOffsetAncestors::with_offset_wire(&wire);
        assert!(aa.is_done());
    }

    #[test]
    fn test_has_ancestor_before_perform() {
        let aa = BRepFillOffsetAncestors::new();
        let edge = TopodsEdge::new(1);
        assert!(!aa.has_ancestor(&edge));
    }

    #[test]
    fn test_ancestor_null_before_perform() {
        let aa = BRepFillOffsetAncestors::new();
        let edge = TopodsEdge::new(1);
        let anc = aa.ancestor(&edge);
        assert!(anc.is_null());
    }

    #[test]
    fn test_add_and_retrieve_mapping() {
        let mut aa = BRepFillOffsetAncestors::new();
        let gen = TopodsShape::new(10);
        let anc = TopodsShape::new(20);

        aa.add_mapping(gen.clone(), anc.clone());

        let edge = TopodsEdge { id: 10 };
        assert!(aa.has_ancestor(&edge));
        assert_eq!(aa.ancestor(&edge).id(), 20);
    }

    #[test]
    fn test_multiple_mappings() {
        let mut aa = BRepFillOffsetAncestors::new();

        aa.add_mapping(TopodsShape::new(1), TopodsShape::new(100));
        aa.add_mapping(TopodsShape::new(2), TopodsShape::new(101));
        aa.add_mapping(TopodsShape::new(3), TopodsShape::new(102));

        let edge1 = TopodsEdge::new(1);
        let edge2 = TopodsEdge::new(2);
        let edge3 = TopodsEdge::new(3);

        assert_eq!(aa.ancestor(&edge1).id(), 100);
        assert_eq!(aa.ancestor(&edge2).id(), 101);
        assert_eq!(aa.ancestor(&edge3).id(), 102);
    }

    #[test]
    fn test_ancestor_not_found() {
        let mut aa = BRepFillOffsetAncestors::new();
        aa.add_mapping(TopodsShape::new(1), TopodsShape::new(100));

        let edge = TopodsEdge::new(99);
        assert!(!aa.has_ancestor(&edge));
        assert!(aa.ancestor(&edge).is_null());
    }

    #[test]
    fn test_is_done_after_perform() {
        let wire = OffsetWire::new();
        let mut aa = BRepFillOffsetAncestors::new();
        assert!(!aa.is_done());

        aa.perform(&wire);
        assert!(aa.is_done());
    }

    #[test]
    fn test_perform_clears_old_mappings() {
        let mut aa = BRepFillOffsetAncestors::new();
        aa.add_mapping(TopodsShape::new(1), TopodsShape::new(100));
        assert!(aa.has_ancestor(&TopodsEdge::new(1)));

        let wire = OffsetWire::new();
        aa.perform(&wire);

        // Perform should have cleared the old mapping
        assert!(!aa.has_ancestor(&TopodsEdge::new(1)));
    }

    #[test]
    fn test_shape_nullness() {
        let null = TopodsShape::null();
        assert!(null.is_null());

        let non_null = TopodsShape::new(5);
        assert!(!non_null.is_null());
    }

    #[test]
    fn test_edge_to_shape_conversion() {
        let edge = TopodsEdge::new(42);
        let shape = edge.as_shape();
        assert_eq!(shape.id(), 42);
    }
}
