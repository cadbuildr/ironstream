// FILE: b_rep_fill_section_placement.rs
// occt: BRepFill_SectionPlacement

#[derive(Debug, Clone)]
pub struct BRepFillSectionPlacement {
    transformation: [f64; 16],
    abscissa_on_path: f64,
    param: f64,
    index: i32,
}

impl BRepFillSectionPlacement {
    pub fn new() -> Self {
        BRepFillSectionPlacement {
            transformation: [0.0; 16],
            abscissa_on_path: 0.0,
            param: 0.0,
            index: 0,
        }
    }

    pub fn with_contact(with_contact: bool, with_correction: bool) -> Self {
        let mut sp = Self::new();
        sp.param = if with_contact { 1.0 } else { 0.0 };
        sp.index = if with_correction { 1 } else { 0 };
        sp
    }

    pub fn transformation(&self) -> &[f64; 16] {
        &self.transformation
    }

    pub fn transformation_mut(&mut self) -> &mut [f64; 16] {
        &mut self.transformation
    }

    pub fn abscissa_on_path(&self) -> f64 {
        self.abscissa_on_path
    }

    pub fn set_abscissa_on_path(&mut self, abscissa: f64) {
        self.abscissa_on_path = abscissa;
    }

    pub fn param(&self) -> f64 {
        self.param
    }

    pub fn index(&self) -> i32 {
        self.index
    }
}

impl Default for BRepFillSectionPlacement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_placement_new() {
        let sp = BRepFillSectionPlacement::new();
        assert_eq!(sp.abscissa_on_path(), 0.0);
        assert_eq!(sp.param(), 0.0);
        assert_eq!(sp.index(), 0);
    }

    #[test]
    fn test_section_placement_with_contact() {
        let sp = BRepFillSectionPlacement::with_contact(true, false);
        assert_eq!(sp.param(), 1.0);
        assert_eq!(sp.index(), 0);
    }

    #[test]
    fn test_section_placement_transformation() {
        let sp = BRepFillSectionPlacement::new();
        assert_eq!(sp.transformation().len(), 16);
    }

    #[test]
    fn test_section_placement_abscissa() {
        let mut sp = BRepFillSectionPlacement::new();
        sp.set_abscissa_on_path(5.5);
        assert_eq!(sp.abscissa_on_path(), 5.5);
    }
}
