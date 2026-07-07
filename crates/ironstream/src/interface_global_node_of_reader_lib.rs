// FILE: interface_global_node_of_reader_lib.rs
// occt: Interface_GlobalNodeOfReaderLib

use std::sync::Arc;

pub type ReaderModuleHandle = Arc<dyn std::any::Any>;
pub type ProtocolHandle = Arc<dyn std::any::Any>;

/// Represents a node in the global reader library
pub struct InterfaceGlobalNodeOfReaderLib {
    themod: Option<ReaderModuleHandle>,
    theprot: Option<ProtocolHandle>,
    thenext: Option<Arc<InterfaceGlobalNodeOfReaderLib>>,
}

impl InterfaceGlobalNodeOfReaderLib {
    /// Creates an empty GlobalNode, with no Next
    pub fn new() -> Self {
        InterfaceGlobalNodeOfReaderLib {
            themod: None,
            theprot: None,
            thenext: None,
        }
    }

    /// Adds a Module bound with a Protocol to the list
    pub fn add(&mut self, amodule: ReaderModuleHandle, aprotocol: ProtocolHandle) {
        self.themod = Some(amodule);
        self.theprot = Some(aprotocol);
    }

    /// Returns the Module stored in a given GlobalNode
    pub fn module(&self) -> Option<ReaderModuleHandle> {
        self.themod.clone()
    }

    /// Returns the attached Protocol stored in a given GlobalNode
    pub fn protocol(&self) -> Option<ProtocolHandle> {
        self.theprot.clone()
    }

    /// Returns the Next GlobalNode
    pub fn next(&self) -> Option<Arc<InterfaceGlobalNodeOfReaderLib>> {
        self.thenext.clone()
    }
}

impl Default for InterfaceGlobalNodeOfReaderLib {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_node() {
        let node = InterfaceGlobalNodeOfReaderLib::new();
        assert!(node.module().is_none());
        assert!(node.protocol().is_none());
        assert!(node.next().is_none());
    }

    #[test]
    fn test_add_module_protocol() {
        let mut node = InterfaceGlobalNodeOfReaderLib::new();
        let module = Arc::new(42);
        let protocol = Arc::new("protocol");

        node.add(module.clone(), protocol.clone());

        assert!(node.module().is_some());
        assert!(node.protocol().is_some());
    }
}
