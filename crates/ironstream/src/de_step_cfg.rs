// FILE: de_step_cfg.rs
// occt: DE_StepConfiguration, DE_IgesConfiguration, DE_ConfigurationNode,
//       DE_ShapeFixParameters

/// STEP schema variant for reading/writing.
/// occt: STEPControl_SchemaName
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepSchema {
    Ap203,
    Ap214,
    Ap242,
    Auto,
}

impl Default for StepSchema {
    fn default() -> Self { Self::Ap242 }
}

/// STEP write transfer mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepWriteMode {
    AsIs,
    FacetedBrep,
    ShellBasedSurfaceModel,
    ManifoldSolidBrep,
    GeometricallyBoundedSurfaceModel,
    GeometricallyBoundedWireframeModel,
}

impl Default for StepWriteMode {
    fn default() -> Self { Self::AsIs }
}

/// IGES entity type filter for reading.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IgesReadMode {
    AllEntities,
    OnlyVisible,
    OnlyTopLevel,
}

impl Default for IgesReadMode {
    fn default() -> Self { Self::OnlyVisible }
}

/// STEP read/write configuration.
/// occt: DE_StepConfiguration / DE_ConfigurationNode
#[derive(Clone, Debug)]
pub struct DeStepConfig {
    pub schema: StepSchema,
    pub write_mode: StepWriteMode,
    pub read_same_param: bool,
    pub read_surface_curve: bool,
    pub max_tolerance: f64,
    pub unit_name: String,
    pub unit_factor: f64,     // scale factor to mm
    pub write_assembly: bool,
    pub write_pcurves: bool,
    pub write_pmt_info: bool,
    pub is_active: bool,
}

impl Default for DeStepConfig {
    fn default() -> Self {
        Self {
            schema: StepSchema::Ap242,
            write_mode: StepWriteMode::AsIs,
            read_same_param: true,
            read_surface_curve: true,
            max_tolerance: 1e-3,
            unit_name: String::from("MM"),
            unit_factor: 1.0,
            write_assembly: true,
            write_pcurves: true,
            write_pmt_info: false,
            is_active: true,
        }
    }
}

impl DeStepConfig {
    pub fn new() -> Self { Self::default() }

    pub fn set_schema(&mut self, s: StepSchema) { self.schema = s; }
    pub fn set_write_mode(&mut self, m: StepWriteMode) { self.write_mode = m; }
    pub fn set_max_tolerance(&mut self, t: f64) { self.max_tolerance = t.max(1e-10); }
    pub fn set_unit(&mut self, name: &str, factor: f64) {
        self.unit_name = name.to_string();
        self.unit_factor = factor;
    }
    pub fn set_write_pcurves(&mut self, v: bool) { self.write_pcurves = v; }
    pub fn set_write_assembly(&mut self, v: bool) { self.write_assembly = v; }
    pub fn set_active(&mut self, v: bool) { self.is_active = v; }

    pub fn schema(&self) -> StepSchema { self.schema }
    pub fn write_mode(&self) -> StepWriteMode { self.write_mode }
    pub fn is_active(&self) -> bool { self.is_active }
    pub fn max_tolerance(&self) -> f64 { self.max_tolerance }
}

/// IGES read/write configuration.
/// occt: DE_IgesConfiguration
#[derive(Clone, Debug)]
pub struct DeIgesConfig {
    pub read_mode: IgesReadMode,
    pub read_bspline_continuity: u32,  // 0,1,2
    pub write_brep_mode: bool,         // true=write as brep, false=as iges surfaces
    pub write_plane_mode: bool,
    pub author: String,
    pub organization: String,
    pub max_tolerance: f64,
    pub is_active: bool,
}

impl Default for DeIgesConfig {
    fn default() -> Self {
        Self {
            read_mode: IgesReadMode::OnlyVisible,
            read_bspline_continuity: 1,
            write_brep_mode: false,
            write_plane_mode: true,
            author: String::new(),
            organization: String::new(),
            max_tolerance: 1e-3,
            is_active: true,
        }
    }
}

impl DeIgesConfig {
    pub fn new() -> Self { Self::default() }

    pub fn set_read_mode(&mut self, m: IgesReadMode) { self.read_mode = m; }
    pub fn set_bspline_continuity(&mut self, c: u32) { self.read_bspline_continuity = c.min(2); }
    pub fn set_author(&mut self, a: &str) { self.author = a.to_string(); }
    pub fn set_write_brep_mode(&mut self, v: bool) { self.write_brep_mode = v; }
    pub fn set_active(&mut self, v: bool) { self.is_active = v; }

    pub fn read_mode(&self) -> IgesReadMode { self.read_mode }
    pub fn is_active(&self) -> bool { self.is_active }
}

/// Shape fix parameters for import cleanup.
/// occt: DE_ShapeFixParameters
#[derive(Clone, Debug)]
pub struct DeShapeFixParams {
    pub fix_solid: bool,
    pub fix_shell: bool,
    pub fix_face: bool,
    pub fix_wire: bool,
    pub fix_orientation: bool,
    pub fix_small_area_wire: bool,
    pub min_edge_length: f64,
    pub tolerance_3d: f64,
}

impl Default for DeShapeFixParams {
    fn default() -> Self {
        Self {
            fix_solid: true, fix_shell: true, fix_face: true,
            fix_wire: true, fix_orientation: true,
            fix_small_area_wire: false,
            min_edge_length: 1e-7,
            tolerance_3d: 1e-6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_config_defaults() {
        let c = DeStepConfig::new();
        assert_eq!(c.schema(), StepSchema::Ap242);
        assert!(c.is_active());
        assert!((c.max_tolerance() - 1e-3).abs() < 1e-10);
    }

    #[test]
    fn step_config_set_schema() {
        let mut c = DeStepConfig::new();
        c.set_schema(StepSchema::Ap203);
        assert_eq!(c.schema(), StepSchema::Ap203);
    }

    #[test]
    fn step_config_tolerance_clamped() {
        let mut c = DeStepConfig::new();
        c.set_max_tolerance(-1.0);
        assert!(c.max_tolerance() >= 1e-10);
    }

    #[test]
    fn iges_config_defaults() {
        let c = DeIgesConfig::new();
        assert_eq!(c.read_mode(), IgesReadMode::OnlyVisible);
        assert!(c.is_active());
    }

    #[test]
    fn iges_config_bspline_continuity_clamped() {
        let mut c = DeIgesConfig::new();
        c.set_bspline_continuity(10);
        assert_eq!(c.read_bspline_continuity, 2);
    }
}
