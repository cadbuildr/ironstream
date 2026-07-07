// FILE: t_naming_ptr_node.rs
// occt: TNaming_PtrNode

/// Pointer to a naming node.
/// TODO: In OCCT, wraps TNaming_Node*
pub struct TNamingPtrNode;

impl TNamingPtrNode {
    /// Creates a new pointer node.
    pub fn new() -> Self {
        TNamingPtrNode
    }
}

impl Default for TNamingPtrNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ptr_node() {
        let _ = TNamingPtrNode::new();
    }
}
