// FILE: top_ope_b_rep_build_wire_edge_set.rs
// occt: TopOpeBRepBuild_WireEdgeSet

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildWireEdgeSet {
    edges: Vec<i32>,
    current_index: usize,
}

impl TopOpeBRepBuildWireEdgeSet {
    pub fn new() -> Self {
        TopOpeBRepBuildWireEdgeSet {
            edges: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_edge(&mut self, edge_id: i32) {
        self.edges.push(edge_id);
    }

    pub fn init_edges(&mut self) {
        self.current_index = 0;
    }

    pub fn more_edges(&self) -> bool {
        self.current_index < self.edges.len()
    }

    pub fn next_edge(&mut self) {
        self.current_index += 1;
    }

    pub fn edge(&self) -> Option<i32> {
        if self.current_index < self.edges.len() {
            Some(self.edges[self.current_index])
        } else {
            None
        }
    }

    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }
}

impl Default for TopOpeBRepBuildWireEdgeSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_edge_set_new() {
        let wes = TopOpeBRepBuildWireEdgeSet::new();
        assert_eq!(wes.num_edges(), 0);
    }

    #[test]
    fn test_wire_edge_set_add_edge() {
        let mut wes = TopOpeBRepBuildWireEdgeSet::new();
        wes.add_edge(1);
        wes.add_edge(2);
        assert_eq!(wes.num_edges(), 2);
    }

    #[test]
    fn test_wire_edge_set_iteration() {
        let mut wes = TopOpeBRepBuildWireEdgeSet::new();
        wes.add_edge(5);
        wes.add_edge(15);

        wes.init_edges();
        assert!(wes.more_edges());
        assert_eq!(wes.edge(), Some(5));

        wes.next_edge();
        assert!(wes.more_edges());
        assert_eq!(wes.edge(), Some(15));

        wes.next_edge();
        assert!(!wes.more_edges());
    }

    #[test]
    fn test_wire_edge_set_default() {
        let wes = TopOpeBRepBuildWireEdgeSet::default();
        assert_eq!(wes.num_edges(), 0);
    }
}
