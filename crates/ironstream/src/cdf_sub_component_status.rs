// FILE: cdf_sub_component_status.rs
// occt: CDF_SubComponentStatus

/// Enum representing CDF_SubComponentStatus from OpenCascade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum CDF_SubComponentStatus {
    CDF_SCS_Consistent,
    CDF_SCS_Unconsistent,
    CDF_SCS_Stored,
    CDF_SCS_Modified,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdf_sub_component_status_variants() {
        // Discriminants follow OCCT declaration order in CDF_SubComponentStatus.hxx
        assert_eq!(CDF_SubComponentStatus::CDF_SCS_Consistent as u8, 0);
        assert_eq!(CDF_SubComponentStatus::CDF_SCS_Unconsistent as u8, 1);
        assert_eq!(CDF_SubComponentStatus::CDF_SCS_Stored as u8, 2);
        assert_eq!(CDF_SubComponentStatus::CDF_SCS_Modified as u8, 3);
        assert_ne!(
            CDF_SubComponentStatus::CDF_SCS_Stored,
            CDF_SubComponentStatus::CDF_SCS_Modified
        );
    }
}
