// FILE: step_control_controller.rs
// occt: STEPControl_Controller

/// Controller for STEP reading and writing
pub struct STEPControl_Controller;

impl STEPControl_Controller {
    pub fn new() -> Self {
        STEPControl_Controller
    }
}

impl Default for STEPControl_Controller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _controller = STEPControl_Controller::new();
    }
}
