// FILE: xcaf_dim_tol_objects_datum_modifiers_sequence.rs
// occt: XCAFDimTolObjects_DatumModifiersSequence
//
// Faithful port of OCCT XCAFDimTolObjects_DatumModifiersSequence
// (Deprecated/NCollectionAliases/XCAFDimTolObjects_DatumModifiersSequence.hxx/.cxx):
// a sequence (ordered list) of datum modifiers in XCAF dimension/tolerance context.

/// Local representation of a datum modifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DatumModifier {
    None,
    Projected,
    Deactivated,
    Maximum,
    Minimum,
    Average,
}

impl DatumModifier {
    pub fn to_string(&self) -> &'static str {
        match self {
            DatumModifier::None => "NONE",
            DatumModifier::Projected => "PROJECTED",
            DatumModifier::Deactivated => "DEACTIVATED",
            DatumModifier::Maximum => "MAXIMUM",
            DatumModifier::Minimum => "MINIMUM",
            DatumModifier::Average => "AVERAGE",
        }
    }
}

/// Port of XCAFDimTolObjects_DatumModifiersSequence.
#[derive(Debug, Clone, PartialEq)]
pub struct XcafDimTolObjectsDatumModifiersSequence {
    modifiers: Vec<DatumModifier>,
}

impl XcafDimTolObjectsDatumModifiersSequence {
    /// Create an empty sequence.
    pub fn new() -> Self {
        XcafDimTolObjectsDatumModifiersSequence {
            modifiers: Vec::new(),
        }
    }

    /// Add modifier to sequence.
    pub fn append(&mut self, modifier: DatumModifier) {
        self.modifiers.push(modifier);
    }

    /// Get modifier at index.
    pub fn value(&self, index: usize) -> Option<DatumModifier> {
        self.modifiers.get(index).copied()
    }

    /// Set modifier at index.
    pub fn set_value(&mut self, index: usize, modifier: DatumModifier) -> bool {
        if index < self.modifiers.len() {
            self.modifiers[index] = modifier;
            true
        } else {
            false
        }
    }

    /// Remove modifier at index.
    pub fn remove(&mut self, index: usize) -> Option<DatumModifier> {
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
    pub fn modifiers(&self) -> &[DatumModifier] {
        &self.modifiers
    }
}

impl Default for XcafDimTolObjectsDatumModifiersSequence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_empty() {
        let seq = XcafDimTolObjectsDatumModifiersSequence::new();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn modifier_to_string() {
        assert_eq!(DatumModifier::Projected.to_string(), "PROJECTED");
        assert_eq!(DatumModifier::Deactivated.to_string(), "DEACTIVATED");
    }

    #[test]
    fn append() {
        let mut seq = XcafDimTolObjectsDatumModifiersSequence::new();
        seq.append(DatumModifier::Projected);
        seq.append(DatumModifier::Maximum);
        assert_eq!(seq.length(), 2);
    }

    #[test]
    fn value() {
        let mut seq = XcafDimTolObjectsDatumModifiersSequence::new();
        seq.append(DatumModifier::Projected);
        seq.append(DatumModifier::Minimum);
        assert_eq!(seq.value(0), Some(DatumModifier::Projected));
        assert_eq!(seq.value(1), Some(DatumModifier::Minimum));
        assert_eq!(seq.value(2), None);
    }

    #[test]
    fn set_value() {
        let mut seq = XcafDimTolObjectsDatumModifiersSequence::new();
        seq.append(DatumModifier::Projected);
        assert!(seq.set_value(0, DatumModifier::Average));
        assert_eq!(seq.value(0), Some(DatumModifier::Average));
        assert!(!seq.set_value(10, DatumModifier::None));
    }

    #[test]
    fn remove() {
        let mut seq = XcafDimTolObjectsDatumModifiersSequence::new();
        seq.append(DatumModifier::Projected);
        seq.append(DatumModifier::Maximum);
        assert_eq!(seq.length(), 2);
        let removed = seq.remove(0);
        assert_eq!(removed, Some(DatumModifier::Projected));
        assert_eq!(seq.length(), 1);
    }

    #[test]
    fn clear() {
        let mut seq = XcafDimTolObjectsDatumModifiersSequence::new();
        seq.append(DatumModifier::Projected);
        seq.append(DatumModifier::Deactivated);
        assert_eq!(seq.length(), 2);
        seq.clear();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn modifiers() {
        let mut seq = XcafDimTolObjectsDatumModifiersSequence::new();
        seq.append(DatumModifier::None);
        seq.append(DatumModifier::Projected);
        let mods = seq.modifiers();
        assert_eq!(mods.len(), 2);
        assert_eq!(mods[0], DatumModifier::None);
        assert_eq!(mods[1], DatumModifier::Projected);
    }
}
