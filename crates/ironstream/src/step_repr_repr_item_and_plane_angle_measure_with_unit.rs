// FILE: step_repr_repr_item_and_plane_angle_measure_with_unit.rs
// occt: StepRepr_ReprItemAndPlaneAngleMeasureWithUnit

/// Placeholder for PlaneAngleMeasureWithUnit
#[derive(Clone, Debug, PartialEq)]
pub struct PlaneAngleMeasureWithUnit {
    value: f64,
    unit: String,
}

/// Represents a representation item combined with a plane angle measure with unit (STEP AP203/AP214).
pub struct ReprItemAndPlaneAngleMeasureWithUnit {
    plane_angle_measure_with_unit: Option<PlaneAngleMeasureWithUnit>,
}

impl ReprItemAndPlaneAngleMeasureWithUnit {
    /// Create a new ReprItemAndPlaneAngleMeasureWithUnit
    pub fn new() -> Self {
        ReprItemAndPlaneAngleMeasureWithUnit {
            plane_angle_measure_with_unit: None,
        }
    }

    /// Set the plane angle measure with unit
    pub fn set_plane_angle_measure_with_unit(&mut self, pamwu: PlaneAngleMeasureWithUnit) {
        self.plane_angle_measure_with_unit = Some(pamwu);
    }

    /// Get the plane angle measure with unit
    pub fn get_plane_angle_measure_with_unit(&self) -> Option<&PlaneAngleMeasureWithUnit> {
        self.plane_angle_measure_with_unit.as_ref()
    }
}

impl Default for ReprItemAndPlaneAngleMeasureWithUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let item = ReprItemAndPlaneAngleMeasureWithUnit::new();
        assert!(item.get_plane_angle_measure_with_unit().is_none());
    }

    #[test]
    fn test_set_and_get_angle_measure() {
        let mut item = ReprItemAndPlaneAngleMeasureWithUnit::new();
        let measure = PlaneAngleMeasureWithUnit {
            value: 90.0,
            unit: "degree".to_string(),
        };
        item.set_plane_angle_measure_with_unit(measure.clone());
        assert_eq!(item.get_plane_angle_measure_with_unit(), Some(&measure));
    }
}
