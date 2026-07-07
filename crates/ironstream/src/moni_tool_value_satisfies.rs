// FILE: moni_tool_value_satisfies.rs
// occt: MoniTool_ValueSatisfies

/// A function pointer type for value satisfaction checking.
/// Takes an optional string value and returns whether it satisfies some constraint.
pub type MoniToolValueSatisfies = fn(Option<&str>) -> bool;

/// Default satisfies function - always returns true (no constraint).
pub fn always_satisfies(_val: Option<&str>) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_satisfies() {
        assert!(always_satisfies(None));
        assert!(always_satisfies(Some("")));
        assert!(always_satisfies(Some("test")));
    }

    #[test]
    fn test_custom_satisfies() {
        let non_empty: MoniToolValueSatisfies = |val| val.map_or(false, |s| !s.is_empty());

        assert!(!non_empty(None));
        assert!(!non_empty(Some("")));
        assert!(non_empty(Some("value")));
    }

    #[test]
    fn test_satisfies_numeric() {
        let is_numeric: MoniToolValueSatisfies = |val| {
            val.map_or(false, |s| s.parse::<f64>().is_ok())
        };

        assert!(!is_numeric(None));
        assert!(is_numeric(Some("42")));
        assert!(is_numeric(Some("3.14")));
        assert!(!is_numeric(Some("abc")));
    }
}
