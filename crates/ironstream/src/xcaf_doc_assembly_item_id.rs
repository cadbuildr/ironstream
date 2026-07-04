// FILE: xcaf_doc_assembly_item_id.rs
// occt: XCAFDoc_AssemblyItemId

/// Unique item identifier in the hierarchical product structure.
/// A full path to an assembly component in the "part-of" graph starting from the root node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XCAFDoc_AssemblyItemId {
    path: Vec<String>,
}

impl XCAFDoc_AssemblyItemId {
    /// Constructs an empty item ID.
    pub fn new() -> Self {
        XCAFDoc_AssemblyItemId {
            path: Vec::new(),
        }
    }

    /// Constructs an item ID from a list of strings, where every string is a label entry.
    pub fn from_path(path: Vec<String>) -> Self {
        XCAFDoc_AssemblyItemId { path }
    }

    /// Constructs an item ID from a formatted path, where label entries are separated by '/' symbol.
    pub fn from_string(s: &str) -> Self {
        let mut path = Vec::new();
        for entry in s.split('/') {
            let trimmed = entry.trim();
            if !trimmed.is_empty() {
                path.push(trimmed.to_string());
            }
        }
        XCAFDoc_AssemblyItemId { path }
    }

    /// Initializes the item ID from a list of strings, where every string is a label entry.
    pub fn init_from_path(&mut self, path: Vec<String>) {
        self.path = path;
    }

    /// Initializes the item ID from a formatted path, where label entries are separated by '/' symbol.
    pub fn init_from_string(&mut self, s: &str) {
        self.path.clear();
        for entry in s.split('/') {
            let trimmed = entry.trim();
            if !trimmed.is_empty() {
                self.path.push(trimmed.to_string());
            }
        }
    }

    /// Returns true if the full path is empty, otherwise - false.
    pub fn is_null(&self) -> bool {
        self.path.is_empty()
    }

    /// Clears the full path.
    pub fn nullify(&mut self) {
        self.path.clear();
    }

    /// Checks if this item is a child of the given item.
    /// Returns true if the item is a child of theOther item, otherwise - false.
    pub fn is_child(&self, other: &XCAFDoc_AssemblyItemId) -> bool {
        if self.path.len() <= other.path.len() {
            return false;
        }

        for (i, entry) in other.path.iter().enumerate() {
            if i >= self.path.len() || self.path[i] != *entry {
                return false;
            }
        }
        true
    }

    /// Checks if this item is a direct child of the given item.
    /// Returns true if the item is a direct child of theOther item, otherwise - false.
    pub fn is_direct_child(&self, other: &XCAFDoc_AssemblyItemId) -> bool {
        self.path.len() == other.path.len() + 1 && self.is_child(other)
    }

    /// Checks for item IDs equality.
    /// Returns true if this ID is equal to theOther, otherwise - false.
    pub fn is_equal(&self, other: &XCAFDoc_AssemblyItemId) -> bool {
        self.path == other.path
    }

    /// Returns the full path as a list of label entries.
    pub fn path(&self) -> &[String] {
        &self.path
    }

    /// Returns the full path as a formatted string.
    pub fn to_string_formatted(&self) -> String {
        self.path.join("/")
    }
}

impl Default for XCAFDoc_AssemblyItemId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::hash::Hash for XCAFDoc_AssemblyItemId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string_formatted().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_id() {
        let id = XCAFDoc_AssemblyItemId::new();
        assert!(id.is_null());
        assert!(id.path().is_empty());
    }

    #[test]
    fn test_from_path() {
        let path = vec!["root".to_string(), "sub1".to_string(), "sub2".to_string()];
        let id = XCAFDoc_AssemblyItemId::from_path(path);
        assert!(!id.is_null());
        assert_eq!(id.path().len(), 3);
    }

    #[test]
    fn test_from_string() {
        let id = XCAFDoc_AssemblyItemId::from_string("root/sub1/sub2");
        assert!(!id.is_null());
        assert_eq!(id.path().len(), 3);
        assert_eq!(id.to_string_formatted(), "root/sub1/sub2");
    }

    #[test]
    fn test_is_child() {
        let parent = XCAFDoc_AssemblyItemId::from_string("root/sub1");
        let child = XCAFDoc_AssemblyItemId::from_string("root/sub1/sub2");
        let unrelated = XCAFDoc_AssemblyItemId::from_string("root/sub3");

        assert!(child.is_child(&parent));
        assert!(!parent.is_child(&child));
        assert!(!unrelated.is_child(&parent));
    }

    #[test]
    fn test_is_direct_child() {
        let parent = XCAFDoc_AssemblyItemId::from_string("root/sub1");
        let direct_child = XCAFDoc_AssemblyItemId::from_string("root/sub1/sub2");
        let indirect_child = XCAFDoc_AssemblyItemId::from_string("root/sub1/sub2/sub3");

        assert!(direct_child.is_direct_child(&parent));
        assert!(!indirect_child.is_direct_child(&parent));
    }

    #[test]
    fn test_equality() {
        let id1 = XCAFDoc_AssemblyItemId::from_string("root/sub1");
        let id2 = XCAFDoc_AssemblyItemId::from_string("root/sub1");
        let id3 = XCAFDoc_AssemblyItemId::from_string("root/sub2");

        assert!(id1.is_equal(&id2));
        assert!(!id1.is_equal(&id3));
    }

    #[test]
    fn test_nullify() {
        let mut id = XCAFDoc_AssemblyItemId::from_string("root/sub1");
        assert!(!id.is_null());
        id.nullify();
        assert!(id.is_null());
    }
}
