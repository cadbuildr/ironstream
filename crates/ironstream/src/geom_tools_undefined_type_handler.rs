// FILE: geom_tools_undefined_type_handler.rs
// occt: GeomTools_UndefinedTypeHandler

/// Handler for undefined geometric types.
pub struct GeomToolsUndefinedTypeHandler {
    error_message: String,
}

impl GeomToolsUndefinedTypeHandler {
    /// Create a handler for undefined types.
    pub fn new() -> Self {
        GeomToolsUndefinedTypeHandler {
            error_message: String::new(),
        }
    }

    /// Handle an undefined type.
    pub fn handle(&mut self, type_name: &str) {
        self.error_message = format!("Undefined type: {}", type_name);
    }

    /// Get the error message.
    pub fn error_message(&self) -> &str {
        &self.error_message
    }

    /// Clear the error message.
    pub fn clear(&mut self) {
        self.error_message.clear();
    }
}

impl Default for GeomToolsUndefinedTypeHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_handler() {
        let handler = GeomToolsUndefinedTypeHandler::new();
        assert_eq!(handler.error_message(), "");
    }

    #[test]
    fn test_handle_undefined() {
        let mut handler = GeomToolsUndefinedTypeHandler::new();
        handler.handle("MyCustomType");
        assert!(handler.error_message().contains("MyCustomType"));
    }

    #[test]
    fn test_clear_error() {
        let mut handler = GeomToolsUndefinedTypeHandler::new();
        handler.handle("SomeType");
        assert!(!handler.error_message().is_empty());
        handler.clear();
        assert_eq!(handler.error_message(), "");
    }
}
