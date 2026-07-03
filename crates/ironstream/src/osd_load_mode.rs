// FILE: osd_load_mode.rs
// occt: OSD_LoadMode

/// Dynamic library loading mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadMode {
    /// Lazy binding (resolve symbols on demand)
    RtldLazy,
    /// Eager binding (resolve all symbols immediately)
    RtldNow,
}

impl LoadMode {
    /// Get string representation.
    pub fn as_str(&self) -> &str {
        match self {
            LoadMode::RtldLazy => "RTLD_LAZY",
            LoadMode::RtldNow => "RTLD_NOW",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_mode_variants() {
        assert_eq!(LoadMode::RtldLazy.as_str(), "RTLD_LAZY");
        assert_eq!(LoadMode::RtldNow.as_str(), "RTLD_NOW");
    }

    #[test]
    fn test_equality() {
        assert_eq!(LoadMode::RtldLazy, LoadMode::RtldLazy);
        assert_ne!(LoadMode::RtldLazy, LoadMode::RtldNow);
    }
}
