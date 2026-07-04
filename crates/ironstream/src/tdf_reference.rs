// FILE: tdf_reference.rs
// occt: TDF_Reference

/// Attribute storing a reference to another label.
pub struct TdfReference {
    // TODO: TDF_Label reference_label
}

impl TdfReference {
    /// Creates a new reference attribute.
    pub fn new() -> Self {
        TdfReference {}
    }

    /// Sets the referenced label.
    /// TODO: Accept TDF_Label
    pub fn set_label(&mut self) {
        // TODO: Implement set logic
    }

    /// Gets the referenced label.
    /// TODO: Return TDF_Label
    pub fn label(&self) {
        // TODO: Implement get logic
    }
}

impl Default for TdfReference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference() {
        let reference = TdfReference::new();
        let _ = reference;
    }

    #[test]
    fn test_reference_set() {
        let mut reference = TdfReference::new();
        reference.set_label();
    }
}
