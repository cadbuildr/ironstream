// FILE: b_spl_c_lib_knot_distribution.rs

//! Enumeration describing the repartition of the knots sequence.
//! If all the knots differ by the same positive constant from the preceding knot,
//! the "KnotDistribution" is Uniform; otherwise it is NonUniform.

// occt: BSplCLib_KnotDistribution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnotDistribution {
    NonUniform,
    Uniform,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knot_distribution_creation() {
        let non_uniform = KnotDistribution::NonUniform;
        let uniform = KnotDistribution::Uniform;

        assert_eq!(non_uniform, KnotDistribution::NonUniform);
        assert_eq!(uniform, KnotDistribution::Uniform);
        assert_ne!(non_uniform, uniform);
    }

    #[test]
    fn test_knot_distribution_clone() {
        let dist = KnotDistribution::Uniform;
        let cloned = dist.clone();
        assert_eq!(dist, cloned);
    }

    #[test]
    fn test_knot_distribution_copy() {
        let dist = KnotDistribution::NonUniform;
        let copied = dist;
        assert_eq!(dist, copied);
    }

    #[test]
    fn test_knot_distribution_debug() {
        let dist = KnotDistribution::Uniform;
        let debug_str = format!("{:?}", dist);
        assert_eq!(debug_str, "Uniform");
    }
}
