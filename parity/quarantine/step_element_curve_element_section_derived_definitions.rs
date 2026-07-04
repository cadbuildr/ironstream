// FILE: step_element_curve_element_section_derived_definitions.rs
// occt: StepElement_CurveElementSectionDerivedDefinitions

use std::cell::RefCell;
use std::rc::Rc;

/// Representation of STEP entity CurveElementSectionDerivedDefinitions.
/// Inherits from StepElement_CurveElementSectionDefinition.
#[derive(Clone)]
pub struct CurveElementSectionDerivedDefinitions {
    cross_sectional_area: f64,
    shear_area: Option<Vec<MeasureOrUnspecifiedValue>>,
    second_moment_of_area: Option<Vec<f64>>,
    torsional_constant: f64,
    warping_constant: MeasureOrUnspecifiedValue,
    location_of_centroid: Option<Vec<MeasureOrUnspecifiedValue>>,
    location_of_shear_centre: Option<Vec<MeasureOrUnspecifiedValue>>,
    location_of_non_structural_mass: Option<Vec<MeasureOrUnspecifiedValue>>,
    non_structural_mass: MeasureOrUnspecifiedValue,
    polar_moment: MeasureOrUnspecifiedValue,
}

/// A union type representing either a measurement or an unspecified value.
#[derive(Clone, Debug, PartialEq)]
pub enum MeasureOrUnspecifiedValue {
    ContextDependentMeasure(f64),
    UnspecifiedValue,
}

impl CurveElementSectionDerivedDefinitions {
    /// Creates an empty instance.
    pub fn new() -> Self {
        Self {
            cross_sectional_area: 0.0,
            shear_area: None,
            second_moment_of_area: None,
            torsional_constant: 0.0,
            warping_constant: MeasureOrUnspecifiedValue::UnspecifiedValue,
            location_of_centroid: None,
            location_of_shear_centre: None,
            location_of_non_structural_mass: None,
            non_structural_mass: MeasureOrUnspecifiedValue::UnspecifiedValue,
            polar_moment: MeasureOrUnspecifiedValue::UnspecifiedValue,
        }
    }

    /// Initializes all fields.
    pub fn init(
        &mut self,
        cross_sectional_area: f64,
        shear_area: Option<Vec<MeasureOrUnspecifiedValue>>,
        second_moment_of_area: Option<Vec<f64>>,
        torsional_constant: f64,
        warping_constant: MeasureOrUnspecifiedValue,
        location_of_centroid: Option<Vec<MeasureOrUnspecifiedValue>>,
        location_of_shear_centre: Option<Vec<MeasureOrUnspecifiedValue>>,
        location_of_non_structural_mass: Option<Vec<MeasureOrUnspecifiedValue>>,
        non_structural_mass: MeasureOrUnspecifiedValue,
        polar_moment: MeasureOrUnspecifiedValue,
    ) {
        self.cross_sectional_area = cross_sectional_area;
        self.shear_area = shear_area;
        self.second_moment_of_area = second_moment_of_area;
        self.torsional_constant = torsional_constant;
        self.warping_constant = warping_constant;
        self.location_of_centroid = location_of_centroid;
        self.location_of_shear_centre = location_of_shear_centre;
        self.location_of_non_structural_mass = location_of_non_structural_mass;
        self.non_structural_mass = non_structural_mass;
        self.polar_moment = polar_moment;
    }

    pub fn cross_sectional_area(&self) -> f64 {
        self.cross_sectional_area
    }

    pub fn set_cross_sectional_area(&mut self, val: f64) {
        self.cross_sectional_area = val;
    }

    pub fn shear_area(&self) -> Option<&Vec<MeasureOrUnspecifiedValue>> {
        self.shear_area.as_ref()
    }

    pub fn set_shear_area(&mut self, val: Option<Vec<MeasureOrUnspecifiedValue>>) {
        self.shear_area = val;
    }

    pub fn second_moment_of_area(&self) -> Option<&Vec<f64>> {
        self.second_moment_of_area.as_ref()
    }

    pub fn set_second_moment_of_area(&mut self, val: Option<Vec<f64>>) {
        self.second_moment_of_area = val;
    }

    pub fn torsional_constant(&self) -> f64 {
        self.torsional_constant
    }

    pub fn set_torsional_constant(&mut self, val: f64) {
        self.torsional_constant = val;
    }

    pub fn warping_constant(&self) -> &MeasureOrUnspecifiedValue {
        &self.warping_constant
    }

    pub fn set_warping_constant(&mut self, val: MeasureOrUnspecifiedValue) {
        self.warping_constant = val;
    }

    pub fn location_of_centroid(&self) -> Option<&Vec<MeasureOrUnspecifiedValue>> {
        self.location_of_centroid.as_ref()
    }

    pub fn set_location_of_centroid(&mut self, val: Option<Vec<MeasureOrUnspecifiedValue>>) {
        self.location_of_centroid = val;
    }

    pub fn location_of_shear_centre(&self) -> Option<&Vec<MeasureOrUnspecifiedValue>> {
        self.location_of_shear_centre.as_ref()
    }

    pub fn set_location_of_shear_centre(&mut self, val: Option<Vec<MeasureOrUnspecifiedValue>>) {
        self.location_of_shear_centre = val;
    }

    pub fn location_of_non_structural_mass(&self) -> Option<&Vec<MeasureOrUnspecifiedValue>> {
        self.location_of_non_structural_mass.as_ref()
    }

    pub fn set_location_of_non_structural_mass(
        &mut self,
        val: Option<Vec<MeasureOrUnspecifiedValue>>,
    ) {
        self.location_of_non_structural_mass = val;
    }

    pub fn non_structural_mass(&self) -> &MeasureOrUnspecifiedValue {
        &self.non_structural_mass
    }

    pub fn set_non_structural_mass(&mut self, val: MeasureOrUnspecifiedValue) {
        self.non_structural_mass = val;
    }

    pub fn polar_moment(&self) -> &MeasureOrUnspecifiedValue {
        &self.polar_moment
    }

    pub fn set_polar_moment(&mut self, val: MeasureOrUnspecifiedValue) {
        self.polar_moment = val;
    }
}

impl Default for CurveElementSectionDerivedDefinitions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_and_accessors() {
        let mut def = CurveElementSectionDerivedDefinitions::new();

        def.set_cross_sectional_area(5.0);
        assert_eq!(def.cross_sectional_area(), 5.0);

        def.set_torsional_constant(2.5);
        assert_eq!(def.torsional_constant(), 2.5);

        let warping = MeasureOrUnspecifiedValue::ContextDependentMeasure(1.5);
        def.set_warping_constant(warping.clone());
        assert_eq!(*def.warping_constant(), warping);
    }

    #[test]
    fn test_array_fields() {
        let mut def = CurveElementSectionDerivedDefinitions::new();

        let shear_vals = vec![
            MeasureOrUnspecifiedValue::ContextDependentMeasure(1.0),
            MeasureOrUnspecifiedValue::ContextDependentMeasure(2.0),
        ];
        def.set_shear_area(Some(shear_vals.clone()));
        assert!(def.shear_area().is_some());
        assert_eq!(def.shear_area().unwrap().len(), 2);

        let moments = vec![3.0, 4.0, 5.0];
        def.set_second_moment_of_area(Some(moments.clone()));
        assert_eq!(def.second_moment_of_area().unwrap().len(), 3);
    }

    #[test]
    fn test_measure_or_unspecified_value() {
        let measure = MeasureOrUnspecifiedValue::ContextDependentMeasure(3.14);
        match measure {
            MeasureOrUnspecifiedValue::ContextDependentMeasure(val) => {
                assert!((val - 3.14).abs() < 1e-6);
            }
            _ => panic!("Expected ContextDependentMeasure"),
        }

        let unspec = MeasureOrUnspecifiedValue::UnspecifiedValue;
        match unspec {
            MeasureOrUnspecifiedValue::UnspecifiedValue => (),
            _ => panic!("Expected UnspecifiedValue"),
        }
    }

    #[test]
    fn test_init_method() {
        let mut def = CurveElementSectionDerivedDefinitions::new();

        let centroid = vec![
            MeasureOrUnspecifiedValue::ContextDependentMeasure(0.5),
            MeasureOrUnspecifiedValue::ContextDependentMeasure(0.5),
        ];

        def.init(
            10.0,
            None,
            None,
            3.0,
            MeasureOrUnspecifiedValue::ContextDependentMeasure(2.0),
            Some(centroid.clone()),
            None,
            None,
            MeasureOrUnspecifiedValue::ContextDependentMeasure(0.5),
            MeasureOrUnspecifiedValue::UnspecifiedValue,
        );

        assert_eq!(def.cross_sectional_area(), 10.0);
        assert_eq!(def.torsional_constant(), 3.0);
        assert!(def.location_of_centroid().is_some());
        assert_eq!(def.location_of_centroid().unwrap().len(), 2);
    }
}
