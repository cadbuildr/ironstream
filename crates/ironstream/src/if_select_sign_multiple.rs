// FILE: if_select_sign_multiple.rs
// occt: IFSelect_SignMultiple

/// Signature handler for multiple/complex signatures.
#[derive(Clone, Debug)]
pub struct IFSelectSignMultiple {
    name: String,
}

impl IFSelectSignMultiple {
    /// Creates a SignMultiple
    pub fn new(name: String) -> Self {
        Self { name }
    }

    /// Returns the name
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sig = IFSelectSignMultiple::new("multi".to_string());
        assert_eq!(sig.name(), "multi");
    }
}
