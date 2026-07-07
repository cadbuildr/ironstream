// FILE: if_graph_articulations.rs
// occt: IFGraph_Articulations

/// Interface graph articulation point detector
pub struct Articulations {
    articulation_points: Vec<usize>,
}

impl Articulations {
    /// Create a new articulations analyzer
    pub fn new() -> Self {
        Articulations {
            articulation_points: Vec::new(),
        }
    }

    /// Add an articulation point
    pub fn add_articulation(&mut self, node: usize) {
        if !self.articulation_points.contains(&node) {
            self.articulation_points.push(node);
        }
    }

    /// Get articulation points
    pub fn articulation_points(&self) -> &[usize] {
        &self.articulation_points
    }

    /// Get count of articulation points
    pub fn count(&self) -> usize {
        self.articulation_points.len()
    }
}

impl Default for Articulations {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let analyzer = Articulations::new();
        assert_eq!(analyzer.count(), 0);
    }

    #[test]
    fn test_add_articulation() {
        let mut analyzer = Articulations::new();
        analyzer.add_articulation(2);
        analyzer.add_articulation(5);

        assert_eq!(analyzer.count(), 2);
        assert!(analyzer.articulation_points().contains(&2));
    }
}
