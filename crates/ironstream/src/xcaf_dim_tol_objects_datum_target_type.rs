// FILE: xcaf_dim_tol_objects_datum_target_type.rs
// occt: XCAFDimTolObjects_DatumTargetType

/// Enumeration for datum target types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DatumTargetType {
    Point,
    Line,
    Circle,
    Plane,
    Cylinder,
    Sphere,
}

impl std::fmt::Display for DatumTargetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatumTargetType::Point => write!(f, "Point"),
            DatumTargetType::Line => write!(f, "Line"),
            DatumTargetType::Circle => write!(f, "Circle"),
            DatumTargetType::Plane => write!(f, "Plane"),
            DatumTargetType::Cylinder => write!(f, "Cylinder"),
            DatumTargetType::Sphere => write!(f, "Sphere"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct XCAFDimTolObjects_DatumTargetType {
    target_type: DatumTargetType,
}

impl XCAFDimTolObjects_DatumTargetType {
    /// Create a new datum target type.
    pub fn new(target_type: DatumTargetType) -> Self {
        Self { target_type }
    }

    /// Get the target type.
    pub fn target_type(&self) -> &DatumTargetType {
        &self.target_type
    }
}

impl Default for XCAFDimTolObjects_DatumTargetType {
    fn default() -> Self {
        Self::new(DatumTargetType::Point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_target_type() {
        let target = XCAFDimTolObjects_DatumTargetType::new(DatumTargetType::Circle);
        assert_eq!(target.target_type(), &DatumTargetType::Circle);
    }

    #[test]
    fn test_display() {
        assert_eq!(DatumTargetType::Point.to_string(), "Point");
        assert_eq!(DatumTargetType::Cylinder.to_string(), "Cylinder");
    }

    #[test]
    fn test_default() {
        let target = XCAFDimTolObjects_DatumTargetType::default();
        assert_eq!(target.target_type(), &DatumTargetType::Point);
    }
}
