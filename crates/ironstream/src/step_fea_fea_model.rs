// FILE: step_fea_fea_model.rs
// occt: StepFEA_FeaModel

/// Representation of STEP entity FeaModel
#[derive(Debug, Clone)]
pub struct StepFeaFeaModel {
    name: String,
    creating_software: String,
    intended_analysis_code: Vec<String>,
    description: String,
    analysis_type: String,
}

impl StepFeaFeaModel {
    /// Creates a new empty FeaModel
    pub fn new() -> Self {
        StepFeaFeaModel {
            name: String::new(),
            creating_software: String::new(),
            intended_analysis_code: Vec::new(),
            description: String::new(),
            analysis_type: String::new(),
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        creating_software: String,
        intended_analysis_code: Vec<String>,
        description: String,
        analysis_type: String,
    ) {
        self.name = name;
        self.creating_software = creating_software;
        self.intended_analysis_code = intended_analysis_code;
        self.description = description;
        self.analysis_type = analysis_type;
    }

    /// Returns field CreatingSoftware
    pub fn creating_software(&self) -> &str {
        &self.creating_software
    }

    /// Set field CreatingSoftware
    pub fn set_creating_software(&mut self, creating_software: String) {
        self.creating_software = creating_software;
    }

    /// Returns field IntendedAnalysisCode
    pub fn intended_analysis_code(&self) -> &[String] {
        &self.intended_analysis_code
    }

    /// Set field IntendedAnalysisCode
    pub fn set_intended_analysis_code(&mut self, intended_analysis_code: Vec<String>) {
        self.intended_analysis_code = intended_analysis_code;
    }

    /// Returns field Description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Set field Description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Returns field AnalysisType
    pub fn analysis_type(&self) -> &str {
        &self.analysis_type
    }

    /// Set field AnalysisType
    pub fn set_analysis_type(&mut self, analysis_type: String) {
        self.analysis_type = analysis_type;
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

impl Default for StepFeaFeaModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_model_creation() {
        let model = StepFeaFeaModel::new();
        assert_eq!(model.name(), "");
        assert_eq!(model.creating_software(), "");
        assert_eq!(model.intended_analysis_code().len(), 0);
        assert_eq!(model.description(), "");
        assert_eq!(model.analysis_type(), "");
    }

    #[test]
    fn test_fea_model_init() {
        let mut model = StepFeaFeaModel::new();
        model.init(
            "Model".to_string(),
            "CAD Tool".to_string(),
            vec!["Linear".to_string()],
            "Test Model".to_string(),
            "Structural".to_string(),
        );

        assert_eq!(model.name(), "Model");
        assert_eq!(model.creating_software(), "CAD Tool");
        assert_eq!(model.intended_analysis_code(), &["Linear".to_string()]);
        assert_eq!(model.description(), "Test Model");
        assert_eq!(model.analysis_type(), "Structural");
    }

    #[test]
    fn test_fea_model_setters() {
        let mut model = StepFeaFeaModel::new();
        model.set_name("Test".to_string());
        model.set_creating_software("Software".to_string());
        model.set_intended_analysis_code(vec!["Nonlinear".to_string()]);
        model.set_description("Desc".to_string());
        model.set_analysis_type("Thermal".to_string());

        assert_eq!(model.name(), "Test");
        assert_eq!(model.creating_software(), "Software");
        assert_eq!(model.intended_analysis_code(), &["Nonlinear".to_string()]);
        assert_eq!(model.description(), "Desc");
        assert_eq!(model.analysis_type(), "Thermal");
    }
}
