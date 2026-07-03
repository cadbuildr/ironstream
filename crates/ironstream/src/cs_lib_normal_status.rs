// FILE: cs_lib_normal_status.rs
// occt: CSLib_NormalStatus

//! Status of surface normal computation.
//!
//! Describes the result of attempting to compute the normal N to a surface,
//! including cases involving derivatives of the normal (DN/du, DN/dv).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CSLibNormalStatus {
    /// Surface is singular at the point (normal undefined).
    Singular,
    /// Normal is well-defined and computed successfully.
    Defined,
    /// Infinite normals possible.
    InfinityOfSolutions,
    /// DN/du has null length: ||DN/du|| <= Resolution.
    D1NuIsNull,
    /// DN/dv has null length: ||DN/dv|| <= Resolution.
    D1NvIsNull,
    /// Both normal derivatives are null.
    D1NIsNull,
    /// DN/du is negligible compared to DN/dv.
    D1NuNvRatioIsNull,
    /// DN/dv is negligible compared to DN/du.
    D1NvNuRatioIsNull,
    /// DN/du and DN/dv are parallel.
    D1NuIsParallelD1Nv,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_variants() {
        // Verify all variants exist and can be constructed
        let _ = CSLibNormalStatus::Singular;
        let _ = CSLibNormalStatus::Defined;
        let _ = CSLibNormalStatus::InfinityOfSolutions;
        let _ = CSLibNormalStatus::D1NuIsNull;
        let _ = CSLibNormalStatus::D1NvIsNull;
        let _ = CSLibNormalStatus::D1NIsNull;
        let _ = CSLibNormalStatus::D1NuNvRatioIsNull;
        let _ = CSLibNormalStatus::D1NvNuRatioIsNull;
        let _ = CSLibNormalStatus::D1NuIsParallelD1Nv;
    }

    #[test]
    fn test_enum_equality() {
        assert_eq!(CSLibNormalStatus::Singular, CSLibNormalStatus::Singular);
        assert_ne!(CSLibNormalStatus::Singular, CSLibNormalStatus::Defined);
    }

    #[test]
    fn test_enum_debug() {
        assert_eq!(format!("{:?}", CSLibNormalStatus::Defined), "Defined");
    }
}
