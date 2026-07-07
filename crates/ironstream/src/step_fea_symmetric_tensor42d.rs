// FILE: step_fea_symmetric_tensor42d.rs
// occt: StepFEA_SymmetricTensor42d

/// Representation of STEP SELECT type SymmetricTensor42d
#[derive(Debug, Clone)]
pub struct StepFeaSymmetricTensor42d {
    values: Vec<f64>,
}

impl StepFeaSymmetricTensor42d {
    /// Creates a new empty SymmetricTensor42d
    pub fn new() -> Self {
        StepFeaSymmetricTensor42d {
            values: Vec::new(),
        }
    }

    /// Returns AnisotropicSymmetricTensor42d
    pub fn anisotropic_symmetric_tensor42d(&self) -> &[f64] {
        &self.values
    }

    /// Set AnisotropicSymmetricTensor42d
    pub fn set_anisotropic_symmetric_tensor42d(&mut self, values: Vec<f64>) {
        self.values = values;
    }
}

impl Default for StepFeaSymmetricTensor42d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetric_tensor42d_creation() {
        let tensor = StepFeaSymmetricTensor42d::new();
        assert_eq!(tensor.anisotropic_symmetric_tensor42d().len(), 0);
    }

    #[test]
    fn test_symmetric_tensor42d_setters() {
        let mut tensor = StepFeaSymmetricTensor42d::new();
        let values = vec![1.0, 2.0, 3.0, 4.0];
        tensor.set_anisotropic_symmetric_tensor42d(values);

        assert_eq!(tensor.anisotropic_symmetric_tensor42d(), &[1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_symmetric_tensor42d_clone() {
        let mut tensor = StepFeaSymmetricTensor42d::new();
        tensor.set_anisotropic_symmetric_tensor42d(vec![1.0, 2.0]);
        let cloned = tensor.clone();

        assert_eq!(cloned.anisotropic_symmetric_tensor42d(), &[1.0, 2.0]);
    }
}
