// FILE: aspect_x_atom.rs
// occt: Aspect_XAtom

/// Custom identifiers (atoms) for X window custom named properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AspectXAtom {
    /// Atom for WM_DELETE_WINDOW message.
    DeleteWindow = 0,
}

impl AspectXAtom {
    /// Convert from numeric value to enum variant.
    pub fn from_value(value: u32) -> Option<Self> {
        match value {
            0 => Some(AspectXAtom::DeleteWindow),
            _ => None,
        }
    }

    /// Get the numeric value of this atom.
    pub fn as_value(&self) -> u32 {
        *self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_atom_value() {
        assert_eq!(AspectXAtom::DeleteWindow.as_value(), 0);
    }

    #[test]
    fn test_x_atom_from_value() {
        let atom = AspectXAtom::from_value(0);
        assert_eq!(atom, Some(AspectXAtom::DeleteWindow));

        let invalid = AspectXAtom::from_value(999);
        assert_eq!(invalid, None);
    }

    #[test]
    fn test_x_atom_round_trip() {
        let original = AspectXAtom::DeleteWindow;
        let value = original.as_value();
        let restored = AspectXAtom::from_value(value);
        assert_eq!(restored, Some(original));
    }

    #[test]
    fn test_x_atom_equality() {
        let atom1 = AspectXAtom::DeleteWindow;
        let atom2 = AspectXAtom::DeleteWindow;
        assert_eq!(atom1, atom2);
    }

    #[test]
    fn test_x_atom_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(AspectXAtom::DeleteWindow);
        assert!(set.contains(&AspectXAtom::DeleteWindow));
    }
}
