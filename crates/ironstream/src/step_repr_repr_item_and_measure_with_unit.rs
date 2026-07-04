// FILE: step_repr_repr_item_and_measure_with_unit.rs
// occt: StepRepr_ReprItemAndMeasureWithUnit

/// Placeholder for MeasureRepresentationItem
#[derive(Clone, Debug, PartialEq)]
pub struct MeasureRepresentationItem {
    name: String,
}

/// Placeholder for MeasureWithUnit
#[derive(Clone, Debug, PartialEq)]
pub struct MeasureWithUnit {
    value: f64,
}

/// Placeholder for RepresentationItem
#[derive(Clone, Debug, PartialEq)]
pub struct RepresentationItem {
    name: String,
}

/// Base class for complex types combining measure representation item with measure with unit and representation item.
pub struct ReprItemAndMeasureWithUnit {
    measure_representation_item: Option<MeasureRepresentationItem>,
    measure_with_unit: Option<MeasureWithUnit>,
    representation_item: Option<RepresentationItem>,
}

impl ReprItemAndMeasureWithUnit {
    /// Create a new ReprItemAndMeasureWithUnit
    pub fn new() -> Self {
        ReprItemAndMeasureWithUnit {
            measure_representation_item: None,
            measure_with_unit: None,
            representation_item: None,
        }
    }

    /// Initialize with measure with unit and representation item
    pub fn init(
        &mut self,
        mwu: MeasureWithUnit,
        ri: RepresentationItem,
    ) {
        self.measure_with_unit = Some(mwu);
        self.representation_item = Some(ri);
    }

    /// Get the measure representation item
    pub fn get_measure_representation_item(&self) -> Option<&MeasureRepresentationItem> {
        self.measure_representation_item.as_ref()
    }

    /// Set the measure with unit
    pub fn set_measure_with_unit(&mut self, mwu: MeasureWithUnit) {
        self.measure_with_unit = Some(mwu);
    }

    /// Get the measure with unit
    pub fn get_measure_with_unit(&self) -> Option<&MeasureWithUnit> {
        self.measure_with_unit.as_ref()
    }

    /// Get the representation item
    pub fn get_representation_item(&self) -> Option<&RepresentationItem> {
        self.representation_item.as_ref()
    }
}

impl Default for ReprItemAndMeasureWithUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let item = ReprItemAndMeasureWithUnit::new();
        assert!(item.get_measure_representation_item().is_none());
        assert!(item.get_measure_with_unit().is_none());
        assert!(item.get_representation_item().is_none());
    }

    #[test]
    fn test_init() {
        let mut item = ReprItemAndMeasureWithUnit::new();
        let mwu = MeasureWithUnit { value: 42.0 };
        let ri = RepresentationItem {
            name: "test".to_string(),
        };
        item.init(mwu.clone(), ri.clone());
        assert_eq!(item.get_measure_with_unit(), Some(&mwu));
        assert_eq!(item.get_representation_item(), Some(&ri));
    }

    #[test]
    fn test_set_measure_with_unit() {
        let mut item = ReprItemAndMeasureWithUnit::new();
        let mwu = MeasureWithUnit { value: 100.5 };
        item.set_measure_with_unit(mwu.clone());
        assert_eq!(item.get_measure_with_unit(), Some(&mwu));
    }
}
