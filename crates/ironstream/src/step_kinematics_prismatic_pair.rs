// FILE: step_kinematics_prismatic_pair.rs
// occt: StepKinematics_PrismaticPair

/// Representation of STEP entity PrismaticPair.
/// A kinematic pair that allows translation along a single axis.
#[derive(Clone, Debug, Default)]
pub struct StepKinematicsPrismaticPair;

impl StepKinematicsPrismaticPair {
    pub fn new() -> Self {
        StepKinematicsPrismaticPair
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _pair = StepKinematicsPrismaticPair::new();
    }
}
