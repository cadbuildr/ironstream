// FILE: step_fea_symmetric_tensor23d_member.rs
// occt: StepFEA_SymmetricTensor23dMember

/// Representation of member for STEP SELECT type SymmetricTensor23d
#[derive(Debug, Clone)]
pub struct StepFeaSymmetricTensor23dMember {
    mycase: i32,
}

impl StepFeaSymmetricTensor23dMember {
    /// Creates a new empty SymmetricTensor23dMember
    pub fn new() -> Self {
        StepFeaSymmetricTensor23dMember { mycase: 0 }
    }

    /// Returns True if has name
    pub fn has_name(&self) -> bool {
        true
    }

    /// Returns set name
    pub fn name(&self) -> &str {
        match self.mycase {
            1 => "IsotropicSymmetricTensor23d",
            2 => "OrthotropicSymmetricTensor23d",
            3 => "AnisotropicSymmetricTensor23d",
            _ => "",
        }
    }

    /// Set name
    pub fn set_name(&mut self, name: &str) -> bool {
        match name {
            "IsotropicSymmetricTensor23d" => {
                self.mycase = 1;
                true
            }
            "OrthotropicSymmetricTensor23d" => {
                self.mycase = 2;
                true
            }
            "AnisotropicSymmetricTensor23d" => {
                self.mycase = 3;
                true
            }
            _ => false,
        }
    }

    /// Tells if the name matches
    pub fn matches(&self, name: &str) -> bool {
        self.name() == name
    }

    /// Returns the current case
    pub fn mycase(&self) -> i32 {
        self.mycase
    }

    /// Set the case
    pub fn set_mycase(&mut self, case: i32) {
        self.mycase = case;
    }
}

impl Default for StepFeaSymmetricTensor23dMember {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetric_tensor23d_member_creation() {
        let member = StepFeaSymmetricTensor23dMember::new();
        assert!(member.has_name());
        assert_eq!(member.name(), "");
        assert_eq!(member.mycase(), 0);
    }

    #[test]
    fn test_symmetric_tensor23d_member_isotropic() {
        let mut member = StepFeaSymmetricTensor23dMember::new();
        assert!(member.set_name("IsotropicSymmetricTensor23d"));
        assert_eq!(member.name(), "IsotropicSymmetricTensor23d");
        assert_eq!(member.mycase(), 1);
        assert!(member.matches("IsotropicSymmetricTensor23d"));
    }

    #[test]
    fn test_symmetric_tensor23d_member_orthotropic() {
        let mut member = StepFeaSymmetricTensor23dMember::new();
        assert!(member.set_name("OrthotropicSymmetricTensor23d"));
        assert_eq!(member.name(), "OrthotropicSymmetricTensor23d");
        assert_eq!(member.mycase(), 2);
    }

    #[test]
    fn test_symmetric_tensor23d_member_anisotropic() {
        let mut member = StepFeaSymmetricTensor23dMember::new();
        assert!(member.set_name("AnisotropicSymmetricTensor23d"));
        assert_eq!(member.name(), "AnisotropicSymmetricTensor23d");
        assert_eq!(member.mycase(), 3);
    }

    #[test]
    fn test_symmetric_tensor23d_member_invalid() {
        let mut member = StepFeaSymmetricTensor23dMember::new();
        assert!(!member.set_name("InvalidName"));
    }
}
