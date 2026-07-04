// FILE: step_fea_symmetric_tensor43d.rs
// occt: StepFEA_SymmetricTensor43d

//! Representation of STEP SELECT type SymmetricTensor43d (4th order 3D symmetric tensor).

use std::rc::Rc;

/// Types of symmetric tensors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TensorType {
    Anisotropic,
    FeaIsotropic,
    FeaIsoOrthotropic,
    FeaTransverseIsotropic,
    FeaColumnNormalisedOrthotropic,
    FeaColumnNormalisedMonoclinic,
}

impl TensorType {
    pub fn case_num(&self) -> i32 {
        match self {
            TensorType::Anisotropic => 1,
            TensorType::FeaIsotropic => 2,
            TensorType::FeaIsoOrthotropic => 3,
            TensorType::FeaTransverseIsotropic => 4,
            TensorType::FeaColumnNormalisedOrthotropic => 5,
            TensorType::FeaColumnNormalisedMonoclinic => 6,
        }
    }

    pub fn from_case(case: i32) -> Option<TensorType> {
        match case {
            1 => Some(TensorType::Anisotropic),
            2 => Some(TensorType::FeaIsotropic),
            3 => Some(TensorType::FeaIsoOrthotropic),
            4 => Some(TensorType::FeaTransverseIsotropic),
            5 => Some(TensorType::FeaColumnNormalisedOrthotropic),
            6 => Some(TensorType::FeaColumnNormalisedMonoclinic),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TensorType::Anisotropic => "AnisotropicSymmetricTensor43d",
            TensorType::FeaIsotropic => "FeaIsotropicSymmetricTensor43d",
            TensorType::FeaIsoOrthotropic => "FeaIsoOrthotropicSymmetricTensor43d",
            TensorType::FeaTransverseIsotropic => "FeaTransverseIsotropicSymmetricTensor43d",
            TensorType::FeaColumnNormalisedOrthotropic => "FeaColumnNormalisedOrthotropicSymmetricTensor43d",
            TensorType::FeaColumnNormalisedMonoclinic => "FeaColumnNormalisedMonoclinicSymmetricTensor43d",
        }
    }
}

/// A symmetric tensor (4th order, 3D) with its component values
#[derive(Debug, Clone)]
pub struct Tensor {
    components: Vec<f64>,
}

impl Tensor {
    pub fn new(components: Vec<f64>) -> Self {
        Self { components }
    }

    pub fn components(&self) -> &[f64] {
        &self.components
    }

    pub fn set_components(&mut self, components: Vec<f64>) {
        self.components = components;
    }

    pub fn nb_components(&self) -> usize {
        self.components.len()
    }

    pub fn component_value(&self, index: usize) -> Option<f64> {
        self.components.get(index).copied()
    }
}

/// StepFEA_SymmetricTensor43d - a select type entity
#[derive(Debug, Clone)]
pub struct StepFeaSymmetricTensor43d {
    tensor: Option<Rc<Tensor>>,
    tensor_type: Option<TensorType>,
}

impl StepFeaSymmetricTensor43d {
    /// Create a new SymmetricTensor43d
    pub fn new() -> Self {
        Self {
            tensor: None,
            tensor_type: None,
        }
    }

    /// Get the case number (0 if not set)
    pub fn case_num(&self) -> i32 {
        self.tensor_type.map(|t| t.case_num()).unwrap_or(0)
    }

    /// Get the case member (returns the type if set)
    pub fn case_mem(&self) -> i32 {
        self.tensor_type.map(|t| t.case_num()).unwrap_or(0)
    }

    /// Set as AnisotropicSymmetricTensor43d
    pub fn set_anisotropic(&mut self, components: Vec<f64>) {
        self.tensor = Some(Rc::new(Tensor::new(components)));
        self.tensor_type = Some(TensorType::Anisotropic);
    }

    /// Get as AnisotropicSymmetricTensor43d
    pub fn anisotropic(&self) -> Option<&Rc<Tensor>> {
        if matches!(self.tensor_type, Some(TensorType::Anisotropic)) {
            self.tensor.as_ref()
        } else {
            None
        }
    }

    /// Set as FeaIsotropicSymmetricTensor43d
    pub fn set_fea_isotropic(&mut self, components: Vec<f64>) {
        self.tensor = Some(Rc::new(Tensor::new(components)));
        self.tensor_type = Some(TensorType::FeaIsotropic);
    }

    /// Get as FeaIsotropicSymmetricTensor43d
    pub fn fea_isotropic(&self) -> Option<&Rc<Tensor>> {
        if matches!(self.tensor_type, Some(TensorType::FeaIsotropic)) {
            self.tensor.as_ref()
        } else {
            None
        }
    }

    /// Set as FeaIsoOrthotropicSymmetricTensor43d
    pub fn set_fea_iso_orthotropic(&mut self, components: Vec<f64>) {
        self.tensor = Some(Rc::new(Tensor::new(components)));
        self.tensor_type = Some(TensorType::FeaIsoOrthotropic);
    }

    /// Get as FeaIsoOrthotropicSymmetricTensor43d
    pub fn fea_iso_orthotropic(&self) -> Option<&Rc<Tensor>> {
        if matches!(self.tensor_type, Some(TensorType::FeaIsoOrthotropic)) {
            self.tensor.as_ref()
        } else {
            None
        }
    }

    /// Set as FeaTransverseIsotropicSymmetricTensor43d
    pub fn set_fea_transverse_isotropic(&mut self, components: Vec<f64>) {
        self.tensor = Some(Rc::new(Tensor::new(components)));
        self.tensor_type = Some(TensorType::FeaTransverseIsotropic);
    }

    /// Get as FeaTransverseIsotropicSymmetricTensor43d
    pub fn fea_transverse_isotropic(&self) -> Option<&Rc<Tensor>> {
        if matches!(self.tensor_type, Some(TensorType::FeaTransverseIsotropic)) {
            self.tensor.as_ref()
        } else {
            None
        }
    }

    /// Get the underlying tensor if set
    pub fn tensor(&self) -> Option<&Rc<Tensor>> {
        self.tensor.as_ref()
    }

    /// Get the tensor type if set
    pub fn tensor_type(&self) -> Option<TensorType> {
        self.tensor_type
    }

    /// Check if a tensor is set
    pub fn has_value(&self) -> bool {
        self.tensor.is_some()
    }
}

impl Default for StepFeaSymmetricTensor43d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = StepFeaSymmetricTensor43d::new();
        assert_eq!(t.case_num(), 0);
        assert!(!t.has_value());
    }

    #[test]
    fn test_tensor_type_case_num() {
        assert_eq!(TensorType::Anisotropic.case_num(), 1);
        assert_eq!(TensorType::FeaIsotropic.case_num(), 2);
        assert_eq!(TensorType::FeaIsoOrthotropic.case_num(), 3);
    }

    #[test]
    fn test_tensor() {
        let comps = vec![1.0, 2.0, 3.0];
        let tensor = Tensor::new(comps);
        assert_eq!(tensor.nb_components(), 3);
        assert_eq!(tensor.component_value(0), Some(1.0));
        assert_eq!(tensor.component_value(1), Some(2.0));
    }

    #[test]
    fn test_set_anisotropic() {
        let mut t = StepFeaSymmetricTensor43d::new();
        t.set_anisotropic(vec![1.0, 2.0, 3.0]);
        assert_eq!(t.case_num(), 1);
        assert!(t.anisotropic().is_some());
        assert_eq!(t.anisotropic().unwrap().nb_components(), 3);
    }

    #[test]
    fn test_set_fea_isotropic() {
        let mut t = StepFeaSymmetricTensor43d::new();
        t.set_fea_isotropic(vec![5.0, 5.0]);
        assert_eq!(t.case_num(), 2);
        assert!(t.fea_isotropic().is_some());
    }

    #[test]
    fn test_wrong_type_accessor() {
        let mut t = StepFeaSymmetricTensor43d::new();
        t.set_anisotropic(vec![1.0]);
        assert!(t.anisotropic().is_some());
        assert!(t.fea_isotropic().is_none());
    }

    #[test]
    fn test_tensor_type_from_case() {
        assert_eq!(
            TensorType::from_case(1),
            Some(TensorType::Anisotropic)
        );
        assert_eq!(
            TensorType::from_case(2),
            Some(TensorType::FeaIsotropic)
        );
        assert_eq!(TensorType::from_case(0), None);
    }
}
