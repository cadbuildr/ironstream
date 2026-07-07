// FILE: step_kinematics_pair_value.rs
// occt: StepKinematics_PairValue

pub struct PairValue {
    applies_to_pair: Option<Box<dyn std::any::Any>>,
}

impl PairValue {
    pub fn new() -> Self {
        PairValue {
            applies_to_pair: None,
        }
    }

    pub fn init(&mut self, applies_to_pair: Option<Box<dyn std::any::Any>>) {
        self.applies_to_pair = applies_to_pair;
    }

    pub fn applies_to_pair(&self) -> &Option<Box<dyn std::any::Any>> {
        &self.applies_to_pair
    }

    pub fn set_applies_to_pair(&mut self, pair: Option<Box<dyn std::any::Any>>) {
        self.applies_to_pair = pair;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_value_creation() {
        let pv = PairValue::new();
        assert!(pv.applies_to_pair().is_none());
    }

    #[test]
    fn test_init() {
        let mut pv = PairValue::new();
        pv.init(None);
        assert!(pv.applies_to_pair().is_none());
    }
}
