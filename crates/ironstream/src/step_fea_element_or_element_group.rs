// FILE: step_fea_element_or_element_group.rs
// occt: StepFEA_ElementOrElementGroup

//! Representation of STEP SELECT type ElementOrElementGroup.

use std::rc::Rc;

/// Element representation
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

/// Element group representation
#[derive(Debug, Clone)]
pub struct ElementGroup {
    id: String,
    group_name: String,
}

impl ElementGroup {
    pub fn new(id: String, group_name: String) -> Self {
        Self { id, group_name }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn group_name(&self) -> &str {
        &self.group_name
    }
}

/// A select type that can hold either an ElementRepresentation or an ElementGroup
#[derive(Debug, Clone)]
pub enum ElementOrElementGroupType {
    ElementRepresentation(Rc<ElementRepresentation>),
    ElementGroup(Rc<ElementGroup>),
}

/// StepFEA_ElementOrElementGroup - a select type entity
#[derive(Debug, Clone)]
pub struct StepFeaElementOrElementGroup {
    value: Option<ElementOrElementGroupType>,
    case_num: i32,
}

impl StepFeaElementOrElementGroup {
    /// Create a new ElementOrElementGroup select type
    pub fn new() -> Self {
        Self {
            value: None,
            case_num: 0,
        }
    }

    /// Recognize the case number
    /// 1 -> ElementRepresentation
    /// 2 -> ElementGroup
    /// 0 -> Unknown
    pub fn case_num(&self) -> i32 {
        self.case_num
    }

    /// Set value as an ElementRepresentation
    pub fn set_element_representation(&mut self, elem: Rc<ElementRepresentation>) {
        self.value = Some(ElementOrElementGroupType::ElementRepresentation(elem));
        self.case_num = 1;
    }

    /// Get value as an ElementRepresentation (returns None if it's an ElementGroup)
    pub fn element_representation(&self) -> Option<Rc<ElementRepresentation>> {
        match &self.value {
            Some(ElementOrElementGroupType::ElementRepresentation(elem)) => Some(elem.clone()),
            _ => None,
        }
    }

    /// Set value as an ElementGroup
    pub fn set_element_group(&mut self, group: Rc<ElementGroup>) {
        self.value = Some(ElementOrElementGroupType::ElementGroup(group));
        self.case_num = 2;
    }

    /// Get value as an ElementGroup (returns None if it's an ElementRepresentation)
    pub fn element_group(&self) -> Option<Rc<ElementGroup>> {
        match &self.value {
            Some(ElementOrElementGroupType::ElementGroup(grp)) => Some(grp.clone()),
            _ => None,
        }
    }

    /// Check if this select has a value
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }
}

impl Default for StepFeaElementOrElementGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sel = StepFeaElementOrElementGroup::new();
        assert_eq!(sel.case_num(), 0);
        assert!(!sel.has_value());
    }

    #[test]
    fn test_element_representation() {
        let mut sel = StepFeaElementOrElementGroup::new();
        let elem = Rc::new(ElementRepresentation::new(
            "ELEM_1".to_string(),
            "tetra".to_string(),
        ));
        sel.set_element_representation(elem.clone());
        assert_eq!(sel.case_num(), 1);
        assert!(sel.has_value());
        assert!(sel.element_representation().is_some());
        assert!(sel.element_group().is_none());
        assert_eq!(sel.element_representation().unwrap().id(), "ELEM_1");
    }

    #[test]
    fn test_element_group() {
        let mut sel = StepFeaElementOrElementGroup::new();
        let group = Rc::new(ElementGroup::new("GROUP_1".to_string(), "group_a".to_string()));
        sel.set_element_group(group.clone());
        assert_eq!(sel.case_num(), 2);
        assert!(sel.has_value());
        assert!(sel.element_group().is_some());
        assert!(sel.element_representation().is_none());
        assert_eq!(sel.element_group().unwrap().id(), "GROUP_1");
    }

    #[test]
    fn test_element_representation_type() {
        let elem = ElementRepresentation::new("E_X".to_string(), "hex".to_string());
        assert_eq!(elem.id(), "E_X");
        assert_eq!(elem.element_type(), "hex");
    }

    #[test]
    fn test_element_group_type() {
        let group = ElementGroup::new("G_Y".to_string(), "main_group".to_string());
        assert_eq!(group.id(), "G_Y");
        assert_eq!(group.group_name(), "main_group");
    }

    #[test]
    fn test_default() {
        let sel = StepFeaElementOrElementGroup::default();
        assert_eq!(sel.case_num(), 0);
    }
}
