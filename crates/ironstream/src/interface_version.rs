// FILE: interface_version.rs
// occt: Interface_Version

/// Version information
pub const XSTEP_PROCESSOR_VERSION: &str = "Open CASCADE processor";
pub const XSTEP_SYSTEM_VERSION: &str = "Open CASCADE";
pub const XSTEP_CONFIG: &str = "7.7.0";
pub const XSTEP_ULNAMES: &str = "7.7.0";

pub struct InterfaceVersion;

impl InterfaceVersion {
    pub fn processor_version() -> &'static str {
        XSTEP_PROCESSOR_VERSION
    }

    pub fn system_version() -> &'static str {
        XSTEP_SYSTEM_VERSION
    }

    pub fn config() -> &'static str {
        XSTEP_CONFIG
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_version() {
        assert!(!InterfaceVersion::processor_version().is_empty());
    }

    #[test]
    fn test_system_version() {
        assert!(!InterfaceVersion::system_version().is_empty());
    }

    #[test]
    fn test_config() {
        assert!(!InterfaceVersion::config().is_empty());
    }
}
