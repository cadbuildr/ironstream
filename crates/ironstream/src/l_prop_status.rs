// FILE: l_prop_status.rs
// occt: LProp_Status

/// Status of local property computation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LPropStatus {
    /// Undecided status.
    Undecided = 0,
    /// Undefined property.
    Undefined = 1,
    /// Defined property.
    Defined = 2,
    /// Computed property.
    Computed = 3,
}

impl LPropStatus {
    /// Convert from i32 to enum.
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(LPropStatus::Undecided),
            1 => Some(LPropStatus::Undefined),
            2 => Some(LPropStatus::Defined),
            3 => Some(LPropStatus::Computed),
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
    fn test_status_values() {
        assert_eq!(LPropStatus::Undecided.to_i32(), 0);
        assert_eq!(LPropStatus::Undefined.to_i32(), 1);
        assert_eq!(LPropStatus::Defined.to_i32(), 2);
        assert_eq!(LPropStatus::Computed.to_i32(), 3);
    }

    #[test]
    fn test_status_from_i32() {
        assert_eq!(LPropStatus::from_i32(0), Some(LPropStatus::Undecided));
        assert_eq!(LPropStatus::from_i32(1), Some(LPropStatus::Undefined));
        assert_eq!(LPropStatus::from_i32(2), Some(LPropStatus::Defined));
        assert_eq!(LPropStatus::from_i32(3), Some(LPropStatus::Computed));
        assert_eq!(LPropStatus::from_i32(4), None);
    }

    #[test]
    fn test_status_ordering() {
        assert!(LPropStatus::Undecided < LPropStatus::Undefined);
        assert!(LPropStatus::Undefined < LPropStatus::Defined);
        assert!(LPropStatus::Defined < LPropStatus::Computed);
    }

    #[test]
    fn test_status_debug() {
        let s = LPropStatus::Computed;
        let debug_str = format!("{:?}", s);
        assert!(debug_str.contains("Computed"));
    }
}
