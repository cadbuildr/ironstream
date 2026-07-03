// FILE: geom_convert_conv_type.rs
// occt: GeomConvert_ConvType

/// Conversion strategy type for geometric conversions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeomConvertConvType {
    /// Target conversion strategy.
    Target = 0,
    /// Simplest conversion strategy.
    Simplest = 1,
    /// Minimum gap conversion strategy.
    MinGap = 2,
}

impl GeomConvertConvType {
    /// Convert from integer value to enum.
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(GeomConvertConvType::Target),
            1 => Some(GeomConvertConvType::Simplest),
            2 => Some(GeomConvertConvType::MinGap),
            _ => None,
        }
    }

    /// Convert enum to integer value.
    pub fn to_i32(self) -> i32 {
        self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conv_type_values() {
        assert_eq!(GeomConvertConvType::Target.to_i32(), 0);
        assert_eq!(GeomConvertConvType::Simplest.to_i32(), 1);
        assert_eq!(GeomConvertConvType::MinGap.to_i32(), 2);
    }

    #[test]
    fn test_conv_type_from_i32() {
        assert_eq!(GeomConvertConvType::from_i32(0), Some(GeomConvertConvType::Target));
        assert_eq!(GeomConvertConvType::from_i32(1), Some(GeomConvertConvType::Simplest));
        assert_eq!(GeomConvertConvType::from_i32(2), Some(GeomConvertConvType::MinGap));
        assert_eq!(GeomConvertConvType::from_i32(3), None);
    }

    #[test]
    fn test_conv_type_equality() {
        let ct1 = GeomConvertConvType::Target;
        let ct2 = GeomConvertConvType::Target;
        let ct3 = GeomConvertConvType::Simplest;

        assert_eq!(ct1, ct2);
        assert_ne!(ct1, ct3);
    }

    #[test]
    fn test_conv_type_debug() {
        let ct = GeomConvertConvType::MinGap;
        let debug_str = format!("{:?}", ct);
        assert!(debug_str.contains("MinGap"));
    }
}
