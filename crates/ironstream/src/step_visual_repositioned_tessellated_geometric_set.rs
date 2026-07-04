// FILE: step_visual_repositioned_tessellated_geometric_set.rs
// occt: StepVisual_RepositionedTessellatedGeometricSet

use std::sync::Arc;

pub struct HasciiString;
pub struct TessellatedItem;
pub struct Axis2Placement3d;

pub struct RepositionedTessellatedGeometricSet {
    name: Option<Arc<HasciiString>>,
    items: Option<Arc<Vec<Arc<TessellatedItem>>>>,
    location: Option<Arc<Axis2Placement3d>>,
}

impl RepositionedTessellatedGeometricSet {
    pub fn new() -> Self {
        RepositionedTessellatedGeometricSet {
            name: None,
            items: None,
            location: None,
        }
    }

    pub fn init(
        &mut self,
        name: Option<Arc<HasciiString>>,
        items: Option<Arc<Vec<Arc<TessellatedItem>>>>,
        location: Option<Arc<Axis2Placement3d>>,
    ) {
        self.name = name;
        self.items = items;
        self.location = location;
    }

    pub fn location(&self) -> Option<&Arc<Axis2Placement3d>> {
        self.location.as_ref()
    }

    pub fn set_location(&mut self, location: Option<Arc<Axis2Placement3d>>) {
        self.location = location;
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
}

impl Default for RepositionedTessellatedGeometricSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let rtgs = RepositionedTessellatedGeometricSet::new();
        assert!(rtgs.location().is_none());
        assert!(rtgs.name().is_none());
        assert!(rtgs.items().is_none());
    }

    #[test]
    fn test_set_and_get_location() {
        let mut rtgs = RepositionedTessellatedGeometricSet::new();
        let location = Arc::new(Axis2Placement3d);
        rtgs.set_location(Some(location.clone()));
        assert!(rtgs.location().is_some());
    }

    #[test]
    fn test_init() {
        let mut rtgs = RepositionedTessellatedGeometricSet::new();
        let name = Arc::new(HasciiString);
        let items = vec![Arc::new(TessellatedItem)];
        let location = Arc::new(Axis2Placement3d);
        rtgs.init(Some(name), Some(Arc::new(items)), Some(location));

        assert!(rtgs.name().is_some());
        assert!(rtgs.items().is_some());
        assert!(rtgs.location().is_some());
    }
}
