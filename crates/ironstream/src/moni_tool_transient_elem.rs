// FILE: moni_tool_transient_elem.rs
// occt: MoniTool_TransientElem

use std::sync::Arc;

/// A transient element
pub struct MoniToolTransientElem {
    data: Option<Arc<dyn std::any::Any>>,
}

impl MoniToolTransientElem {
    pub fn new() -> Self {
        MoniToolTransientElem { data: None }
    }

    pub fn with_data(data: Arc<dyn std::any::Any>) -> Self {
        MoniToolTransientElem { data: Some(data) }
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

impl Default for MoniToolTransientElem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let elem = MoniToolTransientElem::new();
        assert!(elem.data().is_none());
    }

    #[test]
    fn test_with_data() {
        let data = Arc::new(42);
        let elem = MoniToolTransientElem::with_data(data);
        assert!(elem.data().is_some());
    }
}
