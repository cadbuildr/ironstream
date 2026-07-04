// FILE: t_naming_ptr_attribute.rs
// occt: TNaming_PtrAttribute

/// Pointer to an attribute in the naming framework.
/// TODO: In OCCT, wraps TDF_Attribute*
pub struct TNamingPtrAttribute;

impl TNamingPtrAttribute {
    /// Creates a new pointer attribute.
    pub fn new() -> Self {
        TNamingPtrAttribute
    }
}

impl Default for TNamingPtrAttribute {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ptr_attribute() {
        let _ = TNamingPtrAttribute::new();
    }
}
