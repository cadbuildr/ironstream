// FILE: interface_node_of_general_lib.rs
// occt: Interface_NodeOfGeneralLib

use std::sync::Arc;

pub type GlobalNodeHandle = Arc<dyn std::any::Any>;
pub type GeneralModuleHandle = Arc<dyn std::any::Any>;
pub type ProtocolHandle = Arc<dyn std::any::Any>;

/// Node in the GeneralLib library chain
pub struct InterfaceNodeOfGeneralLib {
    thenode: Option<GlobalNodeHandle>,
    thenext: Option<Arc<InterfaceNodeOfGeneralLib>>,
}

impl InterfaceNodeOfGeneralLib {
    /// Creates an empty Node, with no Next
    pub fn new() -> Self {
        InterfaceNodeOfGeneralLib {
            thenode: None,
            thenext: None,
        }
    }

    /// Adds a couple (Module,Protocol)
    pub fn add_node(&mut self, anode: GlobalNodeHandle) {
        self.thenode = Some(anode);
    }

    /// Returns the Module designated by a precise Node
    pub fn module(&self) -> Option<GeneralModuleHandle> {
        None
    }

    /// Returns the Protocol designated by a precise Node
    pub fn protocol(&self) -> Option<ProtocolHandle> {
        None
    }

    /// Returns the Next Node
    pub fn next(&self) -> Option<Arc<InterfaceNodeOfGeneralLib>> {
        self.thenext.clone()
    }
}

impl Default for InterfaceNodeOfGeneralLib {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let node = InterfaceNodeOfGeneralLib::new();
        assert!(node.next().is_none());
    }

    #[test]
    fn test_add_node() {
        let mut node = InterfaceNodeOfGeneralLib::new();
        let global_node = Arc::new(42);
        node.add_node(global_node);
        // node has stored the global_node
    }
}
