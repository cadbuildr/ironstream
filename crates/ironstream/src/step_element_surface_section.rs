// FILE: step_element_surface_section.rs
// occt: StepElement_SurfaceSection

use super::step_element_measure_or_unspecified_value::MeasureOrUnspecifiedValue;

/// Representation of STEP entity SurfaceSection.
#[derive(Clone)]
pub struct SurfaceSection {
    offset: MeasureOrUnspecifiedValue,
    non_structural_mass: MeasureOrUnspecifiedValue,
    non_structural_mass_offset: MeasureOrUnspecifiedValue,
}

impl SurfaceSection {
    /// Creates a new SurfaceSection.
    pub fn new() -> Self {
        Self {
            offset: MeasureOrUnspecifiedValue::UnspecifiedValue,
            non_structural_mass: MeasureOrUnspecifiedValue::UnspecifiedValue,
            non_structural_mass_offset: MeasureOrUnspecifiedValue::UnspecifiedValue,
        }
    }

    /// Initializes all fields.
    pub fn init(
        &mut self,
        offset: MeasureOrUnspecifiedValue,
        non_structural_mass: MeasureOrUnspecifiedValue,
        non_structural_mass_offset: MeasureOrUnspecifiedValue,
    ) {
        self.offset = offset;
        self.non_structural_mass = non_structural_mass;
        self.non_structural_mass_offset = non_structural_mass_offset;
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
}

impl Default for SurfaceSection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let section = SurfaceSection::new();
        assert_eq!(section.offset(), &MeasureOrUnspecifiedValue::UnspecifiedValue);
        assert_eq!(section.non_structural_mass(), &MeasureOrUnspecifiedValue::UnspecifiedValue);
    }

    #[test]
    fn test_init() {
        let mut section = SurfaceSection::new();
        let offset = MeasureOrUnspecifiedValue::ContextDependentMeasure(1.5);
        let mass = MeasureOrUnspecifiedValue::ContextDependentMeasure(0.5);

        section.init(offset.clone(), mass.clone(), MeasureOrUnspecifiedValue::UnspecifiedValue);

        assert_eq!(section.offset(), &offset);
        assert_eq!(section.non_structural_mass(), &mass);
    }

    #[test]
    fn test_setters() {
        let mut section = SurfaceSection::new();
        let val = MeasureOrUnspecifiedValue::ContextDependentMeasure(2.0);

        section.set_offset(val.clone());
        assert_eq!(section.offset(), &val);

        section.set_non_structural_mass(val.clone());
        assert_eq!(section.non_structural_mass(), &val);

        section.set_non_structural_mass_offset(val.clone());
        assert_eq!(section.non_structural_mass_offset(), &val);
    }
}
