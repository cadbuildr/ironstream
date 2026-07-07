// FILE: hlr_test_out_liner.rs
// occt: HLRTest_OutLiner

//! Outliner for HLR test results.

#[derive(Clone, Debug)]
pub struct OutLiner {
    pub outline_id: usize,
    pub edges: Vec<usize>,
}

impl OutLiner {
    pub fn new(outline_id: usize) -> Self {
        OutLiner {
            outline_id,
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, edge_id: usize) {
        self.edges.push(edge_id);
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn get_outline(&self) -> String {
        format!("Outline {} with {} edges", self.outline_id, self.edge_count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let outliner = OutLiner::new(1);
        assert_eq!(outliner.outline_id, 1);
        assert_eq!(outliner.edge_count(), 0);
    }

    #[test]
    fn test_add_edge() {
        let mut outliner = OutLiner::new(1);
        outliner.add_edge(10);
        outliner.add_edge(20);

        assert_eq!(outliner.edge_count(), 2);
        assert_eq!(outliner.edges[0], 10);
    }

    #[test]
    fn test_get_outline() {
        let mut outliner = OutLiner::new(5);
        outliner.add_edge(1);
        outliner.add_edge(2);

        let result = outliner.get_outline();
        assert!(result.contains("5"));
        assert!(result.contains("2 edges"));
    }

    #[test]
    fn test_empty_outline() {
        let outliner = OutLiner::new(1);
        let result = outliner.get_outline();
        assert!(result.contains("0 edges"));
    }
}
