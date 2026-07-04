// FILE: cdf_try_store_status.rs
// occt: CDF_TryStoreStatus

/// Enum representing CDF_TryStoreStatus from OpenCascade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum CDF_TryStoreStatus {
    CDF_TS_OK,
    CDF_TS_NoCurrentDocument,
    CDF_TS_NoDriver,
    CDF_TS_NoSubComponentDriver,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdf_try_store_status_variants() {
        // Discriminants follow OCCT declaration order in CDF_TryStoreStatus.hxx
        assert_eq!(CDF_TryStoreStatus::CDF_TS_OK as u8, 0);
        assert_eq!(CDF_TryStoreStatus::CDF_TS_NoCurrentDocument as u8, 1);
        assert_eq!(CDF_TryStoreStatus::CDF_TS_NoDriver as u8, 2);
        assert_eq!(CDF_TryStoreStatus::CDF_TS_NoSubComponentDriver as u8, 3);
        assert_ne!(
            CDF_TryStoreStatus::CDF_TS_OK,
            CDF_TryStoreStatus::CDF_TS_NoDriver
        );
    }
}
