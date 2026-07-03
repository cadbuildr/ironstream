// FILE: fillet_surf_status_type.rs
// occt: FilletSurf_StatusType

/// Describes the extremity configuration of a fillet surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilletSurfStatusType {
    /// Both extremities are on edges
    TwoExtremityOnEdge,
    /// One extremity is on an edge
    OneExtremityOnEdge,
    /// No extremities are on edges
    NoExtremityOnEdge,
}

impl Default for FilletSurfStatusType {
    fn default() -> Self {
        Self::NoExtremityOnEdge
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_eq!(
            FilletSurfStatusType::TwoExtremityOnEdge,
            FilletSurfStatusType::TwoExtremityOnEdge
        );
        assert_ne!(
            FilletSurfStatusType::TwoExtremityOnEdge,
            FilletSurfStatusType::OneExtremityOnEdge
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(
            FilletSurfStatusType::default(),
            FilletSurfStatusType::NoExtremityOnEdge
        );
    }

    #[test]
    fn test_copy_semantics() {
        let status = FilletSurfStatusType::OneExtremityOnEdge;
        let status2 = status;
        assert_eq!(status, status2);
    }
}
