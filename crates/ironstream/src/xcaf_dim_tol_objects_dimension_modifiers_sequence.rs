// FILE: xcaf_dim_tol_objects_dimension_modifiers_sequence.rs
// occt: XCAFDimTolObjects_DimensionModifiersSequence
//
// Faithful port of OCCT XCAFDimTolObjects_DimensionModifiersSequence
// (Deprecated/NCollectionAliases/XCAFDimTolObjects_DimensionModifiersSequence.hxx/.cxx):
// a sequence (ordered list) of dimension modifiers in XCAF dimension/tolerance context.

/// Local representation of a dimension modifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DimensionModifier {
    None,
    Plus,
    Minus,
    PlusMinus,
    Diameter,
    Radius,
    SphericalDiameter,
    Controlled,
    Uncontrolled,
}

impl DimensionModifier {
    pub fn to_string(&self) -> &'static str {
        match self {
            DimensionModifier::None => "NONE",
            DimensionModifier::Plus => "PLUS",
            DimensionModifier::Minus => "MINUS",
            DimensionModifier::PlusMinus => "PLUS_MINUS",
            DimensionModifier::Diameter => "DIAMETER",
            DimensionModifier::Radius => "RADIUS",
            DimensionModifier::SphericalDiameter => "SPHERICAL_DIAMETER",
            DimensionModifier::Controlled => "CONTROLLED",
            DimensionModifier::Uncontrolled => "UNCONTROLLED",
        }
    }
}

/// Port of XCAFDimTolObjects_DimensionModifiersSequence.
#[derive(Debug, Clone, PartialEq)]
pub struct XcafDimTolObjectsDimensionModifiersSequence {
    modifiers: Vec<DimensionModifier>,
}

impl XcafDimTolObjectsDimensionModifiersSequence {
    /// Create an empty sequence.
    pub fn new() -> Self {
        XcafDimTolObjectsDimensionModifiersSequence {
            modifiers: Vec::new(),
        }
    }

    /// Add modifier to sequence.
    pub fn append(&mut self, modifier: DimensionModifier) {
        self.modifiers.push(modifier);
    }

    /// Get modifier at index.
    pub fn value(&self, index: usize) -> Option<DimensionModifier> {
        self.modifiers.get(index).copied()
    }

    /// Set modifier at index.
    pub fn set_value(&mut self, index: usize, modifier: DimensionModifier) -> bool {
        if index < self.modifiers.len() {
            self.modifiers[index] = modifier;
            true
        } else {
            false
        }
    }

    /// Remove modifier at index.
    pub fn remove(&mut self, index: usize) -> Option<DimensionModifier> {
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
    pub fn modifiers(&self) -> &[DimensionModifier] {
        &self.modifiers
    }
}

impl Default for XcafDimTolObjectsDimensionModifiersSequence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_empty() {
        let seq = XcafDimTolObjectsDimensionModifiersSequence::new();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn modifier_to_string() {
        assert_eq!(DimensionModifier::Diameter.to_string(), "DIAMETER");
        assert_eq!(DimensionModifier::PlusMinus.to_string(), "PLUS_MINUS");
    }

    #[test]
    fn append() {
        let mut seq = XcafDimTolObjectsDimensionModifiersSequence::new();
        seq.append(DimensionModifier::Plus);
        seq.append(DimensionModifier::Diameter);
        assert_eq!(seq.length(), 2);
    }

    #[test]
    fn value() {
        let mut seq = XcafDimTolObjectsDimensionModifiersSequence::new();
        seq.append(DimensionModifier::Minus);
        seq.append(DimensionModifier::Radius);
        assert_eq!(seq.value(0), Some(DimensionModifier::Minus));
        assert_eq!(seq.value(1), Some(DimensionModifier::Radius));
        assert_eq!(seq.value(2), None);
    }

    #[test]
    fn set_value() {
        let mut seq = XcafDimTolObjectsDimensionModifiersSequence::new();
        seq.append(DimensionModifier::Plus);
        assert!(seq.set_value(0, DimensionModifier::Minus));
        assert_eq!(seq.value(0), Some(DimensionModifier::Minus));
        assert!(!seq.set_value(10, DimensionModifier::None));
    }

    #[test]
    fn remove() {
        let mut seq = XcafDimTolObjectsDimensionModifiersSequence::new();
        seq.append(DimensionModifier::Diameter);
        seq.append(DimensionModifier::SphericalDiameter);
        assert_eq!(seq.length(), 2);
        let removed = seq.remove(0);
        assert_eq!(removed, Some(DimensionModifier::Diameter));
        assert_eq!(seq.length(), 1);
    }

    #[test]
    fn clear() {
        let mut seq = XcafDimTolObjectsDimensionModifiersSequence::new();
        seq.append(DimensionModifier::Plus);
        seq.append(DimensionModifier::Minus);
        assert_eq!(seq.length(), 2);
        seq.clear();
        assert_eq!(seq.length(), 0);
    }

    #[test]
    fn modifiers() {
        let mut seq = XcafDimTolObjectsDimensionModifiersSequence::new();
        seq.append(DimensionModifier::Controlled);
        seq.append(DimensionModifier::Uncontrolled);
        let mods = seq.modifiers();
        assert_eq!(mods.len(), 2);
        assert_eq!(mods[0], DimensionModifier::Controlled);
        assert_eq!(mods[1], DimensionModifier::Uncontrolled);
    }
}
