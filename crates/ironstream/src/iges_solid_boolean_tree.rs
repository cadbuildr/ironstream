// FILE: iges_solid_boolean_tree.rs
// occt: IGESSolid_BooleanTree

//! Boolean tree entity (IGES Type 180, Form 0).
//!
//! Describes a binary tree structure composed of regularized Boolean operations
//! and operands, represented in post-order notation.

use std::fmt;

/// Base IGES entity
pub struct IGESEntity;

/// Placeholder for operand reference
#[derive(Clone, Debug)]
pub struct IGESEntityHandle {
    id: usize,
}

impl IGESEntityHandle {
    pub fn new(id: usize) -> Self {
        IGESEntityHandle { id }
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Boolean operation types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BooleanOp {
    Union = 1,
    Difference = 2,
    Intersection = 3,
}

impl BooleanOp {
    pub fn from_int(val: i32) -> Option<Self> {
        match val {
            1 => Some(BooleanOp::Union),
            2 => Some(BooleanOp::Difference),
            3 => Some(BooleanOp::Intersection),
            _ => None,
        }
    }

    pub fn as_int(&self) -> i32 {
        *self as i32
    }
}

/// Node in a boolean tree (post-order notation)
#[derive(Clone, Debug)]
pub enum TreeNode {
    Operand(IGESEntityHandle),
    Operation(BooleanOp),
}

impl TreeNode {
    pub fn is_operand(&self) -> bool {
        matches!(self, TreeNode::Operand(_))
    }

    pub fn is_operation(&self) -> bool {
        matches!(self, TreeNode::Operation(_))
    }

    pub fn as_operand(&self) -> Option<&IGESEntityHandle> {
        match self {
            TreeNode::Operand(op) => Some(op),
            _ => None,
        }
    }

    pub fn as_operation(&self) -> Option<BooleanOp> {
        match self {
            TreeNode::Operation(op) => Some(*op),
            _ => None,
        }
    }
}

/// Boolean tree structure
pub struct IGESSolidBooleanTree {
    nodes: Vec<TreeNode>,
}

impl IGESSolidBooleanTree {
    /// Creates a new empty boolean tree
    pub fn new() -> Self {
        IGESSolidBooleanTree { nodes: Vec::new() }
    }

    /// Initializes the tree with operands and operations
    ///
    /// Validates that:
    /// - Both arrays have the same length
    /// - Arrays are properly indexed starting at 1 in the IGES convention
    pub fn init(
        &mut self,
        operands: Vec<Option<IGESEntityHandle>>,
        operations: Vec<i32>,
    ) -> Result<(), String> {
        if operands.len() != operations.len() {
            return Err("IGESSolid_BooleanTree: operands and operations arrays must have same length"
                .to_string());
        }

        if operands.is_empty() {
            return Err("IGESSolid_BooleanTree: arrays cannot be empty".to_string());
        }

        // Build tree nodes from operands and operations
        self.nodes.clear();
        for (opt_operand, op_code) in operands.iter().zip(operations.iter()) {
            if let Some(operand) = opt_operand {
                self.nodes.push(TreeNode::Operand(operand.clone()));
            } else {
                // This position contains an operation
                if let Some(op) = BooleanOp::from_int(*op_code) {
                    self.nodes.push(TreeNode::Operation(op));
                } else {
                    return Err(format!("Invalid boolean operation code: {}", op_code));
                }
            }
        }

        Ok(())
    }

    /// Returns the length of the post-order list
    pub fn length(&self) -> usize {
        self.nodes.len()
    }

    /// Returns True if the index-th value is an operand (1-indexed)
    pub fn is_operand(&self, index: usize) -> Option<bool> {
        if index < 1 || index > self.nodes.len() {
            return None;
        }
        Some(self.nodes[index - 1].is_operand())
    }

    /// Returns the index-th operand (1-indexed), or None if it's an operation
    pub fn operand(&self, index: usize) -> Option<IGESEntityHandle> {
        if index < 1 || index > self.nodes.len() {
            return None;
        }
        self.nodes[index - 1]
            .as_operand()
            .map(|h| h.clone())
    }

    /// Returns the index-th operation (1-indexed).
    /// Returns Some(0) if the position holds an operand, as in OCCT
    /// IGESSolid_BooleanTree::Operation which returns 0 in that case
    /// ("It is not an operation. (operations can be : 1-2-3)").
    /// Returns None only if the index is out of range.
    pub fn operation(&self, index: usize) -> Option<i32> {
        if index < 1 || index > self.nodes.len() {
            return None;
        }
        Some(
            self.nodes[index - 1]
                .as_operation()
                .map(|op| op.as_int())
                .unwrap_or(0),
        )
    }

    /// Returns the nodes for internal iteration
    pub fn nodes(&self) -> &[TreeNode] {
        &self.nodes
    }
}

impl fmt::Debug for IGESSolidBooleanTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IGESSolidBooleanTree")
            .field("nodes", &self.nodes)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_op_union() {
        assert_eq!(BooleanOp::Union.as_int(), 1);
        assert_eq!(BooleanOp::from_int(1), Some(BooleanOp::Union));
    }

    #[test]
    fn test_boolean_op_difference() {
        assert_eq!(BooleanOp::Difference.as_int(), 2);
        assert_eq!(BooleanOp::from_int(2), Some(BooleanOp::Difference));
    }

    #[test]
    fn test_boolean_op_intersection() {
        assert_eq!(BooleanOp::Intersection.as_int(), 3);
        assert_eq!(BooleanOp::from_int(3), Some(BooleanOp::Intersection));
    }

    #[test]
    fn test_boolean_op_invalid() {
        assert_eq!(BooleanOp::from_int(4), None);
        assert_eq!(BooleanOp::from_int(0), None);
    }

    #[test]
    fn test_iges_entity_handle_creation() {
        let h = IGESEntityHandle::new(42);
        assert_eq!(h.id(), 42);
        assert!(!h.is_null());
    }

    #[test]
    fn test_iges_entity_handle_null() {
        let h = IGESEntityHandle::new(0);
        assert!(h.is_null());
    }

    #[test]
    fn test_tree_node_operand() {
        let h = IGESEntityHandle::new(1);
        let node = TreeNode::Operand(h.clone());

        assert!(node.is_operand());
        assert!(!node.is_operation());
        assert!(node.as_operand().is_some());
        assert_eq!(node.as_operand().unwrap().id(), 1);
    }

    #[test]
    fn test_tree_node_operation() {
        let node = TreeNode::Operation(BooleanOp::Union);

        assert!(!node.is_operand());
        assert!(node.is_operation());
        assert!(node.as_operation().is_some());
        assert_eq!(node.as_operation(), Some(BooleanOp::Union));
    }

    #[test]
    fn test_boolean_tree_creation() {
        let tree = IGESSolidBooleanTree::new();
        assert_eq!(tree.length(), 0);
    }

    #[test]
    fn test_boolean_tree_init_valid() {
        let mut tree = IGESSolidBooleanTree::new();
        let operands = vec![
            Some(IGESEntityHandle::new(1)),
            Some(IGESEntityHandle::new(2)),
            None,
        ];
        let operations = vec![0, 0, 1]; // Operation 1 = Union

        let result = tree.init(operands, operations);
        assert!(result.is_ok());
        assert_eq!(tree.length(), 3);
    }

    #[test]
    fn test_boolean_tree_init_mismatched_length() {
        let mut tree = IGESSolidBooleanTree::new();
        let operands = vec![Some(IGESEntityHandle::new(1))];
        let operations = vec![0, 0];

        let result = tree.init(operands, operations);
        assert!(result.is_err());
    }

    #[test]
    fn test_boolean_tree_init_empty() {
        let mut tree = IGESSolidBooleanTree::new();
        let operands: Vec<Option<IGESEntityHandle>> = vec![];
        let operations: Vec<i32> = vec![];

        let result = tree.init(operands, operations);
        assert!(result.is_err());
    }

    #[test]
    fn test_boolean_tree_is_operand() {
        let mut tree = IGESSolidBooleanTree::new();
        let operands = vec![Some(IGESEntityHandle::new(1)), None];
        let operations = vec![0, 1];

        tree.init(operands, operations).unwrap();

        assert_eq!(tree.is_operand(1), Some(true));
        assert_eq!(tree.is_operand(2), Some(false));
        assert_eq!(tree.is_operand(0), None);
        assert_eq!(tree.is_operand(3), None);
    }

    #[test]
    fn test_boolean_tree_operand() {
        let mut tree = IGESSolidBooleanTree::new();
        let operands = vec![Some(IGESEntityHandle::new(42)), None];
        let operations = vec![0, 1];

        tree.init(operands, operations).unwrap();

        let op = tree.operand(1);
        assert!(op.is_some());
        assert_eq!(op.unwrap().id(), 42);
    }

    #[test]
    fn test_boolean_tree_operation() {
        let mut tree = IGESSolidBooleanTree::new();
        let operands = vec![Some(IGESEntityHandle::new(1)), None];
        let operations = vec![0, 2]; // Operation 2 = Difference

        tree.init(operands, operations).unwrap();

        assert_eq!(tree.operation(1), Some(0)); // Operand position
        assert_eq!(tree.operation(2), Some(2)); // Operation position
    }

    #[test]
    fn test_boolean_tree_post_order() {
        let mut tree = IGESSolidBooleanTree::new();
        let operands = vec![
            Some(IGESEntityHandle::new(1)),
            Some(IGESEntityHandle::new(2)),
            None,
        ];
        let operations = vec![0, 0, 1]; // A B Union

        tree.init(operands, operations).unwrap();

        assert_eq!(tree.nodes().len(), 3);
        assert!(tree.nodes()[0].is_operand());
        assert!(tree.nodes()[1].is_operand());
        assert!(tree.nodes()[2].is_operation());
    }

    #[test]
    fn test_boolean_tree_invalid_operation() {
        let mut tree = IGESSolidBooleanTree::new();
        let operands = vec![Some(IGESEntityHandle::new(1)), None];
        let operations = vec![0, 99]; // Invalid operation

        let result = tree.init(operands, operations);
        assert!(result.is_err());
    }
}
