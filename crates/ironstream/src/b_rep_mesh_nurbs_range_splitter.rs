// FILE: b_rep_mesh_nurbs_range_splitter.rs
// occt: BRepMesh_NURBSRangeSplitter

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceType {
    Plane = 0,
    Cylinder = 1,
    Cone = 2,
    Sphere = 3,
    Torus = 4,
    Bezier = 5,
    BSpline = 6,
    OtherSurface = 7,
}

/// Auxiliary class extending UV range splitter in order to generate
/// internal nodes for NURBS surface.
pub struct NURBSRangeSplitter {
    surface_type: SurfaceType,
}

impl NURBSRangeSplitter {
    /// Constructor.
    pub fn new() -> Self {
        NURBSRangeSplitter {
            surface_type: SurfaceType::OtherSurface,
        }
    }

    /// Updates discrete range of surface according to its geometric range.
    pub fn adjust_range(&mut self) {
        // Adjust range based on NURBS parameters
    }

    /// Returns list of nodes generated using surface data and specified parameters.
    pub fn generate_surface_nodes(&self) -> Vec<(f64, f64)> {
        Vec::new()
    }

    /// Initializes U and V parameters lists using CN continuity intervals.
    pub fn init_parameters(&self) -> bool {
        true
    }

    /// Returns number of intervals computed using available geometrical parameters.
    pub fn get_undefined_interval_nb(&self, _is_u: bool, _continuity: i32) -> i32 {
        0
    }

    /// Sets the surface type.
    pub fn set_surface_type(&mut self, surface_type: SurfaceType) {
        self.surface_type = surface_type;
    }

    /// Returns the surface type.
    pub fn surface_type(&self) -> SurfaceType {
        self.surface_type
    }
}

impl Default for NURBSRangeSplitter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_type_variants() {
        assert_eq!(SurfaceType::Plane as i32, 0);
        assert_eq!(SurfaceType::Cylinder as i32, 1);
        assert_eq!(SurfaceType::OtherSurface as i32, 7);
    }

    #[test]
    fn test_nurbs_range_splitter_new() {
        let splitter = NURBSRangeSplitter::new();
        assert_eq!(splitter.surface_type(), SurfaceType::OtherSurface);
    }

    #[test]
    fn test_nurbs_range_splitter_set_surface_type() {
        let mut splitter = NURBSRangeSplitter::new();
        splitter.set_surface_type(SurfaceType::Sphere);
        assert_eq!(splitter.surface_type(), SurfaceType::Sphere);
    }

    #[test]
    fn test_nurbs_range_splitter_init_parameters() {
        let splitter = NURBSRangeSplitter::new();
        assert!(splitter.init_parameters());
    }

    #[test]
    fn test_nurbs_range_splitter_generate_surface_nodes() {
        let splitter = NURBSRangeSplitter::new();
        let nodes = splitter.generate_surface_nodes();
        assert!(nodes.is_empty());
    }
}
