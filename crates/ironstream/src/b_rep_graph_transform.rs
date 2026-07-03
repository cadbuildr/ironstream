// FILE: b_rep_graph_transform.rs
// occt: BRepGraph_Transform

/// Minimal implementation of BRepGraph_Transform
pub struct BRepGraphTransform {}

impl Default for BRepGraphTransform {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphTransform::default();
    }
}
