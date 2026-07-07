// FILE: xml_m_data_xtd_constraint_driver.rs
// occt: XmlMDataXtd_ConstraintDriver

/// Constraint type enumeration matching TDataXtd constraint types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstraintType {
    Radius,
    Diameter,
    MinRadius,
    MajRadius,
    Tangent,
    Parallel,
    Perpendicular,
    Concentric,
    Coincident,
    Distance,
    Angle,
    EqualRadius,
    Symmetry,
    MidPoint,
    EqualDistance,
    Fix,
    Rigid,
    // Placement constraints
    From,
    Axis,
    Mate,
    AlignFaces,
    AlignAxes,
    AxesAngle,
}

/// XML serialization driver for constraint attributes.
/// Handles serialization and deserialization of geometric and placement constraints.
pub struct XmlMDataXtdConstraintDriver {
    type_name: String,
}

impl XmlMDataXtdConstraintDriver {
    /// Create a new constraint driver.
    pub fn new() -> Self {
        XmlMDataXtdConstraintDriver {
            type_name: "TDataXtd_Constraint".to_string(),
        }
    }

    /// Get the type name managed by this driver.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Convert constraint type enum to string representation.
    pub fn constraint_type_to_string(ct: ConstraintType) -> &'static str {
        match ct {
            ConstraintType::Radius => "radius",
            ConstraintType::Diameter => "diameter",
            ConstraintType::MinRadius => "minorradius",
            ConstraintType::MajRadius => "majorradius",
            ConstraintType::Tangent => "tangent",
            ConstraintType::Parallel => "parallel",
            ConstraintType::Perpendicular => "perpendicular",
            ConstraintType::Concentric => "concentric",
            ConstraintType::Coincident => "coincident",
            ConstraintType::Distance => "distance",
            ConstraintType::Angle => "angle",
            ConstraintType::EqualRadius => "equalradius",
            ConstraintType::Symmetry => "symmetry",
            ConstraintType::MidPoint => "midpoint",
            ConstraintType::EqualDistance => "equaldist",
            ConstraintType::Fix => "fix",
            ConstraintType::Rigid => "rigid",
            ConstraintType::From => "from",
            ConstraintType::Axis => "axis",
            ConstraintType::Mate => "mate",
            ConstraintType::AlignFaces => "alignfaces",
            ConstraintType::AlignAxes => "alignaxes",
            ConstraintType::AxesAngle => "axesangle",
        }
    }

    /// Convert string representation to constraint type enum.
    pub fn string_to_constraint_type(s: &str) -> Option<ConstraintType> {
        match s {
            "radius" => Some(ConstraintType::Radius),
            "diameter" => Some(ConstraintType::Diameter),
            "minorradius" => Some(ConstraintType::MinRadius),
            "majorradius" => Some(ConstraintType::MajRadius),
            "tangent" => Some(ConstraintType::Tangent),
            "parallel" => Some(ConstraintType::Parallel),
            "perpendicular" => Some(ConstraintType::Perpendicular),
            "concentric" => Some(ConstraintType::Concentric),
            "coincident" => Some(ConstraintType::Coincident),
            "distance" => Some(ConstraintType::Distance),
            "angle" => Some(ConstraintType::Angle),
            "equalradius" => Some(ConstraintType::EqualRadius),
            "symmetry" => Some(ConstraintType::Symmetry),
            "midpoint" => Some(ConstraintType::MidPoint),
            "equaldist" => Some(ConstraintType::EqualDistance),
            "fix" => Some(ConstraintType::Fix),
            "rigid" => Some(ConstraintType::Rigid),
            "from" => Some(ConstraintType::From),
            "axis" => Some(ConstraintType::Axis),
            "mate" => Some(ConstraintType::Mate),
            "alignfaces" => Some(ConstraintType::AlignFaces),
            "alignaxes" => Some(ConstraintType::AlignAxes),
            "axesangle" => Some(ConstraintType::AxesAngle),
            _ => None,
        }
    }
}

impl Default for XmlMDataXtdConstraintDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = XmlMDataXtdConstraintDriver::new();
        assert_eq!(driver.type_name(), "TDataXtd_Constraint");
    }

    #[test]
    fn test_constraint_type_to_string() {
        assert_eq!(XmlMDataXtdConstraintDriver::constraint_type_to_string(ConstraintType::Radius), "radius");
        assert_eq!(XmlMDataXtdConstraintDriver::constraint_type_to_string(ConstraintType::Distance), "distance");
        assert_eq!(XmlMDataXtdConstraintDriver::constraint_type_to_string(ConstraintType::Parallel), "parallel");
    }

    #[test]
    fn test_string_to_constraint_type() {
        assert_eq!(XmlMDataXtdConstraintDriver::string_to_constraint_type("radius"), Some(ConstraintType::Radius));
        assert_eq!(XmlMDataXtdConstraintDriver::string_to_constraint_type("distance"), Some(ConstraintType::Distance));
        assert_eq!(XmlMDataXtdConstraintDriver::string_to_constraint_type("unknown"), None);
    }

    #[test]
    fn test_roundtrip_conversion() {
        for ct in [
            ConstraintType::Radius,
            ConstraintType::Distance,
            ConstraintType::Angle,
            ConstraintType::Parallel,
            ConstraintType::Perpendicular,
        ] {
            let s = XmlMDataXtdConstraintDriver::constraint_type_to_string(ct);
            let ct2 = XmlMDataXtdConstraintDriver::string_to_constraint_type(s);
            assert_eq!(Some(ct), ct2);
        }
    }

    #[test]
    fn test_planar_constraints() {
        let planar = [
            ConstraintType::Radius,
            ConstraintType::Diameter,
            ConstraintType::Tangent,
            ConstraintType::Distance,
            ConstraintType::Angle,
        ];
        for ct in planar {
            let s = XmlMDataXtdConstraintDriver::constraint_type_to_string(ct);
            assert!(!s.is_empty());
        }
    }

    #[test]
    fn test_placement_constraints() {
        let placement = [
            ConstraintType::From,
            ConstraintType::Axis,
            ConstraintType::Mate,
            ConstraintType::AlignFaces,
        ];
        for ct in placement {
            let s = XmlMDataXtdConstraintDriver::constraint_type_to_string(ct);
            assert!(!s.is_empty());
        }
    }
}
