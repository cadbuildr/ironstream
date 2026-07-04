// FILE: step_repr_repr_item_and_plane_angle_measure_with_unit_and_qri.rs
// occt: StepRepr_ReprItemAndPlaneAngleMeasureWithUnitAndQRI

/// Placeholder for PlaneAngleMeasureWithUnit
#[derive(Clone, Debug, PartialEq)]
pub struct PlaneAngleMeasureWithUnit {
    value: f64,
    unit: String,
}

/// Represents a representation item combined with a plane angle measure with unit and qualified representation item (STEP AP203/AP214).
pub struct ReprItemAndPlaneAngleMeasureWithUnitAndQri {
    plane_angle_measure_with_unit: Option<PlaneAngleMeasureWithUnit>,
}

impl ReprItemAndPlaneAngleMeasureWithUnitAndQri {
    /// Create a new ReprItemAndPlaneAngleMeasureWithUnitAndQRI
    pub fn new() -> Self {
        ReprItemAndPlaneAngleMeasureWithUnitAndQri {
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

impl Default for ReprItemAndPlaneAngleMeasureWithUnitAndQri {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let item = ReprItemAndPlaneAngleMeasureWithUnitAndQri::new();
        assert!(item.get_plane_angle_measure_with_unit().is_none());
    }

    #[test]
    fn test_set_and_get_angle_measure() {
        let mut item = ReprItemAndPlaneAngleMeasureWithUnitAndQri::new();
        let measure = PlaneAngleMeasureWithUnit {
            value: 45.0,
            unit: "radian".to_string(),
        };
        item.set_plane_angle_measure_with_unit(measure.clone());
        assert_eq!(item.get_plane_angle_measure_with_unit(), Some(&measure));
    }
}
