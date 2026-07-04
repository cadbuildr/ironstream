// FILE: step_kinematics_kinematic_topology_representation_select.rs
// occt: StepKinematics_KinematicTopologyRepresentationSelect

#[derive(Clone, Debug)]
pub enum KinematicTopologyRepresentationSelectType {
    DirectedStructure,
    NetworkStructure,
    Structure,
}

pub struct KinematicTopologyRepresentationSelect {
    case_num: i32,
    value: Option<Box<dyn std::any::Any>>,
}

impl KinematicTopologyRepresentationSelect {
    pub fn new() -> Self {
        KinematicTopologyRepresentationSelect {
            case_num: 0,
            value: None,
        }
    }

    pub fn case_num(&self) -> i32 {
        self.case_num
    }

    pub fn set_case(&mut self, case: i32, value: Option<Box<dyn std::any::Any>>) {
        self.case_num = case;
        self.value = value;
    }

    pub fn kinematic_topology_directed_structure(&self) -> Option<&Box<dyn std::any::Any>> {
        if self.case_num == 1 {
            self.value.as_ref()
        } else {
            None
        }
    }

    pub fn kinematic_topology_network_structure(&self) -> Option<&Box<dyn std::any::Any>> {
        if self.case_num == 2 {
            self.value.as_ref()
        } else {
            None
        }
    }

    pub fn kinematic_topology_structure(&self) -> Option<&Box<dyn std::any::Any>> {
        if self.case_num == 3 {
            self.value.as_ref()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kinematic_topology_representation_select_creation() {
        let ktrs = KinematicTopologyRepresentationSelect::new();
        assert_eq!(ktrs.case_num(), 0);
    }

    #[test]
    fn test_case_accessors() {
        let mut ktrs = KinematicTopologyRepresentationSelect::new();
        ktrs.set_case(1, None);
        assert!(ktrs.kinematic_topology_directed_structure().is_none());

        let mut ktrs2 = KinematicTopologyRepresentationSelect::new();
        ktrs2.set_case(2, None);
        assert!(ktrs2.kinematic_topology_network_structure().is_none());

        let mut ktrs3 = KinematicTopologyRepresentationSelect::new();
        ktrs3.set_case(3, None);
        assert!(ktrs3.kinematic_topology_structure().is_none());
    }
}
