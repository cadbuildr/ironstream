// FILE: cdm_can_close_status.rs
// occt: CDM_CanCloseStatus

/// Enum representing CDM_CanCloseStatus from OpenCascade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum CDM_CanCloseStatus {
    CDM_CCS_OK,
    CDM_CCS_NotOpen,
    CDM_CCS_UnstoredReferenced,
    CDM_CCS_ModifiedReferenced,
    CDM_CCS_ReferenceRejection,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdm_can_close_status_variants() {
        // Discriminants follow OCCT declaration order in CDM_CanCloseStatus.hxx
        assert_eq!(CDM_CanCloseStatus::CDM_CCS_OK as u8, 0);
        assert_eq!(CDM_CanCloseStatus::CDM_CCS_NotOpen as u8, 1);
        assert_eq!(CDM_CanCloseStatus::CDM_CCS_UnstoredReferenced as u8, 2);
        assert_eq!(CDM_CanCloseStatus::CDM_CCS_ModifiedReferenced as u8, 3);
        assert_eq!(CDM_CanCloseStatus::CDM_CCS_ReferenceRejection as u8, 4);
        assert_ne!(
            CDM_CanCloseStatus::CDM_CCS_OK,
            CDM_CanCloseStatus::CDM_CCS_NotOpen
        );
    }
}
