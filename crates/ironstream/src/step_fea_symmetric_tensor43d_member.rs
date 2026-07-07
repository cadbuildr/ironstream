// FILE: step_fea_symmetric_tensor43d_member.rs
// occt: StepFEA_SymmetricTensor43dMember

//! Representation of member for STEP SELECT type SymmetricTensor43d.

/// Member types for symmetric tensor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TensorMemberType {
    Anisotropic,
    FeaIsotropic,
    FeaIsoOrthotropic,
    FeaTransverseIsotropic,
    FeaColumnNormalisedOrthotropic,
    FeaColumnNormalisedMonoclinic,
}

impl TensorMemberType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TensorMemberType::Anisotropic => "AnisotropicSymmetricTensor43d",
            TensorMemberType::FeaIsotropic => "FeaIsotropicSymmetricTensor43d",
            TensorMemberType::FeaIsoOrthotropic => "FeaIsoOrthotropicSymmetricTensor43d",
            TensorMemberType::FeaTransverseIsotropic => "FeaTransverseIsotropicSymmetricTensor43d",
            TensorMemberType::FeaColumnNormalisedOrthotropic => "FeaColumnNormalisedOrthotropicSymmetricTensor43d",
            TensorMemberType::FeaColumnNormalisedMonoclinic => "FeaColumnNormalisedMonoclinicSymmetricTensor43d",
        }
    }

    pub fn from_name(name: &str) -> Option<TensorMemberType> {
        match name {
            "AnisotropicSymmetricTensor43d" => Some(TensorMemberType::Anisotropic),
            "FeaIsotropicSymmetricTensor43d" => Some(TensorMemberType::FeaIsotropic),
            "FeaIsoOrthotropicSymmetricTensor43d" => Some(TensorMemberType::FeaIsoOrthotropic),
            "FeaTransverseIsotropicSymmetricTensor43d" => {
                Some(TensorMemberType::FeaTransverseIsotropic)
            }
            "FeaColumnNormalisedOrthotropicSymmetricTensor43d" => {
                Some(TensorMemberType::FeaColumnNormalisedOrthotropic)
            }
            "FeaColumnNormalisedMonoclinicSymmetricTensor43d" => {
                Some(TensorMemberType::FeaColumnNormalisedMonoclinic)
            }
            _ => None,
        }
    }
}

/// An array of real numbers representing tensor components
#[derive(Debug, Clone)]
pub struct RealArray {
    values: Vec<f64>,
}

impl RealArray {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn with_values(values: Vec<f64>) -> Self {
        Self { values }
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub fn push(&mut self, value: f64) {
        self.values.push(value);
    }

    pub fn set_value(&mut self, index: usize, value: f64) {
        if index < self.values.len() {
            self.values[index] = value;
        }
    }

    pub fn get_value(&self, index: usize) -> Option<f64> {
        self.values.get(index).copied()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl Default for RealArray {
    fn default() -> Self {
        Self::new()
    }
}

/// StepFEA_SymmetricTensor43dMember - a member of the select type
#[derive(Debug, Clone)]
pub struct StepFeaSymmetricTensor43dMember {
    values: RealArray,
    member_type: Option<TensorMemberType>,
}

impl StepFeaSymmetricTensor43dMember {
    /// Create a new SymmetricTensor43dMember
    pub fn new() -> Self {
        Self {
            values: RealArray::new(),
            member_type: None,
        }
    }

    /// Check if has name
    pub fn has_name(&self) -> bool {
        self.member_type.is_some()
    }

    /// Get the member type name
    pub fn name(&self) -> Option<&'static str> {
        self.member_type.map(|t| t.as_str())
    }

    /// Set name based on string
    pub fn set_name(&mut self, name: &str) -> bool {
        if let Some(member_type) = TensorMemberType::from_name(name) {
            self.member_type = Some(member_type);
            true
        } else {
            false
        }
    }

    /// Check if matches a given name
    pub fn matches(&self, name: &str) -> bool {
        if let Some(member_type) = self.member_type {
            member_type.as_str() == name
        } else {
            false
        }
    }

    /// Get the values
    pub fn values(&self) -> &RealArray {
        &self.values
    }

    /// Get mutable values
    pub fn values_mut(&mut self) -> &mut RealArray {
        &mut self.values
    }

    /// Set the values
    pub fn set_values(&mut self, values: RealArray) {
        self.values = values;
    }

    /// Get the member type
    pub fn member_type(&self) -> Option<TensorMemberType> {
        self.member_type
    }

    /// Set the member type
    pub fn set_member_type(&mut self, member_type: TensorMemberType) {
        self.member_type = Some(member_type);
    }
}

impl Default for StepFeaSymmetricTensor43dMember {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m = StepFeaSymmetricTensor43dMember::new();
        assert!(!m.has_name());
        assert_eq!(m.name(), None);
    }

    #[test]
    fn test_tensor_member_type_as_str() {
        assert_eq!(
            TensorMemberType::Anisotropic.as_str(),
            "AnisotropicSymmetricTensor43d"
        );
        assert_eq!(
            TensorMemberType::FeaIsotropic.as_str(),
            "FeaIsotropicSymmetricTensor43d"
        );
    }

    #[test]
    fn test_tensor_member_type_from_name() {
        assert_eq!(
            TensorMemberType::from_name("AnisotropicSymmetricTensor43d"),
            Some(TensorMemberType::Anisotropic)
        );
        assert_eq!(TensorMemberType::from_name("Unknown"), None);
    }

    #[test]
    fn test_real_array() {
        let mut arr = RealArray::new();
        arr.push(1.5);
        arr.push(2.5);
        assert_eq!(arr.len(), 2);
        assert_eq!(arr.get_value(0), Some(1.5));
        assert_eq!(arr.get_value(1), Some(2.5));
    }

    #[test]
    fn test_set_name() {
        let mut m = StepFeaSymmetricTensor43dMember::new();
        assert!(m.set_name("FeaIsotropicSymmetricTensor43d"));
        assert!(m.has_name());
        assert_eq!(m.name(), Some("FeaIsotropicSymmetricTensor43d"));
    }

    #[test]
    fn test_set_invalid_name() {
        let mut m = StepFeaSymmetricTensor43dMember::new();
        assert!(!m.set_name("InvalidName"));
        assert!(!m.has_name());
    }

    #[test]
    fn test_matches() {
        let mut m = StepFeaSymmetricTensor43dMember::new();
        m.set_name("FeaIsotropicSymmetricTensor43d");
        assert!(m.matches("FeaIsotropicSymmetricTensor43d"));
        assert!(!m.matches("AnisotropicSymmetricTensor43d"));
    }

    #[test]
    fn test_set_member_type() {
        let mut m = StepFeaSymmetricTensor43dMember::new();
        m.set_member_type(TensorMemberType::FeaIsoOrthotropic);
        assert_eq!(m.member_type(), Some(TensorMemberType::FeaIsoOrthotropic));
    }
}
