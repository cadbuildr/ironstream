// FILE: iges_dimen.rs
// occt: IGESDimen

/// This package represents Entities applied to Dimensions
/// ie. Annotation Entities and attached Properties and
/// Associativities.
pub struct IgesDimen;

impl IgesDimen {
    /// Prepares dynamic data (Protocol, Modules) for this package
    pub fn init() {
        // Static initialization for package
    }

    /// Returns the Protocol for this Package
    pub fn protocol() -> IgesDimen_Protocol {
        IgesDimen_Protocol::new()
    }
}

/// Protocol for IGESDimen package
pub struct IgesDimen_Protocol;

impl IgesDimen_Protocol {
    pub fn new() -> Self {
        IgesDimen_Protocol
    }
}

impl Default for IgesDimen_Protocol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        IgesDimen::init();
    }

    #[test]
    fn test_protocol() {
        let _protocol = IgesDimen::protocol();
    }
}
