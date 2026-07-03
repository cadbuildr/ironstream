// FILE: mat_t_list_node_of_list_of_edge.rs
// occt: MAT_TListNodeOfListOfEdge

/// List node for edge lists.
#[derive(Debug, Clone)]
pub struct MatTListNodeOfListOfEdge;

impl MatTListNodeOfListOfEdge {
    pub fn new() -> Self {
        MatTListNodeOfListOfEdge
    }
}

impl Default for MatTListNodeOfListOfEdge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_node() {
        let _ln = MatTListNodeOfListOfEdge::new();
    }
}
