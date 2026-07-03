// FILE: mat_list_of_edge.rs
// occt: MAT_ListOfEdge

/// List of edges in a MAT graph.
#[derive(Debug, Clone)]
pub struct MatListOfEdge {
    nb_items: i32,
}

impl MatListOfEdge {
    pub fn new() -> Self {
        MatListOfEdge { nb_items: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.nb_items == 0
    }

    pub fn number(&self) -> i32 {
        self.nb_items
    }
}

impl Default for MatListOfEdge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_edge() {
        let list = MatListOfEdge::new();
        assert!(list.is_empty());
    }
}
