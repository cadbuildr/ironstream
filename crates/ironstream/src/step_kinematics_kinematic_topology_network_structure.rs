// FILE: step_kinematics_kinematic_topology_network_structure.rs
// occt: StepKinematics_KinematicTopologyNetworkStructure

pub struct KinematicTopologyNetworkStructure {
    parent: Option<Box<dyn std::any::Any>>,
}

impl KinematicTopologyNetworkStructure {
    pub fn new() -> Self {
        KinematicTopologyNetworkStructure { parent: None }
    }

    pub fn init(&mut self, parent: Option<Box<dyn std::any::Any>>) {
        self.parent = parent;
    }

    pub fn parent(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.parent
    }

    pub fn set_parent(&mut self, parent: Option<Box<dyn std::any::Any>>) {
        self.parent = parent;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kinematic_topology_network_structure_creation() {
        let ktns = KinematicTopologyNetworkStructure::new();
        assert!(ktns.parent().is_none());
    }

    #[test]
    fn test_init() {
        let mut ktns = KinematicTopologyNetworkStructure::new();
        ktns.init(None);
        assert!(ktns.parent().is_none());
    }
}
