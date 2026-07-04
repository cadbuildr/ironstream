// FILE: if_graph_compare.rs
// occt: IFGraph_Compare

/// Interface graph comparison analyzer
pub struct Compare {
    equal: bool,
}

impl Compare {
    /// Create a new comparison analyzer
    pub fn new() -> Self {
        Compare { equal: true }
    }

    /// Check if graphs are equal
    pub fn is_equal(&self) -> bool {
        self.equal
    }

    /// Set equality result
    pub fn set_equal(&mut self, equal: bool) {
        self.equal = equal;
    }
}

impl Default for Compare {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let analyzer = Compare::new();
        assert!(analyzer.is_equal());
    }

    #[test]
    fn test_set_equal() {
        let mut analyzer = Compare::new();
        analyzer.set_equal(false);
        assert!(!analyzer.is_equal());
    }
}
