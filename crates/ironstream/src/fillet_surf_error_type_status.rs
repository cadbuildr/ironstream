// FILE: fillet_surf_error_type_status.rs
// occt: FilletSurf_ErrorTypeStatus

/// Error types that can occur in fillet surface operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilletSurfErrorTypeStatus {
    /// Empty edge list
    EmptyList,
    /// Edge is not G1 continuous
    EdgeNotG1,
    /// Faces are not G1 continuous
    FacesNotG1,
    /// Edge is not on the shape
    EdgeNotOnShape,
    /// Edge is not sharp
    NotSharpEdge,
    /// Error during fillet computation
    PbFilletCompute,
}

impl Default for FilletSurfErrorTypeStatus {
    fn default() -> Self {
        Self::EmptyList
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_eq!(FilletSurfErrorTypeStatus::EmptyList, FilletSurfErrorTypeStatus::EmptyList);
        assert_ne!(FilletSurfErrorTypeStatus::EmptyList, FilletSurfErrorTypeStatus::EdgeNotG1);
    }

    #[test]
    fn test_default() {
        assert_eq!(FilletSurfErrorTypeStatus::default(), FilletSurfErrorTypeStatus::EmptyList);
    }

    #[test]
    fn test_copy_semantics() {
        let err = FilletSurfErrorTypeStatus::PbFilletCompute;
        let err2 = err;
        assert_eq!(err, err2);
    }
}
