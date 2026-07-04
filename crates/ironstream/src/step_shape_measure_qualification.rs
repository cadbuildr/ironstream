// FILE: step_shape_measure_qualification.rs
// occt: StepShape_MeasureQualification

//! Added for Dimensional Tolerances

#[derive(Clone, Debug)]
pub struct MeasureQualification {
    name: String,
    description: String,
    qualified_measure: Option<String>,
    qualifiers: Vec<String>,
}

impl MeasureQualification {
    /// Constructor
    pub fn new() -> Self {
        MeasureQualification {
            name: String::new(),
            description: String::new(),
            qualified_measure: None,
            qualifiers: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        description: String,
        qualified_measure: Option<String>,
        qualifiers: Vec<String>,
    ) {
        self.name = name;
        self.description = description;
        self.qualified_measure = qualified_measure;
        self.qualifiers = qualifiers;
    }

    /// Returns Name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set Name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns Description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Set Description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Returns QualifiedMeasure
    pub fn qualified_measure(&self) -> &Option<String> {
        &self.qualified_measure
    }

    /// Set QualifiedMeasure
    pub fn set_qualified_measure(&mut self, measure: Option<String>) {
        self.qualified_measure = measure;
    }

    /// Returns Qualifiers
    pub fn qualifiers(&self) -> &[String] {
        &self.qualifiers
    }

    /// Returns number of qualifiers
    pub fn nb_qualifiers(&self) -> usize {
        self.qualifiers.len()
    }

    /// Set Qualifiers
    pub fn set_qualifiers(&mut self, qualifiers: Vec<String>) {
        self.qualifiers = qualifiers;
    }

    /// Returns qualifier at index (1-based)
    pub fn qualifiers_value(&self, num: usize) -> Option<&String> {
        if num > 0 && num <= self.qualifiers.len() {
            Some(&self.qualifiers[num - 1])
        } else {
            None
        }
    }

    /// Set qualifier at index (1-based)
    pub fn set_qualifiers_value(&mut self, num: usize, qualifier: String) {
        if num > 0 && num <= self.qualifiers.len() {
            self.qualifiers[num - 1] = qualifier;
        }
    }
}

impl Default for MeasureQualification {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mq = MeasureQualification::new();
        assert_eq!(mq.name(), "");
        assert_eq!(mq.description(), "");
        assert!(mq.qualified_measure().is_none());
        assert_eq!(mq.nb_qualifiers(), 0);
    }

    #[test]
    fn test_init() {
        let mut mq = MeasureQualification::new();
        mq.init(
            "Measure1".to_string(),
            "Description1".to_string(),
            Some("measure1".to_string()),
            vec!["q1".to_string(), "q2".to_string()],
        );
        assert_eq!(mq.name(), "Measure1");
        assert_eq!(mq.description(), "Description1");
        assert_eq!(mq.nb_qualifiers(), 2);
    }

    #[test]
    fn test_qualifiers_value() {
        let mut mq = MeasureQualification::new();
        mq.set_qualifiers(vec!["q1".to_string(), "q2".to_string()]);
        assert_eq!(mq.qualifiers_value(1), Some(&"q1".to_string()));
        assert_eq!(mq.qualifiers_value(2), Some(&"q2".to_string()));
        assert_eq!(mq.qualifiers_value(3), None);
    }
}
