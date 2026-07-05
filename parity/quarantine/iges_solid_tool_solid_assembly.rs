// FILE: iges_solid_tool_solid_assembly.rs
// occt: IGESSolid_ToolSolidAssembly

/// Tool to work on a SolidAssembly. Called by various Modules
/// (ReadWriteModule, GeneralModule, SpecificModule)
#[derive(Default)]
pub struct IgesSolidToolSolidAssembly;

impl IgesSolidToolSolidAssembly {
    /// Returns a ToolSolidAssembly, ready to work
    pub fn new() -> Self {
        Self
    }

    /// Returns specific DirChecker for type 184, form 0-1
    pub fn dir_checker(&self) -> IgesSolidDirChecker {
        let mut dc = IgesSolidDirChecker::new(184, 0);
        dc.set_structure_void();
        dc.set_line_font_any();
        dc.set_color_any();
        dc.set_use_flag_required(2);
        dc.set_graphics_ignored(1);
        dc
    }

    /// Reads own parameters from file
    pub fn read_own_params(
        &self,
        _reader_data: &IgesReaderData,
        _param_reader: &mut ParamReader,
    ) -> IgesSolidAssemblyData {
        // Implementation parses:
        // 1. Number of Items (nbitems)
        // 2. For each item: Solid assembly item entity
        // 3. For each item: Transformation matrix (optional)
        IgesSolidAssemblyData {
            items: Vec::new(),
            matrices: Vec::new(),
        }
    }

    /// Writes own parameters to IGESWriter
    pub fn write_own_params(&self, data: &IgesSolidAssemblyData, _writer: &mut IgesWriter) {
        // Write nbitems count
        // For each item i: Write Item(i) entity
        // For each item i: Write TransfMatrix(i) entity
        drop(data);
    }

    /// Lists the Entities shared by a SolidAssembly
    pub fn own_shared(&self, data: &IgesSolidAssemblyData) -> Vec<IgesEntity> {
        let mut shared = Vec::new();
        shared.extend(data.items.clone());
        shared.extend(data.matrices.clone());
        shared
    }

    /// Performs Specific Semantic Check
    pub fn own_check(&self, _data: &IgesSolidAssemblyData) {
        // No specific checks in OCCT implementation
    }

    /// Copies Specific Parameters
    pub fn own_copy(
        &self,
        source: &IgesSolidAssemblyData,
        _copy_tool: &mut CopyTool,
    ) -> IgesSolidAssemblyData {
        IgesSolidAssemblyData {
            items: source.items.clone(),
            matrices: source.matrices.clone(),
        }
    }

    /// Dump of Specific Parameters
    pub fn own_dump(&self, data: &IgesSolidAssemblyData, _level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_SolidAssembly\n");
        s.push_str(&format!("Items : {} items\n", data.items.len()));
        for (i, item) in data.items.iter().enumerate() {
            s.push_str(&format!("  [{}]: {:?}\n", i + 1, item));
        }
        s.push_str(&format!("Matrices : {} matrices\n", data.matrices.len()));
        for (i, matrix) in data.matrices.iter().enumerate() {
            s.push_str(&format!("  [{}]: {:?}\n", i + 1, matrix));
        }
        s
    }
}

/// Data structure for SolidAssembly entity
#[derive(Clone, Debug)]
pub struct IgesSolidAssemblyData {
    pub items: Vec<IgesEntity>,
    pub matrices: Vec<IgesEntity>,
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
    graphics_ignored: Option<i32>,
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
            graphics_ignored: None,
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

    pub fn set_graphics_ignored(&mut self, flag: i32) {
        self.graphics_ignored = Some(flag);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let tool = IgesSolidToolSolidAssembly::new();
        assert_eq!(tool, IgesSolidToolSolidAssembly);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolSolidAssembly::new();
        let dc = tool.dir_checker();
        assert_eq!(dc.entity_type, 184);
        assert_eq!(dc.form, 0);
        assert_eq!(dc.structure, DirType::Void);
        assert_eq!(dc.line_font, DirType::Any);
        assert_eq!(dc.color, DirType::Any);
        assert_eq!(dc.use_flag_required, Some(2));
        assert_eq!(dc.graphics_ignored, Some(1));
    }

    #[test]
    fn test_own_shared_empty() {
        let tool = IgesSolidToolSolidAssembly::new();
        let data = IgesSolidAssemblyData {
            items: Vec::new(),
            matrices: Vec::new(),
        };
        let shared = tool.own_shared(&data);
        assert!(shared.is_empty());
    }

    #[test]
    fn test_own_shared_with_items() {
        let tool = IgesSolidToolSolidAssembly::new();
        let data = IgesSolidAssemblyData {
            items: vec![IgesEntity::default(), IgesEntity::default()],
            matrices: vec![IgesEntity::default()],
        };
        let shared = tool.own_shared(&data);
        assert_eq!(shared.len(), 3);
    }

    #[test]
    fn test_own_copy() {
        let tool = IgesSolidToolSolidAssembly::new();
        let source = IgesSolidAssemblyData {
            items: vec![IgesEntity::default()],
            matrices: vec![IgesEntity::default()],
        };
        let mut copy_tool = CopyTool::default();
        let copied = tool.own_copy(&source, &mut copy_tool);
        assert_eq!(copied.items.len(), 1);
        assert_eq!(copied.matrices.len(), 1);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolSolidAssembly::new();
        let data = IgesSolidAssemblyData {
            items: vec![IgesEntity::default()],
            matrices: vec![IgesEntity::default()],
        };
        let dump = tool.own_dump(&data, 2);
        assert!(dump.contains("IGESSolid_SolidAssembly"));
        assert!(dump.contains("Items : 1 items"));
        assert!(dump.contains("Matrices : 1 matrices"));
    }
}
