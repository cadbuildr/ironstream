// FILE: if_select_modifier.rs
// occt: IFSelect_Modifier

#[derive(Clone, Debug)]
pub struct IfSelectModifier {
    name: String,
}

impl IfSelectModifier {
    pub fn new(name: &str) -> Self {
        IfSelectModifier {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn perform(&self) -> bool {
        true
    }
}

impl Default for IfSelectModifier {
    fn default() -> Self {
        Self::new("modifier")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let m = IfSelectModifier::new("test");
        assert_eq!(m.name(), "test");
    }

    #[test]
    fn test_perform() {
        let m = IfSelectModifier::new("test");
        assert!(m.perform());
    }
}
