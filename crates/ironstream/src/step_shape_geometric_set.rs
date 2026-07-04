// FILE: step_shape_geometric_set.rs
// occt: StepShape_GeometricSet

//! Representation of STEP entity GeometricSet

#[derive(Clone, Debug)]
pub struct GeometricSet {
    name: String,
    elements: Vec<String>, // Placeholder for GeometricSetSelect
}

impl GeometricSet {
    /// Returns a GeometricSet
    pub fn new() -> Self {
        GeometricSet {
            name: String::new(),
            elements: Vec::new(),
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, elements: Vec<String>) {
        self.name = name;
        self.elements = elements;
    }

    /// Set Elements
    pub fn set_elements(&mut self, elements: Vec<String>) {
        self.elements = elements;
    }

    /// Returns Elements
    pub fn elements(&self) -> &[String] {
        &self.elements
    }

    /// Returns value at index (1-based)
    pub fn elements_value(&self, num: usize) -> Option<&String> {
        if num > 0 && num <= self.elements.len() {
            Some(&self.elements[num - 1])
        } else {
            None
        }
    }

    /// Returns the number of elements
    pub fn nb_elements(&self) -> usize {
        self.elements.len()
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

impl Default for GeometricSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let set = GeometricSet::new();
        assert_eq!(set.name(), "");
        assert_eq!(set.nb_elements(), 0);
    }

    #[test]
    fn test_init() {
        let mut set = GeometricSet::new();
        set.init(
            "GeometricSet1".to_string(),
            vec!["elem1".to_string(), "elem2".to_string()],
        );
        assert_eq!(set.name(), "GeometricSet1");
        assert_eq!(set.nb_elements(), 2);
    }

    #[test]
    fn test_elements_value() {
        let mut set = GeometricSet::new();
        set.set_elements(vec!["e1".to_string(), "e2".to_string()]);
        assert_eq!(set.elements_value(1), Some(&"e1".to_string()));
        assert_eq!(set.elements_value(2), Some(&"e2".to_string()));
        assert_eq!(set.elements_value(3), None);
    }
}
