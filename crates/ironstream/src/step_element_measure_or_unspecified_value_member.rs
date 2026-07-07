// FILE: step_element_measure_or_unspecified_value_member.rs
// occt: StepElement_MeasureOrUnspecifiedValueMember

/// Member of STEP SELECT type MeasureOrUnspecifiedValue with named access.
#[derive(Clone, Debug)]
pub struct MeasureOrUnspecifiedValueMember {
    mycase: i32,
    name: Option<String>,
}

impl MeasureOrUnspecifiedValueMember {
    /// Creates a new MeasureOrUnspecifiedValueMember.
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
        let valid_names = ["ContextDependentMeasure", "UnspecifiedValue"];

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

impl Default for MeasureOrUnspecifiedValueMember {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_member() {
        let member = MeasureOrUnspecifiedValueMember::new();
        assert!(!member.has_name());
        assert!(member.name().is_none());
    }

    #[test]
    fn test_set_name_measure() {
        let mut member = MeasureOrUnspecifiedValueMember::new();
        assert!(member.set_name("ContextDependentMeasure"));
        assert_eq!(member.name(), Some("ContextDependentMeasure"));
        assert_eq!(member.mycase(), 1);
    }

    #[test]
    fn test_set_name_unspecified() {
        let mut member = MeasureOrUnspecifiedValueMember::new();
        assert!(member.set_name("UnspecifiedValue"));
        assert_eq!(member.name(), Some("UnspecifiedValue"));
        assert_eq!(member.mycase(), 2);
    }

    #[test]
    fn test_set_name_invalid() {
        let mut member = MeasureOrUnspecifiedValueMember::new();
        assert!(!member.set_name("InvalidName"));
        assert!(member.name().is_none());
    }

    #[test]
    fn test_matches() {
        let mut member = MeasureOrUnspecifiedValueMember::new();
        member.set_name("UnspecifiedValue");
        assert!(member.matches("UnspecifiedValue"));
        assert!(!member.matches("ContextDependentMeasure"));
    }
}
