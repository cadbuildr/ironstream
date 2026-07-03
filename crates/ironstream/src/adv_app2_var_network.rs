// FILE: adv_app2_var_network.rs
// occt: AdvApp2Var_Network

//! Manages a network of approximation nodes and constraints.

pub struct Network {
    nodes: Vec<(f64, f64, f64, f64, f64)>,
    u_patches: i32,
    v_patches: i32,
}

impl Network {
    pub fn new(u_patches: i32, v_patches: i32) -> Self {
        Network {
            nodes: vec![],
            u_patches,
            v_patches,
        }
    }

    pub fn add_node(&mut self, u: f64, v: f64, x: f64, y: f64, z: f64) {
        self.nodes.push((u, v, x, y, z));
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn u_patch_count(&self) -> i32 {
        self.u_patches
    }

    pub fn v_patch_count(&self) -> i32 {
        self.v_patches
    }

    pub fn total_patch_count(&self) -> i32 {
        self.u_patches * self.v_patches
    }
}

impl Default for Network {
    fn default() -> Self {
        Network::new(1, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let network = Network::new(4, 4);
        assert_eq!(network.u_patch_count(), 4);
        assert_eq!(network.v_patch_count(), 4);
    }

    #[test]
    fn test_add_node() {
        let mut network = Network::new(2, 2);
        network.add_node(0.5, 0.5, 1.0, 2.0, 3.0);
        assert_eq!(network.node_count(), 1);
    }

    #[test]
    fn test_total_patch_count() {
        let network = Network::new(3, 4);
        assert_eq!(network.total_patch_count(), 12);
    }
}
