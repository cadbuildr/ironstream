// FILE: b_rep_graph_inc_instance.rs
// occt: BRepGraphInc_Instance

/// Minimal implementation of BRepGraphInc_Instance
pub struct BRepGraphIncInstance {}

impl Default for BRepGraphIncInstance {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphIncInstance::default();
    }
}
