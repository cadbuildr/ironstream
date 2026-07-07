// FILE: moni_tool_sequence_of_element.rs
// occt: MoniTool_SequenceOfElement

use std::rc::Rc;
use std::cell::RefCell;

/// MoniTool_Element represents an element in the monitoring tool.
#[derive(Clone, Debug)]
pub struct MoniToolElement {
    id: i32,
    name: String,
    value: String,
}

impl MoniToolElement {
    pub fn new(id: i32, name: String, value: String) -> Self {
        MoniToolElement { id, name, value }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

/// A handle/reference-counted wrapper for MoniTool_Element.
pub type MoniToolElementHandle = Rc<RefCell<MoniToolElement>>;

/// Deprecated typedef alias for backward compatibility.
/// Original OCCT: `NCollection_Sequence<opencascade::handle<MoniTool_Element>>`
pub type MoniToolSequenceOfElement = Vec<MoniToolElementHandle>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_creation() {
        let element = MoniToolElement::new(1, "Name".to_string(), "Value".to_string());
        assert_eq!(element.id(), 1);
        assert_eq!(element.name(), "Name");
        assert_eq!(element.value(), "Value");
    }

    #[test]
    fn test_element_set_value() {
        let mut element = MoniToolElement::new(1, "Test".to_string(), "OldValue".to_string());
        assert_eq!(element.value(), "OldValue");

        element.set_value("NewValue".to_string());
        assert_eq!(element.value(), "NewValue");
    }

    #[test]
    fn test_sequence_creation() {
        let sequence: MoniToolSequenceOfElement = Vec::new();
        assert!(sequence.is_empty());
        assert_eq!(sequence.len(), 0);
    }

    #[test]
    fn test_sequence_push() {
        let mut sequence: MoniToolSequenceOfElement = Vec::new();

        let elem1 = Rc::new(RefCell::new(MoniToolElement::new(
            1,
            "Element1".to_string(),
            "Value1".to_string(),
        )));
        let elem2 = Rc::new(RefCell::new(MoniToolElement::new(
            2,
            "Element2".to_string(),
            "Value2".to_string(),
        )));

        sequence.push(elem1.clone());
        sequence.push(elem2.clone());

        assert_eq!(sequence.len(), 2);
        assert_eq!(sequence[0].borrow().id(), 1);
        assert_eq!(sequence[1].borrow().id(), 2);
    }

    #[test]
    fn test_sequence_access() {
        let mut sequence: MoniToolSequenceOfElement = Vec::new();

        let elem = Rc::new(RefCell::new(MoniToolElement::new(
            42,
            "TestElement".to_string(),
            "TestValue".to_string(),
        )));
        sequence.push(elem.clone());

        let retrieved = sequence.get(0).unwrap();
        assert_eq!(retrieved.borrow().id(), 42);
        assert_eq!(retrieved.borrow().name(), "TestElement");
    }

    #[test]
    fn test_sequence_iteration() {
        let mut sequence: MoniToolSequenceOfElement = Vec::new();

        for i in 1..=5 {
            let elem = Rc::new(RefCell::new(MoniToolElement::new(
                i,
                format!("Elem{}", i),
                format!("Val{}", i),
            )));
            sequence.push(elem);
        }

        assert_eq!(sequence.len(), 5);

        let mut ids = Vec::new();
        for elem_handle in &sequence {
            ids.push(elem_handle.borrow().id());
        }
        assert_eq!(ids, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sequence_remove() {
        let mut sequence: MoniToolSequenceOfElement = Vec::new();

        let elem1 = Rc::new(RefCell::new(MoniToolElement::new(
            1,
            "E1".to_string(),
            "V1".to_string(),
        )));
        let elem2 = Rc::new(RefCell::new(MoniToolElement::new(
            2,
            "E2".to_string(),
            "V2".to_string(),
        )));
        let elem3 = Rc::new(RefCell::new(MoniToolElement::new(
            3,
            "E3".to_string(),
            "V3".to_string(),
        )));

        sequence.push(elem1.clone());
        sequence.push(elem2.clone());
        sequence.push(elem3.clone());

        assert_eq!(sequence.len(), 3);
        sequence.remove(1);
        assert_eq!(sequence.len(), 2);
        assert_eq!(sequence[0].borrow().id(), 1);
        assert_eq!(sequence[1].borrow().id(), 3);
    }

    #[test]
    fn test_sequence_clear() {
        let mut sequence: MoniToolSequenceOfElement = Vec::new();

        let elem1 = Rc::new(RefCell::new(MoniToolElement::new(
            1,
            "E1".to_string(),
            "V1".to_string(),
        )));
        let elem2 = Rc::new(RefCell::new(MoniToolElement::new(
            2,
            "E2".to_string(),
            "V2".to_string(),
        )));

        sequence.push(elem1);
        sequence.push(elem2);
        assert_eq!(sequence.len(), 2);

        sequence.clear();
        assert_eq!(sequence.len(), 0);
        assert!(sequence.is_empty());
    }
}
