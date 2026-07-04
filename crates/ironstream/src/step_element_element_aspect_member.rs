// FILE: step_element_element_aspect_member.rs
// occt: StepElement_ElementAspectMember

/// Member of STEP SELECT type ElementAspect with named access.
#[derive(Clone, Debug)]
pub struct ElementAspectMember {
    mycase: i32,
    name: Option<String>,
}

impl ElementAspectMember {
    /// Creates a new ElementAspectMember.
    pub fn new() -> Self {
        Self {
            mycase: 0,
            name: None,
        }
    }

    /// Returns true if the member has a name set.
    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    /// Returns the set name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Sets the name from a string.
    pub fn set_name(&mut self, name: &str) -> bool {
        let valid_names = [
            "ElementVolume",
            "Volume3dFace",
            "Volume2dFace",
            "Volume3dEdge",
            "Volume2dEdge",
            "Surface3dFace",
            "Surface2dFace",
            "Surface3dEdge",
            "Surface2dEdge",
            "CurveEdge",
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

    /// Tells if the name matches a given string.
    pub fn matches(&self, name: &str) -> bool {
        self.name.as_deref() == Some(name)
    }

    pub fn mycase(&self) -> i32 {
        self.mycase
    }
}

impl Default for ElementAspectMember {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_member() {
        let member = ElementAspectMember::new();
        assert!(!member.has_name());
        assert!(member.name().is_none());
    }

    #[test]
    fn test_set_name_valid() {
        let mut member = ElementAspectMember::new();
        assert!(member.set_name("ElementVolume"));
        assert_eq!(member.name(), Some("ElementVolume"));
        assert_eq!(member.mycase(), 1);
    }

    #[test]
    fn test_set_name_invalid() {
        let mut member = ElementAspectMember::new();
        assert!(!member.set_name("InvalidName"));
        assert!(member.name().is_none());
    }

    #[test]
    fn test_matches() {
        let mut member = ElementAspectMember::new();
        member.set_name("CurveEdge");
        assert!(member.matches("CurveEdge"));
        assert!(!member.matches("ElementVolume"));
    }

    #[test]
    fn test_all_valid_names() {
        let names = [
            "ElementVolume",
            "Volume3dFace",
            "Volume2dFace",
            "Volume3dEdge",
            "Volume2dEdge",
            "Surface3dFace",
            "Surface2dFace",
            "Surface3dEdge",
            "Surface2dEdge",
            "CurveEdge",
        ];

        for (idx, name) in names.iter().enumerate() {
            let mut member = ElementAspectMember::new();
            assert!(member.set_name(name));
            assert_eq!(member.mycase(), (idx + 1) as i32);
        }
    }
}
