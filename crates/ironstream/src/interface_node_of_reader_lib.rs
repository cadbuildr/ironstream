// FILE: interface_node_of_reader_lib.rs
// occt: Interface_NodeOfReaderLib

use std::sync::Arc;

pub type GlobalNodeHandle = Arc<dyn std::any::Any>;
pub type ReaderModuleHandle = Arc<dyn std::any::Any>;
pub type ProtocolHandle = Arc<dyn std::any::Any>;

/// Node in the ReaderLib library chain
pub struct InterfaceNodeOfReaderLib {
    thenode: Option<GlobalNodeHandle>,
    thenext: Option<Arc<InterfaceNodeOfReaderLib>>,
}

impl InterfaceNodeOfReaderLib {
    /// Creates an empty Node, with no Next
    pub fn new() -> Self {
        InterfaceNodeOfReaderLib {
            thenode: None,
            thenext: None,
        }
    }

    /// Adds a couple (Module,Protocol)
    pub fn add_node(&mut self, anode: GlobalNodeHandle) {
        self.thenode = Some(anode);
    }

    /// Returns the Module designated by a precise Node
    pub fn module(&self) -> Option<ReaderModuleHandle> {
        None
    }

    /// Returns the Protocol designated by a precise Node
    pub fn protocol(&self) -> Option<ProtocolHandle> {
        None
    }

    /// Returns the Next Node
    pub fn next(&self) -> Option<Arc<InterfaceNodeOfReaderLib>> {
        self.thenext.clone()
    }
}

impl Default for InterfaceNodeOfReaderLib {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let node = InterfaceNodeOfReaderLib::new();
        assert!(node.next().is_none());
    }

    #[test]
    fn test_add_node() {
        let mut node = InterfaceNodeOfReaderLib::new();
        let global_node = Arc::new(42);
        node.add_node(global_node);
    }
}
