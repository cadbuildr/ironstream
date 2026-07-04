// FILE: if_select_general_modifier.rs
// occt: IFSelect_GeneralModifier

#[derive(Clone, Debug)]
pub struct IfSelectGeneralModifier {
    name: String,
}

impl IfSelectGeneralModifier {
    pub fn new(name: &str) -> Self {
        IfSelectGeneralModifier {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn apply(&self) -> bool {
        true
    }
}

impl Default for IfSelectGeneralModifier {
    fn default() -> Self {
        Self::new("modifier")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let m = IfSelectGeneralModifier::new("test");
        assert_eq!(m.name(), "test");
    }

    #[test]
    fn test_apply() {
        let m = IfSelectGeneralModifier::new("test");
        assert!(m.apply());
    }
}
