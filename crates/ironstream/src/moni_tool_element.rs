// FILE: moni_tool_element.rs
// occt: MoniTool_Element

use std::sync::Arc;

/// Represents an element
pub struct MoniToolElement {
    data: Option<Arc<dyn std::any::Any>>,
}

impl MoniToolElement {
    pub fn new() -> Self {
        MoniToolElement { data: None }
    }

    pub fn with_data(data: Arc<dyn std::any::Any>) -> Self {
        MoniToolElement { data: Some(data) }
    }

    pub fn data(&self) -> Option<Arc<dyn std::any::Any>> {
        self.data.clone()
    }

    pub fn set_data(&mut self, data: Arc<dyn std::any::Any>) {
        self.data = Some(data);
    }

    pub fn clear(&mut self) {
        self.data = None;
    }
}

impl Default for MoniToolElement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let elem = MoniToolElement::new();
        assert!(elem.data().is_none());
    }

    #[test]
    fn test_with_data() {
        let data = Arc::new(42);
        let elem = MoniToolElement::with_data(data);
        assert!(elem.data().is_some());
    }
}
