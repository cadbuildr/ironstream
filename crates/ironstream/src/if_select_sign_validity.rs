// FILE: if_select_sign_validity.rs
// occt: IFSelect_SignValidity

/// Signature based on entity validity status.
#[derive(Clone, Debug)]
pub struct IFSelectSignValidity {
    name: String,
}

impl IFSelectSignValidity {
    /// Creates a SignValidity
    pub fn new() -> Self {
        Self {
            name: "Validity".to_string(),
        }
    }

    /// Returns the name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for IFSelectSignValidity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sig = IFSelectSignValidity::new();
        assert_eq!(sig.name(), "Validity");
    }

    #[test]
    fn test_default() {
        let sig = IFSelectSignValidity::default();
        assert_eq!(sig.name(), "Validity");
    }
}
