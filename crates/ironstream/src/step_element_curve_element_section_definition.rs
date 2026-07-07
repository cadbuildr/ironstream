// FILE: step_element_curve_element_section_definition.rs
// occt: StepElement_CurveElementSectionDefinition

pub struct CurveElementSectionDefinition {
    pub name: Option<String>,
    pub section_area: Option<f64>,
    pub i11: Option<f64>,
    pub i12: Option<f64>,
    pub i22: Option<f64>,
    pub torsional_constant: Option<f64>,
}

impl CurveElementSectionDefinition {
    pub fn new() -> Self {
        CurveElementSectionDefinition {
            name: None,
            section_area: None,
            i11: None,
            i12: None,
            i22: None,
            torsional_constant: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_section_area(&mut self, area: f64) {
        self.section_area = Some(area);
    }

    pub fn get_section_area(&self) -> Option<f64> {
        self.section_area
    }

    pub fn set_i11(&mut self, val: f64) {
        self.i11 = Some(val);
    }

    pub fn get_i11(&self) -> Option<f64> {
        self.i11
    }

    pub fn set_i12(&mut self, val: f64) {
        self.i12 = Some(val);
    }

    pub fn get_i12(&self) -> Option<f64> {
        self.i12
    }

    pub fn set_i22(&mut self, val: f64) {
        self.i22 = Some(val);
    }

    pub fn get_i22(&self) -> Option<f64> {
        self.i22
    }

    pub fn set_torsional_constant(&mut self, val: f64) {
        self.torsional_constant = Some(val);
    }

    pub fn get_torsional_constant(&self) -> Option<f64> {
        self.torsional_constant
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sect = CurveElementSectionDefinition::new();
        assert!(sect.section_area.is_none());
    }

    #[test]
    fn test_set_section_area() {
        let mut sect = CurveElementSectionDefinition::new();
        sect.set_section_area(10.5);
        assert_eq!(sect.get_section_area(), Some(10.5));
    }

    #[test]
    fn test_set_moments() {
        let mut sect = CurveElementSectionDefinition::new();
        sect.set_i11(2.5);
        sect.set_i12(0.5);
        sect.set_i22(3.0);
        assert_eq!(sect.get_i11(), Some(2.5));
        assert_eq!(sect.get_i12(), Some(0.5));
        assert_eq!(sect.get_i22(), Some(3.0));
    }

    #[test]
    fn test_set_and_get_name() {
        let mut sect = CurveElementSectionDefinition::new();
        sect.set_name("section1".to_string());
        assert_eq!(sect.get_name(), Some("section1"));
    }
}
