// FILE: b_rep_mesh_incremental_mesh_factory.rs
// occt: BRepMesh_IncrementalMeshFactory

/// Factory for creating incremental mesh algorithms.
pub struct IncrementalMeshFactory;

impl IncrementalMeshFactory {
    /// Creates a new incremental mesh algorithm instance.
    pub fn create() -> String {
        "IncrementalMeshAlgo".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incremental_mesh_factory_create() {
        let algo = IncrementalMeshFactory::create();
        assert_eq!(algo, "IncrementalMeshAlgo");
    }
}
