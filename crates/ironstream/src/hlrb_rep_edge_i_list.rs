// FILE: hlrb_rep_edge_i_list.rs
// occt: HLRBRep_EdgeIList

pub struct HlrbrépEdgeIList {
    edges: Vec<i32>,
}

impl HlrbrépEdgeIList {
    pub fn new() -> Self {
        HlrbrépEdgeIList { edges: Vec::new() }
    }

    pub fn add(&mut self, edge: i32) { self.edges.push(edge); }
    pub fn nb_edges(&self) -> usize { self.edges.len() }
    pub fn edge(&self, idx: usize) -> Option<i32> {
        self.edges.get(idx).copied()
    }
    pub fn clear(&mut self) { self.edges.clear(); }
}

impl Default for HlrbrépEdgeIList {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let el = HlrbrépEdgeIList::new();
        assert_eq!(el.nb_edges(), 0);
    }
}
