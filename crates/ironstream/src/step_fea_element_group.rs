// FILE: step_fea_element_group.rs
// occt: StepFEA_ElementGroup

//! Representation of STEP entity ElementGroup in FEA.

use std::rc::Rc;

/// FEA Model reference
#[derive(Debug, Clone)]
pub struct FeaModel {
    id: String,
}

impl FeaModel {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Element representation in FEA
#[derive(Debug, Clone)]
pub struct ElementRepresentation {
    id: String,
    element_type: String,
}

impl ElementRepresentation {
    pub fn new(id: String, element_type: String) -> Self {
        Self { id, element_type }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn element_type(&self) -> &str {
        &self.element_type
    }
}

/// Base FeaGroup structure
#[derive(Debug, Clone)]
pub struct FeaGroup {
    name: Option<String>,
    description: Option<String>,
    model_ref: Option<Rc<FeaModel>>,
}

impl FeaGroup {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            model_ref: None,
        }
    }

    pub fn init(
        &mut self,
        name: String,
        description: String,
        model_ref: Rc<FeaModel>,
    ) {
        self.name = Some(name);
        self.description = Some(description);
        self.model_ref = Some(model_ref);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_description(&mut self, desc: String) {
        self.description = Some(desc);
    }

    pub fn model_ref(&self) -> Option<&Rc<FeaModel>> {
        self.model_ref.as_ref()
    }

    pub fn set_model_ref(&mut self, model: Rc<FeaModel>) {
        self.model_ref = Some(model);
    }
}

impl Default for FeaGroup {
    fn default() -> Self {
        Self::new()
    }
}

/// An ElementGroup is a FeaGroup that contains a collection of elements
#[derive(Debug, Clone)]
pub struct StepFeaElementGroup {
    fea_group: FeaGroup,
    elements: Vec<Rc<ElementRepresentation>>,
}

impl StepFeaElementGroup {
    /// Create a new ElementGroup
    pub fn new() -> Self {
        Self {
            fea_group: FeaGroup::new(),
            elements: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(
        &mut self,
        name: String,
        description: String,
        model_ref: Rc<FeaModel>,
        elements: Vec<Rc<ElementRepresentation>>,
    ) {
        self.fea_group.init(name, description, model_ref);
        self.elements = elements;
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.fea_group.name()
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.fea_group.set_name(name);
    }

    /// Get the description
    pub fn description(&self) -> Option<&str> {
        self.fea_group.description()
    }

    /// Set the description
    pub fn set_description(&mut self, desc: String) {
        self.fea_group.set_description(desc);
    }

    /// Get the model reference
    pub fn model_ref(&self) -> Option<&Rc<FeaModel>> {
        self.fea_group.model_ref()
    }

    /// Set the model reference
    pub fn set_model_ref(&mut self, model: Rc<FeaModel>) {
        self.fea_group.set_model_ref(model);
    }

    /// Get the elements
    pub fn elements(&self) -> &[Rc<ElementRepresentation>] {
        &self.elements
    }

    /// Set the elements
    pub fn set_elements(&mut self, elements: Vec<Rc<ElementRepresentation>>) {
        self.elements = elements;
    }

    /// Get number of elements
    pub fn nb_elements(&self) -> usize {
        self.elements.len()
    }

    /// Get a specific element by index
    pub fn element_value(&self, index: usize) -> Option<&Rc<ElementRepresentation>> {
        self.elements.get(index)
    }
}

impl Default for StepFeaElementGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let eg = StepFeaElementGroup::new();
        assert_eq!(eg.name(), None);
        assert_eq!(eg.nb_elements(), 0);
    }

    #[test]
    fn test_element_representation() {
        let elem = ElementRepresentation::new("ELEM_1".to_string(), "tetrahedron".to_string());
        assert_eq!(elem.id(), "ELEM_1");
        assert_eq!(elem.element_type(), "tetrahedron");
    }

    #[test]
    fn test_init() {
        let mut eg = StepFeaElementGroup::new();
        let model = Rc::new(FeaModel::new("MODEL_1".to_string()));
        let elem1 = Rc::new(ElementRepresentation::new(
            "ELEM_1".to_string(),
            "tetra".to_string(),
        ));
        let elem2 = Rc::new(ElementRepresentation::new(
            "ELEM_2".to_string(),
            "tetra".to_string(),
        ));
        eg.init(
            "group1".to_string(),
            "first group".to_string(),
            model,
            vec![elem1, elem2],
        );
        assert_eq!(eg.name(), Some("group1"));
        assert_eq!(eg.nb_elements(), 2);
    }

    #[test]
    fn test_set_elements() {
        let mut eg = StepFeaElementGroup::new();
        let elem = Rc::new(ElementRepresentation::new("E1".to_string(), "type".to_string()));
        eg.set_elements(vec![elem]);
        assert_eq!(eg.nb_elements(), 1);
    }

    #[test]
    fn test_element_value() {
        let mut eg = StepFeaElementGroup::new();
        let elem = Rc::new(ElementRepresentation::new("E_X".to_string(), "type".to_string()));
        eg.set_elements(vec![elem.clone()]);
        assert!(eg.element_value(0).is_some());
        assert_eq!(eg.element_value(0).unwrap().id(), "E_X");
    }
}
