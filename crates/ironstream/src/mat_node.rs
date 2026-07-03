// FILE: mat_node.rs
// occt: MAT_Node

/// Node of a MAT graph.
#[derive(Debug, Clone)]
pub struct MatNode {
    node_index: i32,
    geom_index: i32,
    distance: f64,
}

impl MatNode {
    pub fn new(geom_index: i32, distance: f64) -> Self {
        MatNode {
            node_index: 0,
            geom_index,
            distance,
        }
    }

    pub fn geom_index(&self) -> i32 {
        self.geom_index
    }

    pub fn index(&self) -> i32 {
        self.node_index
    }

    pub fn set_index(&mut self, idx: i32) {
        self.node_index = idx;
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        let n = MatNode::new(42, 5.5);
        assert_eq!(n.geom_index(), 42);
        assert_eq!(n.distance(), 5.5);
    }
}
