// FILE: step_kinematics_low_order_kinematic_pair_with_motion_coupling.rs
// occt: StepKinematics_LowOrderKinematicPairWithMotionCoupling

pub struct LowOrderKinematicPairWithMotionCoupling;

impl LowOrderKinematicPairWithMotionCoupling {
    pub fn new() -> Self {
        LowOrderKinematicPairWithMotionCoupling
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_order_kinematic_pair_with_motion_coupling_creation() {
        let _pair = LowOrderKinematicPairWithMotionCoupling::new();
    }
}
