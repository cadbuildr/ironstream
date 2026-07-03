// FILE: geom_fill_evolved_section.rs
// occt: GeomFill_EvolvedSection

/// Evolved section for sweep surfaces
pub struct EvolvedSection;

impl EvolvedSection {
    pub fn new() -> Self {
        EvolvedSection
    }
}

impl Default for EvolvedSection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _sect = EvolvedSection::new();
    }

    #[test]
    fn test_default() {
        let _sect = EvolvedSection::default();
    }
}
