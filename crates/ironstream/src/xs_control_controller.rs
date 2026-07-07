// FILE: xs_control_controller.rs
// occt: XSControl_Controller

/// Base controller for exchange format handling.
/// Manages the protocol and transfer logic for a specific exchange format.
#[derive(Clone, Debug)]
pub struct XSControlController {
    /// Controller identifier
    controller_id: u32,
    /// Associated protocol ID
    protocol_id: u32,
}

impl XSControlController {
    /// Creates a new controller.
    pub fn new(protocol_id: u32) -> Self {
        Self {
            controller_id: 1,
            protocol_id,
        }
    }

    /// Returns the controller ID.
    pub fn id(&self) -> u32 {
        self.controller_id
    }

    /// Returns the protocol ID.
    pub fn protocol_id(&self) -> u32 {
        self.protocol_id
    }

    /// Sets the protocol ID.
    pub fn set_protocol_id(&mut self, protocol_id: u32) {
        self.protocol_id = protocol_id;
    }
}

impl Default for XSControlController {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let controller = XSControlController::new(5);
        assert_eq!(controller.id(), 1);
        assert_eq!(controller.protocol_id(), 5);
    }

    #[test]
    fn test_set_protocol_id() {
        let mut controller = XSControlController::new(1);
        controller.set_protocol_id(10);
        assert_eq!(controller.protocol_id(), 10);
    }
}
