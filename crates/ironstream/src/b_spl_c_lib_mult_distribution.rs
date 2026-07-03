// FILE: b_spl_c_lib_mult_distribution.rs

//! Enumeration describing the form of the sequence of multiplicities.
//!
//! MultDistribution is:
//! - Constant: if all the multiplicities have the same value
//! - QuasiConstant: if all the internal knots have the same multiplicity
//!   and the first and last knot have a different multiplicity
//! - NonConstant: in other cases

// occt: BSplCLib_MultDistribution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultDistribution {
    NonConstant,
    Constant,
    QuasiConstant,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mult_distribution_creation() {
        let non_constant = MultDistribution::NonConstant;
        let constant = MultDistribution::Constant;
        let quasi_constant = MultDistribution::QuasiConstant;

        assert_eq!(non_constant, MultDistribution::NonConstant);
        assert_eq!(constant, MultDistribution::Constant);
        assert_eq!(quasi_constant, MultDistribution::QuasiConstant);
    }

    #[test]
    fn test_mult_distribution_distinctness() {
        assert_ne!(MultDistribution::NonConstant, MultDistribution::Constant);
        assert_ne!(MultDistribution::Constant, MultDistribution::QuasiConstant);
        assert_ne!(MultDistribution::NonConstant, MultDistribution::QuasiConstant);
    }

    #[test]
    fn test_mult_distribution_clone() {
        let dist = MultDistribution::QuasiConstant;
        let cloned = dist.clone();
        assert_eq!(dist, cloned);
    }

    #[test]
    fn test_mult_distribution_copy() {
        let dist = MultDistribution::NonConstant;
        let copied = dist;
        assert_eq!(dist, copied);
    }

    #[test]
    fn test_mult_distribution_debug() {
        let dist = MultDistribution::Constant;
        let debug_str = format!("{:?}", dist);
        assert_eq!(debug_str, "Constant");
    }
}
