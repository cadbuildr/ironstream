// FILE: select_mgr_state_of_selection.rs
// occt: SelectMgr_StateOfSelection

/// Different states of a Selection in a ViewerSelector
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SelectMgrStateOfSelection {
    /// ANY selection state (for querying selections)
    Any = -2,
    /// Selection that has never been in Activated state
    /// (almost the same thing as Deactivated)
    Unknown = -1,
    /// Deactivated selection, once been in Activated state
    Deactivated = 0,
    /// Activated selection
    Activated = 1,
}

impl SelectMgrStateOfSelection {
    /// Convert from i32 to SelectMgrStateOfSelection
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            -2 => Some(SelectMgrStateOfSelection::Any),
            -1 => Some(SelectMgrStateOfSelection::Unknown),
            0 => Some(SelectMgrStateOfSelection::Deactivated),
            1 => Some(SelectMgrStateOfSelection::Activated),
            _ => None,
        }
    }

    /// Convert to i32
    pub fn to_int(self) -> i32 {
        self as i32
    }

    /// Returns true if this is an active state
    pub fn is_active(self) -> bool {
        self == SelectMgrStateOfSelection::Activated
    }

    /// Returns true if this is a valid known state
    pub fn is_valid(self) -> bool {
        self != SelectMgrStateOfSelection::Any
    }

    /// Returns a human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            SelectMgrStateOfSelection::Any => "Any",
            SelectMgrStateOfSelection::Unknown => "Unknown",
            SelectMgrStateOfSelection::Deactivated => "Deactivated",
            SelectMgrStateOfSelection::Activated => "Activated",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_values() {
        assert_eq!(SelectMgrStateOfSelection::Any.to_int(), -2);
        assert_eq!(SelectMgrStateOfSelection::Unknown.to_int(), -1);
        assert_eq!(SelectMgrStateOfSelection::Deactivated.to_int(), 0);
        assert_eq!(SelectMgrStateOfSelection::Activated.to_int(), 1);
    }

    #[test]
    fn test_from_int() {
        assert_eq!(
            SelectMgrStateOfSelection::from_int(-2),
            Some(SelectMgrStateOfSelection::Any)
        );
        assert_eq!(
            SelectMgrStateOfSelection::from_int(-1),
            Some(SelectMgrStateOfSelection::Unknown)
        );
        assert_eq!(
            SelectMgrStateOfSelection::from_int(0),
            Some(SelectMgrStateOfSelection::Deactivated)
        );
        assert_eq!(
            SelectMgrStateOfSelection::from_int(1),
            Some(SelectMgrStateOfSelection::Activated)
        );
        assert_eq!(SelectMgrStateOfSelection::from_int(99), None);
    }

    #[test]
    fn test_is_active() {
        assert!(!SelectMgrStateOfSelection::Any.is_active());
        assert!(!SelectMgrStateOfSelection::Unknown.is_active());
        assert!(!SelectMgrStateOfSelection::Deactivated.is_active());
        assert!(SelectMgrStateOfSelection::Activated.is_active());
    }

    #[test]
    fn test_is_valid() {
        assert!(!SelectMgrStateOfSelection::Any.is_valid());
        assert!(SelectMgrStateOfSelection::Unknown.is_valid());
        assert!(SelectMgrStateOfSelection::Deactivated.is_valid());
        assert!(SelectMgrStateOfSelection::Activated.is_valid());
    }

    #[test]
    fn test_name() {
        assert_eq!(SelectMgrStateOfSelection::Any.name(), "Any");
        assert_eq!(SelectMgrStateOfSelection::Unknown.name(), "Unknown");
        assert_eq!(SelectMgrStateOfSelection::Deactivated.name(), "Deactivated");
        assert_eq!(SelectMgrStateOfSelection::Activated.name(), "Activated");
    }

    #[test]
    fn test_roundtrip() {
        for state in [
            SelectMgrStateOfSelection::Any,
            SelectMgrStateOfSelection::Unknown,
            SelectMgrStateOfSelection::Deactivated,
            SelectMgrStateOfSelection::Activated,
        ] {
            let int_val = state.to_int();
            assert_eq!(SelectMgrStateOfSelection::from_int(int_val), Some(state));
        }
    }
}
