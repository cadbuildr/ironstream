// FILE: step_geom_trimming_member.rs
// occt: StepGeom_TrimmingMember

pub struct TrimmingMember {
    name: Option<String>,
    value: f64,
}

impl TrimmingMember {
    pub fn new() -> Self {
        TrimmingMember {
            name: None,
            value: 0.0,
        }
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_name(&mut self, name: Option<String>) -> bool {
        self.name = name;
        true
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trimming_member_creation() {
        let member = TrimmingMember::new();
        assert_eq!(member.has_name(), false);
        assert_eq!(member.value(), 0.0);
    }

    #[test]
    fn test_trimming_member_set_name() {
        let mut member = TrimmingMember::new();
        assert_eq!(member.set_name(Some("PARAMETER_VALUE".to_string())), true);
        assert_eq!(member.has_name(), true);
        assert_eq!(member.name(), Some("PARAMETER_VALUE"));
    }

    #[test]
    fn test_trimming_member_value() {
        let mut member = TrimmingMember::new();
        member.set_value(3.14);
        assert_eq!(member.value(), 3.14);
    }
}
