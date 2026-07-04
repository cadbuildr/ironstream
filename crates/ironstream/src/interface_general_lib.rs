// FILE: interface_general_lib.rs
// occt: Interface_GeneralLib

/// Adds a couple (Module-Protocol) into the global definition set
/// for this class of Library.
pub fn set_global(_amodule: &InterfaceGeneralModuleHandle, _aprotocol: &InterfaceProtocolHandle) {
    // TODO: Global list management not implemented
}

/// Creates a Library which complies with a Protocol
pub struct InterfaceGeneralLib {
    thelist: Option<InterfaceNodeOfGeneralLibHandle>,
    thecurr: Option<InterfaceNodeOfGeneralLibHandle>,
}

/// Handle type alias for interface module
pub type InterfaceGeneralModuleHandle = std::sync::Arc<dyn std::any::Any>;

/// Handle type alias for interface protocol
pub type InterfaceProtocolHandle = std::sync::Arc<dyn std::any::Any>;

/// Handle type alias for node
pub type InterfaceNodeOfGeneralLibHandle = std::sync::Arc<dyn std::any::Any>;

impl InterfaceGeneralLib {
    /// Creates a Library which complies with a Protocol
    pub fn with_protocol(_aprotocol: &InterfaceProtocolHandle) -> Self {
        InterfaceGeneralLib {
            thelist: None,
            thecurr: None,
        }
    }

    /// Creates an empty Library
    pub fn new() -> Self {
        InterfaceGeneralLib {
            thelist: None,
            thecurr: None,
        }
    }

    /// Adds a couple (Module-Protocol) to the Library
    pub fn add_protocol(&mut self, _aprotocol: &InterfaceProtocolHandle) {
        // TODO: Protocol handling
    }

    /// Clears the list of Modules of a library
    pub fn clear(&mut self) {
        self.thelist = None;
        self.thecurr = None;
    }

    /// Sets a library to be defined with the complete Global list
    pub fn set_complete(&mut self) {
        // TODO: Set to complete global list
    }

    /// Selects a Module from the Library, given an Object
    pub fn select(&self, _obj: &std::sync::Arc<dyn std::any::Any>) -> (Option<InterfaceGeneralModuleHandle>, i32) {
        (None, 0)
    }

    /// Starts Iteration on the Modules
    pub fn start(&mut self) {
        self.thecurr = self.thelist.clone();
    }

    /// Returns True if there are more Modules to iterate on
    pub fn more(&self) -> bool {
        self.thecurr.is_some()
    }

    /// Iterates by getting the next Module in the list
    pub fn next(&mut self) {
        // TODO: Advance iterator
    }

    /// Returns the current Module in the Iteration
    pub fn module(&self) -> Option<InterfaceGeneralModuleHandle> {
        None
    }

    /// Returns the current Protocol in the Iteration
    pub fn protocol(&self) -> Option<InterfaceProtocolHandle> {
        None
    }
}

impl Default for InterfaceGeneralLib {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_library() {
        let lib = InterfaceGeneralLib::new();
        assert!(!lib.more());
    }

    #[test]
    fn test_clear() {
        let mut lib = InterfaceGeneralLib::new();
        lib.clear();
        assert!(!lib.more());
    }

    #[test]
    fn test_iteration() {
        let mut lib = InterfaceGeneralLib::new();
        lib.start();
        assert!(!lib.more());
    }
}
