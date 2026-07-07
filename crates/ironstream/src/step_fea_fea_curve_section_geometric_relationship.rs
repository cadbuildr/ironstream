// FILE: step_fea_fea_curve_section_geometric_relationship.rs
// occt: StepFEA_FeaCurveSectionGeometricRelationship

/// Representation of STEP entity FeaCurveSectionGeometricRelationship
#[derive(Debug, Clone)]
pub struct StepFeaFeaCurveSectionGeometricRelationship {
    section_ref: Option<i32>,
    item: Option<i32>,
}

impl StepFeaFeaCurveSectionGeometricRelationship {
    /// Creates a new empty FeaCurveSectionGeometricRelationship
    pub fn new() -> Self {
        StepFeaFeaCurveSectionGeometricRelationship {
            section_ref: None,
            item: None,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, section_ref: Option<i32>, item: Option<i32>) {
        self.section_ref = section_ref;
        self.item = item;
    }

    /// Returns field SectionRef
    pub fn section_ref(&self) -> Option<i32> {
        self.section_ref
    }

    /// Set field SectionRef
    pub fn set_section_ref(&mut self, section_ref: Option<i32>) {
        self.section_ref = section_ref;
    }

    /// Returns field Item
    pub fn item(&self) -> Option<i32> {
        self.item
    }

    /// Set field Item
    pub fn set_item(&mut self, item: Option<i32>) {
        self.item = item;
    }
}

impl Default for StepFeaFeaCurveSectionGeometricRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_curve_section_creation() {
        let curve_section = StepFeaFeaCurveSectionGeometricRelationship::new();
        assert_eq!(curve_section.section_ref(), None);
        assert_eq!(curve_section.item(), None);
    }

    #[test]
    fn test_fea_curve_section_init() {
        let mut curve_section = StepFeaFeaCurveSectionGeometricRelationship::new();
        curve_section.init(Some(1), Some(2));

        assert_eq!(curve_section.section_ref(), Some(1));
        assert_eq!(curve_section.item(), Some(2));
    }

    #[test]
    fn test_fea_curve_section_setters() {
        let mut curve_section = StepFeaFeaCurveSectionGeometricRelationship::new();
        curve_section.set_section_ref(Some(3));
        curve_section.set_item(Some(4));

        assert_eq!(curve_section.section_ref(), Some(3));
        assert_eq!(curve_section.item(), Some(4));
    }
}
