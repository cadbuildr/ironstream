// FILE: interface_reader_lib.rs
// occt: Interface_ReaderLib

use std::sync::Arc;

pub type ReaderModuleHandle = Arc<dyn std::any::Any>;
pub type ProtocolHandle = Arc<dyn std::any::Any>;
pub type NodeHandle = Arc<dyn std::any::Any>;

/// Library of reader modules for a given protocol
pub struct InterfaceReaderLib {
    thelist: Option<NodeHandle>,
    thecurr: Option<NodeHandle>,
}

impl InterfaceReaderLib {
    /// Sets a global module-protocol couple
    pub fn set_global(_amodule: &ReaderModuleHandle, _aprotocol: &ProtocolHandle) {
        // TODO: Global registration
    }

    /// Creates a Library which complies with a Protocol
    pub fn with_protocol(_aprotocol: &ProtocolHandle) -> Self {
        InterfaceReaderLib {
            thelist: None,
            thecurr: None,
        }
    }

    /// Creates an empty Library
    pub fn new() -> Self {
        InterfaceReaderLib {
            thelist: None,
            thecurr: None,
        }
    }

    /// Adds a protocol to the library
    pub fn add_protocol(&mut self, _aprotocol: &ProtocolHandle) {
        // TODO: Add protocol handling
    }

    /// Clears the list of modules
    pub fn clear(&mut self) {
        self.thelist = None;
        self.thecurr = None;
    }

    /// Sets library to complete global list
    pub fn set_complete(&mut self) {
        // TODO: Set to complete
    }

    /// Selects a module for an object
    pub fn select(&self, _obj: &Arc<dyn std::any::Any>) -> (Option<ReaderModuleHandle>, i32) {
        (None, 0)
    }

    /// Starts iteration
    pub fn start(&mut self) {
        self.thecurr = self.thelist.clone();
    }

    /// Returns if more modules
    pub fn more(&self) -> bool {
        self.thecurr.is_some()
    }

    /// Gets next module
    pub fn next(&mut self) {
        // TODO: Advance
    }

    /// Returns current module
    pub fn module(&self) -> Option<ReaderModuleHandle> {
        None
    }

    /// Returns current protocol
    pub fn protocol(&self) -> Option<ProtocolHandle> {
        None
    }
}

impl Default for InterfaceReaderLib {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty() {
        let lib = InterfaceReaderLib::new();
        assert!(!lib.more());
    }

    #[test]
    fn test_clear() {
        let mut lib = InterfaceReaderLib::new();
        lib.clear();
        assert!(!lib.more());
    }

    #[test]
    fn test_iteration() {
        let mut lib = InterfaceReaderLib::new();
        lib.start();
        assert!(!lib.more());
    }
}
