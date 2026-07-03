// FILE: fillet_surf_status_done.rs
// occt: FilletSurf_StatusDone

/// Status of a fillet surface operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilletSurfStatusDone {
    /// Operation succeeded completely
    IsOk,
    /// Operation failed
    IsNotOk,
    /// Operation succeeded partially
    IsPartial,
}

impl FilletSurfStatusDone {
    /// Returns true if the operation was successful
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::IsOk)
    }

    /// Returns true if the operation completed (fully or partially)
    pub fn is_done(&self) -> bool {
        matches!(self, Self::IsOk | Self::IsPartial)
    }
}

impl Default for FilletSurfStatusDone {
    fn default() -> Self {
        Self::IsNotOk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ok() {
        assert!(FilletSurfStatusDone::IsOk.is_ok());
        assert!(!FilletSurfStatusDone::IsPartial.is_ok());
        assert!(!FilletSurfStatusDone::IsNotOk.is_ok());
    }

    #[test]
    fn test_is_done() {
        assert!(FilletSurfStatusDone::IsOk.is_done());
        assert!(FilletSurfStatusDone::IsPartial.is_done());
        assert!(!FilletSurfStatusDone::IsNotOk.is_done());
    }

    #[test]
    fn test_default() {
        assert_eq!(FilletSurfStatusDone::default(), FilletSurfStatusDone::IsNotOk);
    }
}
