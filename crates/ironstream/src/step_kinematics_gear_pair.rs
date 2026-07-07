// FILE: step_kinematics_gear_pair.rs
// occt: StepKinematics_GearPair

pub struct GearPair {
    radius_first_link: f64,
    radius_second_link: f64,
    bevel: f64,
    helical_angle: f64,
    gear_ratio: f64,
}

impl GearPair {
    pub fn new() -> Self {
        GearPair {
            radius_first_link: 0.0,
            radius_second_link: 0.0,
            bevel: 0.0,
            helical_angle: 0.0,
            gear_ratio: 0.0,
        }
    }

    pub fn init(
        &mut self,
        radius_first_link: f64,
        radius_second_link: f64,
        bevel: f64,
        helical_angle: f64,
        gear_ratio: f64,
    ) {
        self.radius_first_link = radius_first_link;
        self.radius_second_link = radius_second_link;
        self.bevel = bevel;
        self.helical_angle = helical_angle;
        self.gear_ratio = gear_ratio;
    }

    pub fn radius_first_link(&self) -> f64 {
        self.radius_first_link
    }

    pub fn set_radius_first_link(&mut self, value: f64) {
        self.radius_first_link = value;
    }

    pub fn radius_second_link(&self) -> f64 {
        self.radius_second_link
    }

    pub fn set_radius_second_link(&mut self, value: f64) {
        self.radius_second_link = value;
    }

    pub fn bevel(&self) -> f64 {
        self.bevel
    }

    pub fn set_bevel(&mut self, value: f64) {
        self.bevel = value;
    }

    pub fn helical_angle(&self) -> f64 {
        self.helical_angle
    }

    pub fn set_helical_angle(&mut self, value: f64) {
        self.helical_angle = value;
    }

    pub fn gear_ratio(&self) -> f64 {
        self.gear_ratio
    }

    pub fn set_gear_ratio(&mut self, value: f64) {
        self.gear_ratio = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_pair_creation() {
        let pair = GearPair::new();
        assert_eq!(pair.radius_first_link(), 0.0);
        assert_eq!(pair.radius_second_link(), 0.0);
        assert_eq!(pair.bevel(), 0.0);
        assert_eq!(pair.helical_angle(), 0.0);
        assert_eq!(pair.gear_ratio(), 0.0);
    }

    #[test]
    fn test_gear_pair_init() {
        let mut pair = GearPair::new();
        pair.init(2.0, 3.0, 0.5, 0.2, 1.5);
        assert_eq!(pair.radius_first_link(), 2.0);
        assert_eq!(pair.radius_second_link(), 3.0);
        assert_eq!(pair.bevel(), 0.5);
        assert_eq!(pair.helical_angle(), 0.2);
        assert_eq!(pair.gear_ratio(), 1.5);
    }

    #[test]
    fn test_gear_pair_setters() {
        let mut pair = GearPair::new();
        pair.set_radius_first_link(1.5);
        pair.set_radius_second_link(2.5);
        pair.set_bevel(0.3);
        pair.set_helical_angle(0.1);
        pair.set_gear_ratio(1.2);

        assert_eq!(pair.radius_first_link(), 1.5);
        assert_eq!(pair.radius_second_link(), 2.5);
        assert_eq!(pair.bevel(), 0.3);
        assert_eq!(pair.helical_angle(), 0.1);
        assert_eq!(pair.gear_ratio(), 1.2);
    }
}
