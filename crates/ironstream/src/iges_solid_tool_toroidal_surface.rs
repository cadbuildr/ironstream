// FILE: iges_solid_tool_toroidal_surface.rs
// occt: IGESSolid_ToolToroidalSurface

//! Tool to work on a ToroidalSurface (IGES type 198, forms 0-1). Called by
//! various Modules (ReadWriteModule, GeneralModule, SpecificModule).
//!
//! External plumbing (parameter stream, reader data, writer, copy tool) is
//! modelled with local helper types; the tool behaviour itself follows
//! IGESSolid_ToolToroidalSurface.cxx.

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

/// An IGES entity, identified by its number and type.
#[derive(Clone, Debug, PartialEq)]
pub struct IgesEntity {
    pub number: u32,
    pub type_number: i32,
}

/// IGES type number of a Point entity (IGESGeom_Point).
pub const POINT_TYPE: i32 = 116;
/// IGES type number of a Direction entity (IGESGeom_Direction).
pub const DIRECTION_TYPE: i32 = 123;

/// Reader data: the pool of entities a file section can reference.
#[derive(Clone, Debug, Default)]
pub struct IgesReaderData {
    entities: Vec<IgesEntity>,
}

impl IgesReaderData {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an entity and returns its number (1-based).
    pub fn add_entity(&mut self, type_number: i32) -> u32 {
        let number = self.entities.len() as u32 + 1;
        self.entities.push(IgesEntity { number, type_number });
        number
    }

    pub fn entity(&self, number: u32) -> Option<&IgesEntity> {
        if number == 0 {
            return None;
        }
        self.entities.get(number as usize - 1)
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

    /// Reads an entity reference which must resolve and match `type_number`.
    pub fn read_typed_entity(
        &mut self,
        ir: &IgesReaderData,
        type_number: i32,
    ) -> Option<IgesEntity> {
        match self.current() {
            Some(IgesParam::Entity(num)) => {
                let num = *num;
                self.advance();
                match ir.entity(num) {
                    Some(ent) if ent.type_number == type_number => Some(ent.clone()),
                    _ => None,
                }
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

    pub fn send_entity(&mut self, ent: &IgesEntity) {
        self.params.push(IgesParam::Entity(ent.number));
    }

    pub fn params(&self) -> &[IgesParam] {
        &self.params
    }

    pub fn into_params(self) -> Vec<IgesParam> {
        self.params
    }
}

/// Copy tool: transfers entities into a new model, renumbering them.
#[derive(Clone, Debug, Default)]
pub struct CopyTool {
    transferred: Vec<IgesEntity>,
}

impl CopyTool {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the transferred counterpart of `ent` (same type, new number).
    pub fn transferred(&mut self, ent: &IgesEntity) -> IgesEntity {
        let number = self.transferred.len() as u32 + 1;
        let new_ent = IgesEntity { number, type_number: ent.type_number };
        self.transferred.push(new_ent.clone());
        new_ent
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
    pub blank_status_ignored: bool,
    pub subordinate_status_required: Option<i32>,
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
            blank_status_ignored: false,
            subordinate_status_required: None,
            hierarchy_status_ignored: false,
        }
    }

    /// True if `(type_number, form)` is acceptable for this checker.
    pub fn is_type_and_form_ok(&self, type_number: i32, form: i32) -> bool {
        type_number == self.type_number && form >= self.form1 && form <= self.form2
    }
}

// ---------------------------------------------------------------------------
// The ToroidalSurface entity data (IGESSolid_ToroidalSurface)
// ---------------------------------------------------------------------------

/// Data of an IGES ToroidalSurface entity. Form 0 is unparametrised
/// (no reference direction); form 1 is parametrised.
#[derive(Clone, Debug, PartialEq)]
pub struct IgesToroidalSurface {
    pub form_number: i32,
    pub center: IgesEntity,
    pub axis: IgesEntity,
    pub major_radius: f64,
    pub minor_radius: f64,
    pub refdir: Option<IgesEntity>,
}

impl IgesToroidalSurface {
    pub fn center(&self) -> &IgesEntity {
        &self.center
    }

    pub fn axis(&self) -> &IgesEntity {
        &self.axis
    }

    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }

    pub fn reference_dir(&self) -> Option<&IgesEntity> {
        self.refdir.as_ref()
    }

    /// True if the surface is parametrised (a reference direction exists).
    pub fn is_parametrised(&self) -> bool {
        self.refdir.is_some()
    }

    pub fn form_number(&self) -> i32 {
        self.form_number
    }
}

// ---------------------------------------------------------------------------
// The tool itself (IGESSolid_ToolToroidalSurface)
// ---------------------------------------------------------------------------

/// Tool to work on a ToroidalSurface entity.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgesSolidToolToroidalSurface;

impl IgesSolidToolToroidalSurface {
    /// Returns a ToolToroidalSurface, ready to work.
    pub fn new() -> Self {
        Self
    }

    /// Reads own parameters: center point, axis direction, major and minor
    /// radii; when form number is 1, also the reference direction.
    /// Mirrors ReadOwnParams.
    pub fn read_own_params(
        &self,
        form_number: i32,
        ir: &IgesReaderData,
        pr: &mut ParamReader,
    ) -> Option<IgesToroidalSurface> {
        let center = pr.read_typed_entity(ir, POINT_TYPE);
        if center.is_none() {
            pr.add_fail("Center point : incorrect reference");
        }
        let axis = pr.read_typed_entity(ir, DIRECTION_TYPE);
        if axis.is_none() {
            pr.add_fail("Axis direction : incorrect reference");
        }
        let maj_rad = pr.read_real();
        if maj_rad.is_none() {
            pr.add_fail("Major Radius : not a real");
        }
        let min_rad = pr.read_real();
        if min_rad.is_none() {
            pr.add_fail("Minor Radius : not a real");
        }

        let mut refdir = None;
        if form_number == 1 {
            refdir = pr.read_typed_entity(ir, DIRECTION_TYPE);
            if refdir.is_none() {
                pr.add_fail("Reference direction : incorrect reference");
            }
        }

        match (center, axis, maj_rad, min_rad) {
            (Some(center), Some(axis), Some(major_radius), Some(minor_radius)) => {
                Some(IgesToroidalSurface {
                    form_number,
                    center,
                    axis,
                    major_radius,
                    minor_radius,
                    refdir,
                })
            }
            _ => None,
        }
    }

    /// Writes own parameters: center, axis, radii, and when parametrised the
    /// reference direction. Mirrors WriteOwnParams.
    pub fn write_own_params(&self, ent: &IgesToroidalSurface, iw: &mut IgesWriter) {
        iw.send_entity(ent.center());
        iw.send_entity(ent.axis());
        iw.send_real(ent.major_radius());
        iw.send_real(ent.minor_radius());
        if ent.is_parametrised() {
            iw.send_entity(ent.reference_dir().expect("parametrised implies refdir"));
        }
    }

    /// Lists the entities shared: center, axis, reference direction
    /// (null handles are skipped, as Interface_EntityIterator does).
    pub fn own_shared(&self, ent: &IgesToroidalSurface) -> Vec<IgesEntity> {
        let mut shared = vec![ent.center().clone(), ent.axis().clone()];
        if let Some(refdir) = ent.reference_dir() {
            shared.push(refdir.clone());
        }
        shared
    }

    /// Copies specific parameters, transferring the referenced entities.
    pub fn own_copy(
        &self,
        another: &IgesToroidalSurface,
        tc: &mut CopyTool,
    ) -> IgesToroidalSurface {
        let center = tc.transferred(another.center());
        let axis = tc.transferred(another.axis());
        let refdir = if another.is_parametrised() {
            another.reference_dir().map(|r| tc.transferred(r))
        } else {
            None
        };
        IgesToroidalSurface {
            form_number: another.form_number,
            center,
            axis,
            major_radius: another.major_radius(),
            minor_radius: another.minor_radius(),
            refdir,
        }
    }

    /// Returns specific DirChecker: type 198, forms 0-1.
    pub fn dir_checker(&self, _ent: &IgesToroidalSurface) -> IgesDirChecker {
        let mut dc = IgesDirChecker::new(198, 0, 1);
        dc.structure = DirDef::Void;
        dc.line_font = DirDef::Any;
        dc.color = DirDef::Any;
        dc.blank_status_ignored = true;
        dc.subordinate_status_required = Some(1);
        dc.hierarchy_status_ignored = true;
        dc
    }

    /// Performs specific semantic check. Mirrors OwnCheck.
    pub fn own_check(&self, ent: &IgesToroidalSurface) -> Vec<String> {
        let mut fails = Vec::new();
        if ent.major_radius() <= 0.0 {
            fails.push("Major Radius : Not Positive".to_string());
        }
        if ent.minor_radius() <= 0.0 {
            fails.push("Minor Radius : Not Positive".to_string());
        }
        if ent.minor_radius() >= ent.major_radius() {
            fails.push("Minor Radius : Value not < Major radius".to_string());
        }
        let fn_expected = if ent.is_parametrised() { 1 } else { 0 };
        if fn_expected != ent.form_number() {
            fails.push("Parametrised Status Mismatches with Form Number".to_string());
        }
        fails
    }

    /// Dump of specific parameters. Mirrors OwnDump.
    pub fn own_dump(&self, ent: &IgesToroidalSurface, level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_ToroidalSurface\n");
        let _sublevel = if level <= 4 { 0 } else { 1 };
        s.push_str("Center : ");
        s.push_str(&format!("entity #{}", ent.center().number));
        s.push('\n');
        s.push_str("Axis direction : ");
        s.push_str(&format!("entity #{}", ent.axis().number));
        s.push('\n');
        s.push_str(&format!(
            "Major Radius : {}  Minor Radius : {}\n",
            ent.major_radius(),
            ent.minor_radius()
        ));
        if ent.is_parametrised() {
            s.push_str("Surface is Parametrised  -  Reference direction : ");
            s.push_str(&format!("entity #{}", ent.reference_dir().unwrap().number));
            s.push('\n');
        } else {
            s.push_str("Surface is UnParametrised\n");
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_parametrised() -> (IgesReaderData, IgesToroidalSurface) {
        let mut ir = IgesReaderData::new();
        let c = ir.add_entity(POINT_TYPE);
        let a = ir.add_entity(DIRECTION_TYPE);
        let r = ir.add_entity(DIRECTION_TYPE);
        let ent = IgesToroidalSurface {
            form_number: 1,
            center: ir.entity(c).unwrap().clone(),
            axis: ir.entity(a).unwrap().clone(),
            major_radius: 10.0,
            minor_radius: 2.0,
            refdir: Some(ir.entity(r).unwrap().clone()),
        };
        (ir, ent)
    }

    fn sample_unparametrised() -> (IgesReaderData, IgesToroidalSurface) {
        let mut ir = IgesReaderData::new();
        let c = ir.add_entity(POINT_TYPE);
        let a = ir.add_entity(DIRECTION_TYPE);
        let ent = IgesToroidalSurface {
            form_number: 0,
            center: ir.entity(c).unwrap().clone(),
            axis: ir.entity(a).unwrap().clone(),
            major_radius: 5.0,
            minor_radius: 1.0,
            refdir: None,
        };
        (ir, ent)
    }

    #[test]
    fn test_tool_creation() {
        let tool = IgesSolidToolToroidalSurface::new();
        assert_eq!(tool, IgesSolidToolToroidalSurface);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolToroidalSurface::new();
        let (_ir, ent) = sample_unparametrised();
        let dc = tool.dir_checker(&ent);
        assert_eq!(dc.type_number, 198);
        assert_eq!(dc.form1, 0);
        assert_eq!(dc.form2, 1);
        assert_eq!(dc.structure, DirDef::Void);
        assert_eq!(dc.line_font, DirDef::Any);
        assert_eq!(dc.color, DirDef::Any);
        assert!(dc.blank_status_ignored);
        assert_eq!(dc.subordinate_status_required, Some(1));
        assert!(dc.hierarchy_status_ignored);
        assert!(dc.is_type_and_form_ok(198, 0));
        assert!(dc.is_type_and_form_ok(198, 1));
        assert!(!dc.is_type_and_form_ok(198, 2));
    }

    #[test]
    fn test_read_own_params_form0() {
        let tool = IgesSolidToolToroidalSurface::new();
        let (ir, expected) = sample_unparametrised();
        let mut pr = ParamReader::new(vec![
            IgesParam::Entity(1),
            IgesParam::Entity(2),
            IgesParam::Real(5.0),
            IgesParam::Real(1.0),
        ]);
        let ent = tool.read_own_params(0, &ir, &mut pr).expect("should read");
        assert!(!pr.has_failed(), "fails: {:?}", pr.fails());
        assert_eq!(ent, expected);
        assert!(!ent.is_parametrised());
    }

    #[test]
    fn test_read_own_params_form1() {
        let tool = IgesSolidToolToroidalSurface::new();
        let (ir, expected) = sample_parametrised();
        let mut pr = ParamReader::new(vec![
            IgesParam::Entity(1),
            IgesParam::Entity(2),
            IgesParam::Real(10.0),
            IgesParam::Real(2.0),
            IgesParam::Entity(3),
        ]);
        let ent = tool.read_own_params(1, &ir, &mut pr).expect("should read");
        assert!(!pr.has_failed(), "fails: {:?}", pr.fails());
        assert_eq!(ent, expected);
        assert!(ent.is_parametrised());
    }

    #[test]
    fn test_read_own_params_wrong_axis_type_fails() {
        let tool = IgesSolidToolToroidalSurface::new();
        let mut ir = IgesReaderData::new();
        let c = ir.add_entity(POINT_TYPE);
        let bad = ir.add_entity(POINT_TYPE); // not a Direction
        let mut pr = ParamReader::new(vec![
            IgesParam::Entity(c),
            IgesParam::Entity(bad),
            IgesParam::Real(5.0),
            IgesParam::Real(1.0),
        ]);
        let ent = tool.read_own_params(0, &ir, &mut pr);
        assert!(ent.is_none());
        assert!(pr.has_failed());
        assert!(pr.fails()[0].contains("Axis direction"));
    }

    #[test]
    fn test_write_read_roundtrip_form1() {
        let tool = IgesSolidToolToroidalSurface::new();
        let (ir, original) = sample_parametrised();
        let mut iw = IgesWriter::new();
        tool.write_own_params(&original, &mut iw);
        assert_eq!(iw.params().len(), 5);

        let mut pr = ParamReader::new(iw.into_params());
        let reread = tool.read_own_params(1, &ir, &mut pr).expect("should read");
        assert!(!pr.has_failed());
        assert_eq!(reread, original);
    }

    #[test]
    fn test_write_read_roundtrip_form0() {
        let tool = IgesSolidToolToroidalSurface::new();
        let (ir, original) = sample_unparametrised();
        let mut iw = IgesWriter::new();
        tool.write_own_params(&original, &mut iw);
        // Unparametrised: no reference direction written.
        assert_eq!(iw.params().len(), 4);

        let mut pr = ParamReader::new(iw.into_params());
        let reread = tool.read_own_params(0, &ir, &mut pr).expect("should read");
        assert!(!pr.has_failed());
        assert_eq!(reread, original);
    }

    #[test]
    fn test_own_shared() {
        let tool = IgesSolidToolToroidalSurface::new();
        let (_ir, ent) = sample_parametrised();
        assert_eq!(tool.own_shared(&ent).len(), 3);
        let (_ir, ent0) = sample_unparametrised();
        assert_eq!(tool.own_shared(&ent0).len(), 2);
    }

    #[test]
    fn test_own_check() {
        let tool = IgesSolidToolToroidalSurface::new();
        let (_ir, ent) = sample_parametrised();
        assert!(tool.own_check(&ent).is_empty());

        let mut bad = ent.clone();
        bad.major_radius = 0.0;
        let fails = tool.own_check(&bad);
        // major not positive AND minor >= major
        assert!(fails.iter().any(|f| f.contains("Major Radius : Not Positive")));
        assert!(fails.iter().any(|f| f.contains("not < Major")));

        let mut bad2 = ent.clone();
        bad2.minor_radius = 20.0; // >= major
        assert!(tool
            .own_check(&bad2)
            .iter()
            .any(|f| f.contains("not < Major")));

        let mut mismatch = ent.clone();
        mismatch.form_number = 0;
        assert!(tool
            .own_check(&mismatch)
            .iter()
            .any(|f| f.contains("Mismatches")));
    }

    #[test]
    fn test_own_copy() {
        let tool = IgesSolidToolToroidalSurface::new();
        let (_ir, source) = sample_parametrised();
        let mut tc = CopyTool::new();
        let copied = tool.own_copy(&source, &mut tc);
        assert_eq!(copied.form_number, 1);
        assert_eq!(copied.major_radius, 10.0);
        assert_eq!(copied.minor_radius, 2.0);
        // Renumbered in transfer order: center, axis, refdir.
        assert_eq!(copied.center.number, 1);
        assert_eq!(copied.axis.number, 2);
        assert_eq!(copied.refdir.as_ref().unwrap().number, 3);
        assert_eq!(copied.center.type_number, POINT_TYPE);
        assert_eq!(copied.axis.type_number, DIRECTION_TYPE);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolToroidalSurface::new();
        let (_ir, ent) = sample_parametrised();
        let dump = tool.own_dump(&ent, 5);
        assert!(dump.starts_with("IGESSolid_ToroidalSurface\n"));
        assert!(dump.contains("Major Radius : 10  Minor Radius : 2"));
        assert!(dump.contains("Surface is Parametrised"));

        let (_ir, ent0) = sample_unparametrised();
        let dump0 = tool.own_dump(&ent0, 5);
        assert!(dump0.contains("Surface is UnParametrised"));
    }
}
