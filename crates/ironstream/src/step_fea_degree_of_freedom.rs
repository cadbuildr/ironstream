// FILE: step_fea_degree_of_freedom.rs
// occt: StepFEA_DegreeOfFreedom

/// Enumeration for degrees of freedom in FEA.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DegreeOfFreedom {
    XTranslation,
    YTranslation,
    ZTranslation,
    XRotation,
    YRotation,
    ZRotation,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_ne!(DegreeOfFreedom::XTranslation, DegreeOfFreedom::YTranslation);
        assert_ne!(DegreeOfFreedom::XRotation, DegreeOfFreedom::YRotation);
        assert_eq!(DegreeOfFreedom::ZTranslation, DegreeOfFreedom::ZTranslation);
    }

    #[test]
    fn test_copy() {
        let dof = DegreeOfFreedom::ZRotation;
        let dof2 = dof;
        assert_eq!(dof, dof2);
    }
}
