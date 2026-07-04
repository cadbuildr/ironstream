// FILE: step_fea_element_geometric_relationship.rs
// occt: StepFEA_ElementGeometricRelationship

//! Representation of STEP entity ElementGeometricRelationship.

use std::rc::Rc;

/// Analysis item within representation
#[derive(Debug, Clone)]
pub struct AnalysisItemWithinRepresentation {
    id: String,
}

impl AnalysisItemWithinRepresentation {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Element aspect
#[derive(Debug, Clone)]
pub struct ElementAspect {
    aspect_type: String,
}

impl ElementAspect {
    pub fn new(aspect_type: String) -> Self {
        Self { aspect_type }
    }

    pub fn aspect_type(&self) -> &str {
        &self.aspect_type
    }
}

/// Element or element group reference
#[derive(Debug, Clone)]
pub struct ElementOrElementGroup {
    id: String,
    group_type: Option<String>,
}

impl ElementOrElementGroup {
    pub fn new(id: String) -> Self {
        Self {
            id,
            group_type: None,
        }
    }

    pub fn with_group_type(id: String, group_type: String) -> Self {
        Self {
            id,
            group_type: Some(group_type),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn group_type(&self) -> Option<&str> {
        self.group_type.as_deref()
    }
}

/// ElementGeometricRelationship defines the geometric relationship between an element and
/// an analysis item within a representation
#[derive(Debug, Clone)]
pub struct StepFeaElementGeometricRelationship {
    element_ref: Option<ElementOrElementGroup>,
    item: Option<Rc<AnalysisItemWithinRepresentation>>,
    aspect: Option<ElementAspect>,
}

impl StepFeaElementGeometricRelationship {
    /// Create a new ElementGeometricRelationship
    pub fn new() -> Self {
        Self {
            element_ref: None,
            item: None,
            aspect: None,
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        element_ref: ElementOrElementGroup,
        item: Rc<AnalysisItemWithinRepresentation>,
        aspect: ElementAspect,
    ) {
        self.element_ref = Some(element_ref);
        self.item = Some(item);
        self.aspect = Some(aspect);
    }

    /// Get the element reference
    pub fn element_ref(&self) -> Option<&ElementOrElementGroup> {
        self.element_ref.as_ref()
    }

    /// Set the element reference
    pub fn set_element_ref(&mut self, element_ref: ElementOrElementGroup) {
        self.element_ref = Some(element_ref);
    }

    /// Get the item
    pub fn item(&self) -> Option<&Rc<AnalysisItemWithinRepresentation>> {
        self.item.as_ref()
    }

    /// Set the item
    pub fn set_item(&mut self, item: Rc<AnalysisItemWithinRepresentation>) {
        self.item = Some(item);
    }

    /// Get the aspect
    pub fn aspect(&self) -> Option<&ElementAspect> {
        self.aspect.as_ref()
    }

    /// Set the aspect
    pub fn set_aspect(&mut self, aspect: ElementAspect) {
        self.aspect = Some(aspect);
    }
}

impl Default for StepFeaElementGeometricRelationship {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let egr = StepFeaElementGeometricRelationship::new();
        assert_eq!(egr.element_ref(), None);
        assert_eq!(egr.item(), None);
        assert_eq!(egr.aspect(), None);
    }

    #[test]
    fn test_element_or_element_group() {
        let elem = ElementOrElementGroup::new("ELEM_1".to_string());
        assert_eq!(elem.id(), "ELEM_1");
        assert_eq!(elem.group_type(), None);
    }

    #[test]
    fn test_element_with_group_type() {
        let elem = ElementOrElementGroup::with_group_type(
            "GROUP_1".to_string(),
            "volumetric".to_string(),
        );
        assert_eq!(elem.id(), "GROUP_1");
        assert_eq!(elem.group_type(), Some("volumetric"));
    }

    #[test]
    fn test_element_aspect() {
        let aspect = ElementAspect::new("surface".to_string());
        assert_eq!(aspect.aspect_type(), "surface");
    }

    #[test]
    fn test_analysis_item() {
        let item = AnalysisItemWithinRepresentation::new("ITEM_1".to_string());
        assert_eq!(item.id(), "ITEM_1");
    }

    #[test]
    fn test_init() {
        let mut egr = StepFeaElementGeometricRelationship::new();
        let element = ElementOrElementGroup::new("ELEM_X".to_string());
        let item = Rc::new(AnalysisItemWithinRepresentation::new("ITEM_X".to_string()));
        let aspect = ElementAspect::new("solid".to_string());
        egr.init(element, item, aspect);
        assert!(egr.element_ref().is_some());
        assert!(egr.item().is_some());
        assert!(egr.aspect().is_some());
    }

    #[test]
    fn test_set_element_ref() {
        let mut egr = StepFeaElementGeometricRelationship::new();
        let element = ElementOrElementGroup::new("ELEM_Y".to_string());
        egr.set_element_ref(element);
        assert!(egr.element_ref().is_some());
    }

    #[test]
    fn test_set_item() {
        let mut egr = StepFeaElementGeometricRelationship::new();
        let item = Rc::new(AnalysisItemWithinRepresentation::new("ITEM_Y".to_string()));
        egr.set_item(item);
        assert!(egr.item().is_some());
    }

    #[test]
    fn test_set_aspect() {
        let mut egr = StepFeaElementGeometricRelationship::new();
        let aspect = ElementAspect::new("edge".to_string());
        egr.set_aspect(aspect);
        assert!(egr.aspect().is_some());
    }
}
