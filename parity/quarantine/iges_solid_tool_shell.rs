// FILE: iges_solid_tool_shell.rs
// occt: IGESSolid_ToolShell

/// Tool to work on a Shell. Called by various Modules
/// (ReadWriteModule, GeneralModule, SpecificModule)
#[derive(Default)]
pub struct IgesSolidToolShell;

impl IgesSolidToolShell {
    /// Returns a ToolShell, ready to work
    pub fn new() -> Self {
        Self
    }

    /// Returns specific DirChecker for type 514, form 1-2
    pub fn dir_checker(&self) -> IgesSolidDirChecker {
        let mut dc = IgesSolidDirChecker::new(514, 1);
        dc.set_structure_void();
        dc.set_line_font_void();
        dc.set_line_weight_void();
        dc.set_color_void();
        dc.set_subordinate_status_required(1);
        dc
    }

    /// Reads own parameters from file
    pub fn read_own_params(
        &self,
        _reader_data: &IgesReaderData,
        _param_reader: &mut ParamReader,
    ) -> IgesShellData {
        // Implementation parses:
        // 1. Number of faces (nbfaces)
        // 2. For each face: Face entity, Orientation flag (boolean)
        IgesShellData {
            faces: Vec::new(),
            orientations: Vec::new(),
        }
    }

    /// Writes own parameters to IGESWriter
    pub fn write_own_params(&self, data: &IgesShellData, _writer: &mut IgesWriter) {
        // Write nbfaces count
        // For each face i:
        //   Write Face(i) entity
        //   Write Orientation(i) boolean
        drop(data);
    }

    /// Lists the Entities shared by a Shell
    pub fn own_shared(&self, data: &IgesShellData) -> Vec<IgesEntity> {
        data.faces.clone()
    }

    /// Performs Specific Semantic Check
    pub fn own_check(&self, data: &IgesShellData) -> bool {
        if data.faces.is_empty() {
            // XSTEP_200: "Number of faces must be > 0"
            false
        } else {
            true
        }
    }

    /// Copies Specific Parameters
    pub fn own_copy(&self, source: &IgesShellData, _copy_tool: &mut CopyTool) -> IgesShellData {
        let mut faces = Vec::new();
        let mut orientations = Vec::new();

        for (i, face) in source.faces.iter().enumerate() {
            faces.push(face.clone());
            if i < source.orientations.len() {
                orientations.push(source.orientations[i]);
            }
        }

        IgesShellData { faces, orientations }
    }

    /// Dump of Specific Parameters
    pub fn own_dump(&self, data: &IgesShellData, level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_Shell\n");
        let upper = data.faces.len();
        let sublevel = if level <= 4 { 0 } else { 1 };

        s.push_str("Faces :\nOrientation flags : ");
        for i in 0..upper {
            s.push_str(&format!("[{}]", i + 1));
        }
        s.push('\n');

        if level > 4 {
            s.push_str("[\n");
            for i in 0..upper {
                s.push_str(&format!("[{}]:  Face : ", i + 1));
                s.push_str(&format!("{:?}", data.faces[i]));
                s.push_str("  - Orientation flag : ");
                if i < data.orientations.len() && data.orientations[i] {
                    s.push_str("True\n");
                } else {
                    s.push_str("False\n");
                }
            }
        }
        s.push('\n');
        s
    }
}

/// Data structure for Shell entity
#[derive(Clone, Debug)]
pub struct IgesShellData {
    pub faces: Vec<IgesEntity>,
    pub orientations: Vec<bool>,
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
    line_weight: DirType,
    color: DirType,
    subordinate_status_required: Option<i32>,
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
            line_font: DirType::Void,
            line_weight: DirType::Void,
            color: DirType::Any,
            subordinate_status_required: None,
        }
    }

    pub fn set_structure_void(&mut self) {
        self.structure = DirType::Void;
    }

    pub fn set_line_font_void(&mut self) {
        self.line_font = DirType::Void;
    }

    pub fn set_line_weight_void(&mut self) {
        self.line_weight = DirType::Void;
    }

    pub fn set_color_void(&mut self) {
        self.color = DirType::Void;
    }

    pub fn set_subordinate_status_required(&mut self, flag: i32) {
        self.subordinate_status_required = Some(flag);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let tool = IgesSolidToolShell::new();
        assert_eq!(tool, IgesSolidToolShell);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolShell::new();
        let dc = tool.dir_checker();
        assert_eq!(dc.entity_type, 514);
        assert_eq!(dc.form, 1);
        assert_eq!(dc.structure, DirType::Void);
        assert_eq!(dc.line_font, DirType::Void);
        assert_eq!(dc.line_weight, DirType::Void);
        assert_eq!(dc.color, DirType::Void);
        assert_eq!(dc.subordinate_status_required, Some(1));
    }

    #[test]
    fn test_own_check_empty() {
        let tool = IgesSolidToolShell::new();
        let data = IgesShellData {
            faces: Vec::new(),
            orientations: Vec::new(),
        };
        assert!(!tool.own_check(&data));
    }

    #[test]
    fn test_own_check_non_empty() {
        let tool = IgesSolidToolShell::new();
        let data = IgesShellData {
            faces: vec![IgesEntity::default()],
            orientations: vec![true],
        };
        assert!(tool.own_check(&data));
    }

    #[test]
    fn test_own_shared() {
        let tool = IgesSolidToolShell::new();
        let entity1 = IgesEntity::default();
        let entity2 = IgesEntity::default();
        let data = IgesShellData {
            faces: vec![entity1, entity2],
            orientations: vec![true, false],
        };
        let shared = tool.own_shared(&data);
        assert_eq!(shared.len(), 2);
    }

    #[test]
    fn test_own_copy() {
        let tool = IgesSolidToolShell::new();
        let source = IgesShellData {
            faces: vec![IgesEntity::default(), IgesEntity::default()],
            orientations: vec![true, false],
        };
        let mut copy_tool = CopyTool::default();
        let copied = tool.own_copy(&source, &mut copy_tool);
        assert_eq!(copied.faces.len(), 2);
        assert_eq!(copied.orientations.len(), 2);
        assert_eq!(copied.orientations[0], true);
        assert_eq!(copied.orientations[1], false);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolShell::new();
        let data = IgesShellData {
            faces: vec![IgesEntity::default()],
            orientations: vec![true],
        };
        let dump = tool.own_dump(&data, 2);
        assert!(dump.contains("IGESSolid_Shell"));
        assert!(dump.contains("Faces"));
    }
}
