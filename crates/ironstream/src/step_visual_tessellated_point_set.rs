// FILE: step_visual_tessellated_point_set.rs
// occt: StepVisual_TessellatedPointSet

use std::sync::Arc;

pub struct HasciiString;
pub struct CoordinatesList;

pub struct TessellatedPointSet {
    name: Option<Arc<HasciiString>>,
    coordinates: Option<Arc<CoordinatesList>>,
}

impl TessellatedPointSet {
    pub fn new() -> Self {
        TessellatedPointSet {
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

impl Default for TessellatedPointSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tps = TessellatedPointSet::new();
        assert!(tps.name().is_none());
        assert!(tps.coordinates().is_none());
    }

    #[test]
    fn test_set_and_get_name() {
        let mut tps = TessellatedPointSet::new();
        let name = Arc::new(HasciiString);
        tps.set_name(Some(name));
        assert!(tps.name().is_some());
    }

    #[test]
    fn test_set_and_get_coordinates() {
        let mut tps = TessellatedPointSet::new();
        let coords = Arc::new(CoordinatesList);
        tps.set_coordinates(Some(coords));
        assert!(tps.coordinates().is_some());
    }
}
