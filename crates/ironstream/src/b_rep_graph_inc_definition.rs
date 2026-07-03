// FILE: b_rep_graph_inc_definition.rs
// occt: BRepGraphInc_Definition

/// Minimal implementation of BRepGraphInc_Definition
pub struct BRepGraphIncDefinition {}

impl Default for BRepGraphIncDefinition {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphIncDefinition::default();
    }
}
