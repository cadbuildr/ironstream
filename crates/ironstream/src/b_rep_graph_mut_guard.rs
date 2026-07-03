// FILE: b_rep_graph_mut_guard.rs
// occt: BRepGraph_MutGuard

/// Minimal implementation of BRepGraph_MutGuard
pub struct BRepGraphMutGuard {}

impl Default for BRepGraphMutGuard {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphMutGuard::default();
    }
}
