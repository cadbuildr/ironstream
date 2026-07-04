// FILE: rw_header_section.rs
// occt: RWHeaderSection

/// Read/Write module for STEP header sections
pub struct RWHeaderSection;

impl RWHeaderSection {
    /// Creates a new RWHeaderSection instance
    pub fn new() -> Self {
        RWHeaderSection
    }
}

impl Default for RWHeaderSection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = RWHeaderSection::new();
    }
}
