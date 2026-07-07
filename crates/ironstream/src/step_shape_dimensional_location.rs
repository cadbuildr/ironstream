// FILE: step_shape_dimensional_location.rs
// occt: StepShape_DimensionalLocation

//! Representation of STEP entity DimensionalLocation

#[derive(Clone, Debug)]
pub struct DimensionalLocation {}

impl DimensionalLocation {
    /// Empty constructor
    pub fn new() -> Self {
        DimensionalLocation {}
    }
}

impl Default for DimensionalLocation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let loc = DimensionalLocation::new();
        let _ = loc;
    }

    #[test]
    fn test_default() {
        let loc = DimensionalLocation::default();
        let _ = loc;
    }
}
