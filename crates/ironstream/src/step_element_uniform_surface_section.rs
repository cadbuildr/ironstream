// FILE: step_element_uniform_surface_section.rs
// occt: StepElement_UniformSurfaceSection

use super::step_element_measure_or_unspecified_value::MeasureOrUnspecifiedValue;

/// Representation of STEP entity UniformSurfaceSection.
/// Inherits from SurfaceSection.
#[derive(Clone)]
pub struct UniformSurfaceSection {
    offset: MeasureOrUnspecifiedValue,
    non_structural_mass: MeasureOrUnspecifiedValue,
    non_structural_mass_offset: MeasureOrUnspecifiedValue,
    thickness: f64,
    bending_thickness: MeasureOrUnspecifiedValue,
    shear_thickness: MeasureOrUnspecifiedValue,
}

impl UniformSurfaceSection {
    /// Creates a new UniformSurfaceSection.
    pub fn new() -> Self {
        Self {
            offset: MeasureOrUnspecifiedValue::UnspecifiedValue,
            non_structural_mass: MeasureOrUnspecifiedValue::UnspecifiedValue,
            non_structural_mass_offset: MeasureOrUnspecifiedValue::UnspecifiedValue,
            thickness: 0.0,
            bending_thickness: MeasureOrUnspecifiedValue::UnspecifiedValue,
            shear_thickness: MeasureOrUnspecifiedValue::UnspecifiedValue,
        }
    }

    /// Initializes all fields.
    pub fn init(
        &mut self,
        offset: MeasureOrUnspecifiedValue,
        non_structural_mass: MeasureOrUnspecifiedValue,
        non_structural_mass_offset: MeasureOrUnspecifiedValue,
        thickness: f64,
        bending_thickness: MeasureOrUnspecifiedValue,
        shear_thickness: MeasureOrUnspecifiedValue,
    ) {
        self.offset = offset;
        self.non_structural_mass = non_structural_mass;
        self.non_structural_mass_offset = non_structural_mass_offset;
        self.thickness = thickness;
        self.bending_thickness = bending_thickness;
        self.shear_thickness = shear_thickness;
    }

    pub fn offset(&self) -> &MeasureOrUnspecifiedValue {
        &self.offset
    }

    pub fn set_offset(&mut self, val: MeasureOrUnspecifiedValue) {
        self.offset = val;
    }

    pub fn non_structural_mass(&self) -> &MeasureOrUnspecifiedValue {
        &self.non_structural_mass
    }

    pub fn set_non_structural_mass(&mut self, val: MeasureOrUnspecifiedValue) {
        self.non_structural_mass = val;
    }

    pub fn non_structural_mass_offset(&self) -> &MeasureOrUnspecifiedValue {
        &self.non_structural_mass_offset
    }

    pub fn set_non_structural_mass_offset(&mut self, val: MeasureOrUnspecifiedValue) {
        self.non_structural_mass_offset = val;
    }

    pub fn thickness(&self) -> f64 {
        self.thickness
    }

    pub fn set_thickness(&mut self, val: f64) {
        self.thickness = val;
    }

    pub fn bending_thickness(&self) -> &MeasureOrUnspecifiedValue {
        &self.bending_thickness
    }

    pub fn set_bending_thickness(&mut self, val: MeasureOrUnspecifiedValue) {
        self.bending_thickness = val;
    }

    pub fn shear_thickness(&self) -> &MeasureOrUnspecifiedValue {
        &self.shear_thickness
    }

    pub fn set_shear_thickness(&mut self, val: MeasureOrUnspecifiedValue) {
        self.shear_thickness = val;
    }
}

impl Default for UniformSurfaceSection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let section = UniformSurfaceSection::new();
        assert_eq!(section.thickness(), 0.0);
        assert_eq!(section.offset(), &MeasureOrUnspecifiedValue::UnspecifiedValue);
    }

    #[test]
    fn test_init() {
        let mut section = UniformSurfaceSection::new();

        section.init(
            MeasureOrUnspecifiedValue::ContextDependentMeasure(1.0),
            MeasureOrUnspecifiedValue::ContextDependentMeasure(0.5),
            MeasureOrUnspecifiedValue::UnspecifiedValue,
            2.5,
            MeasureOrUnspecifiedValue::ContextDependentMeasure(2.0),
            MeasureOrUnspecifiedValue::ContextDependentMeasure(0.1),
        );

        assert_eq!(section.thickness(), 2.5);
        assert!(matches!(
            section.bending_thickness(),
            MeasureOrUnspecifiedValue::ContextDependentMeasure(_)
        ));
    }

    #[test]
    fn test_setters() {
        let mut section = UniformSurfaceSection::new();

        section.set_thickness(3.5);
        assert_eq!(section.thickness(), 3.5);

        let thickness_val = MeasureOrUnspecifiedValue::ContextDependentMeasure(3.0);
        section.set_bending_thickness(thickness_val.clone());
        assert_eq!(section.bending_thickness(), &thickness_val);
    }
}
