// FILE: step_kinematics_actuated_kin_pair_and_order_kin_pair.rs
// occt: StepKinematics_ActuatedKinPairAndOrderKinPair

pub struct ActuatedKinPairAndOrderKinPair {
    actuated_kinematic_pair: Option<Box<dyn std::any::Any>>,
    order_kinematic_pair: Option<Box<dyn std::any::Any>>,
}

impl ActuatedKinPairAndOrderKinPair {
    pub fn new() -> Self {
        ActuatedKinPairAndOrderKinPair {
            actuated_kinematic_pair: None,
            order_kinematic_pair: None,
        }
    }

    pub fn set_actuated_kinematic_pair(&mut self, pair: Option<Box<dyn std::any::Any>>) {
        self.actuated_kinematic_pair = pair;
    }

    pub fn get_actuated_kinematic_pair(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.actuated_kinematic_pair
    }

    pub fn set_order_kinematic_pair(&mut self, pair: Option<Box<dyn std::any::Any>>) {
        self.order_kinematic_pair = pair;
    }

    pub fn get_order_kinematic_pair(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.order_kinematic_pair
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actuated_kin_pair_and_order_kin_pair_creation() {
        let pair = ActuatedKinPairAndOrderKinPair::new();
        assert!(pair.get_actuated_kinematic_pair().is_none());
        assert!(pair.get_order_kinematic_pair().is_none());
    }

    #[test]
    fn test_setters() {
        let mut pair = ActuatedKinPairAndOrderKinPair::new();
        pair.set_actuated_kinematic_pair(None);
        pair.set_order_kinematic_pair(None);

        assert!(pair.get_actuated_kinematic_pair().is_none());
        assert!(pair.get_order_kinematic_pair().is_none());
    }
}
