// FILE: b_rep_graph_inc_populate.rs
// occt: BRepGraphInc_Populate

/// Minimal implementation of BRepGraphInc_Populate
pub struct BRepGraphIncPopulate {}

impl Default for BRepGraphIncPopulate {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphIncPopulate::default();
    }
}
