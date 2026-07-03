// FILE: b_rep_graph_inc_parity_orientation.rs
// occt: BRepGraphInc_ParityOrientation

/// Minimal implementation of BRepGraphInc_ParityOrientation
pub struct BRepGraphIncParityOrientation {}

impl Default for BRepGraphIncParityOrientation {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = BRepGraphIncParityOrientation::default();
    }
}
