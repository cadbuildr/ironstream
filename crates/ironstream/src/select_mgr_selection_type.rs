// FILE: select_mgr_selection_type.rs
// occt: SelectMgr_SelectionType

/// Enumeration of possible selection types
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SelectMgrSelectionType {
    /// Undefined selection type
    Unknown = -1,
    /// Selection by point (frustum with some tolerance or axis)
    Point = 0,
    /// Rectangle selection
    Box = 1,
    /// Polygonal selection
    Polyline = 2,
}

impl SelectMgrSelectionType {
    /// Convert from i32 to SelectMgrSelectionType
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            -1 => Some(SelectMgrSelectionType::Unknown),
            0 => Some(SelectMgrSelectionType::Point),
            1 => Some(SelectMgrSelectionType::Box),
            2 => Some(SelectMgrSelectionType::Polyline),
            _ => None,
        }
    }

    /// Convert to i32
    pub fn to_int(self) -> i32 {
        self as i32
    }

    /// Returns true if this is a valid (known) selection type
    pub fn is_valid(self) -> bool {
        self != SelectMgrSelectionType::Unknown
    }

    /// Returns a human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            SelectMgrSelectionType::Unknown => "Unknown",
            SelectMgrSelectionType::Point => "Point",
            SelectMgrSelectionType::Box => "Box",
            SelectMgrSelectionType::Polyline => "Polyline",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_type_values() {
        assert_eq!(SelectMgrSelectionType::Unknown.to_int(), -1);
        assert_eq!(SelectMgrSelectionType::Point.to_int(), 0);
        assert_eq!(SelectMgrSelectionType::Box.to_int(), 1);
        assert_eq!(SelectMgrSelectionType::Polyline.to_int(), 2);
    }

    #[test]
    fn test_from_int() {
        assert_eq!(
            SelectMgrSelectionType::from_int(-1),
            Some(SelectMgrSelectionType::Unknown)
        );
        assert_eq!(
            SelectMgrSelectionType::from_int(0),
            Some(SelectMgrSelectionType::Point)
        );
        assert_eq!(
            SelectMgrSelectionType::from_int(1),
            Some(SelectMgrSelectionType::Box)
        );
        assert_eq!(
            SelectMgrSelectionType::from_int(2),
            Some(SelectMgrSelectionType::Polyline)
        );
        assert_eq!(SelectMgrSelectionType::from_int(99), None);
    }

    #[test]
    fn test_is_valid() {
        assert!(!SelectMgrSelectionType::Unknown.is_valid());
        assert!(SelectMgrSelectionType::Point.is_valid());
        assert!(SelectMgrSelectionType::Box.is_valid());
        assert!(SelectMgrSelectionType::Polyline.is_valid());
    }

    #[test]
    fn test_name() {
        assert_eq!(SelectMgrSelectionType::Unknown.name(), "Unknown");
        assert_eq!(SelectMgrSelectionType::Point.name(), "Point");
        assert_eq!(SelectMgrSelectionType::Box.name(), "Box");
        assert_eq!(SelectMgrSelectionType::Polyline.name(), "Polyline");
    }

    #[test]
    fn test_roundtrip() {
        for typ in [
            SelectMgrSelectionType::Unknown,
            SelectMgrSelectionType::Point,
            SelectMgrSelectionType::Box,
            SelectMgrSelectionType::Polyline,
        ] {
            let int_val = typ.to_int();
            assert_eq!(SelectMgrSelectionType::from_int(int_val), Some(typ));
        }
    }
}
