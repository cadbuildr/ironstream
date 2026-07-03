// FILE: i_mesh_tools_mesh_algo_factory.rs
// occt: IMeshTools_MeshAlgoFactory

/// Surface types for meshing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeomAbsSurfaceType {
    PlaneSurface = 0,
    CylinderSurface = 1,
    ConeSurface = 2,
    SphereSurface = 3,
    ToroidalSurface = 4,
    BezierSurface = 5,
    BSplineSurface = 6,
    SurfaceOfRevolution = 7,
    SurfaceOfExtrusion = 8,
    OffsetSurface = 9,
    OtherSurface = 10,
}

/// Meshing parameters.
pub struct MeshingParameters {
    pub deflection: f64,
    pub angular_deflection: f64,
}

impl Default for MeshingParameters {
    fn default() -> Self {
        MeshingParameters {
            deflection: 0.1,
            angular_deflection: 0.5,
        }
    }
}

/// Meshing algorithm interface.
pub trait MeshAlgo {
    fn mesh(&self, surface_type: GeomAbsSurfaceType, params: &MeshingParameters) -> bool;
}

/// Base interface for factories producing triangulation algorithms.
pub trait MeshAlgoFactory {
    /// Creates instance of meshing algorithm for the given surface type.
    fn get_algo(
        &self,
        surface_type: GeomAbsSurfaceType,
        params: &MeshingParameters,
    ) -> Option<Box<dyn MeshAlgo>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_type_values() {
        assert_eq!(GeomAbsSurfaceType::PlaneSurface as i32, 0);
        assert_eq!(GeomAbsSurfaceType::SphereSurface as i32, 3);
    }

    #[test]
    fn test_meshing_parameters() {
        let params = MeshingParameters::default();
        assert_eq!(params.deflection, 0.1);
        assert_eq!(params.angular_deflection, 0.5);
    }
}
