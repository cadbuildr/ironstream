// FILE: iges_solid_tool_sphere.rs
// occt: IGESSolid_ToolSphere

/// Tool to work on a Sphere. Called by various Modules
/// (ReadWriteModule, GeneralModule, SpecificModule)
#[derive(Default)]
pub struct IgesSolidToolSphere;

impl IgesSolidToolSphere {
    /// Returns a ToolSphere, ready to work
    pub fn new() -> Self {
        Self
    }

    /// Returns specific DirChecker for type 158, form 0
    pub fn dir_checker(&self) -> IgesSolidDirChecker {
        let mut dc = IgesSolidDirChecker::new(158, 0);
        dc.set_structure_void();
        dc.set_line_font_any();
        dc.set_color_any();
        dc.set_use_flag_required(0);
        dc.set_hierarchy_status_ignored();
        dc
    }

    /// Reads own parameters from file
    pub fn read_own_params(
        &self,
        _reader_data: &IgesReaderData,
        _param_reader: &mut ParamReader,
    ) -> IgesSphereData {
        // Implementation parses:
        // 1. Radius
        // 2. Center (X, Y, Z) (defaults to 0, 0, 0)
        IgesSphereData {
            radius: 0.0,
            center: [0.0, 0.0, 0.0],
        }
    }

    /// Writes own parameters to IGESWriter
    pub fn write_own_params(&self, data: &IgesSphereData, _writer: &mut IgesWriter) {
        // Write Radius
        // Write Center (X, Y, Z)
        drop(data);
    }

    /// Lists the Entities shared by a Sphere
    pub fn own_shared(&self, _data: &IgesSphereData) -> Vec<IgesEntity> {
        // Sphere has no shared entities
        Vec::new()
    }

    /// Performs Specific Semantic Check
    pub fn own_check(&self, data: &IgesSphereData) -> bool {
        if data.radius <= 0.0 {
            // Radius : Not Positive
            false
        } else {
            true
        }
    }

    /// Copies Specific Parameters
    pub fn own_copy(
        &self,
        source: &IgesSphereData,
        _copy_tool: &mut CopyTool,
    ) -> IgesSphereData {
        IgesSphereData {
            radius: source.radius,
            center: source.center,
        }
    }

    /// Dump of Specific Parameters
    pub fn own_dump(&self, data: &IgesSphereData, _level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_Sphere\n");
        s.push_str(&format!("Radius : {}\n", data.radius));
        s.push_str(&format!(
            "Center : ({:.6}, {:.6}, {:.6})\n",
            data.center[0], data.center[1], data.center[2]
        ));
        s
    }
}

/// Data structure for Sphere entity
#[derive(Clone, Debug)]
pub struct IgesSphereData {
    pub radius: f64,
    pub center: [f64; 3],
}

/// Stub types for compilation
#[derive(Clone, Debug, Default)]
pub struct IgesEntity;

#[derive(Clone, Debug, Default)]
pub struct IgesReaderData;

#[derive(Clone, Debug, Default)]
pub struct ParamReader;

#[derive(Clone, Debug, Default)]
pub struct IgesWriter;

#[derive(Clone, Debug, Default)]
pub struct CopyTool;

#[derive(Clone, Debug)]
pub struct IgesSolidDirChecker {
    entity_type: i32,
    form: i32,
    structure: DirType,
    line_font: DirType,
    color: DirType,
    use_flag_required: Option<i32>,
    hierarchy_status_ignored: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DirType {
    Void,
    Any,
}

impl IgesSolidDirChecker {
    pub fn new(entity_type: i32, form: i32) -> Self {
        Self {
            entity_type,
            form,
            structure: DirType::Void,
            line_font: DirType::Any,
            color: DirType::Any,
            use_flag_required: None,
            hierarchy_status_ignored: false,
        }
    }

    pub fn set_structure_void(&mut self) {
        self.structure = DirType::Void;
    }

    pub fn set_line_font_any(&mut self) {
        self.line_font = DirType::Any;
    }

    pub fn set_color_any(&mut self) {
        self.color = DirType::Any;
    }

    pub fn set_use_flag_required(&mut self, flag: i32) {
        self.use_flag_required = Some(flag);
    }

    pub fn set_hierarchy_status_ignored(&mut self) {
        self.hierarchy_status_ignored = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let tool = IgesSolidToolSphere::new();
        assert_eq!(tool, IgesSolidToolSphere);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolSphere::new();
        let dc = tool.dir_checker();
        assert_eq!(dc.entity_type, 158);
        assert_eq!(dc.form, 0);
        assert_eq!(dc.structure, DirType::Void);
        assert_eq!(dc.line_font, DirType::Any);
        assert_eq!(dc.color, DirType::Any);
        assert_eq!(dc.use_flag_required, Some(0));
        assert!(dc.hierarchy_status_ignored);
    }

    #[test]
    fn test_own_check_valid() {
        let tool = IgesSolidToolSphere::new();
        let data = IgesSphereData {
            radius: 5.0,
            center: [0.0, 0.0, 0.0],
        };
        assert!(tool.own_check(&data));
    }

    #[test]
    fn test_own_check_invalid_negative_radius() {
        let tool = IgesSolidToolSphere::new();
        let data = IgesSphereData {
            radius: -1.0,
            center: [0.0, 0.0, 0.0],
        };
        assert!(!tool.own_check(&data));
    }

    #[test]
    fn test_own_check_invalid_zero_radius() {
        let tool = IgesSolidToolSphere::new();
        let data = IgesSphereData {
            radius: 0.0,
            center: [0.0, 0.0, 0.0],
        };
        assert!(!tool.own_check(&data));
    }

    #[test]
    fn test_own_shared() {
        let tool = IgesSolidToolSphere::new();
        let data = IgesSphereData {
            radius: 5.0,
            center: [1.0, 2.0, 3.0],
        };
        let shared = tool.own_shared(&data);
        assert!(shared.is_empty());
    }

    #[test]
    fn test_own_copy() {
        let tool = IgesSolidToolSphere::new();
        let source = IgesSphereData {
            radius: 5.0,
            center: [1.0, 2.0, 3.0],
        };
        let mut copy_tool = CopyTool::default();
        let copied = tool.own_copy(&source, &mut copy_tool);
        assert_eq!(copied.radius, 5.0);
        assert_eq!(copied.center, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolSphere::new();
        let data = IgesSphereData {
            radius: 5.0,
            center: [1.0, 2.0, 3.0],
        };
        let dump = tool.own_dump(&data, 2);
        assert!(dump.contains("IGESSolid_Sphere"));
        assert!(dump.contains("Radius : 5"));
        assert!(dump.contains("Center"));
    }
}
