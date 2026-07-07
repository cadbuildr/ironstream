// FILE: vrml_ascii_text_justification.rs
// occt: Vrml_AsciiTextJustification
//
// Faithful port of OCCT Vrml_AsciiTextJustification (DataExchange/TKDEVRML/Vrml/
// Vrml_AsciiTextJustification.hxx/.cxx): enumeration for text alignment modes.
// LEFT, CENTER, RIGHT justify text in 3D space.

/// Text justification mode enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VrmlAsciiTextJustification {
    /// Left-aligned text.
    Left = 0,
    /// Center-aligned text.
    Center = 1,
    /// Right-aligned text.
    Right = 2,
}

impl VrmlAsciiTextJustification {
    /// Convert to string representation (VRML format).
    pub fn as_str(&self) -> &str {
        match self {
            VrmlAsciiTextJustification::Left => "LEFT",
            VrmlAsciiTextJustification::Center => "CENTER",
            VrmlAsciiTextJustification::Right => "RIGHT",
        }
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "LEFT" => Some(VrmlAsciiTextJustification::Left),
            "CENTER" => Some(VrmlAsciiTextJustification::Center),
            "RIGHT" => Some(VrmlAsciiTextJustification::Right),
            _ => None,
        }
    }

    /// Get the integer representation.
    pub fn as_int(&self) -> i32 {
        *self as i32
    }

    /// Create from integer representation.
    pub fn from_int(val: i32) -> Option<Self> {
        match val {
            0 => Some(VrmlAsciiTextJustification::Left),
            1 => Some(VrmlAsciiTextJustification::Center),
            2 => Some(VrmlAsciiTextJustification::Right),
            _ => None,
        }
    }

    /// Check if this is a valid justification value.
    pub fn is_valid(val: i32) -> bool {
        (0..=2).contains(&val)
    }

    /// Get all possible justification values.
    pub fn all_values() -> &'static [VrmlAsciiTextJustification] {
        &[
            VrmlAsciiTextJustification::Left,
            VrmlAsciiTextJustification::Center,
            VrmlAsciiTextJustification::Right,
        ]
    }

    /// Get a descriptive label.
    pub fn label(&self) -> &str {
        match self {
            VrmlAsciiTextJustification::Left => "Left-aligned",
            VrmlAsciiTextJustification::Center => "Center-aligned",
            VrmlAsciiTextJustification::Right => "Right-aligned",
        }
    }
}

impl Default for VrmlAsciiTextJustification {
    fn default() -> Self {
        VrmlAsciiTextJustification::Left
    }
}

impl std::fmt::Display for VrmlAsciiTextJustification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn justification_as_str() {
        assert_eq!(VrmlAsciiTextJustification::Left.as_str(), "LEFT");
        assert_eq!(VrmlAsciiTextJustification::Center.as_str(), "CENTER");
        assert_eq!(VrmlAsciiTextJustification::Right.as_str(), "RIGHT");
    }

    #[test]
    fn justification_from_str() {
        assert_eq!(
            VrmlAsciiTextJustification::from_str("LEFT"),
            Some(VrmlAsciiTextJustification::Left)
        );
        assert_eq!(
            VrmlAsciiTextJustification::from_str("CENTER"),
            Some(VrmlAsciiTextJustification::Center)
        );
        assert_eq!(
            VrmlAsciiTextJustification::from_str("RIGHT"),
            Some(VrmlAsciiTextJustification::Right)
        );
        assert_eq!(VrmlAsciiTextJustification::from_str("INVALID"), None);
    }

    #[test]
    fn justification_case_insensitive() {
        assert_eq!(
            VrmlAsciiTextJustification::from_str("left"),
            Some(VrmlAsciiTextJustification::Left)
        );
        assert_eq!(
            VrmlAsciiTextJustification::from_str("Center"),
            Some(VrmlAsciiTextJustification::Center)
        );
    }

    #[test]
    fn justification_as_int() {
        assert_eq!(VrmlAsciiTextJustification::Left.as_int(), 0);
        assert_eq!(VrmlAsciiTextJustification::Center.as_int(), 1);
        assert_eq!(VrmlAsciiTextJustification::Right.as_int(), 2);
    }

    #[test]
    fn justification_from_int() {
        assert_eq!(
            VrmlAsciiTextJustification::from_int(0),
            Some(VrmlAsciiTextJustification::Left)
        );
        assert_eq!(
            VrmlAsciiTextJustification::from_int(1),
            Some(VrmlAsciiTextJustification::Center)
        );
        assert_eq!(
            VrmlAsciiTextJustification::from_int(2),
            Some(VrmlAsciiTextJustification::Right)
        );
        assert_eq!(VrmlAsciiTextJustification::from_int(3), None);
        assert_eq!(VrmlAsciiTextJustification::from_int(-1), None);
    }

    #[test]
    fn justification_is_valid() {
        assert!(VrmlAsciiTextJustification::is_valid(0));
        assert!(VrmlAsciiTextJustification::is_valid(1));
        assert!(VrmlAsciiTextJustification::is_valid(2));
        assert!(!VrmlAsciiTextJustification::is_valid(-1));
        assert!(!VrmlAsciiTextJustification::is_valid(3));
    }

    #[test]
    fn justification_all_values() {
        let values = VrmlAsciiTextJustification::all_values();
        assert_eq!(values.len(), 3);
        assert!(values.contains(&VrmlAsciiTextJustification::Left));
        assert!(values.contains(&VrmlAsciiTextJustification::Center));
        assert!(values.contains(&VrmlAsciiTextJustification::Right));
    }

    #[test]
    fn justification_default() {
        assert_eq!(VrmlAsciiTextJustification::default(), VrmlAsciiTextJustification::Left);
    }

    #[test]
    fn justification_label() {
        assert_eq!(VrmlAsciiTextJustification::Left.label(), "Left-aligned");
        assert_eq!(VrmlAsciiTextJustification::Center.label(), "Center-aligned");
        assert_eq!(VrmlAsciiTextJustification::Right.label(), "Right-aligned");
    }

    #[test]
    fn justification_display() {
        assert_eq!(VrmlAsciiTextJustification::Left.to_string(), "LEFT");
        assert_eq!(VrmlAsciiTextJustification::Center.to_string(), "CENTER");
        assert_eq!(VrmlAsciiTextJustification::Right.to_string(), "RIGHT");
    }

    #[test]
    fn justification_equality() {
        assert_eq!(VrmlAsciiTextJustification::Left, VrmlAsciiTextJustification::Left);
        assert_ne!(VrmlAsciiTextJustification::Left, VrmlAsciiTextJustification::Center);
    }

    #[test]
    fn justification_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(VrmlAsciiTextJustification::Left);
        set.insert(VrmlAsciiTextJustification::Center);
        set.insert(VrmlAsciiTextJustification::Right);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn round_trip_str() {
        for justif in VrmlAsciiTextJustification::all_values() {
            let s = justif.as_str();
            let parsed = VrmlAsciiTextJustification::from_str(s);
            assert_eq!(parsed, Some(*justif));
        }
    }

    #[test]
    fn round_trip_int() {
        for justif in VrmlAsciiTextJustification::all_values() {
            let i = justif.as_int();
            let parsed = VrmlAsciiTextJustification::from_int(i);
            assert_eq!(parsed, Some(*justif));
        }
    }
}
