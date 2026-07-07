// FILE: iges_solid_tool_solid_of_revolution.rs
// occt: IGESSolid_ToolSolidOfRevolution

//! Tool to work on a SolidOfRevolution (IGES type 162, forms 0-1). Called
//! by various Modules (ReadWriteModule, GeneralModule, SpecificModule).
//!
//! External plumbing (parameter stream, reader data, writer, copy tool) is
//! modelled with local helper types; the tool behaviour itself follows
//! IGESSolid_ToolSolidOfRevolution.cxx.

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

    pub fn modulus(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Component-wise comparison with tolerance.
    pub fn is_equal(&self, other: &Xyz, eps: f64) -> bool {
        (self.x - other.x).abs() <= eps
            && (self.y - other.y).abs() <= eps
            && (self.z - other.z).abs() <= eps
    }

    /// Normalized copy; panics on null vector (as gp_Dir would).
    pub fn normalized(&self) -> Xyz {
        let m = self.modulus();
        assert!(m > 0.0, "cannot normalize null vector");
        Xyz::new(self.x / m, self.y / m, self.z / m)
    }
}

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
    warnings: Vec<String>,
}

impl ParamReader {
    pub fn new(params: Vec<IgesParam>) -> Self {
        Self { params, pos: 0, fails: Vec::new(), warnings: Vec::new() }
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

    /// Reads an entity reference; a typed read that must resolve.
    pub fn read_entity(&mut self, ir: &IgesReaderData) -> Option<IgesEntity> {
        match self.current() {
            Some(IgesParam::Entity(num)) => {
                let num = *num;
                self.advance();
                ir.entity(num).cloned()
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

    pub fn add_warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }

    pub fn fails(&self) -> &[String] {
        &self.fails
    }

    pub fn warnings(&self) -> &[String] {
        &self.warnings
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
// The SolidOfRevolution entity data (IGESSolid_SolidOfRevolution)
// ---------------------------------------------------------------------------

/// Data of an IGES SolidOfRevolution entity.
#[derive(Clone, Debug, PartialEq)]
pub struct IgesSolidOfRevolution {
    pub curve: IgesEntity,
    pub fraction: f64,
    pub axis_point: Xyz,
    /// Axis direction as stored (from the file or Init call).
    pub axis: Xyz,
}

impl IgesSolidOfRevolution {
    pub fn curve(&self) -> &IgesEntity {
        &self.curve
    }

    pub fn fraction(&self) -> f64 {
        self.fraction
    }

    pub fn axis_point(&self) -> Xyz {
        self.axis_point
    }

    /// The rotation axis, normalized (models gp_Dir conversion).
    pub fn axis(&self) -> Xyz {
        self.axis.normalized()
    }
}

// ---------------------------------------------------------------------------
// The tool itself (IGESSolid_ToolSolidOfRevolution)
// ---------------------------------------------------------------------------

/// Tool to work on a SolidOfRevolution entity.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgesSolidToolSolidOfRevolution;

impl IgesSolidToolSolidOfRevolution {
    /// Returns a ToolSolidOfRevolution, ready to work.
    pub fn new() -> Self {
        Self
    }

    /// Reads own parameters: curve entity, fraction of rotation (default 1),
    /// axis point (default origin) and axis direction (default Z), with the
    /// "poorly unitary" warning. Mirrors ReadOwnParams.
    pub fn read_own_params(
        &self,
        ir: &IgesReaderData,
        pr: &mut ParamReader,
    ) -> Option<IgesSolidOfRevolution> {
        let curve = pr.read_entity(ir);
        if curve.is_none() {
            pr.add_fail("Curve Entity : incorrect reference");
        }

        let mut fraction = 1.0;
        if pr.defined_else_skip() {
            match pr.read_real() {
                Some(v) => fraction = v,
                None => pr.add_fail("Fraction of rotation : not a real"),
            }
        }

        let mut axis_point = Xyz::new(0.0, 0.0, 0.0);
        let mut axis = Xyz::new(0.0, 0.0, 1.0);
        let labels_point = ["Axis Point (X)", "Axis Point (Y)", "Axis Point (Z)"];
        for (k, label) in labels_point.iter().enumerate() {
            if pr.defined_else_skip() {
                match pr.read_real() {
                    Some(v) => match k {
                        0 => axis_point.x = v,
                        1 => axis_point.y = v,
                        _ => axis_point.z = v,
                    },
                    None => pr.add_fail(&format!("{} : not a real", label)),
                }
            }
        }
        let labels_axis = ["Axis direction (I)", "Axis direction (J)", "Axis direction (K)"];
        for (k, label) in labels_axis.iter().enumerate() {
            if pr.defined_else_skip() {
                match pr.read_real() {
                    Some(v) => match k {
                        0 => axis.x = v,
                        1 => axis.y = v,
                        _ => axis.z = v,
                    },
                    None => pr.add_fail(&format!("{} : not a real", label)),
                }
            } else {
                match k {
                    0 => axis.x = 0.0,
                    1 => axis.y = 0.0,
                    _ => axis.z = 1.0,
                }
            }
        }

        let curve = curve?;
        let ent = IgesSolidOfRevolution { curve, fraction, axis_point, axis };

        // Warning if the axis was not unitary (eps 1e-5).
        let eps = 1.0e-5;
        if axis.modulus() > 0.0 && !axis.is_equal(&ent.axis(), eps) {
            pr.add_warning("Axis poorly unitary, normalized");
        }
        Some(ent)
    }

    /// Writes own parameters: curve, fraction, axis point X/Y/Z, axis X/Y/Z
    /// (normalized). Mirrors WriteOwnParams.
    pub fn write_own_params(&self, ent: &IgesSolidOfRevolution, iw: &mut IgesWriter) {
        iw.send_entity(ent.curve());
        iw.send_real(ent.fraction());
        iw.send_real(ent.axis_point().x);
        iw.send_real(ent.axis_point().y);
        iw.send_real(ent.axis_point().z);
        let axis = ent.axis();
        iw.send_real(axis.x);
        iw.send_real(axis.y);
        iw.send_real(axis.z);
    }

    /// Lists the entities shared: the curve entity.
    pub fn own_shared(&self, ent: &IgesSolidOfRevolution) -> Vec<IgesEntity> {
        vec![ent.curve().clone()]
    }

    /// Copies specific parameters, transferring the curve entity.
    pub fn own_copy(
        &self,
        another: &IgesSolidOfRevolution,
        tc: &mut CopyTool,
    ) -> IgesSolidOfRevolution {
        IgesSolidOfRevolution {
            curve: tc.transferred(another.curve()),
            fraction: another.fraction(),
            axis_point: another.axis_point,
            axis: another.axis,
        }
    }

    /// Returns specific DirChecker: type 162, forms 0-1.
    pub fn dir_checker(&self, _ent: &IgesSolidOfRevolution) -> IgesDirChecker {
        let mut dc = IgesDirChecker::new(162, 0, 1);
        dc.structure = DirDef::Void;
        dc.line_font = DirDef::Any;
        dc.color = DirDef::Any;
        dc.use_flag_required = Some(0);
        dc.hierarchy_status_ignored = true;
        dc
    }

    /// Performs specific semantic check: fraction must be in (0, 1].
    pub fn own_check(&self, ent: &IgesSolidOfRevolution) -> Vec<String> {
        let mut fails = Vec::new();
        if ent.fraction() <= 0.0 || ent.fraction() > 1.0 {
            fails.push("Fraction of rotation : Incorrect value".to_string());
        }
        fails
    }

    /// Dump of specific parameters. Mirrors OwnDump.
    pub fn own_dump(&self, ent: &IgesSolidOfRevolution, level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_SolidOfRevolution\n");
        s.push_str("Curve entity   : ");
        s.push_str(&format!("entity #{}", ent.curve().number));
        s.push('\n');
        s.push_str(&format!("Fraction of rotation : {}\n", ent.fraction()));
        if level > 1 {
            let p = ent.axis_point();
            let a = ent.axis();
            s.push_str(&format!("Axis Point     : ({},{},{})\n", p.x, p.y, p.z));
            s.push_str(&format!("Axis direction : ({},{},{})\n", a.x, a.y, a.z));
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CURVE_TYPE: i32 = 100;

    fn sample() -> (IgesReaderData, IgesSolidOfRevolution) {
        let mut ir = IgesReaderData::new();
        let num = ir.add_entity(CURVE_TYPE);
        let ent = IgesSolidOfRevolution {
            curve: ir.entity(num).unwrap().clone(),
            fraction: 0.5,
            axis_point: Xyz::new(1.0, 2.0, 3.0),
            axis: Xyz::new(0.0, 0.0, 1.0),
        };
        (ir, ent)
    }

    #[test]
    fn test_tool_creation() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        assert_eq!(tool, IgesSolidToolSolidOfRevolution);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let (_ir, ent) = sample();
        let dc = tool.dir_checker(&ent);
        assert_eq!(dc.type_number, 162);
        assert_eq!(dc.form1, 0);
        assert_eq!(dc.form2, 1);
        assert_eq!(dc.structure, DirDef::Void);
        assert_eq!(dc.line_font, DirDef::Any);
        assert_eq!(dc.color, DirDef::Any);
        assert_eq!(dc.use_flag_required, Some(0));
        assert!(dc.hierarchy_status_ignored);
        assert!(dc.is_type_and_form_ok(162, 0));
        assert!(dc.is_type_and_form_ok(162, 1));
        assert!(!dc.is_type_and_form_ok(162, 2));
    }

    #[test]
    fn test_read_own_params_full() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let (ir, expected) = sample();
        let mut pr = ParamReader::new(vec![
            IgesParam::Entity(1),
            IgesParam::Real(0.5),
            IgesParam::Real(1.0),
            IgesParam::Real(2.0),
            IgesParam::Real(3.0),
            IgesParam::Real(0.0),
            IgesParam::Real(0.0),
            IgesParam::Real(1.0),
        ]);
        let ent = tool.read_own_params(&ir, &mut pr).expect("should read");
        assert!(!pr.has_failed(), "fails: {:?}", pr.fails());
        assert!(pr.warnings().is_empty());
        assert_eq!(ent, expected);
    }

    #[test]
    fn test_read_own_params_defaults() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let (ir, _) = sample();
        // Everything after the curve defaulted.
        let mut pr = ParamReader::new(vec![
            IgesParam::Entity(1),
            IgesParam::Void, // fraction -> 1.0
            IgesParam::Void,
            IgesParam::Void,
            IgesParam::Void, // axis point -> origin
            IgesParam::Void,
            IgesParam::Void,
            IgesParam::Void, // axis -> (0,0,1)
        ]);
        let ent = tool.read_own_params(&ir, &mut pr).expect("should read");
        assert!(!pr.has_failed());
        assert_eq!(ent.fraction(), 1.0);
        assert_eq!(ent.axis_point(), Xyz::new(0.0, 0.0, 0.0));
        assert_eq!(ent.axis(), Xyz::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_read_own_params_non_unitary_axis_warns() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let (ir, _) = sample();
        let mut pr = ParamReader::new(vec![
            IgesParam::Entity(1),
            IgesParam::Real(1.0),
            IgesParam::Real(0.0),
            IgesParam::Real(0.0),
            IgesParam::Real(0.0),
            IgesParam::Real(3.0), // axis (3,0,0), not unitary
            IgesParam::Real(0.0),
            IgesParam::Real(0.0),
        ]);
        let ent = tool.read_own_params(&ir, &mut pr).expect("should read");
        assert!(!pr.has_failed());
        assert_eq!(pr.warnings().len(), 1);
        assert!(pr.warnings()[0].contains("poorly unitary"));
        assert_eq!(ent.axis(), Xyz::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_read_own_params_bad_curve_fails() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let ir = IgesReaderData::new();
        let mut pr = ParamReader::new(vec![IgesParam::Entity(9), IgesParam::Real(1.0)]);
        let ent = tool.read_own_params(&ir, &mut pr);
        assert!(ent.is_none());
        assert!(pr.has_failed());
    }

    #[test]
    fn test_write_read_roundtrip() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let (ir, original) = sample();
        let mut iw = IgesWriter::new();
        tool.write_own_params(&original, &mut iw);
        assert_eq!(iw.params().len(), 8);

        let mut pr = ParamReader::new(iw.into_params());
        let reread = tool.read_own_params(&ir, &mut pr).expect("should read");
        assert!(!pr.has_failed());
        assert_eq!(reread, original);
    }

    #[test]
    fn test_own_shared() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let (_ir, ent) = sample();
        let shared = tool.own_shared(&ent);
        assert_eq!(shared.len(), 1);
        assert_eq!(shared[0], ent.curve);
    }

    #[test]
    fn test_own_check() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let (_ir, mut ent) = sample();
        assert!(tool.own_check(&ent).is_empty());
        ent.fraction = 0.0;
        assert_eq!(tool.own_check(&ent).len(), 1);
        ent.fraction = 1.5;
        assert_eq!(tool.own_check(&ent).len(), 1);
        ent.fraction = 1.0;
        assert!(tool.own_check(&ent).is_empty());
    }

    #[test]
    fn test_own_copy() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let source = IgesSolidOfRevolution {
            curve: IgesEntity { number: 5, type_number: CURVE_TYPE },
            fraction: 0.25,
            axis_point: Xyz::new(1.0, 1.0, 1.0),
            axis: Xyz::new(0.0, 1.0, 0.0),
        };
        let mut tc = CopyTool::new();
        let copied = tool.own_copy(&source, &mut tc);
        assert_eq!(copied.curve.number, 1); // renumbered
        assert_eq!(copied.fraction, 0.25);
        assert_eq!(copied.axis_point, source.axis_point);
        assert_eq!(copied.axis, source.axis);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolSolidOfRevolution::new();
        let (_ir, ent) = sample();
        let dump = tool.own_dump(&ent, 5);
        assert!(dump.starts_with("IGESSolid_SolidOfRevolution\n"));
        assert!(dump.contains("Curve entity"));
        assert!(dump.contains("Fraction of rotation : 0.5"));
        assert!(dump.contains("Axis Point"));
        assert!(dump.contains("Axis direction"));
    }
}
