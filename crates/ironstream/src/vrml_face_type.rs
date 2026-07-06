// FILE: vrml_face_type.rs
// occt: Vrml_FaceType
//
// Faithful port of OCCT Vrml_FaceType (DataExchange/TKDEVRML/Vrml/
// Vrml_FaceType.hxx/.cxx): enumeration for polygon face types.
// Controls how polygons are rendered (solid, wireframe, or points).

/// Face type rendering mode enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VrmlFaceType {
    /// Render solid filled faces (default).
    Solid = 0,
    /// Render wireframe (edges only).
    Wireframe = 1,
    /// Render vertices only (point cloud).
    Points = 2,
}

impl VrmlFaceType {
    /// Convert to integer representation.
    pub fn as_int(&self) -> i32 {
        *self as i32
    }

    /// Create from integer representation.
    pub fn from_int(val: i32) -> Option<Self> {
        match val {
            0 => Some(VrmlFaceType::Solid),
            1 => Some(VrmlFaceType::Wireframe),
            2 => Some(VrmlFaceType::Points),
            _ => None,
        }
    }

    /// Check if an integer value is valid.
    pub fn is_valid(val: i32) -> bool {
        matches!(val, 0..=2)
    }

    /// Get all valid face type values.
    pub fn all_values() -> &'static [VrmlFaceType] {
        &[VrmlFaceType::Solid, VrmlFaceType::Wireframe, VrmlFaceType::Points]
    }

    /// Get a human-readable description.
    pub fn description(&self) -> &str {
        match self {
            VrmlFaceType::Solid => "Solid faces",
            VrmlFaceType::Wireframe => "Wireframe",
            VrmlFaceType::Points => "Points only",
        }
    }

    /// Get the VRML string representation.
    pub fn as_str(&self) -> &str {
        match self {
            VrmlFaceType::Solid => "SOLID",
            VrmlFaceType::Wireframe => "WIREFRAME",
            VrmlFaceType::Points => "POINTS",
        }
    }

    /// Parse from VRML string representation.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "SOLID" => Some(VrmlFaceType::Solid),
            "WIREFRAME" => Some(VrmlFaceType::Wireframe),
            "POINTS" => Some(VrmlFaceType::Points),
            _ => None,
        }
    }

    /// Check if this is a solid rendering mode.
    pub fn is_solid(&self) -> bool {
        matches!(self, VrmlFaceType::Solid)
    }

    /// Check if this is a wireframe rendering mode.
    pub fn is_wireframe(&self) -> bool {
        matches!(self, VrmlFaceType::Wireframe)
    }

    /// Check if this is points-only rendering mode.
    pub fn is_points(&self) -> bool {
        matches!(self, VrmlFaceType::Points)
    }

    /// Get the number of valid face types.
    pub fn count() -> usize {
        3
    }
}

impl Default for VrmlFaceType {
    fn default() -> Self {
        VrmlFaceType::Solid
    }
}

impl std::fmt::Display for VrmlFaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::cmp::Ord for VrmlFaceType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_int().cmp(&other.as_int())
    }
}

impl std::cmp::PartialOrd for VrmlFaceType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn face_type_as_int() {
        assert_eq!(VrmlFaceType::Solid.as_int(), 0);
        assert_eq!(VrmlFaceType::Wireframe.as_int(), 1);
        assert_eq!(VrmlFaceType::Points.as_int(), 2);
    }

    #[test]
    fn face_type_from_int() {
        assert_eq!(VrmlFaceType::from_int(0), Some(VrmlFaceType::Solid));
        assert_eq!(VrmlFaceType::from_int(1), Some(VrmlFaceType::Wireframe));
        assert_eq!(VrmlFaceType::from_int(2), Some(VrmlFaceType::Points));
        assert_eq!(VrmlFaceType::from_int(-1), None);
        assert_eq!(VrmlFaceType::from_int(3), None);
    }

    #[test]
    fn face_type_is_valid() {
        assert!(VrmlFaceType::is_valid(0));
        assert!(VrmlFaceType::is_valid(1));
        assert!(VrmlFaceType::is_valid(2));
        assert!(!VrmlFaceType::is_valid(-1));
        assert!(!VrmlFaceType::is_valid(3));
    }

    #[test]
    fn face_type_all_values() {
        let values = VrmlFaceType::all_values();
        assert_eq!(values.len(), 3);
        assert!(values.contains(&VrmlFaceType::Solid));
        assert!(values.contains(&VrmlFaceType::Wireframe));
        assert!(values.contains(&VrmlFaceType::Points));
    }

    #[test]
    fn face_type_default() {
        assert_eq!(VrmlFaceType::default(), VrmlFaceType::Solid);
    }

    #[test]
    fn face_type_as_str() {
        assert_eq!(VrmlFaceType::Solid.as_str(), "SOLID");
        assert_eq!(VrmlFaceType::Wireframe.as_str(), "WIREFRAME");
        assert_eq!(VrmlFaceType::Points.as_str(), "POINTS");
    }

    #[test]
    fn face_type_from_str() {
        assert_eq!(VrmlFaceType::from_str("SOLID"), Some(VrmlFaceType::Solid));
        assert_eq!(
            VrmlFaceType::from_str("WIREFRAME"),
            Some(VrmlFaceType::Wireframe)
        );
        assert_eq!(VrmlFaceType::from_str("POINTS"), Some(VrmlFaceType::Points));
        assert_eq!(VrmlFaceType::from_str("INVALID"), None);
    }

    #[test]
    fn face_type_from_str_case_insensitive() {
        assert_eq!(VrmlFaceType::from_str("solid"), Some(VrmlFaceType::Solid));
        assert_eq!(VrmlFaceType::from_str("Wireframe"), Some(VrmlFaceType::Wireframe));
    }

    #[test]
    fn face_type_description() {
        assert_eq!(VrmlFaceType::Solid.description(), "Solid faces");
        assert_eq!(VrmlFaceType::Wireframe.description(), "Wireframe");
        assert_eq!(VrmlFaceType::Points.description(), "Points only");
    }

    #[test]
    fn face_type_is_solid() {
        assert!(VrmlFaceType::Solid.is_solid());
        assert!(!VrmlFaceType::Wireframe.is_solid());
        assert!(!VrmlFaceType::Points.is_solid());
    }

    #[test]
    fn face_type_is_wireframe() {
        assert!(!VrmlFaceType::Solid.is_wireframe());
        assert!(VrmlFaceType::Wireframe.is_wireframe());
        assert!(!VrmlFaceType::Points.is_wireframe());
    }

    #[test]
    fn face_type_is_points() {
        assert!(!VrmlFaceType::Solid.is_points());
        assert!(!VrmlFaceType::Wireframe.is_points());
        assert!(VrmlFaceType::Points.is_points());
    }

    #[test]
    fn face_type_count() {
        assert_eq!(VrmlFaceType::count(), 3);
    }

    #[test]
    fn face_type_display() {
        assert_eq!(VrmlFaceType::Solid.to_string(), "SOLID");
        assert_eq!(VrmlFaceType::Wireframe.to_string(), "WIREFRAME");
        assert_eq!(VrmlFaceType::Points.to_string(), "POINTS");
    }

    #[test]
    fn face_type_equality() {
        assert_eq!(VrmlFaceType::Solid, VrmlFaceType::Solid);
        assert_ne!(VrmlFaceType::Solid, VrmlFaceType::Wireframe);
    }

    #[test]
    fn face_type_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(VrmlFaceType::Solid);
        set.insert(VrmlFaceType::Wireframe);
        set.insert(VrmlFaceType::Points);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn face_type_ordering() {
        assert!(VrmlFaceType::Solid < VrmlFaceType::Wireframe);
        assert!(VrmlFaceType::Wireframe < VrmlFaceType::Points);
        assert!(VrmlFaceType::Solid < VrmlFaceType::Points);
    }

    #[test]
    fn round_trip_str() {
        for face_type in VrmlFaceType::all_values() {
            let s = face_type.as_str();
            let parsed = VrmlFaceType::from_str(s);
            assert_eq!(parsed, Some(*face_type));
        }
    }

    #[test]
    fn round_trip_int() {
        for face_type in VrmlFaceType::all_values() {
            let i = face_type.as_int();
            let parsed = VrmlFaceType::from_int(i);
            assert_eq!(parsed, Some(*face_type));
        }
    }
}
