// FILE: cdm_application.rs
// occt: CDM_Application

/// The manager returned by this virtual method will be
  //! used to search for Format.Retrieval resour
#[derive(Debug, Clone)]
pub struct CDM_Application {
    // TODO: Port fields from OCCT
}

impl CDM_Application {
    /// Creates a new instance
    pub fn new() -> Self {
        CDM_Application {
        }
    }
}

impl Default for CDM_Application {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdm_application_creation() {
        let obj = CDM_Application::new();
        let _default = CDM_Application::default();
        // TODO: Add more tests from OCCT gtest
    }
}
