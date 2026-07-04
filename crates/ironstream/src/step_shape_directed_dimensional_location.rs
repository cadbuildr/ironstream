// FILE: step_shape_directed_dimensional_location.rs
// occt: StepShape_DirectedDimensionalLocation

//! Representation of STEP entity DirectedDimensionalLocation

#[derive(Clone, Debug)]
pub struct DirectedDimensionalLocation {}

impl DirectedDimensionalLocation {
    /// Empty constructor
    pub fn new() -> Self {
        DirectedDimensionalLocation {}
    }
}

impl Default for DirectedDimensionalLocation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let loc = DirectedDimensionalLocation::new();
        let _ = loc;
    }

    #[test]
    fn test_default() {
        let loc = DirectedDimensionalLocation::default();
        let _ = loc;
    }
}
