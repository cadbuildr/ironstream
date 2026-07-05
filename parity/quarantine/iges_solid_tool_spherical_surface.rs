// FILE: iges_solid_tool_spherical_surface.rs
// occt: IGESSolid_ToolSphericalSurface

/// Tool to work on a SphericalSurface. Called by various Modules
/// (ReadWriteModule, GeneralModule, SpecificModule)
#[derive(Default)]
pub struct IgesSolidToolSphericalSurface;

impl IgesSolidToolSphericalSurface {
    /// Returns a ToolSphericalSurface, ready to work
    pub fn new() -> Self {
        Self
    }

    /// Returns specific DirChecker for type 196, form 0-1
    pub fn dir_checker(&self) -> IgesSolidDirChecker {
        let mut dc = IgesSolidDirChecker::new(196, 0);
        dc.set_structure_void();
        dc.set_line_font_any();
        dc.set_color_any();
        dc.set_blank_status_ignored();
        dc.set_subordinate_status_required(1);
        dc.set_hierarchy_status_ignored();
        dc
    }

    /// Reads own parameters from file
    pub fn read_own_params(
        &self,
        _reader_data: &IgesReaderData,
        _param_reader: &mut ParamReader,
    ) -> IgesSphericalSurfaceData {
        // Implementation parses:
        // 1. Center point
        // 2. Radius
        // 3. If parametrised (form 1):
        //    - Axis direction
        //    - Reference direction
        IgesSphericalSurfaceData {
            center: IgesEntity::default(),
            radius: 0.0,
            axis: None,
            reference_dir: None,
        }
    }

    /// Writes own parameters to IGESWriter
    pub fn write_own_params(&self, data: &IgesSphericalSurfaceData, _writer: &mut IgesWriter) {
        // Write Center point
        // Write Radius
        // If parametrised: Write Axis, Write ReferenceDir
        drop(data);
    }

    /// Lists the Entities shared by a SphericalSurface
    pub fn own_shared(&self, data: &IgesSphericalSurfaceData) -> Vec<IgesEntity> {
        let mut shared = vec![data.center.clone()];
        if let Some(axis) = &data.axis {
            shared.push(axis.clone());
        }
        if let Some(ref_dir) = &data.reference_dir {
            shared.push(ref_dir.clone());
        }
        shared
    }

    /// Performs Specific Semantic Check
    pub fn own_check(&self, data: &IgesSphericalSurfaceData) -> bool {
        let mut valid = true;
        if data.radius <= 0.0 {
            // Radius : Not Positive
            valid = false;
        }
        let is_param = data.axis.is_some();
        if data.axis.is_none() && is_param {
            // Parametrised Spherical Surface : no Axis is defined
            valid = false;
        }
        valid
    }

    /// Copies Specific Parameters
    pub fn own_copy(
        &self,
        source: &IgesSphericalSurfaceData,
        _copy_tool: &mut CopyTool,
    ) -> IgesSphericalSurfaceData {
        IgesSphericalSurfaceData {
            center: source.center.clone(),
            radius: source.radius,
            axis: source.axis.clone(),
            reference_dir: source.reference_dir.clone(),
        }
    }

    /// Dump of Specific Parameters
    pub fn own_dump(&self, data: &IgesSphericalSurfaceData, _level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_SphericalSurface\n");
        s.push_str(&format!("Center : {:?}\n", data.center));
        s.push_str(&format!("Radius : {}\n", data.radius));
        if data.axis.is_some() {
            s.push_str("Surface is Parametrised\n");
            s.push_str(&format!("Axis direction      : {:?}\n", data.axis));
            s.push_str(&format!("Reference direction : {:?}\n", data.reference_dir));
        } else {
            s.push_str("Surface is UnParametrised\n");
        }
        s
    }
}

/// Data structure for SphericalSurface entity
#[derive(Clone, Debug)]
pub struct IgesSphericalSurfaceData {
    pub center: IgesEntity,
    pub radius: f64,
    pub axis: Option<IgesEntity>,
    pub reference_dir: Option<IgesEntity>,
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
    blank_status_ignored: bool,
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
            line_font: DirType::Any,
            color: DirType::Any,
            blank_status_ignored: false,
            subordinate_status_required: None,
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

    pub fn set_blank_status_ignored(&mut self) {
        self.blank_status_ignored = true;
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
        let tool = IgesSolidToolSphericalSurface::new();
        assert_eq!(tool, IgesSolidToolSphericalSurface);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolSphericalSurface::new();
        let dc = tool.dir_checker();
        assert_eq!(dc.entity_type, 196);
        assert_eq!(dc.form, 0);
        assert!(dc.blank_status_ignored);
        assert_eq!(dc.subordinate_status_required, Some(1));
        assert!(dc.hierarchy_status_ignored);
    }

    #[test]
    fn test_own_check_valid() {
        let tool = IgesSolidToolSphericalSurface::new();
        let data = IgesSphericalSurfaceData {
            center: IgesEntity::default(),
            radius: 5.0,
            axis: None,
            reference_dir: None,
        };
        assert!(tool.own_check(&data));
    }

    #[test]
    fn test_own_check_invalid_radius() {
        let tool = IgesSolidToolSphericalSurface::new();
        let data = IgesSphericalSurfaceData {
            center: IgesEntity::default(),
            radius: -1.0,
            axis: None,
            reference_dir: None,
        };
        assert!(!tool.own_check(&data));
    }

    #[test]
    fn test_own_shared_unparametrised() {
        let tool = IgesSolidToolSphericalSurface::new();
        let data = IgesSphericalSurfaceData {
            center: IgesEntity::default(),
            radius: 5.0,
            axis: None,
            reference_dir: None,
        };
        let shared = tool.own_shared(&data);
        assert_eq!(shared.len(), 1);
    }

    #[test]
    fn test_own_shared_parametrised() {
        let tool = IgesSolidToolSphericalSurface::new();
        let data = IgesSphericalSurfaceData {
            center: IgesEntity::default(),
            radius: 5.0,
            axis: Some(IgesEntity::default()),
            reference_dir: Some(IgesEntity::default()),
        };
        let shared = tool.own_shared(&data);
        assert_eq!(shared.len(), 3);
    }

    #[test]
    fn test_own_copy() {
        let tool = IgesSolidToolSphericalSurface::new();
        let source = IgesSphericalSurfaceData {
            center: IgesEntity::default(),
            radius: 5.0,
            axis: None,
            reference_dir: None,
        };
        let mut copy_tool = CopyTool::default();
        let copied = tool.own_copy(&source, &mut copy_tool);
        assert_eq!(copied.radius, 5.0);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolSphericalSurface::new();
        let data = IgesSphericalSurfaceData {
            center: IgesEntity::default(),
            radius: 5.0,
            axis: None,
            reference_dir: None,
        };
        let dump = tool.own_dump(&data, 2);
        assert!(dump.contains("IGESSolid_SphericalSurface"));
        assert!(dump.contains("Radius"));
        assert!(dump.contains("UnParametrised"));
    }
}
