// FILE: step_element_unspecified_value.rs
// occt: StepElement_UnspecifiedValue

/// Enumeration for unspecified value marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnspecifiedValue {
    Unspecified,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variant() {
        let val = UnspecifiedValue::Unspecified;
        assert_eq!(val, UnspecifiedValue::Unspecified);
    }

    #[test]
    fn test_copy() {
        let val = UnspecifiedValue::Unspecified;
        let val2 = val;
        assert_eq!(val, val2);
    }

    #[test]
    fn test_debug() {
        let val = UnspecifiedValue::Unspecified;
        assert_eq!(format!("{:?}", val), "Unspecified");
    }
}
