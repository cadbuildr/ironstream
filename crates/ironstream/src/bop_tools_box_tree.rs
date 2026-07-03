// FILE: bop_tools_box_tree.rs
// occt: BOPTools_BoxTree

/// 3D Bounding Volume Hierarchy tree for box selection.
/// This is a type alias specialization for 3D operations.
#[derive(Clone, Debug)]
pub struct BopToolsBoxTree {
    root: Option<Box<TreeNode>>,
    elements: Vec<i32>,
}

#[derive(Clone, Debug)]
struct TreeNode {
    min: [f64; 3],
    max: [f64; 3],
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl BopToolsBoxTree {
    /// Create an empty box tree.
    pub fn new() -> Self {
        BopToolsBoxTree {
            root: None,
            elements: Vec::new(),
        }
    }

    /// Add an element to the tree.
    pub fn add_element(&mut self, elem_id: i32) {
        self.elements.push(elem_id);
    }

    /// Get elements.
    pub fn elements(&self) -> &[i32] {
        &self.elements
    }

    /// Clear the tree.
    pub fn clear(&mut self) {
        self.root = None;
        self.elements.clear();
    }
}

impl Default for BopToolsBoxTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tree = BopToolsBoxTree::new();
        assert_eq!(tree.elements().len(), 0);
    }

    #[test]
    fn test_add_element() {
        let mut tree = BopToolsBoxTree::new();
        tree.add_element(1);
        tree.add_element(2);
        tree.add_element(3);
        assert_eq!(tree.elements().len(), 3);
        assert_eq!(tree.elements()[0], 1);
        assert_eq!(tree.elements()[2], 3);
    }

    #[test]
    fn test_clear() {
        let mut tree = BopToolsBoxTree::new();
        tree.add_element(42);
        tree.clear();
        assert_eq!(tree.elements().len(), 0);
    }
}
