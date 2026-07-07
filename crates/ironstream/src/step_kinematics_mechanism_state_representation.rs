// FILE: step_kinematics_mechanism_state_representation.rs
// occt: StepKinematics_MechanismStateRepresentation

pub struct MechanismStateRepresentation {
    mechanism: Option<Box<dyn std::any::Any>>,
}

impl MechanismStateRepresentation {
    pub fn new() -> Self {
        MechanismStateRepresentation {
            mechanism: None,
        }
    }

    pub fn init(&mut self, mechanism: Option<Box<dyn std::any::Any>>) {
        self.mechanism = mechanism;
    }

    pub fn mechanism(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.mechanism
    }

    pub fn set_mechanism(&mut self, mechanism: Option<Box<dyn std::any::Any>>) {
        self.mechanism = mechanism;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanism_state_representation_creation() {
        let msr = MechanismStateRepresentation::new();
        assert!(msr.mechanism().is_none());
    }

    #[test]
    fn test_init() {
        let mut msr = MechanismStateRepresentation::new();
        msr.init(None);
        assert!(msr.mechanism().is_none());
    }
}
