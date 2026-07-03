// FILE: standard_define_exception.rs
// occt: Standard_DefineException

//! Macro for defining exception classes.

/// Base trait for all exceptions
pub trait StandardException: std::error::Error + std::fmt::Display + std::fmt::Debug {
    /// Gets the exception message
    fn message(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestException {
        msg: String,
    }

    impl std::fmt::Display for TestException {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestException: {}", self.msg)
        }
    }

    impl std::error::Error for TestException {}

    impl StandardException for TestException {
        fn message(&self) -> &str {
            &self.msg
        }
    }

    #[test]
    fn test_standard_exception() {
        let exc = TestException {
            msg: "test".to_string(),
        };
        assert_eq!(exc.message(), "test");
    }
}
