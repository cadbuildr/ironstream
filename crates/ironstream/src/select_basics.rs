// FILE: select_basics.rs
// occt: SelectBasics

/// Interface class for dynamic selection management.
/// Defines global constants and utilities for selection operations.
pub struct SelectBasics;

impl SelectBasics {
    /// Returns the maximum owner priority value.
    pub const fn max_owner_priority() -> i32 {
        9
    }

    /// Returns the minimum owner priority value.
    pub const fn min_owner_priority() -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_owner_priority() {
        assert_eq!(SelectBasics::max_owner_priority(), 9);
    }

    #[test]
    fn test_min_owner_priority() {
        assert_eq!(SelectBasics::min_owner_priority(), 0);
    }

    #[test]
    fn test_priority_range() {
        let min = SelectBasics::min_owner_priority();
        let max = SelectBasics::max_owner_priority();
        assert!(min <= max);
        assert_eq!(max - min, 9);
    }
}
