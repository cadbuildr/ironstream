// FILE: interface_global_node_of_general_lib.rs
// occt: Interface_GlobalNodeOfGeneralLib

use std::sync::Arc;

pub type GeneralModuleHandle = Arc<dyn std::any::Any>;
pub type ProtocolHandle = Arc<dyn std::any::Any>;

/// Represents a node in the global library
pub struct InterfaceGlobalNodeOfGeneralLib {
    themod: Option<GeneralModuleHandle>,
    theprot: Option<ProtocolHandle>,
    thenext: Option<Arc<InterfaceGlobalNodeOfGeneralLib>>,
}

impl InterfaceGlobalNodeOfGeneralLib {
    /// Creates an empty GlobalNode, with no Next
    pub fn new() -> Self {
        InterfaceGlobalNodeOfGeneralLib {
            themod: None,
            theprot: None,
            thenext: None,
        }
    }

    /// Adds a Module bound with a Protocol to the list
    pub fn add(&mut self, amodule: GeneralModuleHandle, aprotocol: ProtocolHandle) {
        self.themod = Some(amodule);
        self.theprot = Some(aprotocol);
    }

    /// Returns the Module stored in a given GlobalNode
    pub fn module(&self) -> Option<GeneralModuleHandle> {
        self.themod.clone()
    }

    /// Returns the attached Protocol stored in a given GlobalNode
    pub fn protocol(&self) -> Option<ProtocolHandle> {
        self.theprot.clone()
    }

    /// Returns the Next GlobalNode
    pub fn next(&self) -> Option<Arc<InterfaceGlobalNodeOfGeneralLib>> {
        self.thenext.clone()
    }
}

impl Default for InterfaceGlobalNodeOfGeneralLib {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_node() {
        let node = InterfaceGlobalNodeOfGeneralLib::new();
        assert!(node.module().is_none());
        assert!(node.protocol().is_none());
        assert!(node.next().is_none());
    }

    #[test]
    fn test_add_module_protocol() {
        let mut node = InterfaceGlobalNodeOfGeneralLib::new();
        let module = Arc::new(42);
        let protocol = Arc::new("protocol");

        node.add(module.clone(), protocol.clone());

        assert!(node.module().is_some());
        assert!(node.protocol().is_some());
    }
}
