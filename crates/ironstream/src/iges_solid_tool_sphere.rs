// FILE: iges_solid_tool_sphere.rs
// occt: IGESSolid_ToolSphere

//! Tool to work on a Sphere (IGES type 158, form 0). Called by various
//! Modules (ReadWriteModule, GeneralModule, SpecificModule).
//!
//! External plumbing (parameter stream, writer, copy tool) is modelled with
//! local helper types; the tool behaviour itself follows
//! IGESSolid_ToolSphere.cxx.

// ---------------------------------------------------------------------------
// Local model of the IGES parameter stream (external plumbing)
// ---------------------------------------------------------------------------

/// One parameter of an IGES parameter section.
#[derive(Clone, Debug, PartialEq)]
pub enum IgesParam {
    Integer(i32),
    Real(f64),
    /// Reference to an entity, by its number in the reader data (0 = null).
    Entity(u32),
    /// Defaulted (omitted) parameter.
    Void,
}

/// Minimal 3D coordinate triple (models gp_XYZ).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Xyz {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// Sequential reader over a parameter list, collecting check messages.
#[derive(Clone, Debug)]
pub struct ParamReader {
    params: Vec<IgesParam>,
    pos: usize,
    fails: Vec<String>,
}

impl ParamReader {
    pub fn new(params: Vec<IgesParam>) -> Self {
        Self { params, pos: 0, fails: Vec::new() }
    }

    fn current(&self) -> Option<&IgesParam> {
        self.params.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    /// OCCT DefinedElseSkip: true if the current parameter is defined
    /// (present and not void); otherwise skips it and returns false.
    pub fn defined_else_skip(&mut self) -> bool {
        match self.current() {
            Some(IgesParam::Void) => {
                self.advance();
                false
            }
            Some(_) => true,
            None => false,
        }
    }

    /// Reads a real value; integer parameters are accepted and converted.
    pub fn read_real(&mut self) -> Option<f64> {
        match self.current() {
            Some(IgesParam::Real(v)) => {
                let v = *v;
                self.advance();
                Some(v)
            }
            Some(IgesParam::Integer(v)) => {
                let v = *v as f64;
                self.advance();
                Some(v)
            }
            _ => {
                self.advance();
                None
            }
        }
    }

    pub fn add_fail(&mut self, msg: &str) {
        self.fails.push(msg.to_string());
    }

    pub fn fails(&self) -> &[String] {
        &self.fails
    }

    pub fn has_failed(&self) -> bool {
        !self.fails.is_empty()
    }
}

/// Writer collecting the parameters an entity sends.
#[derive(Clone, Debug, Default)]
pub struct IgesWriter {
    params: Vec<IgesParam>,
}

impl IgesWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn send_real(&mut self, v: f64) {
        self.params.push(IgesParam::Real(v));
    }

    pub fn params(&self) -> &[IgesParam] {
        &self.params
    }

    pub fn into_params(self) -> Vec<IgesParam> {
        self.params
    }
}

/// Copy tool placeholder: a Sphere references no other entity, so the copy
/// tool has nothing to transfer (mirrors the unused TC parameter in OCCT).
#[derive(Clone, Debug, Default)]
pub struct CopyTool;

impl CopyTool {
    pub fn new() -> Self {
        Self
    }
}

/// Def-status of a directory field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirDef {
    Void,
    Any,
}

/// Local model of IGESData_DirChecker.
#[derive(Clone, Debug)]
pub struct IgesDirChecker {
    pub type_number: i32,
    pub form1: i32,
    pub form2: i32,
    pub structure: DirDef,
    pub line_font: DirDef,
    pub color: DirDef,
    pub use_flag_required: Option<i32>,
    pub hierarchy_status_ignored: bool,
}

impl IgesDirChecker {
    pub fn new(type_number: i32, form1: i32, form2: i32) -> Self {
        Self {
            type_number,
            form1,
            form2,
            structure: DirDef::Any,
            line_font: DirDef::Any,
            color: DirDef::Any,
            use_flag_required: None,
            hierarchy_status_ignored: false,
        }
    }

    /// True if `(type_number, form)` is acceptable for this checker.
    pub fn is_type_and_form_ok(&self, type_number: i32, form: i32) -> bool {
        type_number == self.type_number && form >= self.form1 && form <= self.form2
    }
}

// ---------------------------------------------------------------------------
// The Sphere entity data (IGESSolid_Sphere)
// ---------------------------------------------------------------------------

/// Data of an IGES Sphere entity: radius and center.
#[derive(Clone, Debug, PartialEq)]
pub struct IgesSphere {
    pub radius: f64,
    pub center: Xyz,
}

impl IgesSphere {
    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn center(&self) -> Xyz {
        self.center
    }
}

// ---------------------------------------------------------------------------
// The tool itself (IGESSolid_ToolSphere)
// ---------------------------------------------------------------------------

/// Tool to work on a Sphere entity.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgesSolidToolSphere;

impl IgesSolidToolSphere {
    /// Returns a ToolSphere, ready to work.
    pub fn new() -> Self {
        Self
    }

    /// Reads own parameters: radius (required), then center X/Y/Z each
    /// defaulting to 0. Mirrors ReadOwnParams.
    pub fn read_own_params(&self, pr: &mut ParamReader) -> Option<IgesSphere> {
        let radius = pr.read_real();
        if radius.is_none() {
            pr.add_fail("Radius : not a real");
        }

        let mut center = Xyz::new(0.0, 0.0, 0.0);
        let labels = ["Center (X)", "Center (Y)", "Center (Z)"];
        for (k, label) in labels.iter().enumerate() {
            if pr.defined_else_skip() {
                match pr.read_real() {
                    Some(v) => match k {
                        0 => center.x = v,
                        1 => center.y = v,
                        _ => center.z = v,
                    },
                    None => pr.add_fail(&format!("{} : not a real", label)),
                }
            }
        }

        radius.map(|radius| IgesSphere { radius, center })
    }

    /// Writes own parameters: radius, center X/Y/Z. Mirrors WriteOwnParams.
    pub fn write_own_params(&self, ent: &IgesSphere, iw: &mut IgesWriter) {
        iw.send_real(ent.radius());
        iw.send_real(ent.center().x);
        iw.send_real(ent.center().y);
        iw.send_real(ent.center().z);
    }

    /// Lists the entities shared by a Sphere: none.
    pub fn own_shared(&self, _ent: &IgesSphere) -> Vec<()> {
        Vec::new()
    }

    /// Copies specific parameters (no entity to transfer).
    pub fn own_copy(&self, another: &IgesSphere, _tc: &mut CopyTool) -> IgesSphere {
        IgesSphere { radius: another.radius(), center: another.center() }
    }

    /// Returns specific DirChecker: type 158, form 0.
    pub fn dir_checker(&self, _ent: &IgesSphere) -> IgesDirChecker {
        let mut dc = IgesDirChecker::new(158, 0, 0);
        dc.structure = DirDef::Void;
        dc.line_font = DirDef::Any;
        dc.color = DirDef::Any;
        dc.use_flag_required = Some(0);
        dc.hierarchy_status_ignored = true;
        dc
    }

    /// Performs specific semantic check: radius must be positive.
    pub fn own_check(&self, ent: &IgesSphere) -> Vec<String> {
        let mut fails = Vec::new();
        if ent.radius() <= 0.0 {
            fails.push("Radius : Not Positive".to_string());
        }
        fails
    }

    /// Dump of specific parameters. Mirrors OwnDump.
    pub fn own_dump(&self, ent: &IgesSphere, level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_Sphere\n");
        s.push_str(&format!("Radius : {}\n", ent.radius()));
        s.push_str("Center : ");
        if level > 1 {
            let c = ent.center();
            s.push_str(&format!("({},{},{})", c.x, c.y, c.z));
        }
        s.push('\n');
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> IgesSphere {
        IgesSphere { radius: 2.5, center: Xyz::new(1.0, -2.0, 3.0) }
    }

    #[test]
    fn test_tool_creation() {
        let tool = IgesSolidToolSphere::new();
        assert_eq!(tool, IgesSolidToolSphere);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolSphere::new();
        let dc = tool.dir_checker(&sample());
        assert_eq!(dc.type_number, 158);
        assert_eq!(dc.form1, 0);
        assert_eq!(dc.form2, 0);
        assert_eq!(dc.structure, DirDef::Void);
        assert_eq!(dc.line_font, DirDef::Any);
        assert_eq!(dc.color, DirDef::Any);
        assert_eq!(dc.use_flag_required, Some(0));
        assert!(dc.hierarchy_status_ignored);
        assert!(dc.is_type_and_form_ok(158, 0));
        assert!(!dc.is_type_and_form_ok(158, 1));
        assert!(!dc.is_type_and_form_ok(514, 0));
    }

    #[test]
    fn test_read_own_params_full() {
        let tool = IgesSolidToolSphere::new();
        let mut pr = ParamReader::new(vec![
            IgesParam::Real(2.5),
            IgesParam::Real(1.0),
            IgesParam::Real(-2.0),
            IgesParam::Real(3.0),
        ]);
        let ent = tool.read_own_params(&mut pr).expect("should read");
        assert!(!pr.has_failed(), "fails: {:?}", pr.fails());
        assert_eq!(ent, sample());
    }

    #[test]
    fn test_read_own_params_default_center() {
        let tool = IgesSolidToolSphere::new();
        let mut pr = ParamReader::new(vec![
            IgesParam::Real(1.0),
            IgesParam::Void,
            IgesParam::Void,
            IgesParam::Void,
        ]);
        let ent = tool.read_own_params(&mut pr).expect("should read");
        assert!(!pr.has_failed());
        assert_eq!(ent.center(), Xyz::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_read_own_params_short_params_default_center() {
        let tool = IgesSolidToolSphere::new();
        // Only the radius present: center defaults to origin.
        let mut pr = ParamReader::new(vec![IgesParam::Real(4.0)]);
        let ent = tool.read_own_params(&mut pr).expect("should read");
        assert!(!pr.has_failed());
        assert_eq!(ent.radius(), 4.0);
        assert_eq!(ent.center(), Xyz::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_read_own_params_missing_radius_fails() {
        let tool = IgesSolidToolSphere::new();
        let mut pr = ParamReader::new(vec![IgesParam::Void]);
        let ent = tool.read_own_params(&mut pr);
        assert!(ent.is_none());
        assert!(pr.has_failed());
        assert!(pr.fails()[0].contains("Radius"));
    }

    #[test]
    fn test_write_read_roundtrip() {
        let tool = IgesSolidToolSphere::new();
        let original = sample();
        let mut iw = IgesWriter::new();
        tool.write_own_params(&original, &mut iw);
        assert_eq!(
            iw.params(),
            &[
                IgesParam::Real(2.5),
                IgesParam::Real(1.0),
                IgesParam::Real(-2.0),
                IgesParam::Real(3.0),
            ]
        );

        let mut pr = ParamReader::new(iw.into_params());
        let reread = tool.read_own_params(&mut pr).expect("should read");
        assert!(!pr.has_failed());
        assert_eq!(reread, original);
    }

    #[test]
    fn test_own_shared_is_empty() {
        let tool = IgesSolidToolSphere::new();
        assert!(tool.own_shared(&sample()).is_empty());
    }

    #[test]
    fn test_own_check() {
        let tool = IgesSolidToolSphere::new();
        assert!(tool.own_check(&sample()).is_empty());
        let bad = IgesSphere { radius: 0.0, center: Xyz::default() };
        let fails = tool.own_check(&bad);
        assert_eq!(fails.len(), 1);
        assert!(fails[0].contains("Not Positive"));
    }

    #[test]
    fn test_own_copy() {
        let tool = IgesSolidToolSphere::new();
        let source = sample();
        let mut tc = CopyTool::new();
        let copied = tool.own_copy(&source, &mut tc);
        assert_eq!(copied, source);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolSphere::new();
        let dump = tool.own_dump(&sample(), 5);
        assert!(dump.starts_with("IGESSolid_Sphere\n"));
        assert!(dump.contains("Radius : 2.5"));
        assert!(dump.contains("Center : (1,-2,3)"));

        let short = tool.own_dump(&sample(), 1);
        assert!(short.contains("Center : \n"));
    }
}
