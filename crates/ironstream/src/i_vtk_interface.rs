// FILE: i_vtk_interface.rs
// occt: IVtk_Interface

/// Main interface for VTK integration.
pub trait IVtk_Interface {
    /// Get the name of the interface.
    fn name(&self) -> &str;

    /// Initialize the interface.
    fn initialize(&mut self) -> bool;

    /// Check if initialized.
    fn is_initialized(&self) -> bool;

    /// Get version information.
    fn version(&self) -> &str;
}

/// Default implementation of IVtk_Interface.
#[derive(Clone, Debug)]
pub struct DefaultInterface {
    name: String,
    initialized: bool,
    version: String,
}

impl DefaultInterface {
    /// Create a new interface.
    pub fn new(name: &str, version: &str) -> Self {
        DefaultInterface {
            name: name.to_string(),
            initialized: false,
            version: version.to_string(),
        }
    }
}

impl IVtk_Interface for DefaultInterface {
    fn name(&self) -> &str {
        &self.name
    }

    fn initialize(&mut self) -> bool {
        self.initialized = true;
        true
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn version(&self) -> &str {
        &self.version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_interface() {
        let iface = DefaultInterface::new("IVtk", "1.0");
        assert_eq!(iface.name(), "IVtk");
        assert_eq!(iface.version(), "1.0");
        assert!(!iface.is_initialized());
    }

    #[test]
    fn test_initialize() {
        let mut iface = DefaultInterface::new("IVtk", "1.0");
        let success = iface.initialize();
        assert!(success);
        assert!(iface.is_initialized());
    }

    #[test]
    fn test_interface_trait() {
        let mut iface: Box<dyn IVtk_Interface> = Box::new(DefaultInterface::new("Test", "2.0"));
        iface.initialize();
        assert!(iface.is_initialized());
        assert_eq!(iface.name(), "Test");
        assert_eq!(iface.version(), "2.0");
    }
}
