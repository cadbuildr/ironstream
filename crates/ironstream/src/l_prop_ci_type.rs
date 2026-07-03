// FILE: l_prop_ci_type.rs
// occt: LProp_CIType

/// Identifies the type of a particular point on a curve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LPropCIType {
    /// A point of inflection.
    Inflection = 0,
    /// A minimum of curvature.
    MinCur = 1,
    /// A maximum of curvature.
    MaxCur = 2,
}

impl LPropCIType {
    /// Convert from i32 to enum.
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(LPropCIType::Inflection),
            1 => Some(LPropCIType::MinCur),
            2 => Some(LPropCIType::MaxCur),
            _ => None,
        }
    }

    /// Convert enum to i32.
    pub fn to_i32(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ci_type_values() {
        assert_eq!(LPropCIType::Inflection.to_i32(), 0);
        assert_eq!(LPropCIType::MinCur.to_i32(), 1);
        assert_eq!(LPropCIType::MaxCur.to_i32(), 2);
    }

    #[test]
    fn test_ci_type_from_i32() {
        assert_eq!(LPropCIType::from_i32(0), Some(LPropCIType::Inflection));
        assert_eq!(LPropCIType::from_i32(1), Some(LPropCIType::MinCur));
        assert_eq!(LPropCIType::from_i32(2), Some(LPropCIType::MaxCur));
        assert_eq!(LPropCIType::from_i32(3), None);
    }

    #[test]
    fn test_ci_type_debug() {
        let ci = LPropCIType::Inflection;
        let debug_str = format!("{:?}", ci);
        assert!(debug_str.contains("Inflection"));
    }
}
