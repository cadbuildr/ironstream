// FILE: step_fea_symmetric_tensor22d.rs
// occt: StepFEA_SymmetricTensor22d

/// Representation of STEP SELECT type SymmetricTensor22d
#[derive(Debug, Clone)]
pub struct StepFeaSymmetricTensor22d {
    values: Vec<f64>,
}

impl StepFeaSymmetricTensor22d {
    /// Creates a new empty SymmetricTensor22d
    pub fn new() -> Self {
        StepFeaSymmetricTensor22d {
            values: Vec::new(),
        }
    }

    /// Returns AnisotropicSymmetricTensor22d
    pub fn anisotropic_symmetric_tensor22d(&self) -> &[f64] {
        &self.values
    }

    /// Set AnisotropicSymmetricTensor22d
    pub fn set_anisotropic_symmetric_tensor22d(&mut self, values: Vec<f64>) {
        self.values = values;
    }
}

impl Default for StepFeaSymmetricTensor22d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetric_tensor22d_creation() {
        let tensor = StepFeaSymmetricTensor22d::new();
        assert_eq!(tensor.anisotropic_symmetric_tensor22d().len(), 0);
    }

    #[test]
    fn test_symmetric_tensor22d_setters() {
        let mut tensor = StepFeaSymmetricTensor22d::new();
        let values = vec![1.0, 2.0, 3.0];
        tensor.set_anisotropic_symmetric_tensor22d(values);

        assert_eq!(tensor.anisotropic_symmetric_tensor22d(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_symmetric_tensor22d_clone() {
        let mut tensor = StepFeaSymmetricTensor22d::new();
        tensor.set_anisotropic_symmetric_tensor22d(vec![1.0, 2.0]);
        let cloned = tensor.clone();

        assert_eq!(cloned.anisotropic_symmetric_tensor22d(), &[1.0, 2.0]);
    }
}
