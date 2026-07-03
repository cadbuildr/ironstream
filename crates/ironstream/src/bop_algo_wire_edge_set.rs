// FILE: bop_algo_wire_edge_set.rs
// occt: BOPAlgo_WireEdgeSet

pub struct BopAlgoWireEdgeSet;

impl BopAlgoWireEdgeSet {
    pub fn new() -> Self {
        BopAlgoWireEdgeSet
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoWireEdgeSet::new();
    }
}
