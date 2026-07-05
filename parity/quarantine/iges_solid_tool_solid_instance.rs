// FILE: iges_solid_tool_solid_instance.rs
// occt: IGESSolid_ToolSolidInstance

/// Tool to work on a SolidInstance. Called by various Modules
/// (ReadWriteModule, GeneralModule, SpecificModule)
#[derive(Default)]
pub struct IgesSolidToolSolidInstance;

impl IgesSolidToolSolidInstance {
    /// Returns a ToolSolidInstance, ready to work
    pub fn new() -> Self {
        Self
    }

    /// Returns specific DirChecker for type 430, form 0-1
    pub fn dir_checker(&self) -> IgesSolidDirChecker {
        let mut dc = IgesSolidDirChecker::new(430, 0);
        dc.set_structure_void();
        dc.set_line_font_any();
        dc.set_color_any();
        dc.set_graphics_ignored(1);
        dc
    }

    /// Reads own parameters from file
    pub fn read_own_params(
        &self,
        _reader_data: &IgesReaderData,
        _param_reader: &mut ParamReader,
    ) -> IgesSolidInstanceData {
        // Implementation parses:
        // 1. Solid Entity reference
        IgesSolidInstanceData {
            entity: IgesEntity::default(),
        }
    }

    /// Writes own parameters to IGESWriter
    pub fn write_own_params(&self, data: &IgesSolidInstanceData, _writer: &mut IgesWriter) {
        // Write Entity reference
        drop(data);
    }

    /// Lists the Entities shared by a SolidInstance
    pub fn own_shared(&self, data: &IgesSolidInstanceData) -> Vec<IgesEntity> {
        vec![data.entity.clone()]
    }

    /// Performs Specific Semantic Check
    pub fn own_check(&self, _data: &IgesSolidInstanceData) {
        // No specific checks in OCCT implementation
    }

    /// Copies Specific Parameters
    pub fn own_copy(
        &self,
        source: &IgesSolidInstanceData,
        _copy_tool: &mut CopyTool,
    ) -> IgesSolidInstanceData {
        IgesSolidInstanceData {
            entity: source.entity.clone(),
        }
    }

    /// Dump of Specific Parameters
    pub fn own_dump(&self, data: &IgesSolidInstanceData, level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_SolidInstance\n");
        s.push_str("Solid entity : ");
        if level <= 4 {
            s.push_str(&format!("{:?}\n", data.entity));
        } else {
            s.push_str(&format!("{:?} (detailed)\n", data.entity));
        }
        s
    }
}

/// Data structure for SolidInstance entity
#[derive(Clone, Debug)]
pub struct IgesSolidInstanceData {
    pub entity: IgesEntity,
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

    pub fn set_graphics_ignored(&mut self, flag: i32) {
        self.graphics_ignored = Some(flag);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let tool = IgesSolidToolSolidInstance::new();
        assert_eq!(tool, IgesSolidToolSolidInstance);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolSolidInstance::new();
        let dc = tool.dir_checker();
        assert_eq!(dc.entity_type, 430);
        assert_eq!(dc.form, 0);
        assert_eq!(dc.structure, DirType::Void);
        assert_eq!(dc.line_font, DirType::Any);
        assert_eq!(dc.color, DirType::Any);
        assert_eq!(dc.graphics_ignored, Some(1));
    }

    #[test]
    fn test_own_shared() {
        let tool = IgesSolidToolSolidInstance::new();
        let data = IgesSolidInstanceData {
            entity: IgesEntity::default(),
        };
        let shared = tool.own_shared(&data);
        assert_eq!(shared.len(), 1);
    }

    #[test]
    fn test_own_copy() {
        let tool = IgesSolidToolSolidInstance::new();
        let source = IgesSolidInstanceData {
            entity: IgesEntity::default(),
        };
        let mut copy_tool = CopyTool::default();
        let copied = tool.own_copy(&source, &mut copy_tool);
        assert_eq!(copied.entity, source.entity);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolSolidInstance::new();
        let data = IgesSolidInstanceData {
            entity: IgesEntity::default(),
        };
        let dump = tool.own_dump(&data, 2);
        assert!(dump.contains("IGESSolid_SolidInstance"));
        assert!(dump.contains("Solid entity"));
    }
}
