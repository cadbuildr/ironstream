// FILE: igescaf_control.rs
// occt: IGESCAFControl

/// IGESCAFControl - IGES reader/writer for CAD assemblies.
pub struct IgescafControl;

impl IgescafControl {
    pub fn new() -> Self {
        Self
    }

    pub fn protocol_name() -> &'static str {
        "IGES CAF Control"
    }

    pub fn version() -> i32 {
        1
    }
}

impl Default for IgescafControl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_name() {
        assert_eq!(IgescafControl::protocol_name(), "IGES CAF Control");
    }

    #[test]
    fn test_version() {
        assert_eq!(IgescafControl::version(), 1);
    }
}
