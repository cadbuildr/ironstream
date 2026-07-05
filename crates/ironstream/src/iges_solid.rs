// FILE: iges_solid.rs
// occt: IGESSolid

//! IGESSolid package for B-Rep and CSG (Constructive Solid Geometry) solid entities.
//!
//! This package prepares dynamic data including protocols and modules for handling
//! IGES solid representation entities.

use std::sync::{Arc, Mutex};

/// Protocol for IGESSolid entities
#[derive(Clone, Debug)]
pub struct IGESSolidProtocol {
    name: String,
}

impl IGESSolidProtocol {
    pub fn new() -> Self {
        IGESSolidProtocol {
            name: "IGESSolid Protocol".to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// General module for IGESSolid
pub struct IGESSolidGeneralModule;

impl IGESSolidGeneralModule {
    pub fn new() -> Self {
        IGESSolidGeneralModule
    }
}

/// Read/Write module for IGESSolid
pub struct IGESSolidReadWriteModule;

impl IGESSolidReadWriteModule {
    pub fn new() -> Self {
        IGESSolidReadWriteModule
    }
}

/// Specific module for IGESSolid
pub struct IGESSolidSpecificModule;

impl IGESSolidSpecificModule {
    pub fn new() -> Self {
        IGESSolidSpecificModule
    }
}

/// Global library for general operations
pub struct InterfaceGeneralLib {
    module: Option<IGESSolidGeneralModule>,
    protocol: Option<IGESSolidProtocol>,
}

impl InterfaceGeneralLib {
    pub fn new() -> Self {
        InterfaceGeneralLib {
            module: None,
            protocol: None,
        }
    }

    pub fn set_global(
        &mut self,
        module: IGESSolidGeneralModule,
        protocol: IGESSolidProtocol,
    ) {
        self.module = Some(module);
        self.protocol = Some(protocol);
    }

    pub fn module(&self) -> Option<&IGESSolidGeneralModule> {
        self.module.as_ref()
    }

    pub fn protocol(&self) -> Option<&IGESSolidProtocol> {
        self.protocol.as_ref()
    }
}

/// Library for reader operations
pub struct InterfaceReaderLib {
    module: Option<IGESSolidReadWriteModule>,
    protocol: Option<IGESSolidProtocol>,
}

impl InterfaceReaderLib {
    pub fn new() -> Self {
        InterfaceReaderLib {
            module: None,
            protocol: None,
        }
    }

    pub fn set_global(
        &mut self,
        module: IGESSolidReadWriteModule,
        protocol: IGESSolidProtocol,
    ) {
        self.module = Some(module);
        self.protocol = Some(protocol);
    }

    pub fn module(&self) -> Option<&IGESSolidReadWriteModule> {
        self.module.as_ref()
    }

    pub fn protocol(&self) -> Option<&IGESSolidProtocol> {
        self.protocol.as_ref()
    }
}

/// Library for writer operations
pub struct IGESDataWriterLib {
    module: Option<IGESSolidReadWriteModule>,
    protocol: Option<IGESSolidProtocol>,
}

impl IGESDataWriterLib {
    pub fn new() -> Self {
        IGESDataWriterLib {
            module: None,
            protocol: None,
        }
    }

    pub fn set_global(
        &mut self,
        module: IGESSolidReadWriteModule,
        protocol: IGESSolidProtocol,
    ) {
        self.module = Some(module);
        self.protocol = Some(protocol);
    }

    pub fn module(&self) -> Option<&IGESSolidReadWriteModule> {
        self.module.as_ref()
    }

    pub fn protocol(&self) -> Option<&IGESSolidProtocol> {
        self.protocol.as_ref()
    }
}

/// Library for specific operations
pub struct IGESDataSpecificLib {
    module: Option<IGESSolidSpecificModule>,
    protocol: Option<IGESSolidProtocol>,
}

impl IGESDataSpecificLib {
    pub fn new() -> Self {
        IGESDataSpecificLib {
            module: None,
            protocol: None,
        }
    }

    pub fn set_global(
        &mut self,
        module: IGESSolidSpecificModule,
        protocol: IGESSolidProtocol,
    ) {
        self.module = Some(module);
        self.protocol = Some(protocol);
    }

    pub fn module(&self) -> Option<&IGESSolidSpecificModule> {
        self.module.as_ref()
    }

    pub fn protocol(&self) -> Option<&IGESSolidProtocol> {
        self.protocol.as_ref()
    }
}

/// Global protocol instance for IGESSolid
static IGES_SOLID_PROTOCOL: Mutex<Option<IGESSolidProtocol>> = Mutex::new(None);

/// IGESSolid package initialization and protocol management
pub struct IGESSolid;

impl IGESSolid {
    /// Initializes the IGESSolid package, setting up all required modules
    pub fn init() {
        // Initialize IGESGeom (parent package)
        // In real implementation: IGESGeom::init();

        let mut proto_lock = IGES_SOLID_PROTOCOL.lock().unwrap();
        if proto_lock.is_none() {
            let protocol = IGESSolidProtocol::new();

            // Set up general library
            let mut gen_lib = InterfaceGeneralLib::new();
            gen_lib.set_global(
                IGESSolidGeneralModule::new(),
                protocol.clone(),
            );

            // Set up reader library
            let mut read_lib = InterfaceReaderLib::new();
            read_lib.set_global(
                IGESSolidReadWriteModule::new(),
                protocol.clone(),
            );

            // Set up writer library
            let mut write_lib = IGESDataWriterLib::new();
            write_lib.set_global(
                IGESSolidReadWriteModule::new(),
                protocol.clone(),
            );

            // Set up specific library
            let mut spec_lib = IGESDataSpecificLib::new();
            spec_lib.set_global(
                IGESSolidSpecificModule::new(),
                protocol.clone(),
            );

            *proto_lock = Some(protocol);
        }
    }

    /// Returns the protocol for IGESSolid
    pub fn protocol() -> Option<IGESSolidProtocol> {
        let proto_lock = IGES_SOLID_PROTOCOL.lock().unwrap();
        proto_lock.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_creation() {
        let proto = IGESSolidProtocol::new();
        assert_eq!(proto.name(), "IGESSolid Protocol");
    }

    #[test]
    fn test_general_module_creation() {
        let _module = IGESSolidGeneralModule::new();
        // Module created successfully
    }

    #[test]
    fn test_read_write_module_creation() {
        let _module = IGESSolidReadWriteModule::new();
        // Module created successfully
    }

    #[test]
    fn test_specific_module_creation() {
        let _module = IGESSolidSpecificModule::new();
        // Module created successfully
    }

    #[test]
    fn test_general_lib_set_global() {
        let mut lib = InterfaceGeneralLib::new();
        let module = IGESSolidGeneralModule::new();
        let protocol = IGESSolidProtocol::new();

        lib.set_global(module, protocol);

        assert!(lib.module().is_some());
        assert!(lib.protocol().is_some());
    }

    #[test]
    fn test_reader_lib_set_global() {
        let mut lib = InterfaceReaderLib::new();
        let module = IGESSolidReadWriteModule::new();
        let protocol = IGESSolidProtocol::new();

        lib.set_global(module, protocol);

        assert!(lib.module().is_some());
        assert!(lib.protocol().is_some());
    }

    #[test]
    fn test_writer_lib_set_global() {
        let mut lib = IGESDataWriterLib::new();
        let module = IGESSolidReadWriteModule::new();
        let protocol = IGESSolidProtocol::new();

        lib.set_global(module, protocol);

        assert!(lib.module().is_some());
        assert!(lib.protocol().is_some());
    }

    #[test]
    fn test_specific_lib_set_global() {
        let mut lib = IGESDataSpecificLib::new();
        let module = IGESSolidSpecificModule::new();
        let protocol = IGESSolidProtocol::new();

        lib.set_global(module, protocol);

        assert!(lib.module().is_some());
        assert!(lib.protocol().is_some());
    }

    #[test]
    fn test_iges_solid_init() {
        IGESSolid::init();
        let protocol = IGESSolid::protocol();
        assert!(protocol.is_some());
    }

    #[test]
    fn test_iges_solid_protocol_after_init() {
        IGESSolid::init();
        let proto1 = IGESSolid::protocol();
        let proto2 = IGESSolid::protocol();

        assert!(proto1.is_some());
        assert!(proto2.is_some());
        // Both should refer to the same initialized protocol
    }

    #[test]
    fn test_protocol_clone() {
        let proto = IGESSolidProtocol::new();
        let proto_clone = proto.clone();

        assert_eq!(proto.name(), proto_clone.name());
    }
}
