// FILE: iges_dimen_protocol.rs
// occt: IGESDimen_Protocol

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
    fn test_protocol_creation() {
        let _protocol = IgesDimen_Protocol::new();
    }
}
