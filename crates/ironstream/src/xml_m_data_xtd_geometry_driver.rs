// FILE: xml_m_data_xtd_geometry_driver.rs
// occt: XmlMDataXtd_GeometryDriver

/// Geometry type enumeration matching TDataXtd geometry types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeometryType {
    Any,
    Point,
    Line,
    Circle,
    Ellipse,
    Spline,
    Plane,
    Cylinder,
}

/// XML serialization driver for geometry attributes.
/// Handles serialization and deserialization of geometric element types.
pub struct XmlMDataXtdGeometryDriver {
    type_name: String,
}

impl XmlMDataXtdGeometryDriver {
    /// Create a new geometry driver.
    pub fn new() -> Self {
        XmlMDataXtdGeometryDriver {
            type_name: "TDataXtd_Geometry".to_string(),
        }
    }

    /// Get the type name managed by this driver.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Convert geometry type enum to string representation.
    pub fn geometry_type_to_string(gt: GeometryType) -> &'static str {
        match gt {
            GeometryType::Any => "any",
            GeometryType::Point => "point",
            GeometryType::Line => "line",
            GeometryType::Circle => "circle",
            GeometryType::Ellipse => "ellipse",
            GeometryType::Spline => "spline",
            GeometryType::Plane => "plane",
            GeometryType::Cylinder => "cylinder",
        }
    }

    /// Convert string representation to geometry type enum.
    pub fn string_to_geometry_type(s: &str) -> Option<GeometryType> {
        match s {
            "any" => Some(GeometryType::Any),
            "point" => Some(GeometryType::Point),
            "line" => Some(GeometryType::Line),
            "circle" => Some(GeometryType::Circle),
            "ellipse" => Some(GeometryType::Ellipse),
            "spline" => Some(GeometryType::Spline),
            "plane" => Some(GeometryType::Plane),
            "cylinder" => Some(GeometryType::Cylinder),
            _ => None,
        }
    }
}

impl Default for XmlMDataXtdGeometryDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_creation() {
        let driver = XmlMDataXtdGeometryDriver::new();
        assert_eq!(driver.type_name(), "TDataXtd_Geometry");
    }

    #[test]
    fn test_geometry_type_to_string() {
        assert_eq!(XmlMDataXtdGeometryDriver::geometry_type_to_string(GeometryType::Point), "point");
        assert_eq!(XmlMDataXtdGeometryDriver::geometry_type_to_string(GeometryType::Line), "line");
        assert_eq!(XmlMDataXtdGeometryDriver::geometry_type_to_string(GeometryType::Circle), "circle");
        assert_eq!(XmlMDataXtdGeometryDriver::geometry_type_to_string(GeometryType::Cylinder), "cylinder");
    }

    #[test]
    fn test_string_to_geometry_type() {
        assert_eq!(XmlMDataXtdGeometryDriver::string_to_geometry_type("point"), Some(GeometryType::Point));
        assert_eq!(XmlMDataXtdGeometryDriver::string_to_geometry_type("circle"), Some(GeometryType::Circle));
        assert_eq!(XmlMDataXtdGeometryDriver::string_to_geometry_type("unknown"), None);
    }

    #[test]
    fn test_roundtrip_conversion() {
        for gt in [
            GeometryType::Point,
            GeometryType::Line,
            GeometryType::Circle,
            GeometryType::Plane,
            GeometryType::Cylinder,
        ] {
            let s = XmlMDataXtdGeometryDriver::geometry_type_to_string(gt);
            let gt2 = XmlMDataXtdGeometryDriver::string_to_geometry_type(s);
            assert_eq!(Some(gt), gt2);
        }
    }

    #[test]
    fn test_all_types_convertible() {
        let types = [
            GeometryType::Any,
            GeometryType::Point,
            GeometryType::Line,
            GeometryType::Circle,
            GeometryType::Ellipse,
            GeometryType::Spline,
            GeometryType::Plane,
            GeometryType::Cylinder,
        ];
        for gt in types {
            let s = XmlMDataXtdGeometryDriver::geometry_type_to_string(gt);
            let gt2 = XmlMDataXtdGeometryDriver::string_to_geometry_type(s);
            assert_eq!(Some(gt), gt2);
        }
    }

    #[test]
    fn test_default_construction() {
        let driver = XmlMDataXtdGeometryDriver::default();
        assert_eq!(driver.type_name(), "TDataXtd_Geometry");
    }
}
