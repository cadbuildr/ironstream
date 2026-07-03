// FILE: g_prop_value_type.rs
// occt: GProp_ValueType

/// Enumeration of global property value types.
/// Used to identify which component of a global properties calculation is being queried.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValueType {
    /// Total mass (or length for curves, area for surfaces, volume for solids)
    Mass = 0,
    /// X-coordinate of center of mass
    CenterMassX = 1,
    /// Y-coordinate of center of mass
    CenterMassY = 2,
    /// Z-coordinate of center of mass
    CenterMassZ = 3,
    /// Inertia tensor component Ixx
    InertiaXX = 4,
    /// Inertia tensor component Iyy
    InertiaYY = 5,
    /// Inertia tensor component Izz
    InertiaZZ = 6,
    /// Inertia tensor component Ixy
    InertiaXY = 7,
    /// Inertia tensor component Ixz
    InertiaXZ = 8,
    /// Inertia tensor component Iyz
    InertiaYZ = 9,
    /// Unknown or invalid value type
    Unknown = 10,
}

impl ValueType {
    /// Returns true if this is a valid value type.
    pub fn is_valid(self) -> bool {
        self != ValueType::Unknown
    }

    /// Returns a human-readable name for this value type.
    pub fn name(self) -> &'static str {
        match self {
            ValueType::Mass => "Mass",
            ValueType::CenterMassX => "CenterMassX",
            ValueType::CenterMassY => "CenterMassY",
            ValueType::CenterMassZ => "CenterMassZ",
            ValueType::InertiaXX => "InertiaXX",
            ValueType::InertiaYY => "InertiaYY",
            ValueType::InertiaZZ => "InertiaZZ",
            ValueType::InertiaXY => "InertiaXY",
            ValueType::InertiaXZ => "InertiaXZ",
            ValueType::InertiaYZ => "InertiaYZ",
            ValueType::Unknown => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_type_mass() {
        assert_eq!(ValueType::Mass as i32, 0);
        assert!(ValueType::Mass.is_valid());
        assert_eq!(ValueType::Mass.name(), "Mass");
    }

    #[test]
    fn test_value_type_center_mass() {
        assert_eq!(ValueType::CenterMassX as i32, 1);
        assert_eq!(ValueType::CenterMassY as i32, 2);
        assert_eq!(ValueType::CenterMassZ as i32, 3);
        assert!(ValueType::CenterMassX.is_valid());
    }

    #[test]
    fn test_value_type_inertia() {
        assert_eq!(ValueType::InertiaXX as i32, 4);
        assert_eq!(ValueType::InertiaYY as i32, 5);
        assert_eq!(ValueType::InertiaZZ as i32, 6);
        assert_eq!(ValueType::InertiaXY as i32, 7);
        assert!(ValueType::InertiaXX.is_valid());
    }

    #[test]
    fn test_value_type_unknown() {
        assert_eq!(ValueType::Unknown as i32, 10);
        assert!(!ValueType::Unknown.is_valid());
        assert_eq!(ValueType::Unknown.name(), "Unknown");
    }

    #[test]
    fn test_value_type_names() {
        assert_eq!(ValueType::InertiaYZ.name(), "InertiaYZ");
        assert_eq!(ValueType::CenterMassX.name(), "CenterMassX");
    }
}
