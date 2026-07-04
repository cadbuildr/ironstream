// FILE: step_shape_face_bound.rs
// occt: StepShape_FaceBound

//! Representation of STEP entity FaceBound

#[derive(Clone, Debug)]
pub struct FaceBound {
    name: String,
    bound: Option<String>, // Placeholder for Loop handle
    orientation: bool,
}

impl FaceBound {
    /// Returns a FaceBound
    pub fn new() -> Self {
        FaceBound {
            name: String::new(),
            bound: None,
            orientation: false,
        }
    }

    /// Initialize all fields
    pub fn init(&mut self, name: String, bound: Option<String>, orientation: bool) {
        self.name = name;
        self.bound = bound;
        self.orientation = orientation;
    }

    /// Set Bound
    pub fn set_bound(&mut self, bound: Option<String>) {
        self.bound = bound;
    }

    /// Returns Bound
    pub fn bound(&self) -> &Option<String> {
        &self.bound
    }

    /// Set Orientation
    pub fn set_orientation(&mut self, orientation: bool) {
        self.orientation = orientation;
    }

    /// Returns Orientation
    pub fn orientation(&self) -> bool {
        self.orientation
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

impl Default for FaceBound {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fb = FaceBound::new();
        assert_eq!(fb.name(), "");
        assert!(!fb.orientation());
        assert!(fb.bound().is_none());
    }

    #[test]
    fn test_init() {
        let mut fb = FaceBound::new();
        fb.init("Bound1".to_string(), Some("loop1".to_string()), true);
        assert_eq!(fb.name(), "Bound1");
        assert!(fb.orientation());
        assert_eq!(fb.bound(), &Some("loop1".to_string()));
    }

    #[test]
    fn test_set_orientation() {
        let mut fb = FaceBound::new();
        fb.set_orientation(true);
        assert!(fb.orientation());
    }

    #[test]
    fn test_set_bound() {
        let mut fb = FaceBound::new();
        fb.set_bound(Some("loop1".to_string()));
        assert_eq!(fb.bound(), &Some("loop1".to_string()));
    }
}
