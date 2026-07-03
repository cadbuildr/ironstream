// FILE: hlrb_rep_edge_builder.rs
// occt: HLRBRep_EdgeBuilder

pub struct HlrbrépEdgeBuilder {
    edges: Vec<([f64; 3], [f64; 3])>,
}

impl HlrbrépEdgeBuilder {
    pub fn new() -> Self {
        HlrbrépEdgeBuilder { edges: Vec::new() }
    }

    pub fn add_edge(&mut self, start: [f64; 3], end: [f64; 3]) {
        self.edges.push((start, end));
    }

    pub fn nb_edges(&self) -> usize { self.edges.len() }
    pub fn edge(&self, idx: usize) -> Option<([f64; 3], [f64; 3])> {
        self.edges.get(idx).copied()
    }
}

impl Default for HlrbrépEdgeBuilder {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let eb = HlrbrépEdgeBuilder::new();
        assert_eq!(eb.nb_edges(), 0);
    }
}
