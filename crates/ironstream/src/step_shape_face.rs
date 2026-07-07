// FILE: step_shape_face.rs
// occt: StepShape_Face

//! Representation of STEP entity Face

#[derive(Clone, Debug)]
pub struct Face {
    name: String,
    bounds: Vec<String>, // Placeholder for FaceBound handles
}

impl Face {
    /// Returns a Face
    pub fn new() -> Self {
        Face {
            name: String::new(),
            bounds: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, bounds: Vec<String>) {
        self.name = name;
        self.bounds = bounds;
    }

    /// Set Bounds
    pub fn set_bounds(&mut self, bounds: Vec<String>) {
        self.bounds = bounds;
    }

    /// Returns Bounds
    pub fn bounds(&self) -> &[String] {
        &self.bounds
    }

    /// Returns value at index (1-based)
    pub fn bounds_value(&self, num: usize) -> Option<&String> {
        if num > 0 && num <= self.bounds.len() {
            Some(&self.bounds[num - 1])
        } else {
            None
        }
    }

    /// Returns the number of bounds
    pub fn nb_bounds(&self) -> usize {
        self.bounds.len()
    }

    /// Returns name field
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for Face {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let face = Face::new();
        assert_eq!(face.name(), "");
        assert_eq!(face.nb_bounds(), 0);
    }

    #[test]
    fn test_init() {
        let mut face = Face::new();
        face.init(
            "Face1".to_string(),
            vec!["bound1".to_string(), "bound2".to_string()],
        );
        assert_eq!(face.name(), "Face1");
        assert_eq!(face.nb_bounds(), 2);
    }

    #[test]
    fn test_bounds_value() {
        let mut face = Face::new();
        face.set_bounds(vec!["b1".to_string(), "b2".to_string(), "b3".to_string()]);
        assert_eq!(face.bounds_value(1), Some(&"b1".to_string()));
        assert_eq!(face.bounds_value(3), Some(&"b3".to_string()));
        assert_eq!(face.bounds_value(4), None);
    }

    #[test]
    fn test_set_bounds() {
        let mut face = Face::new();
        face.set_bounds(vec!["bound1".to_string()]);
        assert_eq!(face.nb_bounds(), 1);
    }
}
