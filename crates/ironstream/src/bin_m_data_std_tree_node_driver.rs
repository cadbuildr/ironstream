// FILE: bin_m_data_std_tree_node_driver.rs
// occt: BinMDataStd_TreeNodeDriver

/// Binary serialization driver for tree node attributes.
/// Handles persistent <-> transient conversion for TreeNode attributes.
pub struct BinMDataStdTreeNodeDriver {
    message_driver: Option<String>,
    type_name: String,
}

impl BinMDataStdTreeNodeDriver {
    /// Creates a new TreeNodeDriver with the given message driver handle.
    pub fn new(message_driver: Option<String>) -> Self {
        BinMDataStdTreeNodeDriver {
            message_driver,
            type_name: "TDataStd_TreeNode".to_string(),
        }
    }

    /// Returns the type name of the attribute object.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Returns the current message driver of this driver.
    pub fn message_driver(&self) -> Option<&str> {
        self.message_driver.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_driver_creation() {
        let driver = BinMDataStdTreeNodeDriver::new(Some("test_messenger".to_string()));
        assert_eq!(driver.type_name(), "TDataStd_TreeNode");
        assert_eq!(driver.message_driver(), Some("test_messenger"));
    }

    #[test]
    fn test_tree_node_driver_no_messenger() {
        let driver = BinMDataStdTreeNodeDriver::new(None);
        assert_eq!(driver.type_name(), "TDataStd_TreeNode");
        assert_eq!(driver.message_driver(), None);
    }
}
