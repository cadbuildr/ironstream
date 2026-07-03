// FILE: b_rep_graph_inc_load.rs
// occt: BRepGraphInc_Load

/// Minimal implementation of BRepGraphInc_Load
pub struct BRepGraphIncLoad {}

impl Default for BRepGraphIncLoad {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphIncLoad::default();
    }
}
