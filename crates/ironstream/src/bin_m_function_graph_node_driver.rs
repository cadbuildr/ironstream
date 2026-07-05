// FILE: bin_m_function_graph_node_driver.rs
// occt: BinMFunction_GraphNodeDriver

/// Binary serialization driver for graph node attributes.
/// Handles persistent <-> transient conversion for GraphNode attributes.
pub struct BinMFunctionGraphNodeDriver {
    message_driver: Option<String>,
    type_name: String,
}

impl BinMFunctionGraphNodeDriver {
    /// Creates a new GraphNodeDriver with the given message driver handle.
    pub fn new(message_driver: Option<String>) -> Self {
        BinMFunctionGraphNodeDriver {
            message_driver,
            type_name: "TFunction_GraphNode".to_string(),
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
    fn test_graph_node_driver_creation() {
        let driver = BinMFunctionGraphNodeDriver::new(Some("test_messenger".to_string()));
        assert_eq!(driver.type_name(), "TFunction_GraphNode");
        assert_eq!(driver.message_driver(), Some("test_messenger"));
    }

    #[test]
    fn test_graph_node_driver_no_messenger() {
        let driver = BinMFunctionGraphNodeDriver::new(None);
        assert_eq!(driver.type_name(), "TFunction_GraphNode");
        assert_eq!(driver.message_driver(), None);
    }
}
