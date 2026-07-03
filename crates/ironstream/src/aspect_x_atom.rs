// FILE: aspect_x_atom.rs
// occt: Aspect_XAtom

/// Defines custom identifiers (atoms) for X window custom named properties.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AspectXAtom {
    /// Delete window atom
    DeleteWindow = 0,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom_values() {
        assert_eq!(AspectXAtom::DeleteWindow as u32, 0);
    }

    #[test]
    fn test_atom_equality() {
        let atom1 = AspectXAtom::DeleteWindow;
        let atom2 = AspectXAtom::DeleteWindow;
        assert_eq!(atom1, atom2);
    }

    #[test]
    fn test_atom_clone() {
        let atom = AspectXAtom::DeleteWindow;
        let cloned = atom;
        assert_eq!(atom, cloned);
    }

    #[test]
    fn test_atom_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(AspectXAtom::DeleteWindow);
        assert!(set.contains(&AspectXAtom::DeleteWindow));
    }

    #[test]
    fn test_atom_debug() {
        let atom = AspectXAtom::DeleteWindow;
        let debug_str = format!("{:?}", atom);
        assert!(debug_str.contains("DeleteWindow"));
    }
}
