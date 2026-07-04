// FILE: header_section.rs
// occt: HeaderSection

/// Represents a STEP header section container
pub struct HeaderSection;

impl HeaderSection {
    /// Creates a new header section instance
    pub fn new() -> Self {
        HeaderSection
    }
}

impl Default for HeaderSection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = HeaderSection::new();
    }

    #[test]
    fn test_default() {
        let _ = HeaderSection::default();
    }
}
