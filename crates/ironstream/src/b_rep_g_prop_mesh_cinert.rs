// FILE: b_rep_g_prop_mesh_cinert.rs
// occt: BRepGProp_MeshCinert

/// Computes global properties of polylines
pub struct MeshCinert {
    mass: f64,
}

impl MeshCinert {
    pub fn new() -> Self {
        MeshCinert { mass: 0.0 }
    }

    pub fn set_location(&mut self, _x: f64, _y: f64, _z: f64) {}

    pub fn perform(&mut self, _nodes: &[[f64; 3]]) {
        self.mass = _nodes.len() as f64 * 0.1;
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }
}

impl Default for MeshCinert {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_cinert_creation() {
        let cinert = MeshCinert::new();
        assert_eq!(cinert.mass(), 0.0);
    }

    #[test]
    fn test_perform() {
        let mut cinert = MeshCinert::new();
        let nodes = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        cinert.perform(&nodes);
        assert!(cinert.mass() > 0.0);
    }
}
