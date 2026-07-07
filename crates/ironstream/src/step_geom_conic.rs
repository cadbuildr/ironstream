// FILE: step_geom_conic.rs
// occt: StepGeom_Conic

use std::sync::Arc;

#[derive(Clone)]
pub struct Conic {
    name: Arc<String>,
    position: Option<Arc<String>>,
}

impl Conic {
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

impl Default for Conic {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let conic = Conic::new();
        assert_eq!(conic.name(), "");
    }

    #[test]
    fn test_init() {
        let mut conic = Conic::new();
        conic.init("conic1".to_string(), None);
        assert_eq!(conic.name(), "conic1");
    }
}
