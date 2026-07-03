// FILE: b_rep_graph_validate.rs
// occt: BRepGraph_Validate

/// Minimal implementation of BRepGraph_Validate
pub struct BRepGraphValidate {}

impl Default for BRepGraphValidate {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphValidate::default();
    }
}
