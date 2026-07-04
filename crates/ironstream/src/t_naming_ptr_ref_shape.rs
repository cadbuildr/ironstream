// FILE: t_naming_ptr_ref_shape.rs
// occt: TNaming_PtrRefShape

/// Pointer to a reference shape in the naming framework.
/// TODO: In OCCT, wraps TNaming_RefShape*
pub struct TNamingPtrRefShape;

impl TNamingPtrRefShape {
    /// Creates a new reference shape pointer.
    pub fn new() -> Self {
        TNamingPtrRefShape
    }
}

impl Default for TNamingPtrRefShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ptr_ref_shape() {
        let _ = TNamingPtrRefShape::new();
    }
}
