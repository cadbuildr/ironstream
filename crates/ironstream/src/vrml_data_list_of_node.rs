// FILE: vrml_data_list_of_node.rs
// occt: VrmlData_ListOfNode

use std::collections::LinkedList;

#[derive(Clone, Debug)]
pub struct VrmlDataListOfNode {
    nodes: LinkedList<String>,
}

impl VrmlDataListOfNode {
    pub fn new() -> Self {
        VrmlDataListOfNode {
            nodes: LinkedList::new(),
        }
    }

    pub fn append(&mut self, node: &str) {
        self.nodes.push_back(node.to_string());
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

impl Default for VrmlDataListOfNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let list = VrmlDataListOfNode::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_append() {
        let mut list = VrmlDataListOfNode::new();
        list.append("node1");
        assert_eq!(list.size(), 1);
    }
}
