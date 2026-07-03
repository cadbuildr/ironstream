// FILE: extrema_ext_flag_o.rs
// occt: Extrema_ExtFlag

/// Flag for extrema calculation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtFlag {
    /// Calculate minimum distances only.
    Min = 0,
    /// Calculate maximum distances only.
    Max = 1,
    /// Calculate both minimum and maximum distances.
    MinMax = 2,
}

impl ExtFlag {
    /// Create from integer value.
    pub fn from_i32(val: i32) -> Option<Self> {
        match val {
            0 => Some(ExtFlag::Min),
            1 => Some(ExtFlag::Max),
            2 => Some(ExtFlag::MinMax),
            _ => None,
        }
    }

    /// Convert to integer value.
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ext_flag_min() {
        assert_eq!(ExtFlag::Min.as_i32(), 0);
    }

    #[test]
    fn test_ext_flag_max() {
        assert_eq!(ExtFlag::Max.as_i32(), 1);
    }

    #[test]
    fn test_ext_flag_minmax() {
        assert_eq!(ExtFlag::MinMax.as_i32(), 2);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(ExtFlag::from_i32(0), Some(ExtFlag::Min));
        assert_eq!(ExtFlag::from_i32(1), Some(ExtFlag::Max));
        assert_eq!(ExtFlag::from_i32(2), Some(ExtFlag::MinMax));
        assert_eq!(ExtFlag::from_i32(3), None);
    }

    #[test]
    fn test_equality() {
        let flag1 = ExtFlag::Min;
        let flag2 = ExtFlag::Min;
        assert_eq!(flag1, flag2);
    }
}
