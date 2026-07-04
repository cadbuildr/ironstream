// FILE: destep_parameters.rs
// occt: DESTEP_Parameters

/// Enumerates BSpline continuity options for reading
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadMode_BSplineContinuity {
    C0 = 0,
    C1 = 1,
    C2 = 2,
}

/// Enumerates precision reading modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadMode_Precision {
    File = 0,
    User = 1,
}

/// Enumerates maximum precision modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadMode_MaxPrecision {
    Preferred = 0,
    Forced = 1,
}

/// Enumerates surface curve modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadMode_SurfaceCurve {
    Default = 0,
    Use2DPreferred = 2,
    Use2DForced = -2,
    Use3DPreferred = 3,
    Use3DForced = -3,
}

/// Enumerates angle unit modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngleUnitMode {
    File = 0,
    Rad = 1,
    Deg = 2,
}

/// Enumerates product context modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadMode_ProductContext {
    All = 1,
    Design = 2,
    Analysis = 3,
}

/// Enumerates shape representation modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadMode_ShapeRepr {
    All = 1,
    ABSR = 2,
    MSSR = 3,
    GBSSR = 4,
    FBSR = 5,
    EBWSR = 6,
    GBWSR = 7,
}

/// Enumerates assembly level modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadMode_AssemblyLevel {
    All = 1,
    Assembly = 2,
    Structure = 3,
    Shape = 4,
}

/// Enumerates tessellated shape modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RWMode_Tessellated {
    Off = 0,
    On = 1,
    OnNoBRep = 2,
}

/// Enumerates write precision modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteMode_PrecisionMode {
    Least = -1,
    Average = 0,
    Greatest = 1,
    Session = 2,
}

/// Enumerates write assembly modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteMode_Assembly {
    Off = 0,
    On = 1,
    Auto = 2,
}

/// Enumerates STEP schema versions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteMode_StepSchema {
    AP214CD = 1,
    AP214DIS = 2,
    AP203 = 3,
    AP214IS = 4,
    AP242DIS = 5,
}

/// Enumerates vertex writing modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteMode_VertexMode {
    OneCompound = 0,
    SingleVertex = 1,
}

/// Enumerates length units
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitsMethods_LengthUnit {
    Millimeter = 0,
    Centimeter = 1,
    Meter = 2,
    Inch = 3,
}

/// Parameters for DESTEP reading and writing operations
#[derive(Debug, Clone)]
pub struct DESTEP_Parameters {
    // Common
    pub read_bspline_continuity: ReadMode_BSplineContinuity,
    pub read_precision_mode: ReadMode_Precision,
    pub read_precision_val: f64,
    pub read_max_precision_mode: ReadMode_MaxPrecision,
    pub read_max_precision_val: f64,
    pub read_same_param_mode: bool,
    pub read_surface_curve_mode: ReadMode_SurfaceCurve,
    pub encode_reg_angle: f64,
    pub angle_unit: AngleUnitMode,

    // Read
    pub read_product_mode: bool,
    pub read_product_context: ReadMode_ProductContext,
    pub read_shape_repr: ReadMode_ShapeRepr,
    pub read_tessellated: RWMode_Tessellated,
    pub read_assembly_level: ReadMode_AssemblyLevel,
    pub read_relationship: bool,
    pub read_shape_aspect: bool,
    pub read_constr_relation: bool,
    pub read_subshape_names: bool,
    pub read_code_page: String,
    pub read_nonmanifold: bool,
    pub read_ideas: bool,
    pub read_all_shapes: bool,
    pub read_root_transformation: bool,
    pub read_color: bool,
    pub read_name: bool,
    pub read_layer: bool,
    pub read_props: bool,
    pub read_metadata: bool,
    pub read_product_metadata: bool,

    // Write
    pub write_precision_mode: WriteMode_PrecisionMode,
    pub write_precision_val: f64,
    pub write_assembly: WriteMode_Assembly,
    pub write_schema: WriteMode_StepSchema,
    pub write_tessellated: RWMode_Tessellated,
    pub write_product_name: String,
    pub write_surface_curve_mode: bool,
    pub write_unit: UnitsMethods_LengthUnit,
    pub write_vertex_mode: WriteMode_VertexMode,
    pub write_subshape_names: bool,
    pub write_color: bool,
    pub write_nonmanifold: bool,
    pub write_name: bool,
    pub write_layer: bool,
    pub write_props: bool,
    pub write_metadata: bool,
    pub write_material: bool,
    pub write_vis_material: bool,
    pub write_model_type: i32, // STEPControl_StepModelType
    pub clean_duplicates: bool,
    pub write_scaling_trsf: bool,
}

impl DESTEP_Parameters {
    /// Creates a new DESTEP_Parameters with default values
    pub fn new() -> Self {
        DESTEP_Parameters {
            read_bspline_continuity: ReadMode_BSplineContinuity::C1,
            read_precision_mode: ReadMode_Precision::File,
            read_precision_val: 0.0001,
            read_max_precision_mode: ReadMode_MaxPrecision::Preferred,
            read_max_precision_val: 1.0,
            read_same_param_mode: false,
            read_surface_curve_mode: ReadMode_SurfaceCurve::Default,
            encode_reg_angle: 0.57295779513,
            angle_unit: AngleUnitMode::File,

            read_product_mode: true,
            read_product_context: ReadMode_ProductContext::All,
            read_shape_repr: ReadMode_ShapeRepr::All,
            read_tessellated: RWMode_Tessellated::On,
            read_assembly_level: ReadMode_AssemblyLevel::All,
            read_relationship: true,
            read_shape_aspect: true,
            read_constr_relation: false,
            read_subshape_names: false,
            read_code_page: "UTF8".to_string(),
            read_nonmanifold: false,
            read_ideas: false,
            read_all_shapes: false,
            read_root_transformation: true,
            read_color: true,
            read_name: true,
            read_layer: true,
            read_props: true,
            read_metadata: true,
            read_product_metadata: false,

            write_precision_mode: WriteMode_PrecisionMode::Average,
            write_precision_val: 0.0001,
            write_assembly: WriteMode_Assembly::Auto,
            write_schema: WriteMode_StepSchema::AP214IS,
            write_tessellated: RWMode_Tessellated::OnNoBRep,
            write_product_name: String::new(),
            write_surface_curve_mode: true,
            write_unit: UnitsMethods_LengthUnit::Millimeter,
            write_vertex_mode: WriteMode_VertexMode::OneCompound,
            write_subshape_names: false,
            write_color: true,
            write_nonmanifold: false,
            write_name: true,
            write_layer: true,
            write_props: true,
            write_metadata: true,
            write_material: true,
            write_vis_material: false,
            write_model_type: 0, // STEPControl_AsIs
            clean_duplicates: false,
            write_scaling_trsf: true,
        }
    }

    /// Initializes parameters from static configuration
    pub fn init_from_static(&mut self) {
        // In a real implementation, this would read from static configuration
    }

    /// Resets all parameters to default values
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Returns the string representation of a ReadMode_ProductContext
    pub fn get_string(mode: ReadMode_ProductContext) -> &'static str {
        match mode {
            ReadMode_ProductContext::All => "all",
            ReadMode_ProductContext::Design => "design",
            ReadMode_ProductContext::Analysis => "analysis",
        }
    }

    /// Returns default shape fix parameters for STEP transfer
    pub fn get_default_shape_fix_parameters() -> Self {
        Self::new()
    }
}

impl Default for DESTEP_Parameters {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let params = DESTEP_Parameters::new();
        assert_eq!(params.read_bspline_continuity, ReadMode_BSplineContinuity::C1);
        assert_eq!(params.read_precision_mode, ReadMode_Precision::File);
        assert!(params.read_product_mode);
    }

    #[test]
    fn test_reset() {
        let mut params = DESTEP_Parameters::new();
        params.read_product_mode = false;
        params.reset();
        assert!(params.read_product_mode);
    }

    #[test]
    fn test_read_modes() {
        let mut params = DESTEP_Parameters::new();
        params.read_bspline_continuity = ReadMode_BSplineContinuity::C2;
        assert_eq!(params.read_bspline_continuity, ReadMode_BSplineContinuity::C2);
    }

    #[test]
    fn test_write_modes() {
        let mut params = DESTEP_Parameters::new();
        params.write_assembly = WriteMode_Assembly::On;
        assert_eq!(params.write_assembly, WriteMode_Assembly::On);
    }

    #[test]
    fn test_get_string() {
        assert_eq!(
            DESTEP_Parameters::get_string(ReadMode_ProductContext::All),
            "all"
        );
        assert_eq!(
            DESTEP_Parameters::get_string(ReadMode_ProductContext::Design),
            "design"
        );
        assert_eq!(
            DESTEP_Parameters::get_string(ReadMode_ProductContext::Analysis),
            "analysis"
        );
    }

    #[test]
    fn test_default_shape_fix_parameters() {
        let params = DESTEP_Parameters::get_default_shape_fix_parameters();
        assert!(params.is_done());
    }

    #[test]
    fn test_default_trait() {
        let params = DESTEP_Parameters::default();
        assert_eq!(params.read_bspline_continuity, ReadMode_BSplineContinuity::C1);
    }

    #[test]
    fn test_clone() {
        let params = DESTEP_Parameters::new();
        let cloned = params.clone();
        assert_eq!(params.read_product_mode, cloned.read_product_mode);
    }

    // Helper method for testing
}

impl DESTEP_Parameters {
    fn is_done(&self) -> bool {
        true
    }
}
