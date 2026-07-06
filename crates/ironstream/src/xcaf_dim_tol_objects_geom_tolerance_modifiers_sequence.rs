// FILE: xcaf_dim_tol_objects_geom_tolerance_modifiers_sequence.rs
// occt: XCAFDimTolObjects_GeomToleranceModifiersSequence
//
// Faithful port of OCCT XCAFDimTolObjects_GeomToleranceModifiersSequence
// (Deprecated/NCollectionAliases/XCAFDimTolObjects_GeomToleranceModifiersSequence.hxx/.cxx):
// a sequence (ordered list) of geometric tolerance modifiers in XCAF context.

/// Local representation of a geometric tolerance modifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeomToleranceModifier {
    None,
    AllOverLength,
    PerUnitLength,
    Projected,
    UncertaintyPlus,
    UncertaintyMinus,
    Envelope,
    ResponsibleMeasurement,
}

impl GeomToleranceModifier {
    pub fn to_string(&self) -> &'static str {
        match self {
            GeomToleranceModifier::None => "NONE",
            GeomToleranceModifier::AllOverLength => "ALL_OVER_LENGTH",
            GeomToleranceModifier::PerUnitLength => "PER_UNIT_LENGTH",
            GeomToleranceModifier::Projected => "PROJECTED",
            GeomToleranceModifier::UncertaintyPlus => "UNCERTAINTY_PLUS",
            GeomToleranceModifier::UncertaintyMinus => "UNCERTAINTY_MINUS",
            GeomToleranceModifier::Envelope => "ENVELOPE",
            GeomToleranceModifier::ResponsibleMeasurement => "RESPONSIBLE_MEASUREMENT",
        }
    }
}

/// Port of XCAFDimTolObjects_GeomToleranceModifiersSequence.
#[derive(Debug, Clone, PartialEq)]
pub struct XcafDimTolObjectsGeomToleranceModifiersSequence {
    modifiers: Vec<GeomToleranceModifier>,
}

impl XcafDimTolObjectsGeomToleranceModifiersSequence {
    /// Create an empty sequence.
    pub fn new() -> Self {
        XcafDimTolObjectsGeomToleranceModifiersSequence {
            modifiers: Vec::new(),
        }
    }

    /// Add modifier to sequence.
    pub fn append(&mut self, modifier: GeomToleranceModifier) {
        self.modifiers.push(modifier);
    }

    /// Get modifier at index.
    pub fn value(&self, index: usize) -> Option<GeomToleranceModifier> {
        self.modifiers.get(index).copied()
    }

    /// Set modifier at index.
    pub fn set_value(&mut self, index: usize, modifier: GeomToleranceModifier) -> bool {
        if index < self.modifiers.len() {
            self.modifiers[index] = modifier;
            true
        } else {
            false
        }
    }

    /// Remove modifier at index.
    pub fn remove(&mut self, index: usize) -> Option<GeomToleranceModifier> {
        if index < self.modifiers.len() {
            Some(self.modifiers.remove(index))
        } else {
            None
        }
    }

    /// Get length of sequence.
    pub fn length(&self) -> usize {
        self.modifiers.len()
    }

    /// Clear sequence.
    pub fn clear(&mut self) {
        self.modifiers.clear();
    }

    /// Get all modifiers as slice.
    pub fn modifiers(&self) -> &[GeomToleranceModifier] {
        &self.modifiers
    }
}

impl Default for XcafDimTolObjectsGeomToleranceModifiersSequence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_empty() {
        let seq = XcafDimTolObjectsGeomToleranceModifiersSequence::new();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn modifier_to_string() {
        assert_eq!(GeomToleranceModifier::AllOverLength.to_string(), "ALL_OVER_LENGTH");
        assert_eq!(GeomToleranceModifier::Projected.to_string(), "PROJECTED");
    }

    #[test]
    fn append() {
        let mut seq = XcafDimTolObjectsGeomToleranceModifiersSequence::new();
        seq.append(GeomToleranceModifier::Projected);
        seq.append(GeomToleranceModifier::Envelope);
        assert_eq!(seq.length(), 2);
    }

    #[test]
    fn value() {
        let mut seq = XcafDimTolObjectsGeomToleranceModifiersSequence::new();
        seq.append(GeomToleranceModifier::AllOverLength);
        seq.append(GeomToleranceModifier::PerUnitLength);
        assert_eq!(seq.value(0), Some(GeomToleranceModifier::AllOverLength));
        assert_eq!(seq.value(1), Some(GeomToleranceModifier::PerUnitLength));
        assert_eq!(seq.value(2), None);
    }

    #[test]
    fn set_value() {
        let mut seq = XcafDimTolObjectsGeomToleranceModifiersSequence::new();
        seq.append(GeomToleranceModifier::UncertaintyPlus);
        assert!(seq.set_value(0, GeomToleranceModifier::UncertaintyMinus));
        assert_eq!(seq.value(0), Some(GeomToleranceModifier::UncertaintyMinus));
        assert!(!seq.set_value(10, GeomToleranceModifier::None));
    }

    #[test]
    fn remove() {
        let mut seq = XcafDimTolObjectsGeomToleranceModifiersSequence::new();
        seq.append(GeomToleranceModifier::Envelope);
        seq.append(GeomToleranceModifier::ResponsibleMeasurement);
        assert_eq!(seq.length(), 2);
        let removed = seq.remove(0);
        assert_eq!(removed, Some(GeomToleranceModifier::Envelope));
        assert_eq!(seq.length(), 1);
    }

    #[test]
    fn clear() {
        let mut seq = XcafDimTolObjectsGeomToleranceModifiersSequence::new();
        seq.append(GeomToleranceModifier::Projected);
        seq.append(GeomToleranceModifier::AllOverLength);
        assert_eq!(seq.length(), 2);
        seq.clear();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn modifiers() {
        let mut seq = XcafDimTolObjectsGeomToleranceModifiersSequence::new();
        seq.append(GeomToleranceModifier::Projected);
        seq.append(GeomToleranceModifier::Envelope);
        let mods = seq.modifiers();
        assert_eq!(mods.len(), 2);
        assert_eq!(mods[0], GeomToleranceModifier::Projected);
        assert_eq!(mods[1], GeomToleranceModifier::Envelope);
    }

    #[test]
    fn all_modifier_variants() {
        let mut seq = XcafDimTolObjectsGeomToleranceModifiersSequence::new();
        seq.append(GeomToleranceModifier::None);
        seq.append(GeomToleranceModifier::AllOverLength);
        seq.append(GeomToleranceModifier::PerUnitLength);
        seq.append(GeomToleranceModifier::Projected);
        seq.append(GeomToleranceModifier::UncertaintyPlus);
        seq.append(GeomToleranceModifier::UncertaintyMinus);
        seq.append(GeomToleranceModifier::Envelope);
        seq.append(GeomToleranceModifier::ResponsibleMeasurement);
        assert_eq!(seq.length(), 8);
    }
}
