// FILE: step_geom_geometric_representation_item.rs
// occt: StepGeom_GeometricRepresentationItem

//! Base class for geometric representation items.

#[derive(Debug, Clone)]
pub struct StepGeomGeometricRepresentationItem {
    name: Option<String>,
}

impl StepGeomGeometricRepresentationItem {
    pub fn new() -> Self {
        Self { name: None }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl Default for StepGeomGeometricRepresentationItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let item = StepGeomGeometricRepresentationItem::new();
        assert_eq!(item.name(), None);
    }

    #[test]
    fn test_set_name() {
        let mut item = StepGeomGeometricRepresentationItem::new();
        item.set_name("geom_item".to_string());
        assert_eq!(item.name(), Some("geom_item"));
    }

    #[test]
    fn test_default() {
        let item = StepGeomGeometricRepresentationItem::default();
        assert_eq!(item.name(), None);
    }
}
