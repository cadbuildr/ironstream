// FILE: iges_solid_tool_solid_instance.rs
// occt: IGESSolid_ToolSolidInstance

//! Tool to work on a SolidInstance (IGES type 430, form 0-1). Called by
//! various Modules (ReadWriteModule, GeneralModule, SpecificModule).
//!
//! External plumbing (parameter stream, reader data, writer, copy tool) is
//! modelled with local helper types; the tool behaviour itself follows
//! IGESSolid_ToolSolidInstance.cxx.

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
    pub graphics_ignored: Option<i32>,
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
            graphics_ignored: None,
        }
    }

    /// True if `(type_number, form)` is acceptable for this checker.
    pub fn is_type_and_form_ok(&self, type_number: i32, form: i32) -> bool {
        type_number == self.type_number && form >= self.form1 && form <= self.form2
    }
}

// ---------------------------------------------------------------------------
// The SolidInstance entity data (IGESSolid_SolidInstance)
// ---------------------------------------------------------------------------

/// Data of an IGES SolidInstance entity: one referenced solid entity.
#[derive(Clone, Debug, PartialEq)]
pub struct IgesSolidInstance {
    pub entity: IgesEntity,
}

impl IgesSolidInstance {
    pub fn entity(&self) -> &IgesEntity {
        &self.entity
    }
}

// ---------------------------------------------------------------------------
// The tool itself (IGESSolid_ToolSolidInstance)
// ---------------------------------------------------------------------------

/// Tool to work on a SolidInstance entity.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgesSolidToolSolidInstance;

impl IgesSolidToolSolidInstance {
    /// Returns a ToolSolidInstance, ready to work.
    pub fn new() -> Self {
        Self
    }

    /// Reads own parameters: the referenced solid entity.
    /// Mirrors ReadOwnParams; returns None when the reference is bad.
    pub fn read_own_params(
        &self,
        ir: &IgesReaderData,
        pr: &mut ParamReader,
    ) -> Option<IgesSolidInstance> {
        match pr.read_entity(ir) {
            Some(entity) => Some(IgesSolidInstance { entity }),
            None => {
                pr.add_fail("Solid Entity : incorrect reference");
                None
            }
        }
    }

    /// Writes own parameters: the referenced entity. Mirrors WriteOwnParams.
    pub fn write_own_params(&self, ent: &IgesSolidInstance, iw: &mut IgesWriter) {
        iw.send_entity(ent.entity());
    }

    /// Lists the entities shared by a SolidInstance: the referenced entity.
    pub fn own_shared(&self, ent: &IgesSolidInstance) -> Vec<IgesEntity> {
        vec![ent.entity().clone()]
    }

    /// Copies specific parameters, transferring the referenced entity.
    pub fn own_copy(&self, another: &IgesSolidInstance, tc: &mut CopyTool) -> IgesSolidInstance {
        IgesSolidInstance { entity: tc.transferred(another.entity()) }
    }

    /// Returns specific DirChecker: type 430, forms 0-1.
    pub fn dir_checker(&self, _ent: &IgesSolidInstance) -> IgesDirChecker {
        let mut dc = IgesDirChecker::new(430, 0, 1);
        dc.structure = DirDef::Void;
        dc.line_font = DirDef::Any;
        dc.color = DirDef::Any;
        dc.graphics_ignored = Some(1);
        dc
    }

    /// Performs specific semantic check: SolidInstance has none.
    pub fn own_check(&self, _ent: &IgesSolidInstance) -> Vec<String> {
        Vec::new()
    }

    /// Dump of specific parameters. Mirrors OwnDump.
    pub fn own_dump(&self, ent: &IgesSolidInstance, level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_SolidInstance\n");
        s.push_str("Solid entity : ");
        let sublevel = if level <= 4 { 0 } else { 1 };
        if sublevel > 0 {
            s.push_str(&format!(
                "entity #{} type {}",
                ent.entity().number,
                ent.entity().type_number
            ));
        } else {
            s.push_str(&format!("entity #{}", ent.entity().number));
        }
        s.push('\n');
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOLID_TYPE: i32 = 186;

    fn sample() -> (IgesReaderData, IgesSolidInstance) {
        let mut ir = IgesReaderData::new();
        let num = ir.add_entity(SOLID_TYPE);
        let ent = IgesSolidInstance { entity: ir.entity(num).unwrap().clone() };
        (ir, ent)
    }

    #[test]
    fn test_tool_creation() {
        let tool = IgesSolidToolSolidInstance::new();
        assert_eq!(tool, IgesSolidToolSolidInstance);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolSolidInstance::new();
        let (_ir, ent) = sample();
        let dc = tool.dir_checker(&ent);
        assert_eq!(dc.type_number, 430);
        assert_eq!(dc.form1, 0);
        assert_eq!(dc.form2, 1);
        assert_eq!(dc.structure, DirDef::Void);
        assert_eq!(dc.line_font, DirDef::Any);
        assert_eq!(dc.color, DirDef::Any);
        assert_eq!(dc.graphics_ignored, Some(1));
        assert!(dc.is_type_and_form_ok(430, 0));
        assert!(dc.is_type_and_form_ok(430, 1));
        assert!(!dc.is_type_and_form_ok(430, 2));
        assert!(!dc.is_type_and_form_ok(184, 0));
    }

    #[test]
    fn test_read_own_params() {
        let tool = IgesSolidToolSolidInstance::new();
        let (ir, expected) = sample();
        let mut pr = ParamReader::new(vec![IgesParam::Entity(1)]);
        let ent = tool.read_own_params(&ir, &mut pr).expect("should read");
        assert!(!pr.has_failed());
        assert_eq!(ent, expected);
    }

    #[test]
    fn test_read_own_params_bad_reference_fails() {
        let tool = IgesSolidToolSolidInstance::new();
        let ir = IgesReaderData::new();
        let mut pr = ParamReader::new(vec![IgesParam::Entity(99)]);
        let ent = tool.read_own_params(&ir, &mut pr);
        assert!(ent.is_none());
        assert!(pr.has_failed());
    }

    #[test]
    fn test_write_read_roundtrip() {
        let tool = IgesSolidToolSolidInstance::new();
        let (ir, original) = sample();
        let mut iw = IgesWriter::new();
        tool.write_own_params(&original, &mut iw);
        assert_eq!(iw.params(), &[IgesParam::Entity(1)]);

        let mut pr = ParamReader::new(iw.into_params());
        let reread = tool.read_own_params(&ir, &mut pr).expect("should read");
        assert!(!pr.has_failed());
        assert_eq!(reread, original);
    }

    #[test]
    fn test_own_shared() {
        let tool = IgesSolidToolSolidInstance::new();
        let (_ir, ent) = sample();
        let shared = tool.own_shared(&ent);
        assert_eq!(shared.len(), 1);
        assert_eq!(shared[0], ent.entity);
    }

    #[test]
    fn test_own_check_is_empty() {
        let tool = IgesSolidToolSolidInstance::new();
        let (_ir, ent) = sample();
        assert!(tool.own_check(&ent).is_empty());
    }

    #[test]
    fn test_own_copy() {
        let tool = IgesSolidToolSolidInstance::new();
        let source = IgesSolidInstance {
            entity: IgesEntity { number: 42, type_number: SOLID_TYPE },
        };
        let mut tc = CopyTool::new();
        let copied = tool.own_copy(&source, &mut tc);
        // Transferred entity is renumbered in the target model.
        assert_eq!(copied.entity.number, 1);
        assert_eq!(copied.entity.type_number, SOLID_TYPE);
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolSolidInstance::new();
        let (_ir, ent) = sample();
        let dump = tool.own_dump(&ent, 5);
        assert!(dump.starts_with("IGESSolid_SolidInstance\n"));
        assert!(dump.contains("Solid entity :"));
        assert!(dump.contains("entity #1"));
    }
}
