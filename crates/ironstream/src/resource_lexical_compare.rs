// FILE: resource_lexical_compare.rs
// occt: Resource_LexicalCompare

//! Utilities for lexical comparison of strings.

/// Compares two strings lexicographically
pub fn compare(left: &str, right: &str) -> i32 {
    use std::cmp::Ordering;
    match left.cmp(right) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Case-insensitive lexical comparison
pub fn compare_ignore_case(left: &str, right: &str) -> i32 {
    use std::cmp::Ordering;
    let left_lower = left.to_lowercase();
    let right_lower = right.to_lowercase();
    match left_lower.cmp(&right_lower) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_equal() {
        assert_eq!(compare("abc", "abc"), 0);
    }

    #[test]
    fn test_compare_less() {
        assert_eq!(compare("abc", "def"), -1);
    }

    #[test]
    fn test_compare_greater() {
        assert_eq!(compare("xyz", "abc"), 1);
    }

    #[test]
    fn test_compare_ignore_case_equal() {
        assert_eq!(compare_ignore_case("ABC", "abc"), 0);
    }

    #[test]
    fn test_compare_ignore_case_less() {
        assert_eq!(compare_ignore_case("abc", "DEF"), -1);
    }
}
