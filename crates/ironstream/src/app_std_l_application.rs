// FILE: app_std_l_application.rs
// occt: AppStdL_Application

/// Legacy application class defining resources for lite OCAF documents.
/// TODO: Extends TDocStd_Application
pub struct AppStdLApplication;

impl AppStdLApplication {
    /// Returns the file name containing application resources.
    pub fn resources_name() -> &'static str {
        "AppStdL" // Placeholder resource name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_resources_name() {
        let name = AppStdLApplication::resources_name();
        assert_eq!(name, "AppStdL");
    }
}
