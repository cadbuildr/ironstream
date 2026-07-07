// FILE: step_kinematics_mechanism_representation.rs
// occt: StepKinematics_MechanismRepresentation

use super::step_kinematics_kinematic_topology_representation_select::KinematicTopologyRepresentationSelect;

pub struct MechanismRepresentation {
    represented_topology: KinematicTopologyRepresentationSelect,
}

impl MechanismRepresentation {
    pub fn new() -> Self {
        MechanismRepresentation {
            represented_topology: KinematicTopologyRepresentationSelect::new(),
        }
    }

    pub fn init(&mut self, represented_topology: KinematicTopologyRepresentationSelect) {
        self.represented_topology = represented_topology;
    }

    pub fn represented_topology(&self) -> &KinematicTopologyRepresentationSelect {
        &self.represented_topology
    }

    pub fn set_represented_topology(&mut self, topology: KinematicTopologyRepresentationSelect) {
        self.represented_topology = topology;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanism_representation_creation() {
        let mr = MechanismRepresentation::new();
        assert_eq!(mr.represented_topology().case_num(), 0);
    }

    #[test]
    fn test_init() {
        let mut mr = MechanismRepresentation::new();
        let topology = KinematicTopologyRepresentationSelect::new();
        mr.init(topology);
        assert_eq!(mr.represented_topology().case_num(), 0);
    }
}
