// FILE: if_graph_all_shared.rs
// occt: IFGraph_AllShared

/// Interface graph analyzer for shared components
pub struct AllShared {
    shared_count: i32,
}

impl AllShared {
    /// Create a new all shared analyzer
    pub fn new() -> Self {
        AllShared { shared_count: 0 }
    }

    /// Get shared component count
    pub fn shared_count(&self) -> i32 {
        self.shared_count
    }

    /// Add a shared component
    pub fn add_shared(&mut self) {
        self.shared_count += 1;
    }
}

impl Default for AllShared {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let analyzer = AllShared::new();
        assert_eq!(analyzer.shared_count(), 0);
    }

    #[test]
    fn test_add_shared() {
        let mut analyzer = AllShared::new();
        analyzer.add_shared();
        analyzer.add_shared();

        assert_eq!(analyzer.shared_count(), 2);
    }
}
