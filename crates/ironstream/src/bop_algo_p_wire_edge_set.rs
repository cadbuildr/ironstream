// FILE: bop_algo_p_wire_edge_set.rs
// occt: BOPAlgo_PWireEdgeSet

pub struct BopAlgoPWireEdgeSet;

impl BopAlgoPWireEdgeSet {
    pub fn new() -> Self {
        BopAlgoPWireEdgeSet
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BopAlgoPWireEdgeSet::new();
    }
}
