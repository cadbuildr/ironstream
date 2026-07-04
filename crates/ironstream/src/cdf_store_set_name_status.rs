// FILE: cdf_store_set_name_status.rs
// occt: CDF_StoreSetNameStatus

/// Enum representing CDF_StoreSetNameStatus from OpenCascade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CDF_StoreSetNameStatus {
    /// CDF_SSNS_OK
    OK = 0,
    /// CDF_SSNS_ReplacingAnExistentDocument
    ReplacingAnExistentDocument = 1,
    /// CDF_SSNS_OpenDocument
    OpenDocument = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdf_store_set_name_status_variants() {
        assert_eq!(CDF_StoreSetNameStatus::OK as u8, 0);
        assert_eq!(CDF_StoreSetNameStatus::ReplacingAnExistentDocument as u8, 1);
        assert_eq!(CDF_StoreSetNameStatus::OpenDocument as u8, 2);
        assert_ne!(
            CDF_StoreSetNameStatus::OK,
            CDF_StoreSetNameStatus::OpenDocument
        );
    }
}
