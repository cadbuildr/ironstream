// FILE: iges_defs.rs
// occt: IGESDefs

//! To embody general definitions of Entities (Parameters, Tables ...).

use std::sync::OnceLock;

static PROTOCOL: OnceLock<IgesDefsProtocol> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct IgesDefsProtocol;

/// Prepares dynamic data (Protocol, Modules) for this package
pub fn init() {
    let _ = PROTOCOL.get_or_init(|| {
        IgesDefsProtocol
    });
}

/// Returns the Protocol for this Package
pub fn protocol() -> IgesDefsProtocol {
    PROTOCOL.get_or_init(|| {
        init();
        IgesDefsProtocol
    }).clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        init();
        init();
        // Idempotent
    }

    #[test]
    fn test_protocol() {
        let p1 = protocol();
        let p2 = protocol();
        assert_eq!(format!("{:?}", p1), format!("{:?}", p2));
    }
}
