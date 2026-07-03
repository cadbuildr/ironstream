// FILE: standard_assert.rs
// occt: Standard_Assert

/// Assertion utilities for debugging
pub struct StandardAssert;

impl StandardAssert {
    /// Helper function to check a condition
    pub fn check_condition(condition: bool, message: &str) -> Result<(), String> {
        if !condition {
            Err(message.to_string())
        } else {
            Ok(())
        }
    }

    /// Helper function to do nothing (used in assert macros)
    pub fn do_nothing() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_condition_true() {
        let result = StandardAssert::check_condition(true, "should pass");
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_condition_false() {
        let result = StandardAssert::check_condition(false, "should fail");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "should fail");
    }

    #[test]
    fn test_do_nothing() {
        StandardAssert::do_nothing();
    }
}
