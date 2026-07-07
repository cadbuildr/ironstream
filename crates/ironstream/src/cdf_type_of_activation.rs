// FILE: cdf_type_of_activation.rs
// occt: CDF_TypeOfActivation

/// Enum representing CDF_TypeOfActivation from OpenCascade:
/// how a retrieved document relates to a document already in the session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum CDF_TypeOfActivation {
    CDF_TOA_New = 0,
    CDF_TOA_Modified = 1,
    CDF_TOA_Unchanged = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdf_type_of_activation_variants() {
        assert_eq!(CDF_TypeOfActivation::CDF_TOA_New as u8, 0);
        assert_eq!(CDF_TypeOfActivation::CDF_TOA_Modified as u8, 1);
        assert_eq!(CDF_TypeOfActivation::CDF_TOA_Unchanged as u8, 2);
    }

    #[test]
    fn test_equality_and_copy() {
        let a = CDF_TypeOfActivation::CDF_TOA_Modified;
        let b = a;
        assert_eq!(a, b);
        assert_ne!(a, CDF_TypeOfActivation::CDF_TOA_New);
    }
}
