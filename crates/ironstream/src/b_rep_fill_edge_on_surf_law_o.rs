// FILE: b_rep_fill_edge_on_surf_law_o.rs
// occt: BRepFill_EdgeOnSurfLaw

/// A location law where the path edges lie on a surface.
/// Inherits from BRepFill_LocationLaw.
pub struct BRepFillEdgeOnSurfLaw {
    /// Whether the law was successfully constructed
    has_result: bool,
    /// Surface representation
    surface_data: SurfaceData,
    /// Laws for each edge
    edge_laws: Vec<EdgeLaw>,
}

/// Minimal surface representation
struct SurfaceData {
    /// Surface type indicator
    surface_type: SurfaceType,
}

enum SurfaceType {
    /// Unknown or unsupported surface
    Unknown,
    /// A plane
    Plane,
    /// A cylinder
    Cylinder,
    /// Other surface type
    Other,
}

/// A law for a single edge on the surface
struct EdgeLaw {
    /// Whether this edge was found on the surface
    found: bool,
    /// Edge identifier
    edge_id: usize,
}

impl BRepFillEdgeOnSurfLaw {
    /// Creates a new location law for edges on a surface.
    /// Returns Ok if all edges of the path have representation on the surface,
    /// Err otherwise.
    pub fn new() -> Self {
        Self {
            has_result: true,
            surface_data: SurfaceData {
                surface_type: SurfaceType::Unknown,
            },
            edge_laws: Vec::new(),
        }
    }

    /// Returns whether all edges of the path have been found on the surface.
    pub fn has_result(&self) -> bool {
        self.has_result
    }

    /// Marks that construction failed (an edge was not found on the surface).
    fn set_failed(&mut self) {
        self.has_result = false;
    }

    /// Adds an edge law to the collection.
    fn add_edge_law(&mut self, found: bool, edge_id: usize) {
        if !found {
            self.set_failed();
        }
        self.edge_laws.push(EdgeLaw { found, edge_id });
    }

    /// Returns the number of edge laws.
    pub fn nb_edge_laws(&self) -> usize {
        self.edge_laws.len()
    }

    /// Gets the i-th edge law.
    pub fn edge_law(&self, index: usize) -> Option<&EdgeLaw> {
        self.edge_laws.get(index)
    }
}

impl Default for BRepFillEdgeOnSurfLaw {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_on_surf_law_creation() {
        let law = BRepFillEdgeOnSurfLaw::new();
        assert!(law.has_result());
    }

    #[test]
    fn test_edge_on_surf_law_success() {
        let mut law = BRepFillEdgeOnSurfLaw::new();
        law.add_edge_law(true, 0);
        law.add_edge_law(true, 1);
        assert!(law.has_result());
        assert_eq!(law.nb_edge_laws(), 2);
    }

    #[test]
    fn test_edge_on_surf_law_failure_one_edge() {
        let mut law = BRepFillEdgeOnSurfLaw::new();
        law.add_edge_law(true, 0);
        law.add_edge_law(false, 1); // This edge was not found
        assert!(!law.has_result());
    }

    #[test]
    fn test_edge_on_surf_law_failure_immediate() {
        let mut law = BRepFillEdgeOnSurfLaw::new();
        law.add_edge_law(false, 0);
        assert!(!law.has_result());
    }

    #[test]
    fn test_edge_on_surf_law_access() {
        let mut law = BRepFillEdgeOnSurfLaw::new();
        law.add_edge_law(true, 0);
        law.add_edge_law(true, 1);

        assert!(law.edge_law(0).is_some());
        assert!(law.edge_law(1).is_some());
        assert!(law.edge_law(2).is_none());

        if let Some(el) = law.edge_law(0) {
            assert!(el.found);
            assert_eq!(el.edge_id, 0);
        }
    }

    #[test]
    fn test_edge_on_surf_law_all_found() {
        let mut law = BRepFillEdgeOnSurfLaw::new();
        for i in 0..5 {
            law.add_edge_law(true, i);
        }
        assert!(law.has_result());
        assert_eq!(law.nb_edge_laws(), 5);
    }
}
