// FILE: b_rep_polygon_on_closed_triangulation.rs
// occt: BRep_PolygonOnClosedTriangulation

/// Polygon defined on a closed triangulation.
pub struct BRepPolygonOnClosedTriangulation {
    triangulation_id: u32,
    vertices_indices_1: Vec<u32>,
    vertices_indices_2: Vec<u32>,
    closed: bool,
}

impl BRepPolygonOnClosedTriangulation {
    /// Create a new polygon on closed triangulation.
    pub fn new(
        triangulation_id: u32,
        vertices_indices_1: Vec<u32>,
        vertices_indices_2: Vec<u32>,
    ) -> Self {
        Self {
            triangulation_id,
            vertices_indices_1,
            vertices_indices_2,
            closed: true,
        }
    }

    /// Get the triangulation id.
    pub fn triangulation_id(&self) -> u32 {
        self.triangulation_id
    }

    /// Get vertex indices for first copy.
    pub fn vertices_indices_1(&self) -> &[u32] {
        &self.vertices_indices_1
    }

    /// Get vertex indices for second copy.
    pub fn vertices_indices_2(&self) -> &[u32] {
        &self.vertices_indices_2
    }

    /// Check if triangulation is closed.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Get number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.vertices_indices_1.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let poly = BRepPolygonOnClosedTriangulation::new(1, vec![0, 1, 2], vec![3, 4, 5]);

        assert_eq!(poly.triangulation_id(), 1);
        assert!(poly.is_closed());
        assert_eq!(poly.vertex_count(), 3);
        assert_eq!(poly.vertices_indices_1(), &[0, 1, 2]);
        assert_eq!(poly.vertices_indices_2(), &[3, 4, 5]);
    }
}
