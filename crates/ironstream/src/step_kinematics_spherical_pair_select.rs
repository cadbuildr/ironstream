// FILE: step_kinematics_spherical_pair_select.rs
// occt: StepKinematics_SphericalPairSelect

/// Representation of STEP SELECT type SphericalPairSelect.
/// Can be one of: SphericalPair or SphericalPairWithPin.
#[derive(Clone, Debug)]
pub enum StepKinematicsSphericalPairSelect {
    SphericalPair,
    SphericalPairWithPin,
}

impl Default for StepKinematicsSphericalPairSelect {
    fn default() -> Self {
        StepKinematicsSphericalPairSelect::SphericalPair
    }
}

impl StepKinematicsSphericalPairSelect {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_spherical_pair(&self) -> bool {
        matches!(self, StepKinematicsSphericalPairSelect::SphericalPair)
    }

    pub fn is_spherical_pair_with_pin(&self) -> bool {
        matches!(self, StepKinematicsSphericalPairSelect::SphericalPairWithPin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let sel = StepKinematicsSphericalPairSelect::new();
        assert!(sel.is_spherical_pair());
    }

    #[test]
    fn test_variants() {
        let s1 = StepKinematicsSphericalPairSelect::SphericalPair;
        assert!(s1.is_spherical_pair());

        let s2 = StepKinematicsSphericalPairSelect::SphericalPairWithPin;
        assert!(s2.is_spherical_pair_with_pin());
    }
}
