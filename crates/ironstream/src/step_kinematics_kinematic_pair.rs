// FILE: step_kinematics_kinematic_pair.rs
// occt: StepKinematics_KinematicPair

pub struct KinematicPair {
    item_defined_transformation: Option<Box<dyn std::any::Any>>,
    joint: Option<Box<dyn std::any::Any>>,
}

impl KinematicPair {
    pub fn new() -> Self {
        KinematicPair {
            item_defined_transformation: None,
            joint: None,
        }
    }

    pub fn init(
        &mut self,
        item_defined_transformation: Option<Box<dyn std::any::Any>>,
        joint: Option<Box<dyn std::any::Any>>,
    ) {
        self.item_defined_transformation = item_defined_transformation;
        self.joint = joint;
    }

    pub fn item_defined_transformation(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.item_defined_transformation
    }

    pub fn set_item_defined_transformation(
        &mut self,
        transformation: Option<Box<dyn std::any::Any>>,
    ) {
        self.item_defined_transformation = transformation;
    }

    pub fn joint(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.joint
    }

    pub fn set_joint(&mut self, joint: Option<Box<dyn std::any::Any>>) {
        self.joint = joint;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kinematic_pair_creation() {
        let pair = KinematicPair::new();
        assert!(pair.item_defined_transformation().is_none());
        assert!(pair.joint().is_none());
    }

    #[test]
    fn test_kinematic_pair_init() {
        let mut pair = KinematicPair::new();
        pair.init(None, None);
        assert!(pair.item_defined_transformation().is_none());
        assert!(pair.joint().is_none());
    }
}
