// FILE: iges_solid_tool_solid_of_linear_extrusion.rs
// occt: IGESSolid_ToolSolidOfLinearExtrusion

//! Tool to work on a SolidOfLinearExtrusion (IGES type 164, form 0). Called
//! by various Modules (ReadWriteModule, GeneralModule, SpecificModule).
//!
//! External plumbing (parameter stream, reader data, writer, copy tool) is
//! modelled with local helper types; the tool behaviour itself follows
//! IGESSolid_ToolSolidOfLinearExtrusion.cxx.

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
// The SolidOfLinearExtrusion entity data (IGESSolid_SolidOfLinearExtrusion)
// ---------------------------------------------------------------------------

/// Data of an IGES SolidOfLinearExtrusion entity.
#[derive(Clone, Debug, PartialEq)]
pub struct IgesSolidOfLinearExtrusion {
    pub curve: IgesEntity,
    pub extrusion_length: f64,
    /// Direction as stored (from the file or Init call).
    pub direction: Xyz,
}

impl IgesSolidOfLinearExtrusion {
    pub fn curve(&self) -> &IgesEntity {
        &self.curve
    }

    pub fn extrusion_length(&self) -> f64 {
        self.extrusion_length
    }

    /// The extrusion direction, normalized (models gp_Dir conversion).
    pub fn extrusion_direction(&self) -> Xyz {
        self.direction.normalized()
    }
}

// ---------------------------------------------------------------------------
// The tool itself (IGESSolid_ToolSolidOfLinearExtrusion)
// ---------------------------------------------------------------------------

/// Tool to work on a SolidOfLinearExtrusion entity.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgesSolidToolSolidOfLinearExtrusion;

impl IgesSolidToolSolidOfLinearExtrusion {
    /// Returns a ToolSolidOfLinearExtrusion, ready to work.
    pub fn new() -> Self {
        Self
    }

    /// Reads own parameters: curve entity, extrusion length, then direction
    /// components with defaults (0, 0, 1). Mirrors ReadOwnParams, including
    /// the "poorly unitary" warning.
    pub fn read_own_params(
        &self,
        ir: &IgesReaderData,
        pr: &mut ParamReader,
    ) -> Option<IgesSolidOfLinearExtrusion> {
        let curve = pr.read_entity(ir);
        if curve.is_none() {
            pr.add_fail("Curve Entity : incorrect reference");
        }
        let length = pr.read_real();
        if length.is_none() {
            pr.add_fail("Length of extrusion : not a real");
        }

        let mut direction = Xyz::new(0.0, 0.0, 1.0);
        if pr.defined_else_skip() {
            if let Some(v) = pr.read_real() {
                direction.x = v;
            } else {
                pr.add_fail("Extrusion direction (I) : not a real");
            }
        } else {
            direction.x = 0.0;
        }
        if pr.defined_else_skip() {
            if let Some(v) = pr.read_real() {
                direction.y = v;
            } else {
                pr.add_fail("Extrusion direction (J) : not a real");
            }
        } else {
            direction.y = 0.0;
        }
        if pr.defined_else_skip() {
            if let Some(v) = pr.read_real() {
                direction.z = v;
            } else {
                pr.add_fail("Extrusion direction (K) : not a real");
            }
        } else {
            direction.z = 1.0;
        }

        let (curve, length) = match (curve, length) {
            (Some(c), Some(l)) => (c, l),
            _ => return None,
        };

        let ent = IgesSolidOfLinearExtrusion {
            curve,
            extrusion_length: length,
            direction,
        };

        // Warning if the direction was not unitary (eps 1e-5).
        let eps = 1.0e-5;
        if direction.modulus() > 0.0 && !direction.is_equal(&ent.extrusion_direction(), eps) {
            pr.add_warning("Extrusion Direction poorly unitary, normalized");
        }
        Some(ent)
    }

    /// Writes own parameters: curve, length, direction X/Y/Z (normalized).
    /// Mirrors WriteOwnParams.
    pub fn write_own_params(&self, ent: &IgesSolidOfLinearExtrusion, iw: &mut IgesWriter) {
        iw.send_entity(ent.curve());
        iw.send_real(ent.extrusion_length());
        let dir = ent.extrusion_direction();
        iw.send_real(dir.x);
        iw.send_real(dir.y);
        iw.send_real(dir.z);
    }

    /// Lists the entities shared: the curve entity.
    pub fn own_shared(&self, ent: &IgesSolidOfLinearExtrusion) -> Vec<IgesEntity> {
        vec![ent.curve().clone()]
    }

    /// Copies specific parameters, transferring the curve entity.
    pub fn own_copy(
        &self,
        another: &IgesSolidOfLinearExtrusion,
        tc: &mut CopyTool,
    ) -> IgesSolidOfLinearExtrusion {
        IgesSolidOfLinearExtrusion {
            curve: tc.transferred(another.curve()),
            extrusion_length: another.extrusion_length(),
            direction: another.direction,
        }
    }

    /// Returns specific DirChecker: type 164, form 0.
    pub fn dir_checker(&self, _ent: &IgesSolidOfLinearExtrusion) -> IgesDirChecker {
        let mut dc = IgesDirChecker::new(164, 0, 0);
        dc.structure = DirDef::Void;
        dc.line_font = DirDef::Any;
        dc.color = DirDef::Any;
        dc.use_flag_required = Some(0);
        dc.hierarchy_status_ignored = true;
        dc
    }

    /// Performs specific semantic check: extrusion length must be positive.
    pub fn own_check(&self, ent: &IgesSolidOfLinearExtrusion) -> Vec<String> {
        let mut fails = Vec::new();
        if ent.extrusion_length() <= 0.0 {
            fails.push("Length of extrusion : Not Positive".to_string());
        }
        fails
    }

    /// Dump of specific parameters. Mirrors OwnDump.
    pub fn own_dump(&self, ent: &IgesSolidOfLinearExtrusion, level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_SolidOfLinearExtrusion\n");
        s.push_str("Curve entity        : ");
        s.push_str(&format!("entity #{}", ent.curve().number));
        s.push('\n');
        s.push_str(&format!("Extrusion length    : {}\n", ent.extrusion_length()));
        s.push_str("Extrusion direction : ");
        if level > 1 {
            let d = ent.extrusion_direction();
            s.push_str(&format!("({},{},{})", d.x, d.y, d.z));
        }
        s.push('\n');
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CURVE_TYPE: i32 = 100; // circular arc, e.g.

    fn sample() -> (IgesReaderData, IgesSolidOfLinearExtrusion) {
        let mut ir = IgesReaderData::new();
        let num = ir.add_entity(CURVE_TYPE);
        let ent = IgesSolidOfLinearExtrusion {
            curve: ir.entity(num).unwrap().clone(),
            extrusion_length: 5.0,
            direction: Xyz::new(0.0, 0.0, 1.0),
        };
        (ir, ent)
    }

    #[test]
    fn test_tool_creation() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        assert_eq!(tool, IgesSolidToolSolidOfLinearExtrusion);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let (_ir, ent) = sample();
        let dc = tool.dir_checker(&ent);
        assert_eq!(dc.type_number, 164);
        assert_eq!(dc.form1, 0);
        assert_eq!(dc.form2, 0);
        assert_eq!(dc.structure, DirDef::Void);
        assert_eq!(dc.line_font, DirDef::Any);
        assert_eq!(dc.color, DirDef::Any);
        assert_eq!(dc.use_flag_required, Some(0));
        assert!(dc.hierarchy_status_ignored);
        assert!(dc.is_type_and_form_ok(164, 0));
        assert!(!dc.is_type_and_form_ok(164, 1));
    }

    #[test]
    fn test_read_own_params_full() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let (ir, _) = sample();
        let mut pr = ParamReader::new(vec![
            IgesParam::Entity(1),
            IgesParam::Real(5.0),
            IgesParam::Real(0.0),
            IgesParam::Real(0.0),
            IgesParam::Real(1.0),
        ]);
        let ent = tool.read_own_params(&ir, &mut pr).expect("should read");
        assert!(!pr.has_failed(), "fails: {:?}", pr.fails());
        assert!(pr.warnings().is_empty());
        assert_eq!(ent.extrusion_length(), 5.0);
        assert_eq!(ent.extrusion_direction(), Xyz::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_read_own_params_defaults() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let (ir, _) = sample();
        // Direction components all defaulted -> (0, 0, 1)
        let mut pr = ParamReader::new(vec![
            IgesParam::Entity(1),
            IgesParam::Real(2.5),
            IgesParam::Void,
            IgesParam::Void,
            IgesParam::Void,
        ]);
        let ent = tool.read_own_params(&ir, &mut pr).expect("should read");
        assert!(!pr.has_failed());
        assert_eq!(ent.direction, Xyz::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_read_own_params_non_unitary_direction_warns() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let (ir, _) = sample();
        let mut pr = ParamReader::new(vec![
            IgesParam::Entity(1),
            IgesParam::Real(1.0),
            IgesParam::Real(0.0),
            IgesParam::Real(0.0),
            IgesParam::Real(2.0), // length 2, not unitary
        ]);
        let ent = tool.read_own_params(&ir, &mut pr).expect("should read");
        assert!(!pr.has_failed());
        assert_eq!(pr.warnings().len(), 1);
        assert!(pr.warnings()[0].contains("poorly unitary"));
        // Normalized on access:
        assert_eq!(ent.extrusion_direction(), Xyz::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_read_own_params_missing_length_fails() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let (ir, _) = sample();
        let mut pr = ParamReader::new(vec![IgesParam::Entity(1)]);
        let ent = tool.read_own_params(&ir, &mut pr);
        assert!(ent.is_none());
        assert!(pr.has_failed());
    }

    #[test]
    fn test_write_read_roundtrip() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let (ir, original) = sample();
        let mut iw = IgesWriter::new();
        tool.write_own_params(&original, &mut iw);
        assert_eq!(iw.params().len(), 5);

        let mut pr = ParamReader::new(iw.into_params());
        let reread = tool.read_own_params(&ir, &mut pr).expect("should read");
        assert!(!pr.has_failed());
        assert_eq!(reread, original);
    }

    #[test]
    fn test_own_shared() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let (_ir, ent) = sample();
        let shared = tool.own_shared(&ent);
        assert_eq!(shared.len(), 1);
        assert_eq!(shared[0], ent.curve);
    }

    #[test]
    fn test_own_check() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let (_ir, mut ent) = sample();
        assert!(tool.own_check(&ent).is_empty());
        ent.extrusion_length = 0.0;
        let fails = tool.own_check(&ent);
        assert_eq!(fails.len(), 1);
        assert!(fails[0].contains("Not Positive"));
    }

    #[test]
    fn test_own_copy() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let source = IgesSolidOfLinearExtrusion {
            curve: IgesEntity { number: 7, type_number: CURVE_TYPE },
            extrusion_length: 3.0,
            direction: Xyz::new(1.0, 0.0, 0.0),
        };
        let mut tc = CopyTool::new();
        let copied = tool.own_copy(&source, &mut tc);
        assert_eq!(copied.curve.number, 1); // renumbered
        assert_eq!(copied.curve.type_number, CURVE_TYPE);
        assert_eq!(copied.extrusion_length, 3.0);
        assert_eq!(copied.direction, source.direction);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolSolidOfLinearExtrusion::new();
        let (_ir, ent) = sample();
        let dump = tool.own_dump(&ent, 5);
        assert!(dump.starts_with("IGESSolid_SolidOfLinearExtrusion\n"));
        assert!(dump.contains("Curve entity"));
        assert!(dump.contains("Extrusion length    : 5"));
        assert!(dump.contains("Extrusion direction :"));
    }
}
