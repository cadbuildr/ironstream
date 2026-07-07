// FILE: if_graph_connected_componants.rs
// occt: IFGraph_ConnectedComponants

/// Interface graph connected components analyzer
pub struct ConnectedComponants {
    components: Vec<Vec<usize>>,
}

impl ConnectedComponants {
    /// Create a new connected components analyzer
    pub fn new() -> Self {
        ConnectedComponants {
            components: Vec::new(),
        }
    }

    /// Add a component (group of connected nodes)
    pub fn add_component(&mut self, nodes: Vec<usize>) {
        self.components.push(nodes);
    }

    /// Get the number of components
    pub fn num_components(&self) -> usize {
        self.components.len()
    }

    /// Get a component by index
    pub fn component(&self, index: usize) -> Option<&[usize]> {
        self.components.get(index).map(|v| v.as_slice())
    }

    /// Get all components
    pub fn components(&self) -> &[Vec<usize>] {
        &self.components
    }
}

impl Default for ConnectedComponants {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let analyzer = ConnectedComponants::new();
        assert_eq!(analyzer.num_components(), 0);
    }

    #[test]
    fn test_add_component() {
        let mut analyzer = ConnectedComponants::new();
        analyzer.add_component(vec![1, 2, 3]);
        analyzer.add_component(vec![4, 5]);

        assert_eq!(analyzer.num_components(), 2);
        assert_eq!(analyzer.component(0), Some(&[1, 2, 3][..]));
    }
}
