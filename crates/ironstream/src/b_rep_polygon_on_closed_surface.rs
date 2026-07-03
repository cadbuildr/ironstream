// FILE: b_rep_polygon_on_closed_surface.rs
// occt: BRep_PolygonOnClosedSurface

/// Polygon defined on a closed surface.
pub struct BRepPolygonOnClosedSurface {
    surface_id: u32,
    vertices_2d_1: Vec<(f64, f64)>,
    vertices_2d_2: Vec<(f64, f64)>,
    vertices_3d: Vec<(f64, f64, f64)>,
    closed: bool,
}

impl BRepPolygonOnClosedSurface {
    /// Create a new polygon on closed surface.
    pub fn new(
        surface_id: u32,
        vertices_2d_1: Vec<(f64, f64)>,
        vertices_2d_2: Vec<(f64, f64)>,
        vertices_3d: Vec<(f64, f64, f64)>,
    ) -> Self {
        Self {
            surface_id,
            vertices_2d_1,
            vertices_2d_2,
            vertices_3d,
            closed: true,
        }
    }

    /// Get the surface id.
    pub fn surface_id(&self) -> u32 {
        self.surface_id
    }

    /// Get 2D vertices on surface 1.
    pub fn vertices_2d_on_surface1(&self) -> &[(f64, f64)] {
        &self.vertices_2d_1
    }

    /// Get 2D vertices on surface 2.
    pub fn vertices_2d_on_surface2(&self) -> &[(f64, f64)] {
        &self.vertices_2d_2
    }

    /// Get 3D vertices.
    pub fn vertices_3d(&self) -> &[(f64, f64, f64)] {
        &self.vertices_3d
    }

    /// Check if surface is closed.
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Get number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.vertices_3d.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let poly = BRepPolygonOnClosedSurface::new(
            1,
            vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)],
            vec![(0.1, 0.1), (1.1, 0.1), (1.1, 1.1)],
            vec![(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 1.0, 0.0)],
        );

        assert_eq!(poly.surface_id(), 1);
        assert!(poly.is_closed());
        assert_eq!(poly.vertex_count(), 3);
    }
}
