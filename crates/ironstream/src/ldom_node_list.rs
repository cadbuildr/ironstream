// FILE: ldom_node_list.rs
// occt: LDOM_NodeList

/// Represents a list of nodes in the LDOM DOM tree.
#[derive(Clone, Default)]
pub struct LDOMNodeList {
    items: Vec<String>,
}

impl LDOMNodeList {
    /// Empty constructor
    pub fn new() -> Self {
        LDOMNodeList {
            items: Vec::new(),
        }
    }

    /// Copy constructor
    pub fn from_other(other: &LDOMNodeList) -> Self {
        LDOMNodeList {
            items: other.items.clone(),
        }
    }

    /// Nullify the node list
    pub fn set_null(&mut self) {
        self.items.clear();
    }

    /// Get a node at the specified index
    pub fn item(&self, index: usize) -> Option<&str> {
        self.items.get(index).map(|s| s.as_str())
    }

    /// Get the length of the node list
    pub fn get_length(&self) -> usize {
        self.items.len()
    }

    /// Append a node to the list (internal)
    pub fn append(&mut self, node: String) {
        self.items.push(node);
    }

    /// Check for equality with null
    pub fn is_null(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_node_list() {
        let list = LDOMNodeList::new();
        assert_eq!(list.get_length(), 0);
        assert!(list.is_null());
    }

    #[test]
    fn test_append_item() {
        let mut list = LDOMNodeList::new();
        list.append("node1".to_string());
        list.append("node2".to_string());
        assert_eq!(list.get_length(), 2);
    }

    #[test]
    fn test_item() {
        let mut list = LDOMNodeList::new();
        list.append("first".to_string());
        list.append("second".to_string());
        assert_eq!(list.item(0), Some("first"));
        assert_eq!(list.item(1), Some("second"));
        assert_eq!(list.item(2), None);
    }

    #[test]
    fn test_copy_constructor() {
        let mut list1 = LDOMNodeList::new();
        list1.append("node".to_string());
        let list2 = LDOMNodeList::from_other(&list1);
        assert_eq!(list2.get_length(), 1);
        assert_eq!(list2.item(0), Some("node"));
    }

    #[test]
    fn test_nullify() {
        let mut list = LDOMNodeList::new();
        list.append("node".to_string());
        list.set_null();
        assert!(list.is_null());
    }
}
