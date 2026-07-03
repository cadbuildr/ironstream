// FILE: draft_vertex_info.rs
// occt: Draft_VertexInfo

/// Information container for a vertex in draft operations.
pub struct DraftVertexInfo {
    edges: Vec<usize>,
    params: Vec<f64>,
    current_idx: usize,
}

impl DraftVertexInfo {
    /// Creates a new empty vertex info.
    pub fn new() -> Self {
        DraftVertexInfo {
            edges: Vec::new(),
            params: Vec::new(),
            current_idx: 0,
        }
    }

    /// Adds an edge to the vertex info.
    pub fn add(&mut self, edge_id: usize) {
        self.edges.push(edge_id);
        self.params.push(0.0);
    }

    /// Initializes the edge iterator.
    pub fn init_edge_iterator(&mut self) {
        self.current_idx = 0;
    }

    /// Returns whether there are more edges to iterate.
    pub fn more_edge(&self) -> bool {
        self.current_idx < self.edges.len()
    }

    /// Moves to the next edge.
    pub fn next_edge(&mut self) {
        if self.current_idx < self.edges.len() {
            self.current_idx += 1;
        }
    }

    /// Gets the parameter for a given edge.
    pub fn parameter(&self, edge_id: usize) -> Option<f64> {
        self.edges
            .iter()
            .position(|&id| id == edge_id)
            .map(|idx| self.params[idx])
    }

    /// Changes the parameter for a given edge.
    pub fn change_parameter(&mut self, edge_id: usize, param: f64) -> bool {
        if let Some(idx) = self.edges.iter().position(|&id| id == edge_id) {
            self.params[idx] = param;
            true
        } else {
            false
        }
    }
}

impl Default for DraftVertexInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_info_creation() {
        let info = DraftVertexInfo::new();
        assert!(!info.more_edge());
    }

    #[test]
    fn test_vertex_info_add_edge() {
        let mut info = DraftVertexInfo::new();
        info.add(1);
        info.add(2);
        info.init_edge_iterator();
        assert!(info.more_edge());
    }

    #[test]
    fn test_vertex_info_parameter() {
        let mut info = DraftVertexInfo::new();
        info.add(1);
        info.change_parameter(1, 0.5);
        assert_eq!(info.parameter(1), Some(0.5));
    }

    #[test]
    fn test_vertex_info_iterator() {
        let mut info = DraftVertexInfo::new();
        info.add(1);
        info.add(2);
        info.init_edge_iterator();
        assert!(info.more_edge());
        info.next_edge();
        assert!(info.more_edge());
        info.next_edge();
        assert!(!info.more_edge());
    }
}
