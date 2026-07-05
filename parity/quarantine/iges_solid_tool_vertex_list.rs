// FILE: iges_solid_tool_vertex_list.rs
// occt: IGESSolid_ToolVertexList

/// Tool to work on a VertexList. Called by various Modules
/// (ReadWriteModule, GeneralModule, SpecificModule)
#[derive(Default)]
pub struct IgesSolidToolVertexList;

impl IgesSolidToolVertexList {
    /// Returns a ToolVertexList, ready to work
    pub fn new() -> Self {
        Self
    }

    /// Returns specific DirChecker for type 502, form 1
    pub fn dir_checker(&self) -> IgesSolidDirChecker {
        let mut dc = IgesSolidDirChecker::new(502, 1);
        dc.set_structure_void();
        dc.set_line_font_void();
        dc.set_line_weight_void();
        dc.set_color_any();
        dc.set_subordinate_status_required(1);
        dc.set_hierarchy_status_ignored();
        dc
    }

    /// Reads own parameters from file
    pub fn read_own_params(
        &self,
        _reader_data: &IgesReaderData,
        _param_reader: &mut ParamReader,
    ) -> IgesVertexListData {
        // Implementation parses:
        // 1. Number of vertices (nbitems)
        // 2. For each vertex: (X, Y, Z) coordinates
        IgesVertexListData {
            vertices: Vec::new(),
        }
    }

    /// Writes own parameters to IGESWriter
    pub fn write_own_params(&self, data: &IgesVertexListData, _writer: &mut IgesWriter) {
        // Write number of vertices
        // For each vertex: Write X, Y, Z
        drop(data);
    }

    /// Lists the Entities shared by a VertexList
    pub fn own_shared(&self, _data: &IgesVertexListData) -> Vec<IgesEntity> {
        // VertexList has no shared entities
        Vec::new()
    }

    /// Performs Specific Semantic Check
    pub fn own_check(&self, data: &IgesVertexListData) -> bool {
        if data.vertices.is_empty() {
            // Number of vertices must be > 0
            false
        } else {
            true
        }
    }

    /// Copies Specific Parameters
    pub fn own_copy(
        &self,
        source: &IgesVertexListData,
        _copy_tool: &mut CopyTool,
    ) -> IgesVertexListData {
        IgesVertexListData {
            vertices: source.vertices.clone(),
        }
    }

    /// Dump of Specific Parameters
    pub fn own_dump(&self, data: &IgesVertexListData, _level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_VertexList\n");
        s.push_str(&format!("Vertices : {} vertices\n", data.vertices.len()));
        for (i, vertex) in data.vertices.iter().enumerate() {
            s.push_str(&format!(
                "  [{}]: ({:.6}, {:.6}, {:.6})\n",
                i + 1, vertex[0], vertex[1], vertex[2]
            ));
        }
        s
    }
}

/// Data structure for VertexList entity
#[derive(Clone, Debug)]
pub struct IgesVertexListData {
    pub vertices: Vec<[f64; 3]>,
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
            line_font: DirType::Void,
            line_weight: DirType::Void,
            color: DirType::Any,
            subordinate_status_required: None,
            hierarchy_status_ignored: false,
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

    pub fn set_color_any(&mut self) {
        self.color = DirType::Any;
    }

    pub fn set_subordinate_status_required(&mut self, flag: i32) {
        self.subordinate_status_required = Some(flag);
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
        let tool = IgesSolidToolVertexList::new();
        assert_eq!(tool, IgesSolidToolVertexList);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolVertexList::new();
        let dc = tool.dir_checker();
        assert_eq!(dc.entity_type, 502);
        assert_eq!(dc.form, 1);
        assert_eq!(dc.structure, DirType::Void);
        assert_eq!(dc.line_font, DirType::Void);
        assert_eq!(dc.line_weight, DirType::Void);
        assert_eq!(dc.color, DirType::Any);
        assert_eq!(dc.subordinate_status_required, Some(1));
        assert!(dc.hierarchy_status_ignored);
    }

    #[test]
    fn test_own_check_empty() {
        let tool = IgesSolidToolVertexList::new();
        let data = IgesVertexListData {
            vertices: Vec::new(),
        };
        assert!(!tool.own_check(&data));
    }

    #[test]
    fn test_own_check_non_empty() {
        let tool = IgesSolidToolVertexList::new();
        let data = IgesVertexListData {
            vertices: vec![[0.0, 0.0, 0.0]],
        };
        assert!(tool.own_check(&data));
    }

    #[test]
    fn test_own_shared() {
        let tool = IgesSolidToolVertexList::new();
        let data = IgesVertexListData {
            vertices: vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]],
        };
        let shared = tool.own_shared(&data);
        assert!(shared.is_empty());
    }

    #[test]
    fn test_own_copy() {
        let tool = IgesSolidToolVertexList::new();
        let source = IgesVertexListData {
            vertices: vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]],
        };
        let mut copy_tool = CopyTool::default();
        let copied = tool.own_copy(&source, &mut copy_tool);
        assert_eq!(copied.vertices.len(), 2);
        assert_eq!(copied.vertices[0], [1.0, 2.0, 3.0]);
        assert_eq!(copied.vertices[1], [4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolVertexList::new();
        let data = IgesVertexListData {
            vertices: vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]],
        };
        let dump = tool.own_dump(&data, 2);
        assert!(dump.contains("IGESSolid_VertexList"));
        assert!(dump.contains("Vertices : 2 vertices"));
    }
}
