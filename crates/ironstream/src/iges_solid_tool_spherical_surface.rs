// FILE: iges_solid_tool_spherical_surface.rs
// occt: IGESSolid_ToolSphericalSurface

//! Tool to work on a SphericalSurface (IGES type 196, forms 0-1). Called by
//! various Modules (ReadWriteModule, GeneralModule, SpecificModule).
//!
//! External plumbing (parameter stream, reader data, writer, copy tool) is
//! modelled with local helper types; the tool behaviour itself follows
//! IGESSolid_ToolSphericalSurface.cxx.

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
// The SphericalSurface entity data (IGESSolid_SphericalSurface)
// ---------------------------------------------------------------------------

/// Data of an IGES SphericalSurface entity. Form 0 is unparametrised
/// (no axis / reference direction); form 1 is parametrised.
#[derive(Clone, Debug, PartialEq)]
pub struct IgesSphericalSurface {
    pub form_number: i32,
    pub center: IgesEntity,
    pub radius: f64,
    pub axis: Option<IgesEntity>,
    pub refdir: Option<IgesEntity>,
}

impl IgesSphericalSurface {
    pub fn center(&self) -> &IgesEntity {
        &self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn axis(&self) -> Option<&IgesEntity> {
        self.axis.as_ref()
    }

    pub fn reference_dir(&self) -> Option<&IgesEntity> {
        self.refdir.as_ref()
    }

    /// True if the surface is parametrised (an axis is defined).
    pub fn is_parametrised(&self) -> bool {
        self.axis.is_some()
    }

    pub fn form_number(&self) -> i32 {
        self.form_number
    }
}

// ---------------------------------------------------------------------------
// The tool itself (IGESSolid_ToolSphericalSurface)
// ---------------------------------------------------------------------------

/// Tool to work on a SphericalSurface entity.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgesSolidToolSphericalSurface;

impl IgesSolidToolSphericalSurface {
    /// Returns a ToolSphericalSurface, ready to work.
    pub fn new() -> Self {
        Self
    }

    /// Reads own parameters: center point, radius; when form number is 1
    /// (parametrised) also axis and reference directions.
    /// Mirrors ReadOwnParams.
    pub fn read_own_params(
        &self,
        form_number: i32,
        ir: &IgesReaderData,
        pr: &mut ParamReader,
    ) -> Option<IgesSphericalSurface> {
        let center = pr.read_typed_entity(ir, POINT_TYPE);
        if center.is_none() {
            pr.add_fail("Center point : incorrect reference");
        }
        let radius = pr.read_real();
        if radius.is_none() {
            pr.add_fail("Radius : not a real");
        }

        let mut axis = None;
        let mut refdir = None;
        if form_number == 1 {
            axis = pr.read_typed_entity(ir, DIRECTION_TYPE);
            if axis.is_none() {
                pr.add_fail("Axis direction : incorrect reference");
            }
            refdir = pr.read_typed_entity(ir, DIRECTION_TYPE);
            if refdir.is_none() {
                pr.add_fail("Reference direction : incorrect reference");
            }
        }

        match (center, radius) {
            (Some(center), Some(radius)) => Some(IgesSphericalSurface {
                form_number,
                center,
                radius,
                axis,
                refdir,
            }),
            _ => None,
        }
    }

    /// Writes own parameters: center, radius, and when parametrised the axis
    /// and reference directions. Mirrors WriteOwnParams.
    pub fn write_own_params(&self, ent: &IgesSphericalSurface, iw: &mut IgesWriter) {
        iw.send_entity(ent.center());
        iw.send_real(ent.radius());
        if ent.is_parametrised() {
            iw.send_entity(ent.axis().expect("parametrised implies axis"));
            if let Some(refdir) = ent.reference_dir() {
                iw.send_entity(refdir);
            }
        }
    }

    /// Lists the entities shared: center, axis, reference direction
    /// (null handles are skipped, as Interface_EntityIterator does).
    pub fn own_shared(&self, ent: &IgesSphericalSurface) -> Vec<IgesEntity> {
        let mut shared = vec![ent.center().clone()];
        if let Some(axis) = ent.axis() {
            shared.push(axis.clone());
        }
        if let Some(refdir) = ent.reference_dir() {
            shared.push(refdir.clone());
        }
        shared
    }

    /// Copies specific parameters, transferring the referenced entities.
    pub fn own_copy(
        &self,
        another: &IgesSphericalSurface,
        tc: &mut CopyTool,
    ) -> IgesSphericalSurface {
        let center = tc.transferred(another.center());
        let radius = another.radius();
        if another.is_parametrised() {
            let axis = another.axis().map(|a| tc.transferred(a));
            let refdir = another.reference_dir().map(|r| tc.transferred(r));
            IgesSphericalSurface {
                form_number: another.form_number,
                center,
                radius,
                axis,
                refdir,
            }
        } else {
            IgesSphericalSurface {
                form_number: another.form_number,
                center,
                radius,
                axis: None,
                refdir: None,
            }
        }
    }

    /// Returns specific DirChecker: type 196, forms 0-1.
    pub fn dir_checker(&self, _ent: &IgesSphericalSurface) -> IgesDirChecker {
        let mut dc = IgesDirChecker::new(196, 0, 1);
        dc.structure = DirDef::Void;
        dc.line_font = DirDef::Any;
        dc.color = DirDef::Any;
        dc.blank_status_ignored = true;
        dc.subordinate_status_required = Some(1);
        dc.hierarchy_status_ignored = true;
        dc
    }

    /// Performs specific semantic check. Mirrors OwnCheck.
    pub fn own_check(&self, ent: &IgesSphericalSurface) -> Vec<String> {
        let mut fails = Vec::new();
        if ent.radius() <= 0.0 {
            fails.push("Radius : Not Positive".to_string());
        }
        let fn_expected = if ent.is_parametrised() { 1 } else { 0 };
        if fn_expected != ent.form_number() {
            fails.push("Parametrised Status Mismatches with Form Number".to_string());
        }
        if ent.axis().is_none() && ent.is_parametrised() {
            fails.push("Parametrised Spherical Surface : no Axis is defined".to_string());
        }
        fails
    }

    /// Dump of specific parameters. Mirrors OwnDump.
    pub fn own_dump(&self, ent: &IgesSphericalSurface, level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_SphericalSurface\n");
        let _sublevel = if level <= 4 { 0 } else { 1 };
        s.push_str("Center : ");
        s.push_str(&format!("entity #{}", ent.center().number));
        s.push('\n');
        s.push_str(&format!("Radius : {}\n", ent.radius()));
        if ent.is_parametrised() {
            s.push_str("Surface is Parametrised\n");
            s.push_str("Axis direction      : ");
            s.push_str(&format!("entity #{}", ent.axis().unwrap().number));
            s.push('\n');
            s.push_str("Reference direction : ");
            match ent.reference_dir() {
                Some(r) => s.push_str(&format!("entity #{}", r.number)),
                None => s.push_str("(null)"),
            }
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

    fn sample_parametrised() -> (IgesReaderData, IgesSphericalSurface) {
        let mut ir = IgesReaderData::new();
        let c = ir.add_entity(POINT_TYPE);
        let a = ir.add_entity(DIRECTION_TYPE);
        let r = ir.add_entity(DIRECTION_TYPE);
        let ent = IgesSphericalSurface {
            form_number: 1,
            center: ir.entity(c).unwrap().clone(),
            radius: 3.0,
            axis: Some(ir.entity(a).unwrap().clone()),
            refdir: Some(ir.entity(r).unwrap().clone()),
        };
        (ir, ent)
    }

    fn sample_unparametrised() -> (IgesReaderData, IgesSphericalSurface) {
        let mut ir = IgesReaderData::new();
        let c = ir.add_entity(POINT_TYPE);
        let ent = IgesSphericalSurface {
            form_number: 0,
            center: ir.entity(c).unwrap().clone(),
            radius: 1.5,
            axis: None,
            refdir: None,
        };
        (ir, ent)
    }

    #[test]
    fn test_tool_creation() {
        let tool = IgesSolidToolSphericalSurface::new();
        assert_eq!(tool, IgesSolidToolSphericalSurface);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolSphericalSurface::new();
        let (_ir, ent) = sample_unparametrised();
        let dc = tool.dir_checker(&ent);
        assert_eq!(dc.type_number, 196);
        assert_eq!(dc.form1, 0);
        assert_eq!(dc.form2, 1);
        assert_eq!(dc.structure, DirDef::Void);
        assert_eq!(dc.line_font, DirDef::Any);
        assert_eq!(dc.color, DirDef::Any);
        assert!(dc.blank_status_ignored);
        assert_eq!(dc.subordinate_status_required, Some(1));
        assert!(dc.hierarchy_status_ignored);
        assert!(dc.is_type_and_form_ok(196, 0));
        assert!(dc.is_type_and_form_ok(196, 1));
        assert!(!dc.is_type_and_form_ok(196, 2));
    }

    #[test]
    fn test_read_own_params_form0() {
        let tool = IgesSolidToolSphericalSurface::new();
        let (ir, expected) = sample_unparametrised();
        let mut pr = ParamReader::new(vec![IgesParam::Entity(1), IgesParam::Real(1.5)]);
        let ent = tool.read_own_params(0, &ir, &mut pr).expect("should read");
        assert!(!pr.has_failed(), "fails: {:?}", pr.fails());
        assert_eq!(ent, expected);
        assert!(!ent.is_parametrised());
    }

    #[test]
    fn test_read_own_params_form1() {
        let tool = IgesSolidToolSphericalSurface::new();
        let (ir, expected) = sample_parametrised();
        let mut pr = ParamReader::new(vec![
            IgesParam::Entity(1),
            IgesParam::Real(3.0),
            IgesParam::Entity(2),
            IgesParam::Entity(3),
        ]);
        let ent = tool.read_own_params(1, &ir, &mut pr).expect("should read");
        assert!(!pr.has_failed(), "fails: {:?}", pr.fails());
        assert_eq!(ent, expected);
        assert!(ent.is_parametrised());
    }

    #[test]
    fn test_read_own_params_wrong_center_type_fails() {
        let tool = IgesSolidToolSphericalSurface::new();
        let mut ir = IgesReaderData::new();
        let bad = ir.add_entity(DIRECTION_TYPE); // not a Point
        let mut pr = ParamReader::new(vec![IgesParam::Entity(bad), IgesParam::Real(1.0)]);
        let ent = tool.read_own_params(0, &ir, &mut pr);
        assert!(ent.is_none());
        assert!(pr.has_failed());
        assert!(pr.fails()[0].contains("Center point"));
    }

    #[test]
    fn test_write_read_roundtrip_form1() {
        let tool = IgesSolidToolSphericalSurface::new();
        let (ir, original) = sample_parametrised();
        let mut iw = IgesWriter::new();
        tool.write_own_params(&original, &mut iw);
        assert_eq!(iw.params().len(), 4);

        let mut pr = ParamReader::new(iw.into_params());
        let reread = tool.read_own_params(1, &ir, &mut pr).expect("should read");
        assert!(!pr.has_failed());
        assert_eq!(reread, original);
    }

    #[test]
    fn test_write_read_roundtrip_form0() {
        let tool = IgesSolidToolSphericalSurface::new();
        let (ir, original) = sample_unparametrised();
        let mut iw = IgesWriter::new();
        tool.write_own_params(&original, &mut iw);
        // Unparametrised: only center and radius are written.
        assert_eq!(iw.params().len(), 2);

        let mut pr = ParamReader::new(iw.into_params());
        let reread = tool.read_own_params(0, &ir, &mut pr).expect("should read");
        assert!(!pr.has_failed());
        assert_eq!(reread, original);
    }

    #[test]
    fn test_own_shared() {
        let tool = IgesSolidToolSphericalSurface::new();
        let (_ir, ent) = sample_parametrised();
        assert_eq!(tool.own_shared(&ent).len(), 3);
        let (_ir, ent0) = sample_unparametrised();
        assert_eq!(tool.own_shared(&ent0).len(), 1);
    }

    #[test]
    fn test_own_check() {
        let tool = IgesSolidToolSphericalSurface::new();
        let (_ir, ent) = sample_parametrised();
        assert!(tool.own_check(&ent).is_empty());

        // Non-positive radius
        let mut bad = ent.clone();
        bad.radius = 0.0;
        assert!(tool.own_check(&bad).iter().any(|f| f.contains("Not Positive")));

        // Form number mismatch: parametrised data with form 0
        let mut mismatch = ent.clone();
        mismatch.form_number = 0;
        assert!(tool
            .own_check(&mismatch)
            .iter()
            .any(|f| f.contains("Mismatches")));

        // Unparametrised data with form 1
        let (_ir, mut ent0) = sample_unparametrised();
        ent0.form_number = 1;
        assert!(tool
            .own_check(&ent0)
            .iter()
            .any(|f| f.contains("Mismatches")));
    }

    #[test]
    fn test_own_copy() {
        let tool = IgesSolidToolSphericalSurface::new();
        let (_ir, source) = sample_parametrised();
        let mut tc = CopyTool::new();
        let copied = tool.own_copy(&source, &mut tc);
        assert_eq!(copied.form_number, 1);
        assert_eq!(copied.radius, 3.0);
        assert_eq!(copied.center.type_number, POINT_TYPE);
        assert_eq!(copied.axis.as_ref().unwrap().type_number, DIRECTION_TYPE);
        assert_eq!(copied.refdir.as_ref().unwrap().type_number, DIRECTION_TYPE);
        // Renumbered in transfer order: center, axis, refdir.
        assert_eq!(copied.center.number, 1);
        assert_eq!(copied.axis.as_ref().unwrap().number, 2);
        assert_eq!(copied.refdir.as_ref().unwrap().number, 3);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolSphericalSurface::new();
        let (_ir, ent) = sample_parametrised();
        let dump = tool.own_dump(&ent, 5);
        assert!(dump.starts_with("IGESSolid_SphericalSurface\n"));
        assert!(dump.contains("Radius : 3"));
        assert!(dump.contains("Surface is Parametrised"));

        let (_ir, ent0) = sample_unparametrised();
        let dump0 = tool.own_dump(&ent0, 5);
        assert!(dump0.contains("Surface is UnParametrised"));
    }
}
