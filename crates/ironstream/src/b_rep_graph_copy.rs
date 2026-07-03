// FILE: b_rep_graph_copy.rs
// occt: BRepGraph_Copy

/// Minimal implementation of BRepGraph_Copy
pub struct BRepGraphCopy {}

impl Default for BRepGraphCopy {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphCopy::default();
    }
}
