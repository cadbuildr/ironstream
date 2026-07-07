// FILE: step_visual_tessellated_geometric_set.rs
// occt: StepVisual_TessellatedGeometricSet

use std::sync::Arc;

pub struct HasciiString;
pub struct TessellatedItem;

pub struct TessellatedGeometricSet {
    name: Option<Arc<HasciiString>>,
    items: Option<Arc<Vec<Arc<TessellatedItem>>>>,
}

impl TessellatedGeometricSet {
    pub fn new() -> Self {
        TessellatedGeometricSet {
            name: None,
            items: None,
        }
    }

    pub fn name(&self) -> Option<&Arc<HasciiString>> {
        self.name.as_ref()
    }

    pub fn set_name(&mut self, name: Option<Arc<HasciiString>>) {
        self.name = name;
    }

    pub fn items(&self) -> Option<&Arc<Vec<Arc<TessellatedItem>>>> {
        self.items.as_ref()
    }

    pub fn set_items(&mut self, items: Option<Arc<Vec<Arc<TessellatedItem>>>>) {
        self.items = items;
    }

    pub fn nb_items(&self) -> usize {
        self.items.as_ref().map(|i| i.len()).unwrap_or(0)
    }
}

impl Default for TessellatedGeometricSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tgs = TessellatedGeometricSet::new();
        assert!(tgs.name().is_none());
        assert_eq!(tgs.nb_items(), 0);
    }

    #[test]
    fn test_set_and_get_name() {
        let mut tgs = TessellatedGeometricSet::new();
        let name = Arc::new(HasciiString);
        tgs.set_name(Some(name));
        assert!(tgs.name().is_some());
    }

    #[test]
    fn test_set_and_get_items() {
        let mut tgs = TessellatedGeometricSet::new();
        let items = vec![Arc::new(TessellatedItem)];
        tgs.set_items(Some(Arc::new(items)));
        assert_eq!(tgs.nb_items(), 1);
    }
}
