// FILE: step_geom_elementary_surface.rs
// occt: StepGeom_ElementarySurface

use std::sync::Arc;

#[derive(Clone)]
pub struct ElementarySurface {
    name: Arc<String>,
    position: Option<Arc<String>>,
}

impl ElementarySurface {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
            position: None,
        }
    }

    pub fn init(&mut self, name: String, position: Option<String>) {
        self.name = Arc::new(name);
        self.position = position.map(|p| Arc::new(p));
    }

    pub fn set_position(&mut self, position: String) {
        self.position = Some(Arc::new(position));
    }

    pub fn position(&self) -> Option<String> {
        self.position.as_ref().map(|p| p.as_ref().clone())
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for ElementarySurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let es = ElementarySurface::new();
        assert_eq!(es.name(), "");
    }

    #[test]
    fn test_init() {
        let mut es = ElementarySurface::new();
        es.init("elementary".to_string(), None);
        assert_eq!(es.name(), "elementary");
    }
}
