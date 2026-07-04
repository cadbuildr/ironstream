// FILE: step_fea_symmetric_tensor23d.rs
// occt: StepFEA_SymmetricTensor23d

/// Representation of STEP SELECT type SymmetricTensor23d
#[derive(Debug, Clone)]
pub enum StepFeaSymmetricTensor23d {
    IsotropicSymmetricTensor23d(f64),
    OrthotropicSymmetricTensor23d(Vec<f64>),
    AnisotropicSymmetricTensor23d(Vec<f64>),
}

impl StepFeaSymmetricTensor23d {
    /// Returns IsotropicSymmetricTensor23d value
    pub fn isotropic_symmetric_tensor23d(&self) -> Option<f64> {
        match self {
            StepFeaSymmetricTensor23d::IsotropicSymmetricTensor23d(val) => Some(*val),
            _ => None,
        }
    }

    /// Returns OrthotropicSymmetricTensor23d value
    pub fn orthotropic_symmetric_tensor23d(&self) -> Option<&[f64]> {
        match self {
            StepFeaSymmetricTensor23d::OrthotropicSymmetricTensor23d(val) => Some(val),
            _ => None,
        }
    }

    /// Returns AnisotropicSymmetricTensor23d value
    pub fn anisotropic_symmetric_tensor23d(&self) -> Option<&[f64]> {
        match self {
            StepFeaSymmetricTensor23d::AnisotropicSymmetricTensor23d(val) => Some(val),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetric_tensor23d_isotropic() {
        let tensor = StepFeaSymmetricTensor23d::IsotropicSymmetricTensor23d(1.5);
        assert_eq!(tensor.isotropic_symmetric_tensor23d(), Some(1.5));
        assert_eq!(tensor.orthotropic_symmetric_tensor23d(), None);
        assert_eq!(tensor.anisotropic_symmetric_tensor23d(), None);
    }

    #[test]
    fn test_symmetric_tensor23d_orthotropic() {
        let values = vec![1.0, 2.0, 3.0];
        let tensor = StepFeaSymmetricTensor23d::OrthotropicSymmetricTensor23d(values);
        assert_eq!(tensor.isotropic_symmetric_tensor23d(), None);
        assert_eq!(tensor.orthotropic_symmetric_tensor23d(), Some(&[1.0, 2.0, 3.0][..]));
        assert_eq!(tensor.anisotropic_symmetric_tensor23d(), None);
    }

    #[test]
    fn test_symmetric_tensor23d_anisotropic() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let tensor = StepFeaSymmetricTensor23d::AnisotropicSymmetricTensor23d(values);
        assert_eq!(tensor.isotropic_symmetric_tensor23d(), None);
        assert_eq!(tensor.orthotropic_symmetric_tensor23d(), None);
        assert_eq!(tensor.anisotropic_symmetric_tensor23d(), Some(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0][..]));
    }

    #[test]
    fn test_symmetric_tensor23d_clone() {
        let tensor = StepFeaSymmetricTensor23d::IsotropicSymmetricTensor23d(2.5);
        let cloned = tensor.clone();
        assert_eq!(cloned.isotropic_symmetric_tensor23d(), Some(2.5));
    }
}
