// FILE: iges_dimen_sectioned_area.rs
// occt: IGESDimen_SectionedArea

/// Defines SectionedArea, Type <404> Form <2>
/// in package IGESDimen
pub struct IgesDimen_SectionedArea {
    pattern: Vec<(f64, f64)>,
    spacing: f64,
}

impl IgesDimen_SectionedArea {
    pub fn new() -> Self {
        IgesDimen_SectionedArea {
            pattern: Vec::new(),
            spacing: 0.0,
        }
    }

    pub fn init(&mut self, pattern: Vec<(f64, f64)>, spacing: f64) {
        self.pattern = pattern;
        self.spacing = spacing;
    }

    pub fn nb_patterns(&self) -> usize {
        self.pattern.len()
    }

    pub fn pattern_point(&self, index: usize) -> Option<(f64, f64)> {
        if index == 0 || index > self.pattern.len() {
            return None;
        }
        Some(self.pattern[index - 1])
    }

    pub fn spacing(&self) -> f64 {
        self.spacing
    }
}

impl Default for IgesDimen_SectionedArea {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sectioned_area_creation() {
        let area = IgesDimen_SectionedArea::new();
        assert_eq!(area.nb_patterns(), 0);
    }
}
