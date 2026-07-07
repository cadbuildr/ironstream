// FILE: app_std_application.rs
// occt: AppStd_Application

/// AppStd_Application
/// Legacy class defining resources name for standard OCAF documents
pub struct AppStdApplication {
    resources_name: String,
}

impl AppStdApplication {
    /// Create a new AppStd_Application
    pub fn new() -> Self {
        AppStdApplication {
            resources_name: "AppStd".to_string(),
        }
    }

    /// Returns the file name which contains application resources
    pub fn resources_name(&self) -> &str {
        &self.resources_name
    }

    /// Set the resources name
    pub fn set_resources_name(&mut self, name: String) {
        self.resources_name = name;
    }
}

impl Default for AppStdApplication {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let app = AppStdApplication::new();
        assert_eq!(app.resources_name(), "AppStd");
    }

    #[test]
    fn test_default() {
        let app = AppStdApplication::default();
        assert_eq!(app.resources_name(), "AppStd");
    }

    #[test]
    fn test_set_resources_name() {
        let mut app = AppStdApplication::new();
        app.set_resources_name("MyResources".to_string());
        assert_eq!(app.resources_name(), "MyResources");
    }

    #[test]
    fn test_resources_name_consistency() {
        let app = AppStdApplication::new();
        let name = app.resources_name();
        assert!(!name.is_empty());
        assert_eq!(name, "AppStd");
    }
}
