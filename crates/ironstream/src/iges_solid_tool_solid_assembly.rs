// FILE: iges_solid_tool_solid_assembly.rs
// occt: IGESSolid_ToolSolidAssembly

//! Tool to work on a SolidAssembly (IGES type 184, form 0-1). Called by
//! various Modules (ReadWriteModule, GeneralModule, SpecificModule).
//!
//! External plumbing (parameter stream, reader data, writer, copy tool) is
//! modelled with local helper types; the tool behaviour itself follows
//! IGESSolid_ToolSolidAssembly.cxx.

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

/// IGES type number of a TransformationMatrix entity.
pub const TRANSFORMATION_MATRIX_TYPE: i32 = 124;

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

    /// Reads an entity reference where a null (0) reference is accepted
    /// (OCCT ReadEntity with canBeNull = true). Returns:
    /// - Some(Some(ent)) for a resolved reference
    /// - Some(None) for an explicit null / void parameter
    /// - None for an unresolved or malformed parameter
    pub fn read_entity_or_null(&mut self, ir: &IgesReaderData) -> Option<Option<IgesEntity>> {
        match self.current() {
            Some(IgesParam::Entity(0)) | Some(IgesParam::Void) => {
                self.advance();
                Some(None)
            }
            Some(IgesParam::Entity(num)) => {
                let num = *num;
                self.advance();
                ir.entity(num).cloned().map(Some)
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

    pub fn send_integer(&mut self, v: i32) {
        self.params.push(IgesParam::Integer(v));
    }

    pub fn send_entity(&mut self, ent: &IgesEntity) {
        self.params.push(IgesParam::Entity(ent.number));
    }

    /// Sends an optional entity; a null handle is written as reference 0.
    pub fn send_entity_or_null(&mut self, ent: Option<&IgesEntity>) {
        match ent {
            Some(e) => self.params.push(IgesParam::Entity(e.number)),
            None => self.params.push(IgesParam::Entity(0)),
        }
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
            use_flag_required: None,
            graphics_ignored: None,
        }
    }

    /// True if `(type_number, form)` is acceptable for this checker.
    pub fn is_type_and_form_ok(&self, type_number: i32, form: i32) -> bool {
        type_number == self.type_number && form >= self.form1 && form <= self.form2
    }
}

// ---------------------------------------------------------------------------
// The SolidAssembly entity data (IGESSolid_SolidAssembly)
// ---------------------------------------------------------------------------

/// Data of an IGES SolidAssembly entity: items with transformation matrices.
/// A matrix may be null (identity / no transformation).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IgesSolidAssembly {
    pub items: Vec<IgesEntity>,
    pub matrices: Vec<Option<IgesEntity>>,
}

impl IgesSolidAssembly {
    pub fn nb_items(&self) -> usize {
        self.items.len()
    }

    pub fn item(&self, index1: usize) -> &IgesEntity {
        &self.items[index1 - 1]
    }

    pub fn transf_matrix(&self, index1: usize) -> Option<&IgesEntity> {
        self.matrices[index1 - 1].as_ref()
    }
}

// ---------------------------------------------------------------------------
// The tool itself (IGESSolid_ToolSolidAssembly)
// ---------------------------------------------------------------------------

/// Tool to work on a SolidAssembly entity.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgesSolidToolSolidAssembly;

impl IgesSolidToolSolidAssembly {
    /// Returns a ToolSolidAssembly, ready to work.
    pub fn new() -> Self {
        Self
    }

    /// Reads own parameters: number of items, then all item entities, then
    /// all transformation matrices (which may be null). Mirrors ReadOwnParams.
    pub fn read_own_params(
        &self,
        ir: &IgesReaderData,
        pr: &mut ParamReader,
    ) -> IgesSolidAssembly {
        let mut items: Vec<IgesEntity> = Vec::new();
        let mut matrices: Vec<Option<IgesEntity>> = Vec::new();

        let nbitems = pr.read_integer();
        match nbitems {
            Some(n) if n > 0 => {
                for _ in 0..n {
                    match pr.read_entity(ir) {
                        Some(item) => items.push(item),
                        None => pr.add_fail("Solid assembly items : incorrect reference"),
                    }
                }
                for _ in 0..n {
                    match pr.read_entity_or_null(ir) {
                        Some(matrix) => {
                            if let Some(m) = &matrix {
                                if m.type_number != TRANSFORMATION_MATRIX_TYPE {
                                    pr.add_fail("Matrices : incorrect type");
                                    matrices.push(None);
                                    continue;
                                }
                            }
                            matrices.push(matrix);
                        }
                        None => pr.add_fail("Matrices : incorrect reference"),
                    }
                }
            }
            _ => {
                pr.add_fail("Number of Items : Not Positive");
            }
        }

        IgesSolidAssembly { items, matrices }
    }

    /// Writes own parameters: item count, all items, then all matrices.
    /// Mirrors WriteOwnParams.
    pub fn write_own_params(&self, ent: &IgesSolidAssembly, iw: &mut IgesWriter) {
        let nbitems = ent.nb_items();
        iw.send_integer(nbitems as i32);
        for i in 1..=nbitems {
            iw.send_entity(ent.item(i));
        }
        for i in 1..=nbitems {
            iw.send_entity_or_null(ent.transf_matrix(i));
        }
    }

    /// Lists the entities shared by a SolidAssembly: items then matrices.
    pub fn own_shared(&self, ent: &IgesSolidAssembly) -> Vec<IgesEntity> {
        let mut shared = Vec::new();
        for i in 1..=ent.nb_items() {
            shared.push(ent.item(i).clone());
        }
        for i in 1..=ent.nb_items() {
            if let Some(m) = ent.transf_matrix(i) {
                shared.push(m.clone());
            }
        }
        shared
    }

    /// Copies specific parameters, transferring items and matrices.
    pub fn own_copy(&self, another: &IgesSolidAssembly, tc: &mut CopyTool) -> IgesSolidAssembly {
        let nbitems = another.nb_items();
        let mut items = Vec::with_capacity(nbitems);
        let mut matrices = Vec::with_capacity(nbitems);
        for i in 1..=nbitems {
            items.push(tc.transferred(another.item(i)));
        }
        for i in 1..=nbitems {
            matrices.push(another.transf_matrix(i).map(|m| tc.transferred(m)));
        }
        IgesSolidAssembly { items, matrices }
    }

    /// Returns specific DirChecker: type 184, forms 0-1.
    pub fn dir_checker(&self, _ent: &IgesSolidAssembly) -> IgesDirChecker {
        let mut dc = IgesDirChecker::new(184, 0, 1);
        dc.structure = DirDef::Void;
        dc.line_font = DirDef::Any;
        dc.color = DirDef::Any;
        dc.use_flag_required = Some(2);
        dc.graphics_ignored = Some(1);
        dc
    }

    /// Performs specific semantic check: SolidAssembly has none.
    pub fn own_check(&self, _ent: &IgesSolidAssembly) -> Vec<String> {
        Vec::new()
    }

    /// Dump of specific parameters. Mirrors OwnDump.
    pub fn own_dump(&self, ent: &IgesSolidAssembly, level: i32) -> String {
        let mut s = String::new();
        s.push_str("IGESSolid_SolidAssembly\n");
        s.push_str("Items : ");
        s.push_str(&format!("(count : {})", ent.nb_items()));
        if level > 4 {
            for i in 1..=ent.nb_items() {
                s.push_str(&format!(" [{}]: entity #{}", i, ent.item(i).number));
            }
        }
        s.push('\n');
        s.push_str("Matrices : ");
        s.push_str(&format!("(count : {})", ent.nb_items()));
        if level > 4 {
            for i in 1..=ent.nb_items() {
                match ent.transf_matrix(i) {
                    Some(m) => s.push_str(&format!(" [{}]: entity #{}", i, m.number)),
                    None => s.push_str(&format!(" [{}]: (null)", i)),
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

    const SOLID_TYPE: i32 = 186; // e.g. ManifoldSolid item

    fn sample() -> (IgesReaderData, IgesSolidAssembly) {
        let mut ir = IgesReaderData::new();
        let i1 = ir.add_entity(SOLID_TYPE);
        let i2 = ir.add_entity(SOLID_TYPE);
        let m1 = ir.add_entity(TRANSFORMATION_MATRIX_TYPE);
        let ent = IgesSolidAssembly {
            items: vec![
                ir.entity(i1).unwrap().clone(),
                ir.entity(i2).unwrap().clone(),
            ],
            matrices: vec![Some(ir.entity(m1).unwrap().clone()), None],
        };
        (ir, ent)
    }

    #[test]
    fn test_tool_creation() {
        let tool = IgesSolidToolSolidAssembly::new();
        assert_eq!(tool, IgesSolidToolSolidAssembly);
    }

    #[test]
    fn test_dir_checker_configuration() {
        let tool = IgesSolidToolSolidAssembly::new();
        let dc = tool.dir_checker(&IgesSolidAssembly::default());
        assert_eq!(dc.type_number, 184);
        assert_eq!(dc.form1, 0);
        assert_eq!(dc.form2, 1);
        assert_eq!(dc.structure, DirDef::Void);
        assert_eq!(dc.line_font, DirDef::Any);
        assert_eq!(dc.color, DirDef::Any);
        assert_eq!(dc.use_flag_required, Some(2));
        assert_eq!(dc.graphics_ignored, Some(1));
        assert!(dc.is_type_and_form_ok(184, 0));
        assert!(dc.is_type_and_form_ok(184, 1));
        assert!(!dc.is_type_and_form_ok(184, 2));
    }

    #[test]
    fn test_read_own_params() {
        let tool = IgesSolidToolSolidAssembly::new();
        let (ir, expected) = sample();
        let mut pr = ParamReader::new(vec![
            IgesParam::Integer(2),
            IgesParam::Entity(1),
            IgesParam::Entity(2),
            IgesParam::Entity(3),
            IgesParam::Entity(0), // null matrix for second item
        ]);
        let ent = tool.read_own_params(&ir, &mut pr);
        assert!(!pr.has_failed(), "fails: {:?}", pr.fails());
        assert_eq!(ent, expected);
        assert_eq!(ent.nb_items(), 2);
        assert!(ent.transf_matrix(1).is_some());
        assert!(ent.transf_matrix(2).is_none());
    }

    #[test]
    fn test_read_own_params_bad_count_fails() {
        let tool = IgesSolidToolSolidAssembly::new();
        let ir = IgesReaderData::new();
        let mut pr = ParamReader::new(vec![IgesParam::Integer(-1)]);
        let ent = tool.read_own_params(&ir, &mut pr);
        assert!(pr.has_failed());
        assert!(pr.fails()[0].contains("Not Positive"));
        assert_eq!(ent.nb_items(), 0);
    }

    #[test]
    fn test_read_own_params_wrong_matrix_type_fails() {
        let tool = IgesSolidToolSolidAssembly::new();
        let mut ir = IgesReaderData::new();
        let i1 = ir.add_entity(SOLID_TYPE);
        let bad = ir.add_entity(SOLID_TYPE); // not a transformation matrix
        let mut pr = ParamReader::new(vec![
            IgesParam::Integer(1),
            IgesParam::Entity(i1),
            IgesParam::Entity(bad),
        ]);
        let _ent = tool.read_own_params(&ir, &mut pr);
        assert!(pr.has_failed());
        assert!(pr.fails()[0].contains("Matrices"));
    }

    #[test]
    fn test_write_read_roundtrip() {
        let tool = IgesSolidToolSolidAssembly::new();
        let (ir, original) = sample();
        let mut iw = IgesWriter::new();
        tool.write_own_params(&original, &mut iw);
        assert_eq!(iw.params()[0], IgesParam::Integer(2));
        assert_eq!(iw.params().len(), 1 + 2 + 2);
        // Null matrix is written as reference 0.
        assert_eq!(iw.params()[4], IgesParam::Entity(0));

        let mut pr = ParamReader::new(iw.into_params());
        let reread = tool.read_own_params(&ir, &mut pr);
        assert!(!pr.has_failed());
        assert_eq!(reread, original);
    }

    #[test]
    fn test_own_shared() {
        let tool = IgesSolidToolSolidAssembly::new();
        let (_ir, ent) = sample();
        let shared = tool.own_shared(&ent);
        // 2 items + 1 non-null matrix
        assert_eq!(shared.len(), 3);
        assert_eq!(shared[0].number, 1);
        assert_eq!(shared[1].number, 2);
        assert_eq!(shared[2].number, 3);
    }

    #[test]
    fn test_own_check_is_empty() {
        let tool = IgesSolidToolSolidAssembly::new();
        let (_ir, ent) = sample();
        assert!(tool.own_check(&ent).is_empty());
        assert!(tool.own_check(&IgesSolidAssembly::default()).is_empty());
    }

    #[test]
    fn test_own_copy() {
        let tool = IgesSolidToolSolidAssembly::new();
        let (_ir, source) = sample();
        let mut tc = CopyTool::new();
        let copied = tool.own_copy(&source, &mut tc);
        assert_eq!(copied.nb_items(), 2);
        assert_eq!(copied.item(1).type_number, SOLID_TYPE);
        assert_eq!(copied.item(2).type_number, SOLID_TYPE);
        assert!(copied.transf_matrix(1).is_some());
        assert_eq!(
            copied.transf_matrix(1).unwrap().type_number,
            TRANSFORMATION_MATRIX_TYPE
        );
        assert!(copied.transf_matrix(2).is_none());
    }

    #[test]
    fn test_own_dump() {
        let tool = IgesSolidToolSolidAssembly::new();
        let (_ir, ent) = sample();
        let dump = tool.own_dump(&ent, 5);
        assert!(dump.starts_with("IGESSolid_SolidAssembly\n"));
        assert!(dump.contains("Items :"));
        assert!(dump.contains("Matrices :"));
        assert!(dump.contains("(null)"));
    }
}
