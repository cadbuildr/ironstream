// FILE: model_definitions.rs
// occt: ModelDefinitions

/// GUID constants for geometric and modeling objects
pub struct Guid {
    pub guid_string: String,
}

impl Guid {
    pub fn new(guid_string: &str) -> Self {
        Guid {
            guid_string: guid_string.to_string(),
        }
    }
}

/// GUID for geometry objects
pub const GEOMOBJECT_GUID: &str = "6c6915ab-775f-4475-859e-befd74d26a23";

/// GUIDs for attachment types
pub const ATTCH_GUID: &str = "12e94554-6dbc-11d4-b9c8-0060b0ee281b";
pub const XTTCH_GUID: &str = "12e94555-6dbc-11d4-b9c8-0060b0ee281b";

/// GUIDs for point types
pub const PTXYZ_GUID: &str = "12e94556-6dbc-11d4-b9c8-0060b0ee281b";
pub const PTALINE_GUID: &str = "12e94557-6dbc-11d4-b9c8-0060b0ee281b";
pub const PRRLINE_GUID: &str = "12e94558-6dbc-11d4-b9c8-0060b0ee281b";
pub const PMIRR_GUID: &str = "12e94559-6dbc-11d4-b9c8-0060b0ee281b";

/// GUIDs for primitive shapes
pub const BOX_GUID: &str = "12e94543-6dbc-11d4-b9c8-0060b0ee281b";
pub const SPH_GUID: &str = "12e94544-6dbc-11d4-b9c8-0060b0ee281b";
pub const CYL_GUID: &str = "12e94545-6dbc-11d4-b9c8-0060b0ee281b";
pub const CONE_GUID: &str = "12e94546-6dbc-11d4-b9c8-0060b0ee281b";
pub const TORUS_GUID: &str = "12e94547-6dbc-11d4-b9c8-0060b0ee281b";

/// GUIDs for boolean operations
pub const CUT_GUID: &str = "12e94548-6dbc-11d4-b9c8-0060b0ee281b";
pub const FUSE_GUID: &str = "12e94549-6dbc-11d4-b9c8-0060b0ee281b";
pub const COMMON_GUID: &str = "12e9454a-6dbc-11d4-b9c8-0060b0ee281b";
pub const SECTION_GUID: &str = "12e9454b-6dbc-11d4-b9c8-0060b0ee281b";

/// GUIDs for transformations
pub const PRISM_GUID: &str = "12e94550-6dbc-11d4-b9c8-0060b0ee281b";
pub const FULREVOL_GUID: &str = "12e94551-6dbc-11d4-b9c8-0060b0ee281b";
pub const SECREVOL_GUID: &str = "12e94552-6dbc-11d4-b9c8-0060b0ee281b";
pub const FILLT_GUID: &str = "12e94553-6dbc-11d4-b9c8-0060b0ee281b";
pub const CHAMF_GUID: &str = "12e9455a-6dbc-11d4-b9c8-0060b0ee281b";
pub const OFFSET_GUID: &str = "12e9455b-6dbc-11d4-b9c8-0060b0ee281b";

/// GUIDs for construction geometries
pub const PNTXYZ_GUID: &str = "12e9455c-6dbc-11d4-b9c8-0060b0ee281b";
pub const PNTRLT_GUID: &str = "12e9455d-6dbc-11d4-b9c8-0060b0ee281b";
pub const LINE3D_GUID: &str = "12e9455e-6dbc-11d4-b9c8-0060b0ee281b";
pub const WIRE_GUID: &str = "12e9455f-6dbc-11d4-b9c8-0060b0ee281b";

/// Function structure label indices
pub const FUNCTION_ARGUMENTS_LABEL: i32 = 1;
pub const FUNCTION_RESULT_LABEL: i32 = 2;

/// Parameter indices for Box parameters
pub const BOX_DX: i32 = 1;
pub const BOX_DY: i32 = 2;
pub const BOX_DZ: i32 = 3;

/// Parameter indices for Cylinder parameters
pub const CYL_RADIUS: i32 = 1;
pub const CYL_HEIGHT: i32 = 2;
pub const CYL_AXIS: i32 = 3;

/// Parameter indices for Boolean operations
pub const ATTACH_ARG: i32 = 1;
pub const BOOL_TOOL: i32 = 1;
pub const SECT_OBJECT: i32 = 1;
pub const SECT_TOOL: i32 = 2;

/// Parameter indices for Fillet operations
pub const FILLET_RADIUS: i32 = 1;
pub const FILLET_SURFTYPE: i32 = 2;
pub const FILLET_PATH: i32 = 3;

/// Parameter indices for point transformations
pub const PTRANSF_DX: i32 = 1;
pub const PTRANSF_DY: i32 = 2;
pub const PTRANSF_DZ: i32 = 3;
pub const PTRANSF_OFF: i32 = 1;
pub const PTRANSF_ANG: i32 = 1;
pub const PTRANSF_LINE: i32 = 2;
pub const PTRANSF_PLANE: i32 = 1;

/// Parameter indices for Prism
pub const PRISM_BASIS: i32 = 1;
pub const PRISM_HEIGHT: i32 = 2;
pub const PRISM_DIR: i32 = 3;

/// Parameter indices for Revolution
pub const REVOL_BASIS: i32 = 1;
pub const REVOL_AXIS: i32 = 2;
pub const REVOL_ANGLE: i32 = 3;
pub const REVOL_REV: i32 = 4;

/// Parameter indices for Sphere
pub const SPHERE_CENTER: i32 = 1;
pub const SPHERE_RADIUS: i32 = 2;

/// Parameter indices for Point construction
pub const PNT_DX: i32 = 1;
pub const PNT_DY: i32 = 2;
pub const PNT_DZ: i32 = 3;
pub const PNTRLT_REF: i32 = 4;

/// Parameter indices for Line3D
pub const LINE3D_TYPE: i32 = 1;
pub const LINE3D_PNTNB: i32 = 2;

/// Status/Result codes
pub const DONE: i32 = 0;
pub const NOTDONE: i32 = 9999;
pub const ALGO_FAILED: i32 = 11;
pub const RESULT_NOT_VALID: i32 = 12;
pub const WRONG_AXIS: i32 = 13;
pub const WRONG_ARGUMENT: i32 = 14;
pub const UNSUPPORTED_FUNCTION: i32 = 15;
pub const NULL_RESULT: i32 = 16;
pub const WRONG_CONTEXT: i32 = 14;
pub const NAMING_FAILED: i32 = 15;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guid_creation() {
        let guid = Guid::new(GEOMOBJECT_GUID);
        assert_eq!(guid.guid_string, "6c6915ab-775f-4475-859e-befd74d26a23");
    }

    #[test]
    fn test_guid_constants() {
        assert!(!GEOMOBJECT_GUID.is_empty());
        assert!(!BOX_GUID.is_empty());
        assert!(!CYL_GUID.is_empty());
        assert!(!SPHERE_RADIUS.to_string().is_empty());
    }

    #[test]
    fn test_function_labels() {
        assert_eq!(FUNCTION_ARGUMENTS_LABEL, 1);
        assert_eq!(FUNCTION_RESULT_LABEL, 2);
    }

    #[test]
    fn test_box_parameters() {
        assert_eq!(BOX_DX, 1);
        assert_eq!(BOX_DY, 2);
        assert_eq!(BOX_DZ, 3);
    }

    #[test]
    fn test_cylinder_parameters() {
        assert_eq!(CYL_RADIUS, 1);
        assert_eq!(CYL_HEIGHT, 2);
        assert_eq!(CYL_AXIS, 3);
    }

    #[test]
    fn test_status_codes() {
        assert_eq!(DONE, 0);
        assert_eq!(NOTDONE, 9999);
        assert_eq!(ALGO_FAILED, 11);
        assert_eq!(RESULT_NOT_VALID, 12);
        assert_eq!(NULL_RESULT, 16);
    }

    #[test]
    fn test_prism_parameters() {
        assert_eq!(PRISM_BASIS, 1);
        assert_eq!(PRISM_HEIGHT, 2);
        assert_eq!(PRISM_DIR, 3);
    }

    #[test]
    fn test_point_parameters() {
        assert_eq!(PNT_DX, 1);
        assert_eq!(PNT_DY, 2);
        assert_eq!(PNT_DZ, 3);
        assert_eq!(PNTRLT_REF, 4);
    }

    #[test]
    fn test_fillet_parameters() {
        assert_eq!(FILLET_RADIUS, 1);
        assert_eq!(FILLET_SURFTYPE, 2);
        assert_eq!(FILLET_PATH, 3);
    }

    #[test]
    fn test_guid_for_primitives() {
        assert_eq!(BOX_GUID, "12e94543-6dbc-11d4-b9c8-0060b0ee281b");
        assert_eq!(CYL_GUID, "12e94545-6dbc-11d4-b9c8-0060b0ee281b");
    }

    #[test]
    fn test_guid_for_operations() {
        assert_eq!(CUT_GUID, "12e94548-6dbc-11d4-b9c8-0060b0ee281b");
        assert_eq!(FUSE_GUID, "12e94549-6dbc-11d4-b9c8-0060b0ee281b");
    }
}
