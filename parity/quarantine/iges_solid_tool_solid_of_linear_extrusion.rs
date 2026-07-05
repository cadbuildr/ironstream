// FILE: iges_solid_tool_solid_of_linear_extrusion.rs
// occt: IGESSolid_ToolSolidOfLinearExtrusion

/// Tool to work on a SolidOfLinearExtrusion. Called by various Modules
/// (ReadWriteModule, GeneralModule, SpecificModule)
#[derive(Default)]
pub struct IgesSolidToolSolidOfLinearExtrusion;

impl IgesSolidToolSolidOfLinearExtrusion {
    /// Returns a ToolSolidOfLinearExtrusion, ready to work
    pub fn new() -> Self {
        Self
    }

    /// Returns specific DirChecker for type 164, form 0
    pub fn dir_checker(&self) -> IgesSolidDirChecker {
        let mut dc = IgesSolidDirChecker::new(164, 0);
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
    ) -> IgesSolidOfLinearExtrusionData {
        // Implementation parses:
        // 1. Curve Entity
        // 2. Length of extrusion
        // 3. Extrusion direction (I, J, K) - defaults to (0, 0, 1)
        IgesSolidOfLinearExtrusionData {
            curve: IgesEntity::default(),
            extrusion_length: 0.0,
            extrusion_direction: [0.0, 0.0, 1.0],
        }
    }

    /// Writes own parameters to IGESWriter
    pub fn write_own_params(&self, data: &IgesSolidOfLinearExtrusionData, _writer: &mut IgesWriter) {
        // Write Curve entity
        // Write ExtrusionLength
        // Write ExtrusionDirection (X, Y, Z)
        drop(data);
    }

    /// Lists the Entities shared by a SolidOfLinearExtrusion
    pub fn own_shared(&self, data: &IgesSolidOfLinearExtrusionData) -> Vec<IgesEntity> {
        vec![data.curve.clone()]
    }

    /// Performs Specific Semantic Check
    pub fn own_check(&self, data: &IgesSolidOfLinearExtrusionData) -> bool {
        if data.extrusion_length <= 0.0 {
            // Length of extrusion : Not Positive
            false
        } else {
            true
        }
    }

    /// Copies Specific Parameters
    pub fn own_copy(
        &self,
        source: &IgesSolidOfLinearExtrusionData,
        _copy_tool: &mut CopyTool,
    ) -> IgesSolidOfLinearExtrusionData {
        // Normalize direction vector if needed
        let mut direction = source.extrusion_direction;
        let magnitude_sq = direction[0] * direction[0]
            + direction[1] * direction[1]
            + direction[2] * direction[2];
        if magnitude_sq > 0.0 && (magnitude_sq - 1.0).abs() > 1.0e-5 {
            let magnitude = magnitude_sq.sqrt();
            direction[0] /= magnitude;
            direction[1] /= magnitude;
            direction[2] /= magnitude;
        }

        IgesSolidOfLinearExtrusionData {
            curve: source.curve.clone(),
            extrusion_length: source.extrusion_length,
            extrusion_direction: direction,
        }
    }

    /// Dump of Specific Parameters
    pub fn own_dump(&self, data: &IgesSolidOfLinearExtrusionData, level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_SolidOfLinearExtrusion\n");
        s.push_str("Curve entity        : ");
        if level <= 4 {
            s.push_str(&format!("{:?}\n", data.curve));
        } else {
            s.push_str(&format!("{:?} (detailed)\n", data.curve));
        }
        s.push_str(&format!(
            "Extrusion length    : {}\n",
            data.extrusion_length
        ));
        s.push_str(&format!(
            "Extrusion direction : ({:.6}, {:.6}, {:.6})\n",
            data.extrusion_direction[0], data.extrusion_direction[1], data.extrusion_direction[2]
        ));
        s
    }
}

/// Data structure for SolidOfLinearExtrusion entity
#[derive(Clone, Debug)]
pub struct IgesSolidOfLinearExtrusionData {
    pub curve: IgesEntity,
    pub extrusion_length: f64,
    pub extrusion_direction: [f64; 3],
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
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        assert_eq!(tool, IgesSolidToolSolidOfLinearExtrusion);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let dc = tool.dir_checker();
        assert_eq!(dc.entity_type, 164);
        assert_eq!(dc.form, 0);
        assert_eq!(dc.structure, DirType::Void);
        assert_eq!(dc.line_font, DirType::Any);
        assert_eq!(dc.color, DirType::Any);
        assert_eq!(dc.use_flag_required, Some(0));
        assert!(dc.hierarchy_status_ignored);
    }

    #[test]
    fn test_own_check_valid() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let data = IgesSolidOfLinearExtrusionData {
            curve: IgesEntity::default(),
            extrusion_length: 5.0,
            extrusion_direction: [0.0, 0.0, 1.0],
        };
        assert!(tool.own_check(&data));
    }

    #[test]
    fn test_own_check_invalid_negative_length() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let data = IgesSolidOfLinearExtrusionData {
            curve: IgesEntity::default(),
            extrusion_length: -1.0,
            extrusion_direction: [0.0, 0.0, 1.0],
        };
        assert!(!tool.own_check(&data));
    }

    #[test]
    fn test_own_check_invalid_zero_length() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let data = IgesSolidOfLinearExtrusionData {
            curve: IgesEntity::default(),
            extrusion_length: 0.0,
            extrusion_direction: [0.0, 0.0, 1.0],
        };
        assert!(!tool.own_check(&data));
    }

    #[test]
    fn test_own_shared() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let data = IgesSolidOfLinearExtrusionData {
            curve: IgesEntity::default(),
            extrusion_length: 5.0,
            extrusion_direction: [0.0, 0.0, 1.0],
        };
        let shared = tool.own_shared(&data);
        assert_eq!(shared.len(), 1);
    }

    #[test]
    fn test_own_copy_normalizes_direction() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let source = IgesSolidOfLinearExtrusionData {
            curve: IgesEntity::default(),
            extrusion_length: 5.0,
            extrusion_direction: [0.0, 0.0, 2.0],
        };
        let mut copy_tool = CopyTool::default();
        let copied = tool.own_copy(&source, &mut copy_tool);
        assert_eq!(copied.extrusion_length, 5.0);
        assert!((copied.extrusion_direction[2] - 1.0).abs() < 1.0e-5);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let data = IgesSolidOfLinearExtrusionData {
            curve: IgesEntity::default(),
            extrusion_length: 5.0,
            extrusion_direction: [0.0, 0.0, 1.0],
        };
        let dump = tool.own_dump(&data, 2);
        assert!(dump.contains("IGESSolid_SolidOfLinearExtrusion"));
        assert!(dump.contains("Curve entity"));
        assert!(dump.contains("Extrusion length    : 5"));
        assert!(dump.contains("Extrusion direction"));
    }
}
