// FILE: if_select_sign_type.rs
// occt: IFSelect_SignType

/// Signature based on entity type.
#[derive(Clone, Debug)]
pub struct IFSelectSignType {
    name: String,
}

impl IFSelectSignType {
    /// Creates a SignType
    pub fn new() -> Self {
        Self {
            name: "Type".to_string(),
        }
    }

    /// Returns the name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for IFSelectSignType {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sig = IFSelectSignType::new();
        assert_eq!(sig.name(), "Type");
    }

    #[test]
    fn test_default() {
        let sig = IFSelectSignType::default();
        assert_eq!(sig.name(), "Type");
    }
}
