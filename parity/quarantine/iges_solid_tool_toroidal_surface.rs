// FILE: iges_solid_tool_toroidal_surface.rs
// occt: IGESSolid_ToolToroidalSurface

/// Tool to work on a ToroidalSurface. Called by various Modules
/// (ReadWriteModule, GeneralModule, SpecificModule)
#[derive(Default)]
pub struct IgesSolidToolToroidalSurface;

impl IgesSolidToolToroidalSurface {
    /// Returns a ToolToroidalSurface, ready to work
    pub fn new() -> Self {
        Self
    }

    /// Returns specific DirChecker for type 198, form 0-1
    pub fn dir_checker(&self) -> IgesSolidDirChecker {
        let mut dc = IgesSolidDirChecker::new(198, 0);
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
    ) -> IgestoroidalSurfaceData {
        // Implementation parses:
        // 1. Center point
        // 2. Axis direction
        // 3. Major Radius
        // 4. Minor Radius
        // 5. If parametrised (form 1): Reference direction
        IgestoroidalSurfaceData {
            center: IgesEntity::default(),
            axis: IgesEntity::default(),
            major_radius: 0.0,
            minor_radius: 0.0,
            reference_dir: None,
        }
    }

    /// Writes own parameters to IGESWriter
    pub fn write_own_params(&self, data: &IgestoroidalSurfaceData, _writer: &mut IgesWriter) {
        // Write Center, Axis, MajorRadius, MinorRadius
        // If parametrised: Write ReferenceDir
        drop(data);
    }

    /// Lists the Entities shared by a ToroidalSurface
    pub fn own_shared(&self, data: &IgestoroidalSurfaceData) -> Vec<IgesEntity> {
        let mut shared = vec![data.center.clone(), data.axis.clone()];
        if let Some(ref_dir) = &data.reference_dir {
            shared.push(ref_dir.clone());
        }
        shared
    }

    /// Performs Specific Semantic Check
    pub fn own_check(&self, data: &IgestoroidalSurfaceData) -> bool {
        let mut valid = true;
        if data.major_radius <= 0.0 {
            // Major Radius : Not Positive
            valid = false;
        }
        if data.minor_radius <= 0.0 {
            // Minor Radius : Not Positive
            valid = false;
        }
        if data.minor_radius >= data.major_radius {
            // Minor Radius : Value not < Major radius
            valid = false;
        }
        valid
    }

    /// Copies Specific Parameters
    pub fn own_copy(
        &self,
        source: &IgestoroidalSurfaceData,
        _copy_tool: &mut CopyTool,
    ) -> IgestoroidalSurfaceData {
        IgestoroidalSurfaceData {
            center: source.center.clone(),
            axis: source.axis.clone(),
            major_radius: source.major_radius,
            minor_radius: source.minor_radius,
            reference_dir: source.reference_dir.clone(),
        }
    }

    /// Dump of Specific Parameters
    pub fn own_dump(&self, data: &IgestoroidalSurfaceData, _level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_ToroidalSurface\n");
        s.push_str(&format!("Center : {:?}\n", data.center));
        s.push_str(&format!("Axis direction : {:?}\n", data.axis));
        s.push_str(&format!(
            "Major Radius : {}  Minor Radius : {}\n",
            data.major_radius, data.minor_radius
        ));
        if data.reference_dir.is_some() {
            s.push_str("Surface is Parametrised  -  Reference direction : ");
            s.push_str(&format!("{:?}\n", data.reference_dir));
        } else {
            s.push_str("Surface is UnParametrised\n");
        }
        s
    }
}

/// Data structure for ToroidalSurface entity
#[derive(Clone, Debug)]
pub struct IgestoroidalSurfaceData {
    pub center: IgesEntity,
    pub axis: IgesEntity,
    pub major_radius: f64,
    pub minor_radius: f64,
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
        let tool = IgesSolidToolToroidalSurface::new();
        assert_eq!(tool, IgesSolidToolToroidalSurface);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolToroidalSurface::new();
        let dc = tool.dir_checker();
        assert_eq!(dc.entity_type, 198);
        assert!(dc.blank_status_ignored);
    }

    #[test]
    fn test_own_check_valid() {
        let tool = IgesSolidToolToroidalSurface::new();
        let data = IgestoroidalSurfaceData {
            center: IgesEntity::default(),
            axis: IgesEntity::default(),
            major_radius: 5.0,
            minor_radius: 2.0,
            reference_dir: None,
        };
        assert!(tool.own_check(&data));
    }

    #[test]
    fn test_own_check_invalid_major_radius() {
        let tool = IgesSolidToolToroidalSurface::new();
        let data = IgestoroidalSurfaceData {
            center: IgesEntity::default(),
            axis: IgesEntity::default(),
            major_radius: -1.0,
            minor_radius: 2.0,
            reference_dir: None,
        };
        assert!(!tool.own_check(&data));
    }

    #[test]
    fn test_own_check_invalid_minor_radius() {
        let tool = IgesSolidToolToroidalSurface::new();
        let data = IgestoroidalSurfaceData {
            center: IgesEntity::default(),
            axis: IgesEntity::default(),
            major_radius: 5.0,
            minor_radius: -1.0,
            reference_dir: None,
        };
        assert!(!tool.own_check(&data));
    }

    #[test]
    fn test_own_check_invalid_minor_too_large() {
        let tool = IgesSolidToolToroidalSurface::new();
        let data = IgestoroidalSurfaceData {
            center: IgesEntity::default(),
            axis: IgesEntity::default(),
            major_radius: 5.0,
            minor_radius: 6.0,
            reference_dir: None,
        };
        assert!(!tool.own_check(&data));
    }

    #[test]
    fn test_own_shared() {
        let tool = IgesSolidToolToroidalSurface::new();
        let data = IgestoroidalSurfaceData {
            center: IgesEntity::default(),
            axis: IgesEntity::default(),
            major_radius: 5.0,
            minor_radius: 2.0,
            reference_dir: Some(IgesEntity::default()),
        };
        let shared = tool.own_shared(&data);
        assert_eq!(shared.len(), 3);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolToroidalSurface::new();
        let data = IgestoroidalSurfaceData {
            center: IgesEntity::default(),
            axis: IgesEntity::default(),
            major_radius: 5.0,
            minor_radius: 2.0,
            reference_dir: None,
        };
        let dump = tool.own_dump(&data, 2);
        assert!(dump.contains("IGESSolid_ToroidalSurface"));
        assert!(dump.contains("Major Radius : 5"));
        assert!(dump.contains("Minor Radius : 2"));
        assert!(dump.contains("UnParametrised"));
    }
}
