// FILE: stepcaf_control_gdt_property.rs
// occt: STEPCAFControl_GDTProperty

/// Dimension Modifier enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionModif {
    Modifier(i32),
}

/// Dimension Form Variance enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionFormVariance {
    FormVariance(i32),
}

/// Dimension Grade enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionGrade {
    Grade(i32),
}

/// Dimension Type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionType {
    Diameter,
    Radius,
    Linear,
    Other(i32),
}

/// Datum Target Type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatumTargetType {
    Point,
    Line,
    Circle,
    Sphere,
    Other(i32),
}

/// Dimension Qualifier enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionQualifier {
    Max,
    Min,
    Nominal,
    Other(i32),
}

/// Geometric Tolerance Type Value enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeomToleranceTypeValue {
    Straightness,
    Flatness,
    Circularity,
    Cylindricity,
    LineProfile,
    SurfaceProfile,
    Parallelism,
    Perpendicularity,
    Angularity,
    Position,
    Runout,
    Other(i32),
}

/// Geometric Tolerance Modifier enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeomToleranceModif {
    Modifier(i32),
}

/// Geometric Tolerance Type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeomToleranceType {
    Straightness,
    Flatness,
    Other(i32),
}

/// This class provides tools for access (read) the GDT properties.
pub struct STEPCAFControl_GDTProperty;

impl STEPCAFControl_GDTProperty {
    pub fn new() -> Self {
        STEPCAFControl_GDTProperty
    }

    /// Get dimension modifiers from CompoundRepresentationItem
    pub fn get_dim_modifiers(
        _critem: (),
        _modifiers: &mut Vec<DimensionModif>,
    ) {
        // TODO: Implement based on actual STEP compound representation analysis
    }

    /// Get dimension class of tolerance from LimitsAndFits
    pub fn get_dim_class_of_tolerance(
        _laf: (),
        _is_hole: &mut bool,
        _form_variance: &mut DimensionFormVariance,
        _grade: &mut DimensionGrade,
    ) {
        // TODO: Implement based on actual STEP limits and fits properties
    }

    /// Get dimension type from string name
    pub fn get_dim_type(name: &str) -> Option<DimensionType> {
        match name {
            "DIAMETER" => Some(DimensionType::Diameter),
            "RADIUS" => Some(DimensionType::Radius),
            "LINEAR" => Some(DimensionType::Linear),
            _ => None,
        }
    }

    /// Get datum target type from description string
    pub fn get_datum_target_type(description: &str) -> Option<DatumTargetType> {
        match description {
            "POINT" => Some(DatumTargetType::Point),
            "LINE" => Some(DatumTargetType::Line),
            "CIRCLE" => Some(DatumTargetType::Circle),
            "SPHERE" => Some(DatumTargetType::Sphere),
            _ => None,
        }
    }

    /// Get dimension qualifier type from description string
    pub fn get_dim_qualifier_type(description: &str) -> Option<DimensionQualifier> {
        match description {
            "MAX" => Some(DimensionQualifier::Max),
            "MIN" => Some(DimensionQualifier::Min),
            "NOMINAL" => Some(DimensionQualifier::Nominal),
            _ => None,
        }
    }

    /// Get tolerance value type from description string
    pub fn get_tol_value_type(description: &str) -> Option<GeomToleranceTypeValue> {
        match description {
            "STRAIGHTNESS" => Some(GeomToleranceTypeValue::Straightness),
            "FLATNESS" => Some(GeomToleranceTypeValue::Flatness),
            "CIRCULARITY" => Some(GeomToleranceTypeValue::Circularity),
            "CYLINDRICITY" => Some(GeomToleranceTypeValue::Cylindricity),
            "LINE_PROFILE" => Some(GeomToleranceTypeValue::LineProfile),
            "SURFACE_PROFILE" => Some(GeomToleranceTypeValue::SurfaceProfile),
            "PARALLELISM" => Some(GeomToleranceTypeValue::Parallelism),
            "PERPENDICULARITY" => Some(GeomToleranceTypeValue::Perpendicularity),
            "ANGULARITY" => Some(GeomToleranceTypeValue::Angularity),
            "POSITION" => Some(GeomToleranceTypeValue::Position),
            "RUNOUT" => Some(GeomToleranceTypeValue::Runout),
            _ => None,
        }
    }

    /// Get tolerance value type string from enum
    pub fn get_tol_value_type_str(toltype: GeomToleranceTypeValue) -> &'static str {
        match toltype {
            GeomToleranceTypeValue::Straightness => "STRAIGHTNESS",
            GeomToleranceTypeValue::Flatness => "FLATNESS",
            GeomToleranceTypeValue::Circularity => "CIRCULARITY",
            GeomToleranceTypeValue::Cylindricity => "CYLINDRICITY",
            GeomToleranceTypeValue::LineProfile => "LINE_PROFILE",
            GeomToleranceTypeValue::SurfaceProfile => "SURFACE_PROFILE",
            GeomToleranceTypeValue::Parallelism => "PARALLELISM",
            GeomToleranceTypeValue::Perpendicularity => "PERPENDICULARITY",
            GeomToleranceTypeValue::Angularity => "ANGULARITY",
            GeomToleranceTypeValue::Position => "POSITION",
            GeomToleranceTypeValue::Runout => "RUNOUT",
            GeomToleranceTypeValue::Other(_) => "OTHER",
        }
    }

    /// Get dimension type name string from enum
    pub fn get_dim_type_name(dimtype: DimensionType) -> &'static str {
        match dimtype {
            DimensionType::Diameter => "DIAMETER",
            DimensionType::Radius => "RADIUS",
            DimensionType::Linear => "LINEAR",
            DimensionType::Other(_) => "OTHER",
        }
    }

    /// Get dimension qualifier name string from enum
    pub fn get_dim_qualifier_name(qualifier: DimensionQualifier) -> &'static str {
        match qualifier {
            DimensionQualifier::Max => "MAX",
            DimensionQualifier::Min => "MIN",
            DimensionQualifier::Nominal => "NOMINAL",
            DimensionQualifier::Other(_) => "OTHER",
        }
    }

    /// Get dimension modifier name string from enum
    pub fn get_dim_modifier_name(_modifier: DimensionModif) -> &'static str {
        "MODIFIER"
    }

    /// Get datum target name string from enum
    pub fn get_datum_target_name(datum_type: DatumTargetType) -> &'static str {
        match datum_type {
            DatumTargetType::Point => "POINT",
            DatumTargetType::Line => "LINE",
            DatumTargetType::Circle => "CIRCLE",
            DatumTargetType::Sphere => "SPHERE",
            DatumTargetType::Other(_) => "OTHER",
        }
    }

    /// Convert geometric tolerance type to geometric tolerance enum
    pub fn get_geom_tolerance_type_from_xcaf(
        _xcaf_type: GeomToleranceType,
    ) -> GeomToleranceTypeValue {
        GeomToleranceTypeValue::Straightness
    }

    /// Convert STEP geometric tolerance type to XCAF type
    pub fn get_geom_tolerance_type_to_xcaf(
        _step_type: (),
    ) -> GeomToleranceType {
        GeomToleranceType::Straightness
    }
}

impl Default for STEPCAFControl_GDTProperty {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_dim_type() {
        assert_eq!(
            STEPCAFControl_GDTProperty::get_dim_type("DIAMETER"),
            Some(DimensionType::Diameter)
        );
        assert_eq!(
            STEPCAFControl_GDTProperty::get_dim_type("RADIUS"),
            Some(DimensionType::Radius)
        );
        assert_eq!(
            STEPCAFControl_GDTProperty::get_dim_type("LINEAR"),
            Some(DimensionType::Linear)
        );
        assert_eq!(STEPCAFControl_GDTProperty::get_dim_type("UNKNOWN"), None);
    }

    #[test]
    fn test_get_datum_target_type() {
        assert_eq!(
            STEPCAFControl_GDTProperty::get_datum_target_type("POINT"),
            Some(DatumTargetType::Point)
        );
        assert_eq!(
            STEPCAFControl_GDTProperty::get_datum_target_type("CIRCLE"),
            Some(DatumTargetType::Circle)
        );
    }

    #[test]
    fn test_get_tol_value_type() {
        assert_eq!(
            STEPCAFControl_GDTProperty::get_tol_value_type("STRAIGHTNESS"),
            Some(GeomToleranceTypeValue::Straightness)
        );
        assert_eq!(
            STEPCAFControl_GDTProperty::get_tol_value_type("FLATNESS"),
            Some(GeomToleranceTypeValue::Flatness)
        );
    }

    #[test]
    fn test_get_dim_type_name() {
        assert_eq!(
            STEPCAFControl_GDTProperty::get_dim_type_name(DimensionType::Diameter),
            "DIAMETER"
        );
        assert_eq!(
            STEPCAFControl_GDTProperty::get_dim_type_name(DimensionType::Radius),
            "RADIUS"
        );
    }

    #[test]
    fn test_roundtrip_dim_type() {
        let original = DimensionType::Diameter;
        let name = STEPCAFControl_GDTProperty::get_dim_type_name(original);
        let restored = STEPCAFControl_GDTProperty::get_dim_type(name).unwrap();
        assert_eq!(original, restored);
    }
}
