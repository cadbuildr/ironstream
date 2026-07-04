// FILE: iges_defs_protocol.rs
// occt: IGESDefs_Protocol

//! Protocol for IGES definitions entities.

#[derive(Clone, Debug)]
pub struct Protocol;

impl Protocol {
    pub fn new() -> Self {
        Protocol
    }

    pub fn version(&self) -> &str {
        "1.0"
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let proto = Protocol::new();
        assert_eq!(proto.version(), "1.0");
    }

    #[test]
    fn test_default() {
        let p1 = Protocol::new();
        let p2 = Protocol::default();
        assert_eq!(format!("{:?}", p1), format!("{:?}", p2));
    }
}
