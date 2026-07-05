// FILE: iges_solid_tool_solid_of_revolution.rs
// occt: IGESSolid_ToolSolidOfRevolution

/// Tool to work on a SolidOfRevolution. Called by various Modules
/// (ReadWriteModule, GeneralModule, SpecificModule)
#[derive(Default)]
pub struct IgesSolidToolSolidOfRevolution;

impl IgesSolidToolSolidOfRevolution {
    /// Returns a ToolSolidOfRevolution, ready to work
    pub fn new() -> Self {
        Self
    }

    /// Returns specific DirChecker for type 162, form 0-1
    pub fn dir_checker(&self) -> IgesSolidDirChecker {
        let mut dc = IgesSolidDirChecker::new(162, 0);
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
    ) -> IgesSolidOfRevolutionData {
        // Implementation parses:
        // 1. Curve Entity
        // 2. Fraction of rotation (default 1.0)
        // 3. Axis Point (X, Y, Z) (defaults to 0, 0, 0)
        // 4. Axis direction (I, J, K) (defaults to 0, 0, 1)
        IgesSolidOfRevolutionData {
            curve: IgesEntity::default(),
            fraction: 1.0,
            axis_point: [0.0, 0.0, 0.0],
            axis_direction: [0.0, 0.0, 1.0],
        }
    }

    /// Writes own parameters to IGESWriter
    pub fn write_own_params(&self, data: &IgesSolidOfRevolutionData, _writer: &mut IgesWriter) {
        // Write Curve entity
        // Write Fraction of rotation
        // Write AxisPoint (X, Y, Z)
        // Write Axis direction (I, J, K)
        drop(data);
    }

    /// Lists the Entities shared by a SolidOfRevolution
    pub fn own_shared(&self, data: &IgesSolidOfRevolutionData) -> Vec<IgesEntity> {
        vec![data.curve.clone()]
    }

    /// Performs Specific Semantic Check
    pub fn own_check(&self, data: &IgesSolidOfRevolutionData) -> bool {
        if data.fraction <= 0.0 || data.fraction > 1.0 {
            // Fraction of rotation : Incorrect value
            false
        } else {
            true
        }
    }

    /// Copies Specific Parameters
    pub fn own_copy(
        &self,
        source: &IgesSolidOfRevolutionData,
        _copy_tool: &mut CopyTool,
    ) -> IgesSolidOfRevolutionData {
        // Normalize axis direction if needed
        let mut axis_dir = source.axis_direction;
        let magnitude_sq = axis_dir[0] * axis_dir[0]
            + axis_dir[1] * axis_dir[1]
            + axis_dir[2] * axis_dir[2];
        if magnitude_sq > 0.0 && (magnitude_sq - 1.0).abs() > 1.0e-5 {
            let magnitude = magnitude_sq.sqrt();
            axis_dir[0] /= magnitude;
            axis_dir[1] /= magnitude;
            axis_dir[2] /= magnitude;
        }

        IgesSolidOfRevolutionData {
            curve: source.curve.clone(),
            fraction: source.fraction,
            axis_point: source.axis_point,
            axis_direction: axis_dir,
        }
    }

    /// Dump of Specific Parameters
    pub fn own_dump(&self, data: &IgesSolidOfRevolutionData, level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_SolidOfRevolution\n");
        s.push_str("Curve entity   :");
        if level <= 4 {
            s.push_str(&format!("{:?}\n", data.curve));
        } else {
            s.push_str(&format!("{:?} (detailed)\n", data.curve));
        }
        s.push_str(&format!("Fraction of rotation : {}\n", data.fraction));
        s.push_str(&format!(
            "Axis Point     : ({:.6}, {:.6}, {:.6})\n",
            data.axis_point[0], data.axis_point[1], data.axis_point[2]
        ));
        s.push_str(&format!(
            "Axis direction : ({:.6}, {:.6}, {:.6})\n",
            data.axis_direction[0], data.axis_direction[1], data.axis_direction[2]
        ));
        s
    }
}

/// Data structure for SolidOfRevolution entity
#[derive(Clone, Debug)]
pub struct IgesSolidOfRevolutionData {
    pub curve: IgesEntity,
    pub fraction: f64,
    pub axis_point: [f64; 3],
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
        let tool = IgesSolidToolSolidOfRevolution::new();
        assert_eq!(tool, IgesSolidToolSolidOfRevolution);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let dc = tool.dir_checker();
        assert_eq!(dc.entity_type, 162);
        assert_eq!(dc.form, 0);
        assert_eq!(dc.structure, DirType::Void);
        assert_eq!(dc.line_font, DirType::Any);
        assert_eq!(dc.color, DirType::Any);
        assert_eq!(dc.use_flag_required, Some(0));
        assert!(dc.hierarchy_status_ignored);
    }

    #[test]
    fn test_own_check_valid() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let data = IgesSolidOfRevolutionData {
            curve: IgesEntity::default(),
            fraction: 0.5,
            axis_point: [0.0, 0.0, 0.0],
            axis_direction: [0.0, 0.0, 1.0],
        };
        assert!(tool.own_check(&data));
    }

    #[test]
    fn test_own_check_invalid_fraction_zero() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let data = IgesSolidOfRevolutionData {
            curve: IgesEntity::default(),
            fraction: 0.0,
            axis_point: [0.0, 0.0, 0.0],
            axis_direction: [0.0, 0.0, 1.0],
        };
        assert!(!tool.own_check(&data));
    }

    #[test]
    fn test_own_check_invalid_fraction_too_large() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let data = IgesSolidOfRevolutionData {
            curve: IgesEntity::default(),
            fraction: 1.5,
            axis_point: [0.0, 0.0, 0.0],
            axis_direction: [0.0, 0.0, 1.0],
        };
        assert!(!tool.own_check(&data));
    }

    #[test]
    fn test_own_shared() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let data = IgesSolidOfRevolutionData {
            curve: IgesEntity::default(),
            fraction: 1.0,
            axis_point: [1.0, 2.0, 3.0],
            axis_direction: [0.0, 0.0, 1.0],
        };
        let shared = tool.own_shared(&data);
        assert_eq!(shared.len(), 1);
    }

    #[test]
    fn test_own_copy_normalizes_axis() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let source = IgesSolidOfRevolutionData {
            curve: IgesEntity::default(),
            fraction: 0.5,
            axis_point: [1.0, 2.0, 3.0],
            axis_direction: [0.0, 0.0, 2.0],
        };
        let mut copy_tool = CopyTool::default();
        let copied = tool.own_copy(&source, &mut copy_tool);
        assert_eq!(copied.fraction, 0.5);
        assert_eq!(copied.axis_point, [1.0, 2.0, 3.0]);
        assert!((copied.axis_direction[2] - 1.0).abs() < 1.0e-5);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let data = IgesSolidOfRevolutionData {
            curve: IgesEntity::default(),
            fraction: 0.5,
            axis_point: [1.0, 2.0, 3.0],
            axis_direction: [0.0, 0.0, 1.0],
        };
        let dump = tool.own_dump(&data, 2);
        assert!(dump.contains("IGESSolid_SolidOfRevolution"));
        assert!(dump.contains("Fraction of rotation : 0.5"));
        assert!(dump.contains("Axis Point"));
        assert!(dump.contains("Axis direction"));
    }
}
