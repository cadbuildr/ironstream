// FILE: iges_draw_protocol.rs
// occt: IGESDraw_Protocol

/// Protocol for IGESDraw
pub struct IgesDrawProtocol;

impl IgesDrawProtocol {
    pub fn new() -> Self {
        IgesDrawProtocol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _proto = IgesDrawProtocol::new();
    }
}
