// FILE: step_repr_repr_item_and_measure_with_unit_and_qri.rs
// occt: StepRepr_ReprItemAndMeasureWithUnitAndQRI

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

/// Placeholder for QualifiedRepresentationItem
#[derive(Clone, Debug, PartialEq)]
pub struct QualifiedRepresentationItem {
    name: String,
}

/// Base class for complex types combining measure representation item with measure with unit, representation item, and qualified representation item.
pub struct ReprItemAndMeasureWithUnitAndQri {
    measure_with_unit: Option<MeasureWithUnit>,
    representation_item: Option<RepresentationItem>,
    qualified_representation_item: Option<QualifiedRepresentationItem>,
}

impl ReprItemAndMeasureWithUnitAndQri {
    /// Create a new ReprItemAndMeasureWithUnitAndQRI
    pub fn new() -> Self {
        ReprItemAndMeasureWithUnitAndQri {
            measure_with_unit: None,
            representation_item: None,
            qualified_representation_item: None,
        }
    }

    /// Initialize with measure with unit, representation item, and qualified representation item
    pub fn init(
        &mut self,
        mwu: MeasureWithUnit,
        ri: RepresentationItem,
        qri: QualifiedRepresentationItem,
    ) {
        self.measure_with_unit = Some(mwu);
        self.representation_item = Some(ri);
        self.qualified_representation_item = Some(qri);
    }

    /// Set the qualified representation item
    pub fn set_qualified_representation_item(&mut self, qri: QualifiedRepresentationItem) {
        self.qualified_representation_item = Some(qri);
    }

    /// Get the qualified representation item
    pub fn get_qualified_representation_item(&self) -> Option<&QualifiedRepresentationItem> {
        self.qualified_representation_item.as_ref()
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

impl Default for ReprItemAndMeasureWithUnitAndQri {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let item = ReprItemAndMeasureWithUnitAndQri::new();
        assert!(item.get_qualified_representation_item().is_none());
        assert!(item.get_measure_with_unit().is_none());
        assert!(item.get_representation_item().is_none());
    }

    #[test]
    fn test_init() {
        let mut item = ReprItemAndMeasureWithUnitAndQri::new();
        let mwu = MeasureWithUnit { value: 42.0 };
        let ri = RepresentationItem {
            name: "item".to_string(),
        };
        let qri = QualifiedRepresentationItem {
            name: "qualified".to_string(),
        };
        item.init(mwu.clone(), ri.clone(), qri.clone());
        assert_eq!(item.get_measure_with_unit(), Some(&mwu));
        assert_eq!(item.get_representation_item(), Some(&ri));
        assert_eq!(item.get_qualified_representation_item(), Some(&qri));
    }

    #[test]
    fn test_set_qualified_representation_item() {
        let mut item = ReprItemAndMeasureWithUnitAndQri::new();
        let qri = QualifiedRepresentationItem {
            name: "test_qual".to_string(),
        };
        item.set_qualified_representation_item(qri.clone());
        assert_eq!(item.get_qualified_representation_item(), Some(&qri));
    }
}
