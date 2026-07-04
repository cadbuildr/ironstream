// FILE: step_fea_coordinate_system_type.rs
// occt: StepFEA_CoordinateSystemType

/// Enumeration for FEA coordinate system types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoordinateSystemType {
    Cartesian,
    Cylindrical,
    Spherical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_variants() {
        assert_ne!(CoordinateSystemType::Cartesian, CoordinateSystemType::Cylindrical);
        assert_ne!(CoordinateSystemType::Cylindrical, CoordinateSystemType::Spherical);
        assert_eq!(CoordinateSystemType::Cartesian, CoordinateSystemType::Cartesian);
    }

    #[test]
    fn test_copy() {
        let cst = CoordinateSystemType::Spherical;
        let cst2 = cst;
        assert_eq!(cst, cst2);
    }

    #[test]
    fn test_debug() {
        let cst = CoordinateSystemType::Cylindrical;
        assert_eq!(format!("{:?}", cst), "Cylindrical");
    }
}
