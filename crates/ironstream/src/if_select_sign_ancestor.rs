// FILE: if_select_sign_ancestor.rs
// occt: IFSelect_SignAncestor

/// Returns ancestor information for entities in the signature.
#[derive(Clone, Debug)]
pub struct IFSelectSignAncestor {
    name: String,
}

impl IFSelectSignAncestor {
    /// Creates a SignAncestor
    pub fn new() -> Self {
        Self {
            name: "Ancestor".to_string(),
        }
    }

    /// Returns the name of this signature
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for IFSelectSignAncestor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sig = IFSelectSignAncestor::new();
        assert_eq!(sig.name(), "Ancestor");
    }

    #[test]
    fn test_default() {
        let sig = IFSelectSignAncestor::default();
        assert_eq!(sig.name(), "Ancestor");
    }
}
