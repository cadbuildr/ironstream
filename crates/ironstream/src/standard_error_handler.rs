// FILE: standard_error_handler.rs
// occt: Standard_ErrorHandler

//! Support of handling of C signals as C++-style exceptions.
//! This is a simplified Rust implementation without actual setjmp/longjmp.

/// Error handler for catching exceptions
#[derive(Debug)]
pub struct StandardErrorHandler {
    caught_error: Option<String>,
}

impl StandardErrorHandler {
    /// Creates a new error handler
    pub fn new() -> Self {
        StandardErrorHandler {
            caught_error: None,
        }
    }

    /// Destroy and check if there is a raised exception
    pub fn destroy(&mut self) {
        self.caught_error = None;
    }

    /// Throws exception if one was caught
    pub fn raise(&self) -> Result<(), String> {
        if let Some(ref msg) = self.caught_error {
            Err(msg.clone())
        } else {
            Ok(())
        }
    }

    /// Sets the caught error message
    pub fn set_error(&mut self, message: String) {
        self.caught_error = Some(message);
    }

    /// Checks if currently running in try block
    pub fn is_in_try_block() -> bool {
        false // Simplified: always false in Rust version
    }

    /// Gets current error
    pub fn error(&self) -> Option<&str> {
        self.caught_error.as_deref()
    }
}

impl Default for StandardErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Callback trait for error handling
pub trait ErrorCallback {
    /// Called when error handler is destroyed
    fn destroy_callback(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_error_handler_new() {
        let handler = StandardErrorHandler::new();
        assert!(handler.caught_error.is_none());
    }

    #[test]
    fn test_standard_error_handler_set_error() {
        let mut handler = StandardErrorHandler::new();
        handler.set_error("test error".to_string());
        assert_eq!(handler.error(), Some("test error"));
    }

    #[test]
    fn test_standard_error_handler_raise() {
        let mut handler = StandardErrorHandler::new();
        let result = handler.raise();
        assert!(result.is_ok());

        handler.set_error("error".to_string());
        let result = handler.raise();
        assert!(result.is_err());
    }

    #[test]
    fn test_standard_error_handler_is_in_try_block() {
        assert!(!StandardErrorHandler::is_in_try_block());
    }
}
