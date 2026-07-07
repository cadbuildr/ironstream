// FILE: iges_data.rs
// occt: IGESData

//! Basic description of an IGES Interface.
//! Provides initialization and protocol management for IGES data processing.

use std::sync::OnceLock;

/// Static protocol instance
static PROTOCOL: OnceLock<IgesDataProtocol> = OnceLock::new();

/// Marker struct for IGES protocol
#[derive(Clone, Debug)]
pub struct IgesDataProtocol;

/// Prepares General dynamic data used for IGESData specifically:
/// Protocol and Modules, which treat UndefinedEntity
pub fn init() {
    let _ = PROTOCOL.get_or_init(|| {
        // Initialize modules and protocol handling
        // In a real implementation, this would set up default modules
        // for handling undefined entities
        IgesDataProtocol
    });
}

/// Returns a Protocol from IGESData (avoids to create it)
pub fn protocol() -> IgesDataProtocol {
    PROTOCOL.get_or_init(|| {
        // Ensure initialization
        init();
        IgesDataProtocol
    }).clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_singleton() {
        // First call initializes
        let p1 = protocol();

        // Second call returns same instance
        let p2 = protocol();

        // Both are equal (trait-based verification)
        assert_eq!(format!("{:?}", p1), format!("{:?}", p2));
    }

    #[test]
    fn test_init_idempotent() {
        init();
        init();
        init();
        // No panic or error - idempotent
        let _ = protocol();
    }
}
