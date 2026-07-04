// FILE: step_fea_fea_axis2_placement3d.rs
// occt: StepFEA_FeaAxis2Placement3d

/// Representation of STEP entity FeaAxis2Placement3d
#[derive(Debug, Clone)]
pub struct StepFeaFeaAxis2Placement3d {
    name: String,
    system_type: i32,
    description: String,
}

impl StepFeaFeaAxis2Placement3d {
    /// Creates a new empty FeaAxis2Placement3d
    pub fn new() -> Self {
        StepFeaFeaAxis2Placement3d {
            name: String::new(),
            system_type: 0,
            description: String::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, system_type: i32, description: String) {
        self.name = name;
        self.system_type = system_type;
        self.description = description;
    }

    /// Returns field SystemType
    pub fn system_type(&self) -> i32 {
        self.system_type
    }

    /// Set field SystemType
    pub fn set_system_type(&mut self, system_type: i32) {
        self.system_type = system_type;
    }

    /// Returns field Description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Set field Description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Returns field name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set field name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for StepFeaFeaAxis2Placement3d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_axis2_placement3d_creation() {
        let placement = StepFeaFeaAxis2Placement3d::new();
        assert_eq!(placement.name(), "");
        assert_eq!(placement.system_type(), 0);
        assert_eq!(placement.description(), "");
    }

    #[test]
    fn test_fea_axis2_placement3d_init() {
        let mut placement = StepFeaFeaAxis2Placement3d::new();
        placement.init("Placement".to_string(), 1, "Description".to_string());

        assert_eq!(placement.name(), "Placement");
        assert_eq!(placement.system_type(), 1);
        assert_eq!(placement.description(), "Description");
    }

    #[test]
    fn test_fea_axis2_placement3d_setters() {
        let mut placement = StepFeaFeaAxis2Placement3d::new();
        placement.set_name("Test".to_string());
        placement.set_system_type(2);
        placement.set_description("Test Description".to_string());

        assert_eq!(placement.name(), "Test");
        assert_eq!(placement.system_type(), 2);
        assert_eq!(placement.description(), "Test Description");
    }
}
