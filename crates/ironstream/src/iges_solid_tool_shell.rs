// FILE: iges_solid_tool_shell.rs
// occt: IGESSolid_ToolShell

//! Tool to work on a Shell (IGES type 514, forms 1-2). Called by various
//! Modules (ReadWriteModule, GeneralModule, SpecificModule).
//!
//! External plumbing (parameter stream, reader data, writer, copy tool) is
//! modelled with local helper types; the tool behaviour itself follows
//! IGESSolid_ToolShell.cxx.

// ---------------------------------------------------------------------------
// Local model of the IGES parameter stream (external plumbing)
// ---------------------------------------------------------------------------

/// One parameter of an IGES parameter section.
#[derive(Clone, Debug, PartialEq)]
pub enum IgesParam {
    Integer(i32),
    Real(f64),
    /// Reference to an entity, by its number in the reader data.
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

    pub fn read_integer(&mut self) -> Option<i32> {
        match self.current() {
            Some(IgesParam::Integer(v)) => {
                let v = *v;
                self.advance();
                Some(v)
            }
            _ => {
                self.advance();
                None
            }
        }
    }

    /// Reads a boolean stored as integer 0/1.
    pub fn read_boolean(&mut self) -> Option<bool> {
        match self.current() {
            Some(IgesParam::Integer(v)) => {
                let v = *v;
                self.advance();
                Some(v != 0)
            }
            _ => {
                self.advance();
                None
            }
        }
    }

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

    pub fn send_fail(&mut self, msg: &str) {
        self.fails.push(msg.to_string());
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

    pub fn send_integer(&mut self, v: i32) {
        self.params.push(IgesParam::Integer(v));
    }

    pub fn send_boolean(&mut self, v: bool) {
        self.params.push(IgesParam::Integer(if v { 1 } else { 0 }));
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
    pub line_weight: DirDef,
    pub color: DirDef,
    pub subordinate_status_required: Option<i32>,
}

impl IgesDirChecker {
    pub fn new(type_number: i32, form1: i32, form2: i32) -> Self {
        Self {
            type_number,
            form1,
            form2,
            structure: DirDef::Any,
            line_font: DirDef::Any,
            line_weight: DirDef::Any,
            color: DirDef::Any,
            subordinate_status_required: None,
        }
    }

    /// True if `(type_number, form)` is acceptable for this checker.
    pub fn is_type_and_form_ok(&self, type_number: i32, form: i32) -> bool {
        type_number == self.type_number && form >= self.form1 && form <= self.form2
    }
}

// ---------------------------------------------------------------------------
// The Shell entity data (IGESSolid_Shell)
// ---------------------------------------------------------------------------

/// Data of an IGES Shell entity: faces with orientation flags.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IgesSolidShell {
    pub faces: Vec<IgesEntity>,
    pub orientations: Vec<bool>,
}

impl IgesSolidShell {
    pub fn nb_faces(&self) -> usize {
        self.faces.len()
    }

    pub fn face(&self, index1: usize) -> &IgesEntity {
        &self.faces[index1 - 1]
    }

    pub fn orientation(&self, index1: usize) -> bool {
        self.orientations[index1 - 1]
    }
}

// ---------------------------------------------------------------------------
// The tool itself (IGESSolid_ToolShell)
// ---------------------------------------------------------------------------

/// Tool to work on a Shell entity.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgesSolidToolShell;

impl IgesSolidToolShell {
    /// Returns a ToolShell, ready to work.
    pub fn new() -> Self {
        Self
    }

    /// Reads own parameters: number of faces, then for each face the face
    /// entity and its orientation flag. Mirrors ReadOwnParams.
    pub fn read_own_params(
        &self,
        ir: &IgesReaderData,
        pr: &mut ParamReader,
    ) -> IgesSolidShell {
        let mut faces: Vec<IgesEntity> = Vec::new();
        let mut orientations: Vec<bool> = Vec::new();

        let nbfaces = pr.read_integer();
        match nbfaces {
            Some(n) if n > 0 => {
                for _ in 0..n {
                    match pr.read_entity(ir) {
                        Some(face) => faces.push(face),
                        None => pr.send_fail("XSTEP_201: Face : incorrect reference"),
                    }
                    match pr.read_boolean() {
                        Some(flag) => orientations.push(flag),
                        None => pr.send_fail("XSTEP_180: Orientation flag : not a boolean"),
                    }
                }
            }
            _ => {
                // XSTEP_200 : Number of faces not positive or absent
                pr.send_fail("XSTEP_200: Number of faces : not positive");
            }
        }

        IgesSolidShell { faces, orientations }
    }

    /// Writes own parameters: face count, then per face the entity and the
    /// orientation flag. Mirrors WriteOwnParams.
    pub fn write_own_params(&self, ent: &IgesSolidShell, iw: &mut IgesWriter) {
        let nbfaces = ent.nb_faces();
        iw.send_integer(nbfaces as i32);
        for i in 1..=nbfaces {
            iw.send_entity(ent.face(i));
            iw.send_boolean(ent.orientation(i));
        }
    }

    /// Lists the entities shared by a Shell: its faces.
    pub fn own_shared(&self, ent: &IgesSolidShell) -> Vec<IgesEntity> {
        ent.faces.clone()
    }

    /// Copies specific parameters, transferring the face entities.
    pub fn own_copy(&self, another: &IgesSolidShell, tc: &mut CopyTool) -> IgesSolidShell {
        let nbfaces = another.nb_faces();
        let mut faces = Vec::with_capacity(nbfaces);
        let mut orientations = Vec::with_capacity(nbfaces);
        for i in 1..=nbfaces {
            faces.push(tc.transferred(another.face(i)));
            orientations.push(another.orientation(i));
        }
        IgesSolidShell { faces, orientations }
    }

    /// Returns specific DirChecker: type 514, forms 1-2.
    pub fn dir_checker(&self, _ent: &IgesSolidShell) -> IgesDirChecker {
        let mut dc = IgesDirChecker::new(514, 1, 2);
        dc.structure = DirDef::Void;
        dc.line_font = DirDef::Void;
        dc.line_weight = DirDef::Void;
        dc.color = DirDef::Void;
        dc.subordinate_status_required = Some(1);
        dc
    }

    /// Performs specific semantic check; returns fail messages.
    pub fn own_check(&self, ent: &IgesSolidShell) -> Vec<String> {
        let mut fails = Vec::new();
        if ent.nb_faces() == 0 {
            fails.push("XSTEP_200: Number of faces : not positive".to_string());
        }
        fails
    }

    /// Dump of specific parameters. Mirrors OwnDump.
    pub fn own_dump(&self, ent: &IgesSolidShell, level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_Shell\n");
        let upper = ent.nb_faces();
        let _sublevel = if level <= 4 { 0 } else { 1 };

        s.push_str("Faces :\nOrientation flags : ");
        if level > 0 {
            s.push_str(&format!("(count : {})", upper));
        }
        s.push('\n');
        if level > 4 {
            s.push_str("[\n");
            for i in 1..=upper {
                s.push_str(&format!("[{}]:  Face : ", i));
                s.push_str(&format!("entity #{} type {}", ent.face(i).number, ent.face(i).type_number));
                s.push_str("  - Orientation flag : ");
                if ent.orientation(i) {
                    s.push_str("True\n");
                } else {
                    s.push_str("False\n");
                }
            }
        }
        s.push('\n');
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FACE_TYPE: i32 = 510; // IGES Face entity type

    fn sample_reader_data() -> (IgesReaderData, Vec<u32>) {
        let mut ir = IgesReaderData::new();
        let nums = vec![ir.add_entity(FACE_TYPE), ir.add_entity(FACE_TYPE)];
        (ir, nums)
    }

    #[test]
    fn test_tool_creation() {
        let tool = IgesSolidToolShell::new();
        assert_eq!(tool, IgesSolidToolShell);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolShell::new();
        let dc = tool.dir_checker(&IgesSolidShell::default());
        assert_eq!(dc.type_number, 514);
        assert_eq!(dc.form1, 1);
        assert_eq!(dc.form2, 2);
        assert_eq!(dc.structure, DirDef::Void);
        assert_eq!(dc.line_font, DirDef::Void);
        assert_eq!(dc.line_weight, DirDef::Void);
        assert_eq!(dc.color, DirDef::Void);
        assert_eq!(dc.subordinate_status_required, Some(1));
        assert!(dc.is_type_and_form_ok(514, 1));
        assert!(dc.is_type_and_form_ok(514, 2));
        assert!(!dc.is_type_and_form_ok(514, 3));
        assert!(!dc.is_type_and_form_ok(510, 1));
    }

    #[test]
    fn test_read_own_params() {
        let tool = IgesSolidToolShell::new();
        let (ir, nums) = sample_reader_data();
        let mut pr = ParamReader::new(vec![
            IgesParam::Integer(2),
            IgesParam::Entity(nums[0]),
            IgesParam::Integer(1),
            IgesParam::Entity(nums[1]),
            IgesParam::Integer(0),
        ]);
        let ent = tool.read_own_params(&ir, &mut pr);
        assert!(!pr.has_failed(), "fails: {:?}", pr.fails());
        assert_eq!(ent.nb_faces(), 2);
        assert_eq!(ent.face(1).number, nums[0]);
        assert_eq!(ent.face(2).number, nums[1]);
        assert_eq!(ent.orientation(1), true);
        assert_eq!(ent.orientation(2), false);
    }

    #[test]
    fn test_read_own_params_bad_count_fails() {
        let tool = IgesSolidToolShell::new();
        let ir = IgesReaderData::new();
        let mut pr = ParamReader::new(vec![IgesParam::Integer(0)]);
        let ent = tool.read_own_params(&ir, &mut pr);
        assert!(pr.has_failed());
        assert_eq!(ent.nb_faces(), 0);
        assert!(pr.fails()[0].contains("XSTEP_200"));
    }

    #[test]
    fn test_write_read_roundtrip() {
        let tool = IgesSolidToolShell::new();
        let (ir, nums) = sample_reader_data();
        let original = IgesSolidShell {
            faces: vec![
                ir.entity(nums[0]).unwrap().clone(),
                ir.entity(nums[1]).unwrap().clone(),
            ],
            orientations: vec![false, true],
        };
        let mut iw = IgesWriter::new();
        tool.write_own_params(&original, &mut iw);
        assert_eq!(iw.params().len(), 1 + 2 * 2);
        assert_eq!(iw.params()[0], IgesParam::Integer(2));

        let mut pr = ParamReader::new(iw.into_params());
        let reread = tool.read_own_params(&ir, &mut pr);
        assert!(!pr.has_failed());
        assert_eq!(reread, original);
    }

    #[test]
    fn test_own_shared() {
        let tool = IgesSolidToolShell::new();
        let (ir, nums) = sample_reader_data();
        let ent = IgesSolidShell {
            faces: vec![
                ir.entity(nums[0]).unwrap().clone(),
                ir.entity(nums[1]).unwrap().clone(),
            ],
            orientations: vec![true, false],
        };
        let shared = tool.own_shared(&ent);
        assert_eq!(shared.len(), 2);
        assert_eq!(shared[0].number, nums[0]);
        assert_eq!(shared[1].number, nums[1]);
    }

    #[test]
    fn test_own_check() {
        let tool = IgesSolidToolShell::new();
        let empty = IgesSolidShell::default();
        assert_eq!(tool.own_check(&empty).len(), 1);

        let ok = IgesSolidShell {
            faces: vec![IgesEntity { number: 1, type_number: FACE_TYPE }],
            orientations: vec![true],
        };
        assert!(tool.own_check(&ok).is_empty());
    }

    #[test]
    fn test_own_copy_transfers_entities() {
        let tool = IgesSolidToolShell::new();
        let source = IgesSolidShell {
            faces: vec![
                IgesEntity { number: 7, type_number: FACE_TYPE },
                IgesEntity { number: 9, type_number: FACE_TYPE },
            ],
            orientations: vec![true, false],
        };
        let mut tc = CopyTool::new();
        let copied = tool.own_copy(&source, &mut tc);
        assert_eq!(copied.nb_faces(), 2);
        // Transferred entities are renumbered in the target model.
        assert_eq!(copied.face(1).number, 1);
        assert_eq!(copied.face(2).number, 2);
        assert_eq!(copied.face(1).type_number, FACE_TYPE);
        assert_eq!(copied.orientations, source.orientations);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolShell::new();
        let ent = IgesSolidShell {
            faces: vec![IgesEntity { number: 3, type_number: FACE_TYPE }],
            orientations: vec![true],
        };
        let dump = tool.own_dump(&ent, 5);
        assert!(dump.starts_with("IGESSolid_Shell\n"));
        assert!(dump.contains("Faces :"));
        assert!(dump.contains("Orientation flags :"));
        assert!(dump.contains("Orientation flag : True"));

        let short = tool.own_dump(&ent, 2);
        assert!(!short.contains("Orientation flag : True"));
    }
}
