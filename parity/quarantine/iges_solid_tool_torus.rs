// FILE: iges_solid_tool_torus.rs
// occt: IGESSolid_ToolTorus

/// Tool to work on a Torus. Called by various Modules
/// (ReadWriteModule, GeneralModule, SpecificModule)
#[derive(Default)]
pub struct IgesSolidToolTorus;

impl IgesSolidToolTorus {
    /// Returns a ToolTorus, ready to work
    pub fn new() -> Self {
        Self
    }

    /// Returns specific DirChecker for type 160, form 0
    pub fn dir_checker(&self) -> IgesSolidDirChecker {
        let mut dc = IgesSolidDirChecker::new(160, 0);
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
    ) -> IgestorusData {
        // Implementation parses:
        // 1. Radius of revolution (major radius)
        // 2. Radius of disc (minor radius)
        // 3. Center Point (X, Y, Z) (defaults to 0, 0, 0)
        // 4. Axis direction (I, J, K) (defaults to 0, 0, 1)
        IgestorusData {
            major_radius: 0.0,
            disc_radius: 0.0,
            center_point: [0.0, 0.0, 0.0],
            axis_direction: [0.0, 0.0, 1.0],
        }
    }

    /// Writes own parameters to IGESWriter
    pub fn write_own_params(&self, data: &IgestorusData, _writer: &mut IgesWriter) {
        // Write MajorRadius (Radius of revolution)
        // Write DiscRadius (Radius of disc)
        // Write AxisPoint (X, Y, Z)
        // Write Axis (I, J, K)
        drop(data);
    }

    /// Lists the Entities shared by a Torus
    pub fn own_shared(&self, _data: &IgestorusData) -> Vec<IgesEntity> {
        // Torus has no shared entities
        Vec::new()
    }

    /// Performs Specific Semantic Check
    pub fn own_check(&self, data: &IgestorusData) -> bool {
        let mut valid = true;
        if data.major_radius <= 0.0 {
            // Radius of revolution : Not Positive
            valid = false;
        }
        if data.disc_radius <= 0.0 {
            // Radius of disc : Not Positive
            valid = false;
        }
        if data.disc_radius >= data.major_radius {
            // Radius of disc : is not Less than Radius of revolution
            valid = false;
        }
        valid
    }

    /// Copies Specific Parameters
    pub fn own_copy(
        &self,
        source: &IgestorusData,
        _copy_tool: &mut CopyTool,
    ) -> IgestorusData {
        // Normalize axis direction if needed
        let mut axis_dir = source.axis_direction;
        let magnitude_sq =
            axis_dir[0] * axis_dir[0] + axis_dir[1] * axis_dir[1] + axis_dir[2] * axis_dir[2];
        if magnitude_sq > 0.0 && (magnitude_sq - 1.0).abs() > 1.0e-5 {
            let magnitude = magnitude_sq.sqrt();
            axis_dir[0] /= magnitude;
            axis_dir[1] /= magnitude;
            axis_dir[2] /= magnitude;
        }

        IgestorusData {
            major_radius: source.major_radius,
            disc_radius: source.disc_radius,
            center_point: source.center_point,
            axis_direction: axis_dir,
        }
    }

    /// Dump of Specific Parameters
    pub fn own_dump(&self, data: &IgestorusData, _level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_Torus\n");
        s.push_str(&format!(
            "Radius of revolution : {}  Radius of the disc   : {}\n",
            data.major_radius, data.disc_radius
        ));
        s.push_str(&format!(
            "Center Point   : ({:.6}, {:.6}, {:.6})\n",
            data.center_point[0], data.center_point[1], data.center_point[2]
        ));
        s.push_str(&format!(
            "Axis direction : ({:.6}, {:.6}, {:.6})\n",
            data.axis_direction[0], data.axis_direction[1], data.axis_direction[2]
        ));
        s
    }
}

/// Data structure for Torus entity
#[derive(Clone, Debug)]
pub struct IgestorusData {
    pub major_radius: f64,
    pub disc_radius: f64,
    pub center_point: [f64; 3],
    pub axis_direction: [f64; 3],
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
        let tool = IgesSolidToolTorus::new();
        assert_eq!(tool, IgesSolidToolTorus);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolTorus::new();
        let dc = tool.dir_checker();
        assert_eq!(dc.entity_type, 160);
        assert_eq!(dc.form, 0);
        assert_eq!(dc.structure, DirType::Void);
        assert_eq!(dc.line_font, DirType::Any);
        assert_eq!(dc.color, DirType::Any);
        assert_eq!(dc.use_flag_required, Some(0));
        assert!(dc.hierarchy_status_ignored);
    }

    #[test]
    fn test_own_check_valid() {
        let tool = IgesSolidToolTorus::new();
        let data = IgestorusData {
            major_radius: 5.0,
            disc_radius: 2.0,
            center_point: [0.0, 0.0, 0.0],
            axis_direction: [0.0, 0.0, 1.0],
        };
        assert!(tool.own_check(&data));
    }

    #[test]
    fn test_own_check_invalid_major_negative() {
        let tool = IgesSolidToolTorus::new();
        let data = IgestorusData {
            major_radius: -1.0,
            disc_radius: 2.0,
            center_point: [0.0, 0.0, 0.0],
            axis_direction: [0.0, 0.0, 1.0],
        };
        assert!(!tool.own_check(&data));
    }

    #[test]
    fn test_own_check_invalid_disc_negative() {
        let tool = IgesSolidToolTorus::new();
        let data = IgestorusData {
            major_radius: 5.0,
            disc_radius: -1.0,
            center_point: [0.0, 0.0, 0.0],
            axis_direction: [0.0, 0.0, 1.0],
        };
        assert!(!tool.own_check(&data));
    }

    #[test]
    fn test_own_check_invalid_disc_too_large() {
        let tool = IgesSolidToolTorus::new();
        let data = IgestorusData {
            major_radius: 5.0,
            disc_radius: 6.0,
            center_point: [0.0, 0.0, 0.0],
            axis_direction: [0.0, 0.0, 1.0],
        };
        assert!(!tool.own_check(&data));
    }

    #[test]
    fn test_own_shared() {
        let tool = IgesSolidToolTorus::new();
        let data = IgestorusData {
            major_radius: 5.0,
            disc_radius: 2.0,
            center_point: [1.0, 2.0, 3.0],
            axis_direction: [0.0, 0.0, 1.0],
        };
        let shared = tool.own_shared(&data);
        assert!(shared.is_empty());
    }

    #[test]
    fn test_own_copy_normalizes_axis() {
        let tool = IgesSolidToolTorus::new();
        let source = IgestorusData {
            major_radius: 5.0,
            disc_radius: 2.0,
            center_point: [1.0, 2.0, 3.0],
            axis_direction: [0.0, 0.0, 2.0],
        };
        let mut copy_tool = CopyTool::default();
        let copied = tool.own_copy(&source, &mut copy_tool);
        assert_eq!(copied.major_radius, 5.0);
        assert_eq!(copied.disc_radius, 2.0);
        assert!((copied.axis_direction[2] - 1.0).abs() < 1.0e-5);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolTorus::new();
        let data = IgestorusData {
            major_radius: 5.0,
            disc_radius: 2.0,
            center_point: [1.0, 2.0, 3.0],
            axis_direction: [0.0, 0.0, 1.0],
        };
        let dump = tool.own_dump(&data, 2);
        assert!(dump.contains("IGESSolid_Torus"));
        assert!(dump.contains("Radius of revolution : 5"));
        assert!(dump.contains("Radius of the disc   : 2"));
        assert!(dump.contains("Center Point"));
        assert!(dump.contains("Axis direction"));
    }
}
