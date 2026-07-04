// FILE: xb_rep_mesh_factory.rs
// occt: XBRepMesh_Factory

/// Factory for creating XBRepMesh meshing algorithm instances.
/// This factory is registered under the name "XBRepMesh" and provides
/// an alternative meshing algorithm based on BRepMesh_IncrementalMesh.
pub struct XBRepMeshFactory {
    name: String,
}

impl XBRepMeshFactory {
    /// Constructor. Registers this factory under the name "XBRepMesh".
    pub fn new() -> Self {
        Self {
            name: "XBRepMesh".to_string(),
        }
    }

    /// Returns the factory name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Creates a new meshing algorithm instance.
    /// Parameters:
    /// - lin_deflection: linear deflection for meshing
    /// - ang_deflection: angular deflection for meshing
    ///
    /// Returns a handle to the created meshing algorithm
    pub fn create_algorithm(&self, lin_deflection: f64, ang_deflection: f64) -> MeshingAlgorithm {
        MeshingAlgorithm {
            lin_deflection,
            ang_deflection,
        }
    }
}

impl Default for XBRepMeshFactory {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a meshing algorithm instance created by the factory
pub struct MeshingAlgorithm {
    /// Linear deflection for meshing
    lin_deflection: f64,
    /// Angular deflection for meshing
    ang_deflection: f64,
}

impl MeshingAlgorithm {
    /// Returns the linear deflection
    pub fn lin_deflection(&self) -> f64 {
        self.lin_deflection
    }

    /// Returns the angular deflection
    pub fn ang_deflection(&self) -> f64 {
        self.ang_deflection
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_creation() {
        let factory = XBRepMeshFactory::new();
        assert_eq!(factory.name(), "XBRepMesh");
    }

    #[test]
    fn test_default_trait() {
        let factory = XBRepMeshFactory::default();
        assert_eq!(factory.name(), "XBRepMesh");
    }

    #[test]
    fn test_create_algorithm() {
        let factory = XBRepMeshFactory::new();
        let algo = factory.create_algorithm(0.01, 0.5);
        assert_eq!(algo.lin_deflection(), 0.01);
        assert_eq!(algo.ang_deflection(), 0.5);
    }

    #[test]
    fn test_algorithm_values() {
        let algo = MeshingAlgorithm {
            lin_deflection: 0.001,
            ang_deflection: 0.785,
        };
        assert_eq!(algo.lin_deflection(), 0.001);
        assert_eq!(algo.ang_deflection(), 0.785);
    }
}
