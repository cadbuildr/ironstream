// FILE: xcaf_dim_tol_objects_angular_qualifier.rs
// occt: XCAFDimTolObjects_AngularQualifier

/// Represents an angular qualifier for dimensions and tolerances.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AngularQualifierType {
    /// Degrees
    Degree,
    /// Radians
    Radian,
    /// Gradians
    Gradian,
}

#[derive(Clone, Debug)]
pub struct XCAFDimTolObjects_AngularQualifier {
    qualifier_type: AngularQualifierType,
}

impl XCAFDimTolObjects_AngularQualifier {
    /// Create a new angular qualifier.
    pub fn new(qualifier_type: AngularQualifierType) -> Self {
        Self { qualifier_type }
    }

    /// Get the qualifier type.
    pub fn qualifier_type(&self) -> &AngularQualifierType {
        &self.qualifier_type
    }

    /// Set the qualifier type.
    pub fn set_qualifier_type(&mut self, qualifier_type: AngularQualifierType) {
        self.qualifier_type = qualifier_type;
    }
}

impl Default for XCAFDimTolObjects_AngularQualifier {
    fn default() -> Self {
        Self::new(AngularQualifierType::Degree)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_degree_qualifier() {
        let qual = XCAFDimTolObjects_AngularQualifier::new(AngularQualifierType::Degree);
        assert_eq!(qual.qualifier_type(), &AngularQualifierType::Degree);
    }

    #[test]
    fn test_create_radian_qualifier() {
        let qual = XCAFDimTolObjects_AngularQualifier::new(AngularQualifierType::Radian);
        assert_eq!(qual.qualifier_type(), &AngularQualifierType::Radian);
    }

    #[test]
    fn test_set_qualifier_type() {
        let mut qual = XCAFDimTolObjects_AngularQualifier::new(AngularQualifierType::Degree);
        qual.set_qualifier_type(AngularQualifierType::Gradian);
        assert_eq!(qual.qualifier_type(), &AngularQualifierType::Gradian);
    }

    #[test]
    fn test_default() {
        let qual = XCAFDimTolObjects_AngularQualifier::default();
        assert_eq!(qual.qualifier_type(), &AngularQualifierType::Degree);
    }
}
