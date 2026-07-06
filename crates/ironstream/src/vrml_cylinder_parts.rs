// FILE: vrml_cylinder_parts.rs
// occt: Vrml_CylinderParts
//
// Faithful port of OCCT Vrml_CylinderParts (DataExchange/TKDEVRML/Vrml/
// Vrml_CylinderParts.hxx/.cxx): enumeration for cylinder rendering modes.
// Controls which surfaces of the cylinder geometry are rendered.

/// Cylinder parts rendering mode enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VrmlCylinderParts {
    /// Render only the cylinder sides (lateral surface).
    Sides = 1,
    /// Render only the cylinder top.
    Top = 2,
    /// Render only the cylinder bottom.
    Bottom = 4,
    /// Render sides and top.
    SidesTop = 3,
    /// Render sides and bottom.
    SidesBottom = 5,
    /// Render top and bottom.
    TopBottom = 6,
    /// Render all surfaces (sides, top, bottom).
    All = 7,
}

impl VrmlCylinderParts {
    /// Check if this mode includes the sides.
    pub fn includes_sides(&self) -> bool {
        (*self as i32) & 1 != 0
    }

    /// Check if this mode includes the top.
    pub fn includes_top(&self) -> bool {
        (*self as i32) & 2 != 0
    }

    /// Check if this mode includes the bottom.
    pub fn includes_bottom(&self) -> bool {
        (*self as i32) & 4 != 0
    }

    /// Convert to integer representation.
    pub fn as_int(&self) -> i32 {
        *self as i32
    }

    /// Create from integer representation.
    pub fn from_int(val: i32) -> Option<Self> {
        match val {
            1 => Some(VrmlCylinderParts::Sides),
            2 => Some(VrmlCylinderParts::Top),
            3 => Some(VrmlCylinderParts::SidesTop),
            4 => Some(VrmlCylinderParts::Bottom),
            5 => Some(VrmlCylinderParts::SidesBottom),
            6 => Some(VrmlCylinderParts::TopBottom),
            7 => Some(VrmlCylinderParts::All),
            _ => None,
        }
    }

    /// Check if an integer value is valid.
    pub fn is_valid(val: i32) -> bool {
        matches!(val, 1..=7)
    }

    /// Get all valid cylinder parts values.
    pub fn all_values() -> &'static [VrmlCylinderParts] {
        &[
            VrmlCylinderParts::Sides,
            VrmlCylinderParts::Top,
            VrmlCylinderParts::Bottom,
            VrmlCylinderParts::SidesTop,
            VrmlCylinderParts::SidesBottom,
            VrmlCylinderParts::TopBottom,
            VrmlCylinderParts::All,
        ]
    }

    /// Get a human-readable description.
    pub fn description(&self) -> &str {
        match self {
            VrmlCylinderParts::Sides => "Sides only",
            VrmlCylinderParts::Top => "Top only",
            VrmlCylinderParts::Bottom => "Bottom only",
            VrmlCylinderParts::SidesTop => "Sides and top",
            VrmlCylinderParts::SidesBottom => "Sides and bottom",
            VrmlCylinderParts::TopBottom => "Top and bottom",
            VrmlCylinderParts::All => "Sides, top, and bottom",
        }
    }

    /// Get the VRML string representation.
    pub fn as_str(&self) -> &str {
        match self {
            VrmlCylinderParts::Sides => "SIDES",
            VrmlCylinderParts::Top => "TOP",
            VrmlCylinderParts::Bottom => "BOTTOM",
            VrmlCylinderParts::SidesTop => "SIDES_TOP",
            VrmlCylinderParts::SidesBottom => "SIDES_BOTTOM",
            VrmlCylinderParts::TopBottom => "TOP_BOTTOM",
            VrmlCylinderParts::All => "ALL",
        }
    }

    /// Parse from VRML string representation.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "SIDES" => Some(VrmlCylinderParts::Sides),
            "TOP" => Some(VrmlCylinderParts::Top),
            "BOTTOM" => Some(VrmlCylinderParts::Bottom),
            "SIDES_TOP" => Some(VrmlCylinderParts::SidesTop),
            "SIDES_BOTTOM" => Some(VrmlCylinderParts::SidesBottom),
            "TOP_BOTTOM" => Some(VrmlCylinderParts::TopBottom),
            "ALL" => Some(VrmlCylinderParts::All),
            _ => None,
        }
    }

    /// Bitwise OR operation to combine parts.
    pub fn combine(self, other: VrmlCylinderParts) -> VrmlCylinderParts {
        let combined = (self as i32) | (other as i32);
        VrmlCylinderParts::from_int(combined).unwrap_or(VrmlCylinderParts::All)
    }
}

impl Default for VrmlCylinderParts {
    fn default() -> Self {
        VrmlCylinderParts::All
    }
}

impl std::fmt::Display for VrmlCylinderParts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cylinder_parts_sides() {
        let parts = VrmlCylinderParts::Sides;
        assert!(parts.includes_sides());
        assert!(!parts.includes_top());
        assert!(!parts.includes_bottom());
    }

    #[test]
    fn cylinder_parts_top() {
        let parts = VrmlCylinderParts::Top;
        assert!(!parts.includes_sides());
        assert!(parts.includes_top());
        assert!(!parts.includes_bottom());
    }

    #[test]
    fn cylinder_parts_all() {
        let parts = VrmlCylinderParts::All;
        assert!(parts.includes_sides());
        assert!(parts.includes_top());
        assert!(parts.includes_bottom());
    }

    #[test]
    fn cylinder_parts_as_int() {
        assert_eq!(VrmlCylinderParts::Sides.as_int(), 1);
        assert_eq!(VrmlCylinderParts::Top.as_int(), 2);
        assert_eq!(VrmlCylinderParts::Bottom.as_int(), 4);
        assert_eq!(VrmlCylinderParts::All.as_int(), 7);
    }

    #[test]
    fn cylinder_parts_from_int() {
        assert_eq!(VrmlCylinderParts::from_int(1), Some(VrmlCylinderParts::Sides));
        assert_eq!(VrmlCylinderParts::from_int(2), Some(VrmlCylinderParts::Top));
        assert_eq!(VrmlCylinderParts::from_int(4), Some(VrmlCylinderParts::Bottom));
        assert_eq!(VrmlCylinderParts::from_int(7), Some(VrmlCylinderParts::All));
        assert_eq!(VrmlCylinderParts::from_int(0), None);
        assert_eq!(VrmlCylinderParts::from_int(8), None);
    }

    #[test]
    fn cylinder_parts_is_valid() {
        assert!(VrmlCylinderParts::is_valid(1));
        assert!(VrmlCylinderParts::is_valid(4));
        assert!(VrmlCylinderParts::is_valid(7));
        assert!(!VrmlCylinderParts::is_valid(0));
        assert!(!VrmlCylinderParts::is_valid(8));
        assert!(!VrmlCylinderParts::is_valid(-1));
    }

    #[test]
    fn cylinder_parts_all_values() {
        let values = VrmlCylinderParts::all_values();
        assert_eq!(values.len(), 7);
    }

    #[test]
    fn cylinder_parts_default() {
        assert_eq!(VrmlCylinderParts::default(), VrmlCylinderParts::All);
    }

    #[test]
    fn cylinder_parts_as_str() {
        assert_eq!(VrmlCylinderParts::Sides.as_str(), "SIDES");
        assert_eq!(VrmlCylinderParts::Top.as_str(), "TOP");
        assert_eq!(VrmlCylinderParts::Bottom.as_str(), "BOTTOM");
        assert_eq!(VrmlCylinderParts::All.as_str(), "ALL");
    }

    #[test]
    fn cylinder_parts_from_str() {
        assert_eq!(VrmlCylinderParts::from_str("SIDES"), Some(VrmlCylinderParts::Sides));
        assert_eq!(VrmlCylinderParts::from_str("TOP"), Some(VrmlCylinderParts::Top));
        assert_eq!(VrmlCylinderParts::from_str("ALL"), Some(VrmlCylinderParts::All));
        assert_eq!(VrmlCylinderParts::from_str("INVALID"), None);
    }

    #[test]
    fn cylinder_parts_from_str_case_insensitive() {
        assert_eq!(VrmlCylinderParts::from_str("sides"), Some(VrmlCylinderParts::Sides));
        assert_eq!(VrmlCylinderParts::from_str("Top"), Some(VrmlCylinderParts::Top));
    }

    #[test]
    fn cylinder_parts_description() {
        assert_eq!(VrmlCylinderParts::Sides.description(), "Sides only");
        assert_eq!(VrmlCylinderParts::Top.description(), "Top only");
        assert_eq!(VrmlCylinderParts::Bottom.description(), "Bottom only");
        assert_eq!(VrmlCylinderParts::All.description(), "Sides, top, and bottom");
    }

    #[test]
    fn cylinder_parts_combine() {
        let result = VrmlCylinderParts::Sides.combine(VrmlCylinderParts::Top);
        assert_eq!(result, VrmlCylinderParts::SidesTop);

        let result2 = VrmlCylinderParts::Top.combine(VrmlCylinderParts::Bottom);
        assert_eq!(result2, VrmlCylinderParts::TopBottom);
    }

    #[test]
    fn cylinder_parts_display() {
        assert_eq!(VrmlCylinderParts::Sides.to_string(), "SIDES");
        assert_eq!(VrmlCylinderParts::All.to_string(), "ALL");
    }

    #[test]
    fn cylinder_parts_equality() {
        assert_eq!(VrmlCylinderParts::Sides, VrmlCylinderParts::Sides);
        assert_ne!(VrmlCylinderParts::Sides, VrmlCylinderParts::Top);
    }

    #[test]
    fn cylinder_parts_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(VrmlCylinderParts::Sides);
        set.insert(VrmlCylinderParts::Top);
        set.insert(VrmlCylinderParts::All);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn round_trip_str() {
        for parts in VrmlCylinderParts::all_values() {
            let s = parts.as_str();
            let parsed = VrmlCylinderParts::from_str(s);
            assert_eq!(parsed, Some(*parts));
        }
    }

    #[test]
    fn round_trip_int() {
        for parts in VrmlCylinderParts::all_values() {
            let i = parts.as_int();
            let parsed = VrmlCylinderParts::from_int(i);
            assert_eq!(parsed, Some(*parts));
        }
    }
}
