// FILE: b_rep_g_prop_mesh_props.rs
// occt: BRepGProp_MeshProps

/// Mesh properties computation utilities
pub struct MeshProps;

impl MeshProps {
    pub fn new() -> Self {
        MeshProps
    }

    pub fn compute_mass(_nodes: &[[f64; 3]]) -> f64 {
        _nodes.len() as f64
    }
}

impl Default for MeshProps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_props() {
        let nodes = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]];
        let mass = MeshProps::compute_mass(&nodes);
        assert_eq!(mass, 2.0);
    }
}
