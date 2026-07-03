// FILE: b_rep_graph_inc_reference.rs
// occt: BRepGraphInc_Reference

/// Minimal implementation of BRepGraphInc_Reference
pub struct BRepGraphIncReference {}

impl Default for BRepGraphIncReference {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphIncReference::default();
    }
}
