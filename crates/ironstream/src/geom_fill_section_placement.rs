// FILE: geom_fill_section_placement.rs
// occt: GeomFill_SectionPlacement

/// Section placement for sweep surfaces
pub struct SectionPlacement;

impl SectionPlacement {
    pub fn new() -> Self {
        SectionPlacement
    }
}

impl Default for SectionPlacement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _place = SectionPlacement::new();
    }

    #[test]
    fn test_default() {
        let _place = SectionPlacement::default();
    }
}
