// FILE: step_visual_tessellated_face.rs
// occt: StepVisual_TessellatedFace

use std::sync::Arc;

pub struct HasciiString;
pub struct CoordinatesList;

pub struct TessellatedFace {
    name: Option<Arc<HasciiString>>,
    coordinates: Option<Arc<CoordinatesList>>,
}

impl TessellatedFace {
    pub fn new() -> Self {
        TessellatedFace {
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

impl Default for TessellatedFace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tf = TessellatedFace::new();
        assert!(tf.name().is_none());
        assert!(tf.coordinates().is_none());
    }

    #[test]
    fn test_set_and_get_name() {
        let mut tf = TessellatedFace::new();
        let name = Arc::new(HasciiString);
        tf.set_name(Some(name));
        assert!(tf.name().is_some());
    }

    #[test]
    fn test_set_and_get_coordinates() {
        let mut tf = TessellatedFace::new();
        let coords = Arc::new(CoordinatesList);
        tf.set_coordinates(Some(coords));
        assert!(tf.coordinates().is_some());
    }
}
