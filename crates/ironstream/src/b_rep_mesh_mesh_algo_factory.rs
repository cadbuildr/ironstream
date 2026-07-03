// FILE: b_rep_mesh_mesh_algo_factory.rs
// occt: BRepMesh_MeshAlgoFactory

/// Factory for creating mesh algorithms.
pub struct MeshAlgoFactory;

impl MeshAlgoFactory {
    /// Creates a new mesh algorithm instance.
    pub fn create() -> String {
        "BaseMeshAlgo".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_algo_factory_create() {
        let algo = MeshAlgoFactory::create();
        assert_eq!(algo, "BaseMeshAlgo");
    }
}
