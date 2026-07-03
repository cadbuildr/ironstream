// FILE: ch_fi_ds_type_of_concavity.rs
// occt: ChFiDS_TypeOfConcavity

/// Types of concavity in fillet geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChFiDsTypeOfConcavity {
    /// Concave configuration
    Concave,
    /// Convex configuration
    Convex,
    /// Tangential configuration
    Tangential,
    /// Free boundary
    FreeBound,
    /// Other configuration type
    Other,
    /// Mixed configuration
    Mixed,
}

impl Default for ChFiDsTypeOfConcavity {
    fn default() -> Self {
        Self::Other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_eq!(ChFiDsTypeOfConcavity::Concave, ChFiDsTypeOfConcavity::Concave);
        assert_ne!(ChFiDsTypeOfConcavity::Concave, ChFiDsTypeOfConcavity::Convex);
    }

    #[test]
    fn test_default() {
        assert_eq!(ChFiDsTypeOfConcavity::default(), ChFiDsTypeOfConcavity::Other);
    }

    #[test]
    fn test_copy_semantics() {
        let concavity = ChFiDsTypeOfConcavity::Mixed;
        let concavity2 = concavity;
        assert_eq!(concavity, concavity2);
    }
}
