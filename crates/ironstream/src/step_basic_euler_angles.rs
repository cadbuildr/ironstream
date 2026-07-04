// FILE: step_basic_euler_angles.rs
// occt: StepBasic_EulerAngles

/// Representation of STEP entity EulerAngles
#[derive(Clone, Debug)]
pub struct EulerAngles {
    angles: Option<Vec<f64>>,
}

impl EulerAngles {
    /// Empty constructor
    pub fn new() -> Self {
        Self { angles: None }
    }

    /// Initialize all fields
    pub fn init(&mut self, angles: Vec<f64>) {
        self.angles = Some(angles);
    }

    /// Get angles
    pub fn angles(&self) -> Option<&[f64]> {
        self.angles.as_deref()
    }

    /// Set angles
    pub fn set_angles(&mut self, angles: Vec<f64>) {
        self.angles = Some(angles);
    }
}

impl Default for EulerAngles {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let euler = EulerAngles::new();
        assert!(euler.angles().is_none());
    }

    #[test]
    fn test_init() {
        let mut euler = EulerAngles::new();
        let angles = vec![0.0, 45.0, 90.0];
        euler.init(angles.clone());
        assert_eq!(euler.angles(), Some(angles.as_slice()));
    }

    #[test]
    fn test_set_angles() {
        let mut euler = EulerAngles::new();
        let angles = vec![10.0, 20.0, 30.0];
        euler.set_angles(angles.clone());
        assert_eq!(euler.angles(), Some(angles.as_slice()));
    }

    #[test]
    fn test_angles_preserve_values() {
        let mut euler = EulerAngles::new();
        let angles = vec![1.5, 2.5, 3.5];
        euler.init(angles);
        let result = euler.angles().unwrap();
        assert_eq!(result[0], 1.5);
        assert_eq!(result[1], 2.5);
        assert_eq!(result[2], 3.5);
    }

    #[test]
    fn test_default() {
        let euler = EulerAngles::default();
        assert!(euler.angles().is_none());
    }
}
