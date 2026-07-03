// FILE: geom_fill_sweep_section_generator.rs
// occt: GeomFill_SweepSectionGenerator

/// Generator for sweep surface sections
pub struct SweepSectionGenerator;

impl SweepSectionGenerator {
    pub fn new() -> Self {
        SweepSectionGenerator
    }
}

impl Default for SweepSectionGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _gen = SweepSectionGenerator::new();
    }

    #[test]
    fn test_default() {
        let _gen = SweepSectionGenerator::default();
    }
}
