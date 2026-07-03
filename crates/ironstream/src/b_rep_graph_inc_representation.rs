// FILE: b_rep_graph_inc_representation.rs
// occt: BRepGraphInc_Representation

/// Minimal implementation of BRepGraphInc_Representation
pub struct BRepGraphIncRepresentation {}

impl Default for BRepGraphIncRepresentation {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphIncRepresentation::default();
    }
}
