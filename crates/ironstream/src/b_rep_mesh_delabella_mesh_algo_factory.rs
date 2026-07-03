// FILE: b_rep_mesh_delabella_mesh_algo_factory.rs
// occt: BRepMesh_DelabellaMeshAlgoFactory

/// Factory for creating Delabella mesh algorithms.
pub struct DelabellaMeshAlgoFactory;

impl DelabellaMeshAlgoFactory {
    /// Creates a new Delabella mesh algorithm instance.
    pub fn create() -> String {
        "DelabellaBaseMeshAlgo".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delabella_mesh_algo_factory_create() {
        let algo = DelabellaMeshAlgoFactory::create();
        assert_eq!(algo, "DelabellaBaseMeshAlgo");
    }
}
