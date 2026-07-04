// FILE: t_naming_ref_shape.rs
// occt: TNaming_RefShape

/// Reference shape in the naming framework.
/// Stores a shape reference with associated information.
pub struct TNamingRefShape {
    // TODO: TopoDS_Shape my_shape
    // TODO: TNaming_Evolution evolution info
}

impl TNamingRefShape {
    /// Creates a new reference shape.
    pub fn new() -> Self {
        TNamingRefShape {}
    }
}

impl Default for TNamingRefShape {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ref_shape() {
        let _ref_shape = TNamingRefShape::new();
    }

    #[test]
    fn test_ref_shape_default() {
        let _ref_shape = TNamingRefShape::default();
    }
}
