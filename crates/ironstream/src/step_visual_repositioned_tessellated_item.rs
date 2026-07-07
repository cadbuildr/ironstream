// FILE: step_visual_repositioned_tessellated_item.rs
// occt: StepVisual_RepositionedTessellatedItem

use std::sync::Arc;

pub struct HasciiString;
pub struct Axis2Placement3d;

pub struct RepositionedTessellatedItem {
    name: Option<Arc<HasciiString>>,
    location: Option<Arc<Axis2Placement3d>>,
}

impl RepositionedTessellatedItem {
    pub fn new() -> Self {
        RepositionedTessellatedItem {
            name: None,
            location: None,
        }
    }

    pub fn init(
        &mut self,
        name: Option<Arc<HasciiString>>,
        location: Option<Arc<Axis2Placement3d>>,
    ) {
        self.name = name;
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
}

impl Default for RepositionedTessellatedItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let rti = RepositionedTessellatedItem::new();
        assert!(rti.location().is_none());
        assert!(rti.name().is_none());
    }

    #[test]
    fn test_set_and_get_location() {
        let mut rti = RepositionedTessellatedItem::new();
        let location = Arc::new(Axis2Placement3d);
        rti.set_location(Some(location.clone()));
        assert!(rti.location().is_some());
    }

    #[test]
    fn test_set_and_get_name() {
        let mut rti = RepositionedTessellatedItem::new();
        let name = Arc::new(HasciiString);
        rti.set_name(Some(name.clone()));
        assert!(rti.name().is_some());
    }

    #[test]
    fn test_init() {
        let mut rti = RepositionedTessellatedItem::new();
        let name = Arc::new(HasciiString);
        let location = Arc::new(Axis2Placement3d);
        rti.init(Some(name), Some(location));

        assert!(rti.name().is_some());
        assert!(rti.location().is_some());
    }
}
