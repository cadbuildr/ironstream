// FILE: b_rep_graph_usage_path.rs
// occt: BRepGraph_UsagePath

/// Represents a path of usage/reference between graph items.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BRepGraphUsagePath {
    path: Vec<u32>,
}

impl BRepGraphUsagePath {
    /// Create a new empty usage path.
    pub fn new() -> Self {
        Self { path: Vec::new() }
    }

    /// Create a path from a vector of item ids.
    pub fn from_vec(items: Vec<u32>) -> Self {
        Self { path: items }
    }

    /// Push an item onto the path.
    pub fn push(&mut self, item_id: u32) {
        self.path.push(item_id);
    }

    /// Pop an item from the path.
    pub fn pop(&mut self) -> Option<u32> {
        self.path.pop()
    }

    /// Get the length of the path.
    pub fn len(&self) -> usize {
        self.path.len()
    }

    /// Check if the path is empty.
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    /// Get the path as a slice.
    pub fn as_slice(&self) -> &[u32] {
        &self.path
    }

    /// Get an item at index.
    pub fn get(&self, index: usize) -> Option<u32> {
        self.path.get(index).copied()
    }

    /// Check if path contains an item.
    pub fn contains(&self, item_id: u32) -> bool {
        self.path.contains(&item_id)
    }

    /// Clear the path.
    pub fn clear(&mut self) {
        self.path.clear();
    }

    /// Check if this path is a prefix of another.
    pub fn is_prefix_of(&self, other: &BRepGraphUsagePath) -> bool {
        if self.len() > other.len() {
            return false;
        }
        self.path.iter().zip(other.path.iter()).all(|(a, b)| a == b)
    }
}

impl Default for BRepGraphUsagePath {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let path = BRepGraphUsagePath::new();
        assert!(path.is_empty());
        assert_eq!(path.len(), 0);
    }

    #[test]
    fn test_push_pop() {
        let mut path = BRepGraphUsagePath::new();
        path.push(1);
        path.push(2);

        assert_eq!(path.len(), 2);
        assert_eq!(path.pop(), Some(2));
        assert_eq!(path.pop(), Some(1));
        assert_eq!(path.pop(), None);
    }

    #[test]
    fn test_get() {
        let path = BRepGraphUsagePath::from_vec(vec![10, 20, 30]);
        assert_eq!(path.get(0), Some(10));
        assert_eq!(path.get(1), Some(20));
        assert_eq!(path.get(3), None);
    }

    #[test]
    fn test_contains() {
        let path = BRepGraphUsagePath::from_vec(vec![1, 2, 3]);
        assert!(path.contains(2));
        assert!(!path.contains(5));
    }

    #[test]
    fn test_is_prefix_of() {
        let path1 = BRepGraphUsagePath::from_vec(vec![1, 2]);
        let path2 = BRepGraphUsagePath::from_vec(vec![1, 2, 3]);
        let path3 = BRepGraphUsagePath::from_vec(vec![1, 3]);

        assert!(path1.is_prefix_of(&path2));
        assert!(!path1.is_prefix_of(&path3));
        assert!(!path2.is_prefix_of(&path1));
    }

    #[test]
    fn test_clear() {
        let mut path = BRepGraphUsagePath::from_vec(vec![1, 2, 3]);
        path.clear();
        assert!(path.is_empty());
    }
}
