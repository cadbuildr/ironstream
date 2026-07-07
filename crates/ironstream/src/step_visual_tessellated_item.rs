// FILE: step_visual_tessellated_item.rs
// occt: StepVisual_TessellatedItem

use std::sync::Arc;

pub struct HasciiString;

pub struct TessellatedItem {
    name: Option<Arc<HasciiString>>,
}

impl TessellatedItem {
    pub fn new() -> Self {
        TessellatedItem { name: None }
    }

    pub fn name(&self) -> Option<&Arc<HasciiString>> {
        self.name.as_ref()
    }

    pub fn set_name(&mut self, name: Option<Arc<HasciiString>>) {
        self.name = name;
    }
}

impl Default for TessellatedItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ti = TessellatedItem::new();
        assert!(ti.name().is_none());
    }

    #[test]
    fn test_set_and_get_name() {
        let mut ti = TessellatedItem::new();
        let name = Arc::new(HasciiString);
        ti.set_name(Some(name));
        assert!(ti.name().is_some());
    }
}
