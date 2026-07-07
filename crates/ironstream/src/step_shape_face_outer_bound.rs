// FILE: step_shape_face_outer_bound.rs
// occt: StepShape_FaceOuterBound

//! Representation of STEP entity FaceOuterBound

#[derive(Clone, Debug)]
pub struct FaceOuterBound {
    name: String,
    bound: Option<String>,
    orientation: bool,
}

impl FaceOuterBound {
    /// Returns a FaceOuterBound
    pub fn new() -> Self {
        FaceOuterBound {
            name: String::new(),
            bound: None,
            orientation: false,
        }
    }

    /// Initialize all fields (inherited)
    pub fn init(&mut self, name: String, bound: Option<String>, orientation: bool) {
        self.name = name;
        self.bound = bound;
        self.orientation = orientation;
    }

    /// Set Bound (inherited)
    pub fn set_bound(&mut self, bound: Option<String>) {
        self.bound = bound;
    }

    /// Returns Bound (inherited)
    pub fn bound(&self) -> &Option<String> {
        &self.bound
    }

    /// Set Orientation (inherited)
    pub fn set_orientation(&mut self, orientation: bool) {
        self.orientation = orientation;
    }

    /// Returns Orientation (inherited)
    pub fn orientation(&self) -> bool {
        self.orientation
    }

    /// Returns name field (inherited)
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set name field (inherited)
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for FaceOuterBound {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fob = FaceOuterBound::new();
        assert_eq!(fob.name(), "");
        assert!(!fob.orientation());
        assert!(fob.bound().is_none());
    }

    #[test]
    fn test_init() {
        let mut fob = FaceOuterBound::new();
        fob.init("OuterBound1".to_string(), Some("loop1".to_string()), true);
        assert_eq!(fob.name(), "OuterBound1");
        assert!(fob.orientation());
    }

    #[test]
    fn test_inherited_methods() {
        let mut fob = FaceOuterBound::new();
        fob.set_orientation(false);
        assert!(!fob.orientation());
    }
}
