// FILE: xcaf_app_application.rs
// occt: XCAFApp_Application

/// XCAFApp_Application is the application root class for XCAF (Extended CAF).
/// Manages creation, storage, and retrieval of extended CAD documents.
#[derive(Clone, Debug)]
pub struct XCAFApp_Application {
    name: String,
}

impl XCAFApp_Application {
    /// Create a new XCAF application.
    pub fn new() -> Self {
        Self {
            name: "XCAFApplication".to_string(),
        }
    }

    /// Get the application name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Create a new document.
    pub fn create_document(&self) -> String {
        "Document".to_string()
    }
}

impl Default for XCAFApp_Application {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_application() {
        let app = XCAFApp_Application::new();
        assert_eq!(app.name(), "XCAFApplication");
    }

    #[test]
    fn test_create_document() {
        let app = XCAFApp_Application::new();
        let doc = app.create_document();
        assert!(!doc.is_empty());
    }

    #[test]
    fn test_default() {
        let app = XCAFApp_Application::default();
        assert_eq!(app.name(), "XCAFApplication");
    }
}
