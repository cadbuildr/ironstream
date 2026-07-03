// FILE: geom_fill_section_law.rs
// occt: GeomFill_SectionLaw

/// Base class for section law in sweep surfaces
pub struct SectionLaw;

impl SectionLaw {
    pub fn new() -> Self {
        SectionLaw
    }
}

impl Default for SectionLaw {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _law = SectionLaw::new();
    }

    #[test]
    fn test_default() {
        let _law = SectionLaw::default();
    }
}
