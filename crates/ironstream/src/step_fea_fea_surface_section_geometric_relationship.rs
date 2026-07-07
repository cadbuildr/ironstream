// FILE: step_fea_fea_surface_section_geometric_relationship.rs
// occt: StepFEA_FeaSurfaceSectionGeometricRelationship

/// Representation of STEP entity FeaSurfaceSectionGeometricRelationship
#[derive(Debug, Clone)]
pub struct StepFeaFeaSurfaceSectionGeometricRelationship {
    section_ref: Option<i32>,
    item: Option<i32>,
}

impl StepFeaFeaSurfaceSectionGeometricRelationship {
    /// Creates a new empty FeaSurfaceSectionGeometricRelationship
    pub fn new() -> Self {
        StepFeaFeaSurfaceSectionGeometricRelationship {
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

impl Default for StepFeaFeaSurfaceSectionGeometricRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fea_surface_section_creation() {
        let surface_section = StepFeaFeaSurfaceSectionGeometricRelationship::new();
        assert_eq!(surface_section.section_ref(), None);
        assert_eq!(surface_section.item(), None);
    }

    #[test]
    fn test_fea_surface_section_init() {
        let mut surface_section = StepFeaFeaSurfaceSectionGeometricRelationship::new();
        surface_section.init(Some(5), Some(6));

        assert_eq!(surface_section.section_ref(), Some(5));
        assert_eq!(surface_section.item(), Some(6));
    }

    #[test]
    fn test_fea_surface_section_setters() {
        let mut surface_section = StepFeaFeaSurfaceSectionGeometricRelationship::new();
        surface_section.set_section_ref(Some(7));
        surface_section.set_item(Some(8));

        assert_eq!(surface_section.section_ref(), Some(7));
        assert_eq!(surface_section.item(), Some(8));
    }
}
