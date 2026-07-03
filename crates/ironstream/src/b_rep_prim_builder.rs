// FILE: b_rep_prim_builder.rs
// occt: BRepPrim_Builder

/// Implements abstract Builder with BRep Builder.
#[derive(Debug, Clone)]
pub struct BRepPrimBuilder;

impl BRepPrimBuilder {
    pub fn new() -> Self {
        BRepPrimBuilder
    }
}

impl Default for BRepPrimBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let _b = BRepPrimBuilder::new();
    }
}
