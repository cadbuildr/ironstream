// FILE: b_rep_graph_inc_reconstruct.rs
// occt: BRepGraphInc_Reconstruct

/// Minimal implementation of BRepGraphInc_Reconstruct
pub struct BRepGraphIncReconstruct {}

impl Default for BRepGraphIncReconstruct {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphIncReconstruct::default();
    }
}
