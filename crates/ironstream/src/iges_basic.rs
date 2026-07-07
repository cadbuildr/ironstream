// FILE: iges_basic.rs
// occt: IGESBasic

/// Package for basic IGES entities.
///
/// Provides foundational entity types and protocol management
/// for IGES file format handling.
pub struct IgesBasic;

impl IgesBasic {
    /// Initializes the basic IGES package.
    pub fn init() {
        // Package initialization
    }

    /// Returns the protocol for basic IGES entities.
    pub fn protocol() -> Option<String> {
        Some("IGESBasic_Protocol".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        IgesBasic::init();
    }

    #[test]
    fn test_protocol() {
        let protocol = IgesBasic::protocol();
        assert_eq!(protocol, Some("IGESBasic_Protocol".to_string()));
    }
}
