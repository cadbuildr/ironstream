// FILE: if_graph_all_connected.rs
// occt: IFGraph_AllConnected

/// Interface graph analyzer for all connected components
pub struct AllConnected {
    visited: Vec<bool>,
}

impl AllConnected {
    /// Create a new all connected analyzer
    pub fn new(num_nodes: usize) -> Self {
        AllConnected {
            visited: vec![false; num_nodes],
        }
    }

    /// Mark a node as visited
    pub fn visit(&mut self, node: usize) {
        if node < self.visited.len() {
            self.visited[node] = true;
        }
    }

    /// Check if a node is visited
    pub fn is_visited(&self, node: usize) -> bool {
        self.visited.get(node).copied().unwrap_or(false)
    }

    /// Get the number of visited nodes
    pub fn visited_count(&self) -> usize {
        self.visited.iter().filter(|&&v| v).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let analyzer = AllConnected::new(5);
        assert_eq!(analyzer.visited_count(), 0);
    }

    #[test]
    fn test_visit() {
        let mut analyzer = AllConnected::new(5);
        analyzer.visit(2);
        assert!(analyzer.is_visited(2));
        assert!(!analyzer.is_visited(1));
    }

    #[test]
    fn test_visited_count() {
        let mut analyzer = AllConnected::new(5);
        analyzer.visit(0);
        analyzer.visit(2);
        analyzer.visit(4);

        assert_eq!(analyzer.visited_count(), 3);
    }
}
