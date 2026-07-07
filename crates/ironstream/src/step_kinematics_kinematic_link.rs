// FILE: step_kinematics_kinematic_link.rs
// occt: StepKinematics_KinematicLink

pub struct KinematicLink;

impl KinematicLink {
    pub fn new() -> Self {
        KinematicLink
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kinematic_link_creation() {
        let _link = KinematicLink::new();
    }
}
