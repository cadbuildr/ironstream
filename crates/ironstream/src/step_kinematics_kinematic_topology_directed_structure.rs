// FILE: step_kinematics_kinematic_topology_directed_structure.rs
// occt: StepKinematics_KinematicTopologyDirectedStructure

pub struct KinematicTopologyDirectedStructure {
    parent: Option<Box<dyn std::any::Any>>,
}

impl KinematicTopologyDirectedStructure {
    pub fn new() -> Self {
        KinematicTopologyDirectedStructure { parent: None }
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
    fn test_kinematic_topology_directed_structure_creation() {
        let ktds = KinematicTopologyDirectedStructure::new();
        assert!(ktds.parent().is_none());
    }

    #[test]
    fn test_init() {
        let mut ktds = KinematicTopologyDirectedStructure::new();
        ktds.init(None);
        assert!(ktds.parent().is_none());
    }
}
