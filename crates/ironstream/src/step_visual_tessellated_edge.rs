// FILE: step_visual_tessellated_edge.rs
// occt: StepVisual_TessellatedEdge

use std::sync::Arc;

pub struct HasciiString;
pub struct CoordinatesList;

pub struct TessellatedEdge {
    name: Option<Arc<HasciiString>>,
    coordinates: Option<Arc<CoordinatesList>>,
}

impl TessellatedEdge {
    pub fn new() -> Self {
        TessellatedEdge {
            name: None,
            coordinates: None,
        }
    }

    pub fn name(&self) -> Option<&Arc<HasciiString>> {
        self.name.as_ref()
    }

    pub fn set_name(&mut self, name: Option<Arc<HasciiString>>) {
        self.name = name;
    }

    pub fn coordinates(&self) -> Option<&Arc<CoordinatesList>> {
        self.coordinates.as_ref()
    }

    pub fn set_coordinates(&mut self, coords: Option<Arc<CoordinatesList>>) {
        self.coordinates = coords;
    }
}

impl Default for TessellatedEdge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let te = TessellatedEdge::new();
        assert!(te.name().is_none());
        assert!(te.coordinates().is_none());
    }

    #[test]
    fn test_set_and_get_name() {
        let mut te = TessellatedEdge::new();
        let name = Arc::new(HasciiString);
        te.set_name(Some(name));
        assert!(te.name().is_some());
    }

    #[test]
    fn test_set_and_get_coordinates() {
        let mut te = TessellatedEdge::new();
        let coords = Arc::new(CoordinatesList);
        te.set_coordinates(Some(coords));
        assert!(te.coordinates().is_some());
    }
}
