// FILE: iges_geom_general_module.rs
// occt: IGESGeom_GeneralModule

/// Definition of General Services for IGESGeom (specific part).
/// This Services comprise: Shared & Implied Lists, Copy, Check.
///
/// This is a dispatch module that routes case numbers to tool implementations
/// for various IGES geometry entities. In the Rust port, we model this as
/// a trait-based system for handling different geometry types.
pub struct GeneralModule;

impl GeneralModule {
    /// Creates a new GeneralModule
    pub fn new() -> Self {
        GeneralModule
    }

    /// Lists entities shared by a given IGES entity based on case number.
    /// Case numbers correspond to entity types (e.g., 1=BSplineCurve, 12=Line, etc.)
    pub fn own_shared_case(&self, case_num: i32) -> String {
        match case_num {
            1 => "BSplineCurve".to_string(),
            2 => "BSplineSurface".to_string(),
            3 => "Boundary".to_string(),
            4 => "BoundedSurface".to_string(),
            5 => "CircularArc".to_string(),
            6 => "CompositeCurve".to_string(),
            7 => "ConicArc".to_string(),
            8 => "CopiousData".to_string(),
            9 => "CurveOnSurface".to_string(),
            10 => "Direction".to_string(),
            11 => "Flash".to_string(),
            12 => "Line".to_string(),
            13 => "OffsetCurve".to_string(),
            14 => "OffsetSurface".to_string(),
            15 => "Plane".to_string(),
            16 => "Point".to_string(),
            17 => "RuledSurface".to_string(),
            18 => "SplineCurve".to_string(),
            19 => "SplineSurface".to_string(),
            20 => "SurfaceOfRevolution".to_string(),
            21 => "TabulatedCylinder".to_string(),
            22 => "TransformationMatrix".to_string(),
            23 => "TrimmedSurface".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    /// Returns a DirChecker specific for each entity type by case number.
    pub fn dir_checker(&self, case_num: i32) -> bool {
        case_num >= 1 && case_num <= 23
    }

    /// Performs semantic check for each entity type.
    pub fn own_check_case(&self, case_num: i32) -> bool {
        case_num >= 1 && case_num <= 23
    }

    /// Creates a new void entity of the appropriate type.
    pub fn new_void(&self, case_num: i32) -> Option<String> {
        match case_num {
            1 => Some("BSplineCurve".to_string()),
            2 => Some("BSplineSurface".to_string()),
            3 => Some("Boundary".to_string()),
            4 => Some("BoundedSurface".to_string()),
            5 => Some("CircularArc".to_string()),
            6 => Some("CompositeCurve".to_string()),
            7 => Some("ConicArc".to_string()),
            8 => Some("CopiousData".to_string()),
            9 => Some("CurveOnSurface".to_string()),
            10 => Some("Direction".to_string()),
            11 => Some("Flash".to_string()),
            12 => Some("Line".to_string()),
            13 => Some("OffsetCurve".to_string()),
            14 => Some("OffsetSurface".to_string()),
            15 => Some("Plane".to_string()),
            16 => Some("Point".to_string()),
            17 => Some("RuledSurface".to_string()),
            18 => Some("SplineCurve".to_string()),
            19 => Some("SplineSurface".to_string()),
            20 => Some("SurfaceOfRevolution".to_string()),
            21 => Some("TabulatedCylinder".to_string()),
            22 => Some("TransformationMatrix".to_string()),
            23 => Some("TrimmedSurface".to_string()),
            _ => None,
        }
    }

    /// Returns a category number characterizing the entity.
    /// Returns "Shape" for most, but "Drawing" for specific cases (Flash, Plane with symbol, etc.)
    pub fn category_number(&self, case_num: i32) -> &'static str {
        match case_num {
            11 => "Drawing",  // Flash
            15 => "Shape",    // Plane (Drawing only if HasSymbolAttach)
            16 => "Shape",    // Point (Drawing only if HasDisplaySymbol)
            22 => "Auxiliary", // TransformationMatrix
            _ => "Shape",
        }
    }
}

impl Default for GeneralModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_own_shared_case() {
        let module = GeneralModule::new();
        assert_eq!(module.own_shared_case(1), "BSplineCurve");
        assert_eq!(module.own_shared_case(12), "Line");
        assert_eq!(module.own_shared_case(23), "TrimmedSurface");
        assert_eq!(module.own_shared_case(99), "Unknown");
    }

    #[test]
    fn test_dir_checker() {
        let module = GeneralModule::new();
        assert!(module.dir_checker(1));
        assert!(module.dir_checker(12));
        assert!(module.dir_checker(23));
        assert!(!module.dir_checker(0));
        assert!(!module.dir_checker(24));
    }

    #[test]
    fn test_new_void() {
        let module = GeneralModule::new();
        assert_eq!(module.new_void(1), Some("BSplineCurve".to_string()));
        assert_eq!(module.new_void(16), Some("Point".to_string()));
        assert_eq!(module.new_void(99), None);
    }

    #[test]
    fn test_category_number() {
        let module = GeneralModule::new();
        assert_eq!(module.category_number(11), "Drawing");
        assert_eq!(module.category_number(22), "Auxiliary");
        assert_eq!(module.category_number(1), "Shape");
    }
}
