// FILE: vrml_cone_parts.rs
// occt: Vrml_ConeParts
//
// Faithful port of OCCT Vrml_ConeParts (DataExchange/TKDEVRML/Vrml/
// Vrml_ConeParts.hxx/.cxx): enumeration for cone rendering modes.
// Controls which surfaces of the cone geometry are rendered.

/// Cone parts rendering mode enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VrmlConeParts {
    /// Render only the cone sides (lateral surface).
    Sides = 1,
    /// Render only the cone bottom (base).
    Bottom = 2,
    /// Render both sides and bottom (default).
    All = 3,
}

impl VrmlConeParts {
    /// Check if this mode includes the sides.
    pub fn includes_sides(&self) -> bool {
        (*self as i32) & 1 != 0
    }

    /// Check if this mode includes the bottom.
    pub fn includes_bottom(&self) -> bool {
        (*self as i32) & 2 != 0
    }

    /// Convert to integer representation.
    pub fn as_int(&self) -> i32 {
        *self as i32
    }

    /// Create from integer representation.
    pub fn from_int(val: i32) -> Option<Self> {
        match val {
            1 => Some(VrmlConeParts::Sides),
            2 => Some(VrmlConeParts::Bottom),
            3 => Some(VrmlConeParts::All),
            _ => None,
        }
    }

    /// Check if an integer value is valid.
    pub fn is_valid(val: i32) -> bool {
        matches!(val, 1 | 2 | 3)
    }

    /// Get all valid cone parts values.
    pub fn all_values() -> &'static [VrmlConeParts] {
        &[VrmlConeParts::Sides, VrmlConeParts::Bottom, VrmlConeParts::All]
    }

    /// Get a human-readable description.
    pub fn description(&self) -> &str {
        match self {
            VrmlConeParts::Sides => "Sides only",
            VrmlConeParts::Bottom => "Bottom only",
            VrmlConeParts::All => "Sides and bottom",
        }
    }

    /// Get the VRML string representation.
    pub fn as_str(&self) -> &str {
        match self {
            VrmlConeParts::Sides => "SIDES",
            VrmlConeParts::Bottom => "BOTTOM",
            VrmlConeParts::All => "ALL",
        }
    }

    /// Parse from VRML string representation.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "SIDES" => Some(VrmlConeParts::Sides),
            "BOTTOM" => Some(VrmlConeParts::Bottom),
            "ALL" => Some(VrmlConeParts::All),
            _ => None,
        }
    }

    /// Bitwise OR operation to combine parts.
    pub fn combine(self, other: VrmlConeParts) -> VrmlConeParts {
        match (self as i32) | (other as i32) {
            1 => VrmlConeParts::Sides,
            2 => VrmlConeParts::Bottom,
            3 => VrmlConeParts::All,
            _ => VrmlConeParts::All,
        }
    }

    /// Check if a surface is rendered in this parts mode.
    pub fn renders_sides(&self) -> bool {
        self.includes_sides()
    }

    /// Check if bottom is rendered in this parts mode.
    pub fn renders_bottom(&self) -> bool {
        self.includes_bottom()
    }
}

impl Default for VrmlConeParts {
    fn default() -> Self {
        VrmlConeParts::All
    }
}

impl std::fmt::Display for VrmlConeParts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cone_parts_sides() {
        let parts = VrmlConeParts::Sides;
        assert!(parts.includes_sides());
        assert!(!parts.includes_bottom());
    }

    #[test]
    fn cone_parts_bottom() {
        let parts = VrmlConeParts::Bottom;
        assert!(!parts.includes_sides());
        assert!(parts.includes_bottom());
    }

    #[test]
    fn cone_parts_all() {
        let parts = VrmlConeParts::All;
        assert!(parts.includes_sides());
        assert!(parts.includes_bottom());
    }

    #[test]
    fn cone_parts_as_int() {
        assert_eq!(VrmlConeParts::Sides.as_int(), 1);
        assert_eq!(VrmlConeParts::Bottom.as_int(), 2);
        assert_eq!(VrmlConeParts::All.as_int(), 3);
    }

    #[test]
    fn cone_parts_from_int() {
        assert_eq!(
            VrmlConeParts::from_int(1),
            Some(VrmlConeParts::Sides)
        );
        assert_eq!(
            VrmlConeParts::from_int(2),
            Some(VrmlConeParts::Bottom)
        );
        assert_eq!(VrmlConeParts::from_int(3), Some(VrmlConeParts::All));
        assert_eq!(VrmlConeParts::from_int(0), None);
        assert_eq!(VrmlConeParts::from_int(4), None);
    }

    #[test]
    fn cone_parts_is_valid() {
        assert!(VrmlConeParts::is_valid(1));
        assert!(VrmlConeParts::is_valid(2));
        assert!(VrmlConeParts::is_valid(3));
        assert!(!VrmlConeParts::is_valid(0));
        assert!(!VrmlConeParts::is_valid(4));
        assert!(!VrmlConeParts::is_valid(-1));
    }

    #[test]
    fn cone_parts_all_values() {
        let values = VrmlConeParts::all_values();
        assert_eq!(values.len(), 3);
        assert!(values.contains(&VrmlConeParts::Sides));
        assert!(values.contains(&VrmlConeParts::Bottom));
        assert!(values.contains(&VrmlConeParts::All));
    }

    #[test]
    fn cone_parts_default() {
        assert_eq!(VrmlConeParts::default(), VrmlConeParts::All);
    }

    #[test]
    fn cone_parts_as_str() {
        assert_eq!(VrmlConeParts::Sides.as_str(), "SIDES");
        assert_eq!(VrmlConeParts::Bottom.as_str(), "BOTTOM");
        assert_eq!(VrmlConeParts::All.as_str(), "ALL");
    }

    #[test]
    fn cone_parts_from_str() {
        assert_eq!(
            VrmlConeParts::from_str("SIDES"),
            Some(VrmlConeParts::Sides)
        );
        assert_eq!(
            VrmlConeParts::from_str("BOTTOM"),
            Some(VrmlConeParts::Bottom)
        );
        assert_eq!(VrmlConeParts::from_str("ALL"), Some(VrmlConeParts::All));
        assert_eq!(VrmlConeParts::from_str("INVALID"), None);
    }

    #[test]
    fn cone_parts_from_str_case_insensitive() {
        assert_eq!(
            VrmlConeParts::from_str("sides"),
            Some(VrmlConeParts::Sides)
        );
        assert_eq!(
            VrmlConeParts::from_str("Bottom"),
            Some(VrmlConeParts::Bottom)
        );
    }

    #[test]
    fn cone_parts_description() {
        assert_eq!(VrmlConeParts::Sides.description(), "Sides only");
        assert_eq!(VrmlConeParts::Bottom.description(), "Bottom only");
        assert_eq!(VrmlConeParts::All.description(), "Sides and bottom");
    }

    #[test]
    fn cone_parts_combine() {
        assert_eq!(
            VrmlConeParts::Sides.combine(VrmlConeParts::Bottom),
            VrmlConeParts::All
        );
        assert_eq!(
            VrmlConeParts::Bottom.combine(VrmlConeParts::Sides),
            VrmlConeParts::All
        );
        assert_eq!(
            VrmlConeParts::All.combine(VrmlConeParts::Sides),
            VrmlConeParts::All
        );
    }

    #[test]
    fn cone_parts_renders() {
        let sides = VrmlConeParts::Sides;
        assert!(sides.renders_sides());
        assert!(!sides.renders_bottom());

        let bottom = VrmlConeParts::Bottom;
        assert!(!bottom.renders_sides());
        assert!(bottom.renders_bottom());

        let all = VrmlConeParts::All;
        assert!(all.renders_sides());
        assert!(all.renders_bottom());
    }

    #[test]
    fn cone_parts_display() {
        assert_eq!(VrmlConeParts::Sides.to_string(), "SIDES");
        assert_eq!(VrmlConeParts::Bottom.to_string(), "BOTTOM");
        assert_eq!(VrmlConeParts::All.to_string(), "ALL");
    }

    #[test]
    fn cone_parts_equality() {
        assert_eq!(VrmlConeParts::Sides, VrmlConeParts::Sides);
        assert_ne!(VrmlConeParts::Sides, VrmlConeParts::Bottom);
    }

    #[test]
    fn cone_parts_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(VrmlConeParts::Sides);
        set.insert(VrmlConeParts::Bottom);
        set.insert(VrmlConeParts::All);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn round_trip_str() {
        for parts in VrmlConeParts::all_values() {
            let s = parts.as_str();
            let parsed = VrmlConeParts::from_str(s);
            assert_eq!(parsed, Some(*parts));
        }
    }

    #[test]
    fn round_trip_int() {
        for parts in VrmlConeParts::all_values() {
            let i = parts.as_int();
            let parsed = VrmlConeParts::from_int(i);
            assert_eq!(parsed, Some(*parts));
        }
    }
}
