// FILE: tdf_derived_attribute.rs
// occt: TDF_DerivedAttribute

/// Base class for attributes derived from TDF_Attribute.
/// Provides a template for custom attribute implementations.
pub struct TdfDerivedAttribute;

impl TdfDerivedAttribute {
    /// Creates a new derived attribute.
    pub fn new() -> Self {
        TdfDerivedAttribute
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derived_attribute() {
        let _attr = TdfDerivedAttribute::new();
    }
}
