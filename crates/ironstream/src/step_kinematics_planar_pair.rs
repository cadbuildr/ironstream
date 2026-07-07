// FILE: step_kinematics_planar_pair.rs
// occt: StepKinematics_PlanarPair

pub struct PlanarPair;

impl PlanarPair {
    pub fn new() -> Self {
        PlanarPair
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planar_pair_creation() {
        let _pair = PlanarPair::new();
    }
}
