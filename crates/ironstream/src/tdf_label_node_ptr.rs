// FILE: tdf_label_node_ptr.rs
// occt: TDF_LabelNodePtr

/// Smart pointer to a TDF label node.
/// TODO: In OCCT, typedef for TDF_LabelNode*
pub struct TdfLabelNodePtr;

impl TdfLabelNodePtr {
    /// Creates a new label node pointer.
    pub fn new() -> Self {
        TdfLabelNodePtr
    }
}

impl Default for TdfLabelNodePtr {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_node_ptr() {
        let _ptr = TdfLabelNodePtr::new();
    }
}
