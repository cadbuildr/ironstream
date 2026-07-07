// FILE: step_kinematics_linear_flexible_and_pinion_pair.rs
// occt: StepKinematics_LinearFlexibleAndPinionPair

pub struct LinearFlexibleAndPinionPair {
    pinion_radius: f64,
}

impl LinearFlexibleAndPinionPair {
    pub fn new() -> Self {
        LinearFlexibleAndPinionPair {
            pinion_radius: 0.0,
        }
    }

    pub fn init(&mut self, pinion_radius: f64) {
        self.pinion_radius = pinion_radius;
    }

    pub fn pinion_radius(&self) -> f64 {
        self.pinion_radius
    }

    pub fn set_pinion_radius(&mut self, value: f64) {
        self.pinion_radius = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_flexible_and_pinion_pair_creation() {
        let pair = LinearFlexibleAndPinionPair::new();
        assert_eq!(pair.pinion_radius(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut pair = LinearFlexibleAndPinionPair::new();
        pair.init(2.5);
        assert_eq!(pair.pinion_radius(), 2.5);
    }

    #[test]
    fn test_setter() {
        let mut pair = LinearFlexibleAndPinionPair::new();
        pair.set_pinion_radius(1.5);
        assert_eq!(pair.pinion_radius(), 1.5);
    }
}
