// FILE: b_rep_graph_inc_relations.rs
// occt: BRepGraphInc_Relations

/// Minimal implementation of BRepGraphInc_Relations
pub struct BRepGraphIncRelations {}

impl Default for BRepGraphIncRelations {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphIncRelations::default();
    }
}
