// FILE: step_fea_degree_of_freedom_member.rs
// occt: StepFEA_DegreeOfFreedomMember

/// Member for STEP type DegreeOfFreedom with named access.
#[derive(Clone, Debug)]
pub struct DegreeOfFreedomMember {
    mycase: i32,
    name: Option<String>,
}

impl DegreeOfFreedomMember {
    pub fn new() -> Self {
        Self {
            mycase: 0,
            name: None,
        }
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name(&mut self, name: &str) -> bool {
        let valid_names = [
            "XTranslation",
            "YTranslation",
            "ZTranslation",
            "XRotation",
            "YRotation",
            "ZRotation",
        ];

        if valid_names.contains(&name) {
            self.name = Some(name.to_string());
            self.mycase = valid_names
                .iter()
                .position(|&n| n == name)
                .map(|p| (p + 1) as i32)
                .unwrap_or(0);
            true
        } else {
            false
        }
    }

    pub fn matches(&self, name: &str) -> bool {
        self.name.as_deref() == Some(name)
    }

    pub fn mycase(&self) -> i32 {
        self.mycase
    }
}

impl Default for DegreeOfFreedomMember {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_member() {
        let member = DegreeOfFreedomMember::new();
        assert!(!member.has_name());
    }

    #[test]
    fn test_set_name_valid() {
        let mut member = DegreeOfFreedomMember::new();
        assert!(member.set_name("XRotation"));
        assert_eq!(member.name(), Some("XRotation"));
    }

    #[test]
    fn test_set_name_invalid() {
        let mut member = DegreeOfFreedomMember::new();
        assert!(!member.set_name("InvalidDOF"));
        assert!(member.name().is_none());
    }
}
