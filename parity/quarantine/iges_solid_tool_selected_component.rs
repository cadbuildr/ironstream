// FILE: iges_solid_tool_selected_component.rs
// occt: IGESSolid_ToolSelectedComponent

/// Tool to work on a SelectedComponent. Called by various Modules
/// (ReadWriteModule, GeneralModule, SpecificModule)
#[derive(Default)]
pub struct IgesSolidToolSelectedComponent;

impl IgesSolidToolSelectedComponent {
    /// Returns a ToolSelectedComponent, ready to work
    pub fn new() -> Self {
        Self
    }

    /// Returns specific DirChecker for type 182, form 0
    pub fn dir_checker(&self) -> IgesSolidDirChecker {
        let mut dc = IgesSolidDirChecker::new(182, 0);
        dc.set_structure_void();
        dc.set_line_font_void();
        dc.set_line_weight_void();
        dc.set_color_any();
        dc.set_blank_status_ignored();
        dc.set_use_flag_required(3);
        dc.set_hierarchy_status_ignored();
        dc
    }

    /// Reads own parameters from file
    pub fn read_own_params(
        &self,
        _reader_data: &IgesReaderData,
        _param_reader: &mut ParamReader,
    ) -> IgesSelectedComponentData {
        // Implementation would parse:
        // 1. Boolean Tree Entity
        // 2. Select Point (X, Y, Z coordinates)
        IgesSelectedComponentData::default()
    }

    /// Writes own parameters to IGESWriter
    pub fn write_own_params(&self, data: &IgesSelectedComponentData, _writer: &mut IgesWriter) {
        // Write boolean tree entity reference
        // Write select point coordinates (X, Y, Z)
        drop(data);
    }

    /// Lists the Entities shared by a SelectedComponent
    pub fn own_shared(&self, data: &IgesSelectedComponentData) -> Vec<IgesEntity> {
        vec![data.boolean_tree_entity.clone()]
    }

    /// Performs Specific Semantic Check
    pub fn own_check(&self, _data: &IgesSelectedComponentData) {
        // No specific checks needed for this entity type
    }

    /// Copies Specific Parameters
    pub fn own_copy(
        &self,
        source: &IgesSelectedComponentData,
        _copy_tool: &mut CopyTool,
    ) -> IgesSelectedComponentData {
        IgesSelectedComponentData {
            boolean_tree_entity: source.boolean_tree_entity.clone(),
            select_point: source.select_point,
        }
    }

    /// Dump of Specific Parameters
    pub fn own_dump(&self, data: &IgesSelectedComponentData, level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_SelectedComponent\n");
        s.push_str("Boolean Tree Entity :\n");
        if level <= 4 {
            s.push_str(&format!("  {:?}\n", data.boolean_tree_entity));
        } else {
            s.push_str(&format!("  {:?} (detailed)\n", data.boolean_tree_entity));
        }
        s.push_str(&format!("Selected Point       : ({:.6}, {:.6}, {:.6})\n",
                            data.select_point[0], data.select_point[1], data.select_point[2]));
        s
    }
}

/// Stub types for compilation
#[derive(Clone, Debug)]
pub struct IgesSelectedComponentData {
    pub boolean_tree_entity: IgesEntity,
    pub select_point: [f64; 3],
}

impl Default for IgesSelectedComponentData {
    fn default() -> Self {
        Self {
            boolean_tree_entity: IgesEntity::default(),
            select_point: [0.0, 0.0, 0.0],
        }
    }
}

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
    blank_status_ignored: bool,
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
            line_font: DirType::Void,
            line_weight: DirType::Void,
            color: DirType::Any,
            blank_status_ignored: false,
            use_flag_required: None,
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

    pub fn set_blank_status_ignored(&mut self) {
        self.blank_status_ignored = true;
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
        let tool = IgesSolidToolSelectedComponent::new();
        assert_eq!(tool, IgesSolidToolSelectedComponent);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolSelectedComponent::new();
        let dc = tool.dir_checker();
        assert_eq!(dc.entity_type, 182);
        assert_eq!(dc.form, 0);
        assert_eq!(dc.structure, DirType::Void);
        assert_eq!(dc.line_font, DirType::Void);
        assert_eq!(dc.line_weight, DirType::Void);
        assert_eq!(dc.color, DirType::Any);
        assert!(dc.blank_status_ignored);
        assert_eq!(dc.use_flag_required, Some(3));
        assert!(dc.hierarchy_status_ignored);
    }

    #[test]
    fn test_data_default() {
        let data = IgesSelectedComponentData::default();
        assert_eq!(data.select_point, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_own_shared() {
        let tool = IgesSolidToolSelectedComponent::new();
        let entity = IgesEntity::default();
        let data = IgesSelectedComponentData {
            boolean_tree_entity: entity.clone(),
            select_point: [1.0, 2.0, 3.0],
        };
        let shared = tool.own_shared(&data);
        assert_eq!(shared.len(), 1);
    }

    #[test]
    fn test_own_copy() {
        let tool = IgesSolidToolSelectedComponent::new();
        let source = IgesSelectedComponentData {
            boolean_tree_entity: IgesEntity::default(),
            select_point: [1.5, 2.5, 3.5],
        };
        let mut copy_tool = CopyTool::default();
        let copied = tool.own_copy(&source, &mut copy_tool);
        assert_eq!(copied.select_point, [1.5, 2.5, 3.5]);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolSelectedComponent::new();
        let data = IgesSelectedComponentData {
            boolean_tree_entity: IgesEntity::default(),
            select_point: [1.0, 2.0, 3.0],
        };
        let dump = tool.own_dump(&data, 2);
        assert!(dump.contains("IGESSolid_SelectedComponent"));
        assert!(dump.contains("Boolean Tree Entity"));
        assert!(dump.contains("Selected Point"));
    }
}
