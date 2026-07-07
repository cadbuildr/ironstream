// FILE: step_kinematics_kinematic_property_mechanism_representation.rs
// occt: StepKinematics_KinematicPropertyMechanismRepresentation

pub struct KinematicPropertyMechanismRepresentation {
    base: Option<Box<dyn std::any::Any>>,
}

impl KinematicPropertyMechanismRepresentation {
    pub fn new() -> Self {
        KinematicPropertyMechanismRepresentation { base: None }
    }

    pub fn base(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.base
    }

    pub fn set_base(&mut self, base: Option<Box<dyn std::any::Any>>) {
        self.base = base;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kinematic_property_mechanism_representation_creation() {
        let kpmr = KinematicPropertyMechanismRepresentation::new();
        assert!(kpmr.base().is_none());
    }

    #[test]
    fn test_set_base() {
        let mut kpmr = KinematicPropertyMechanismRepresentation::new();
        kpmr.set_base(None);
        assert!(kpmr.base().is_none());
    }
}
