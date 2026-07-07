// FILE: if_select_sign_category.rs
// occt: IFSelect_SignCategory

/// Returns category information for entities in the signature.
#[derive(Clone, Debug)]
pub struct IFSelectSignCategory {
    name: String,
}

impl IFSelectSignCategory {
    /// Creates a SignCategory
    pub fn new() -> Self {
        Self {
            name: "Category".to_string(),
        }
    }

    /// Returns the name of this signature
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for IFSelectSignCategory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sig = IFSelectSignCategory::new();
        assert_eq!(sig.name(), "Category");
    }

    #[test]
    fn test_default() {
        let sig = IFSelectSignCategory::default();
        assert_eq!(sig.name(), "Category");
    }
}
