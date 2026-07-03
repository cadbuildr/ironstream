// FILE: aspect_window_definition_error.rs
// occt: Aspect_WindowDefinitionError

use std::fmt;

/// Exception raised when a window definition error occurs.
/// This corresponds to OCCT's Aspect_WindowDefinitionError, which inherits from Standard_OutOfRange.
#[derive(Debug, Clone)]
pub struct AspectWindowDefinitionError {
  message: String,
}

impl AspectWindowDefinitionError {
  /// Creates a new window definition error with the given message.
  pub fn new(message: impl Into<String>) -> Self {
    AspectWindowDefinitionError {
      message: message.into(),
    }
  }

  /// Returns the error message.
  pub fn message(&self) -> &str {
    &self.message
  }
}

impl fmt::Display for AspectWindowDefinitionError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Aspect_WindowDefinitionError: {}", self.message)
  }
}

impl std::error::Error for AspectWindowDefinitionError {}

/// Raises an exception if the condition is true.
/// This mirrors the OCCT macro Aspect_WindowDefinitionError_Raise_if.
pub fn raise_if(condition: bool, message: impl Into<String>) -> Result<(), AspectWindowDefinitionError> {
  if condition {
    Err(AspectWindowDefinitionError::new(message))
  } else {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_error_creation() {
    let err = AspectWindowDefinitionError::new("Test error message");
    assert_eq!(err.message(), "Test error message");
  }

  #[test]
  fn test_error_display() {
    let err = AspectWindowDefinitionError::new("Window not properly configured");
    let display_str = format!("{}", err);
    assert!(display_str.contains("Aspect_WindowDefinitionError"));
    assert!(display_str.contains("Window not properly configured"));
  }

  #[test]
  fn test_error_is_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(AspectWindowDefinitionError::new("test"));
    assert_eq!(err.to_string(), "Aspect_WindowDefinitionError: test");
  }

  #[test]
  fn test_raise_if_true() {
    let result = raise_if(true, "This should error");
    assert!(result.is_err());

    if let Err(e) = result {
      assert_eq!(e.message(), "This should error");
    }
  }

  #[test]
  fn test_raise_if_false() {
    let result = raise_if(false, "This should not error");
    assert!(result.is_ok());
  }

  #[test]
  fn test_raise_if_with_condition() {
    let width = 0;
    let height = 100;

    // Should raise if width is invalid
    let result1 = raise_if(width <= 0, "Window width must be positive");
    assert!(result1.is_err());

    // Should not raise if height is valid
    let result2 = raise_if(height <= 0, "Window height must be positive");
    assert!(result2.is_ok());
  }

  #[test]
  fn test_raise_if_chain() {
    let x = 10;
    let y = 20;
    let width = 100;
    let height = 200;

    let result = raise_if(x < 0, "X coordinate invalid")
      .and_then(|_| raise_if(y < 0, "Y coordinate invalid"))
      .and_then(|_| raise_if(width <= 0, "Width must be positive"))
      .and_then(|_| raise_if(height <= 0, "Height must be positive"));

    assert!(result.is_ok());
  }

  #[test]
  fn test_raise_if_chain_with_error() {
    let x = -5;
    let y = 20;
    let width = 100;
    let height = 200;

    let result = raise_if(x < 0, "X coordinate invalid")
      .and_then(|_| raise_if(y < 0, "Y coordinate invalid"))
      .and_then(|_| raise_if(width <= 0, "Width must be positive"))
      .and_then(|_| raise_if(height <= 0, "Height must be positive"));

    assert!(result.is_err());
    if let Err(e) = result {
      assert_eq!(e.message(), "X coordinate invalid");
    }
  }

  #[test]
  fn test_error_clone() {
    let err1 = AspectWindowDefinitionError::new("Original");
    let err2 = err1.clone();
    assert_eq!(err1.message(), err2.message());
  }

  #[test]
  fn test_error_from_string() {
    let msg = String::from("Dynamic message");
    let err = AspectWindowDefinitionError::new(msg);
    assert_eq!(err.message(), "Dynamic message");
  }
}
