// FILE: convert_parameterisation_type.rs
//
// Faithful port of OCCT Convert_ParameterisationType enum.
// Identifies a type of parameterization of a circle or ellipse represented as a BSpline curve.

// occt: Convert_ParameterisationType
/// Identifies a type of parameterization of a circle or ellipse represented as a BSpline curve.
///
/// For a circle with a center C and a radius R, the natural parameterization is angular,
/// using angle Theta as the parameter: X = R*cos(Theta), Y = R*sin(Theta).
/// Similarly, for an ellipse with major radius R and minor radius r:
/// X = R*cos(Theta), Y = r*sin(Theta).
///
/// The process of converting a circle or ellipse into a BSpline curve transforms the
/// angular parameter Theta into a parameter t. Several types of parametric transformations
/// are available:
///
/// - **TgtThetaOver2**: t = tan(Theta/2), ensuring rational parameterization.
///   Number of spans calculated as (1.2 * Delta / Pi) + 1.
/// - **TgtThetaOver2_N**: Same as TgtThetaOver2 but with N fixed spans (N = 1,2,3,4).
/// - **QuasiAngular**: t is very close to the natural angle Theta.
/// - **RationalC1**: Ensures C1 continuous denominator at span junctions.
/// - **Polynomial**: Non-rational polynomial parameterization (degree 7, 8 poles).
///   This is an approximation rather than an exact representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConvertParameterisationType {
    /// t = tan(Theta / 2), most usual method for rational parameterization
    TgtThetaOver2,
    /// t = tan(Theta / 2) with exactly 1 span
    TgtThetaOver2_1,
    /// t = tan(Theta / 2) with exactly 2 spans
    TgtThetaOver2_2,
    /// t = tan(Theta / 2) with exactly 3 spans
    TgtThetaOver2_3,
    /// t = tan(Theta / 2) with exactly 4 spans
    TgtThetaOver2_4,
    /// Quasi-angular parameterization (t close to natural angle Theta)
    QuasiAngular,
    /// Rational parameterization with C1 continuous denominator
    RationalC1,
    /// Polynomial (non-rational) degree-7 approximation with 8 poles
    Polynomial,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameterization_types_are_distinct() {
        // Ensure all variants are distinct and can be compared
        let tgt = ConvertParameterisationType::TgtThetaOver2;
        let tgt1 = ConvertParameterisationType::TgtThetaOver2_1;
        let tgt2 = ConvertParameterisationType::TgtThetaOver2_2;
        let tgt3 = ConvertParameterisationType::TgtThetaOver2_3;
        let tgt4 = ConvertParameterisationType::TgtThetaOver2_4;
        let quasi = ConvertParameterisationType::QuasiAngular;
        let rational_c1 = ConvertParameterisationType::RationalC1;
        let poly = ConvertParameterisationType::Polynomial;

        assert_ne!(tgt, tgt1);
        assert_ne!(tgt1, tgt2);
        assert_ne!(tgt2, tgt3);
        assert_ne!(tgt3, tgt4);
        assert_ne!(tgt4, quasi);
        assert_ne!(quasi, rational_c1);
        assert_ne!(rational_c1, poly);
    }

    #[test]
    fn test_parameterization_types_clone() {
        let tgt = ConvertParameterisationType::TgtThetaOver2;
        let cloned = tgt.clone();
        assert_eq!(tgt, cloned);
    }

    #[test]
    fn test_parameterization_types_debug() {
        let tgt = ConvertParameterisationType::TgtThetaOver2;
        let debug_str = format!("{:?}", tgt);
        assert!(debug_str.contains("TgtThetaOver2"));
    }

    #[test]
    fn test_parameterization_types_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ConvertParameterisationType::TgtThetaOver2);
        set.insert(ConvertParameterisationType::TgtThetaOver2_1);
        assert_eq!(set.len(), 2);
    }
}
