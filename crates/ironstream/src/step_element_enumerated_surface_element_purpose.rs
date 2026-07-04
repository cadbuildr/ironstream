// FILE: step_element_enumerated_surface_element_purpose.rs
// occt: StepElement_EnumeratedSurfaceElementPurpose

/// Enumeration for surface element purpose in FEA.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnumeratedSurfaceElementPurpose {
    MembraneDirect,
    MembraneShear,
    BendingDirect,
    BendingTorsion,
    NormalToPlaneShear,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_variants() {
        assert_ne!(EnumeratedSurfaceElementPurpose::MembraneDirect, EnumeratedSurfaceElementPurpose::MembraneShear);
        assert_ne!(EnumeratedSurfaceElementPurpose::BendingDirect, EnumeratedSurfaceElementPurpose::NormalToPlaneShear);
    }

    #[test]
    fn test_copy() {
        let purpose = EnumeratedSurfaceElementPurpose::BendingTorsion;
        let purpose2 = purpose;
        assert_eq!(purpose, purpose2);
    }

    #[test]
    fn test_debug() {
        let purpose = EnumeratedSurfaceElementPurpose::NormalToPlaneShear;
        assert_eq!(format!("{:?}", purpose), "NormalToPlaneShear");
    }
}
