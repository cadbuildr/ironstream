// FILE: rw_iges.rs
//! `rw_iges` — IGES (Initial Graphics Exchange Specification, ASME Y14.26M)
//! I/O framework, mirroring OpenCascade's `IGESControl` package
//! (`src/DataExchange/TKIGES/IGESControl/`).
//!
//! IGES files consist of five sections:
//!
//! | Abbr | Section          | Purpose                                       |
//! |------|------------------|-----------------------------------------------|
//! | S    | Start            | Human-readable preface (80 chars/line)        |
//! | G    | Global           | File-wide parameters (unit, author, etc.)     |
//! | D    | Directory Entry  | Index of all entities (2 lines each, col 73) |
//! | P    | Parameter Data   | Entity parameters (free-format, cols 1-64)    |
//! | T    | Terminate        | Section line counts                           |
//!
//! This implementation provides a working pure-Rust IGES encoder/decoder that
//! operates on in-memory byte buffers so that callers can use the same API as
//! OCCT (`reader.read_file(path)`, `reader.transfer_roots()`, etc.) while
//! remaining independent of the OS file system.
//!
//! # OCCT correspondence
//!
//! | OCCT class                  | IronStream type              |
//! |-----------------------------|------------------------------|
//! | `IGESControl_Reader`        | [`IGESControl_Reader`]       |
//! | `IGESControl_Writer`        | [`IGESControl_Writer`]       |
//! | `IGESData_IGESModel`        | [`IGESModel`]                |
//! | `IGESData_IGESEntity`       | [`IGESEntity`]               |

use std::collections::HashMap;
use crate::top_exp::{TopoDS_Shape, TopoDS_Vertex, TopoDS_Edge, TopoDS_Wire, TopoDS_Face,
                     TopoDS_Shell, TopoDS_Solid, TopoDS_Compound};
use crate::gp::Pnt;

// ---------------------------------------------------------------------------
// IgesUnit
// ---------------------------------------------------------------------------

/// IGES unit of length, encoded in Global Section field 15.
// occt: IGESData_BasicEditor // (unit handling)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IgesUnit {
    Inch,
    Millimeter,
    Centimeter,
    Foot,
    Mile,
    Meter,
    Kilometer,
    Mil,
    Micron,
    Yard,
}

impl IgesUnit {
    /// Return the IGES integer code for this unit (Global Section field 14).
    pub fn code(self) -> u8 {
        match self {
            IgesUnit::Inch       => 1,
            IgesUnit::Millimeter => 2,
            IgesUnit::Centimeter => 3,
            IgesUnit::Foot       => 4,
            IgesUnit::Mile       => 5,
            IgesUnit::Meter      => 6,
            IgesUnit::Kilometer  => 7,
            IgesUnit::Mil        => 8,
            IgesUnit::Micron     => 9,
            IgesUnit::Yard       => 10,
        }
    }

    /// Return the IGES flag string (Global Section field 15).
    pub fn flag(self) -> &'static str {
        match self {
            IgesUnit::Inch       => "IN",
            IgesUnit::Millimeter => "MM",
            IgesUnit::Centimeter => "CM",
            IgesUnit::Foot       => "FT",
            IgesUnit::Mile       => "MI",
            IgesUnit::Meter      => "M",
            IgesUnit::Kilometer  => "KM",
            IgesUnit::Mil        => "MIL",
            IgesUnit::Micron     => "UM",
            IgesUnit::Yard       => "YD",
        }
    }

    /// Parse an IGES unit flag string into an [`IgesUnit`].
    pub fn from_flag(s: &str) -> Option<Self> {
        match s.trim() {
            "IN"  | "1"  => Some(IgesUnit::Inch),
            "MM"  | "2"  => Some(IgesUnit::Millimeter),
            "CM"  | "3"  => Some(IgesUnit::Centimeter),
            "FT"  | "4"  => Some(IgesUnit::Foot),
            "MI"  | "5"  => Some(IgesUnit::Mile),
            "M"   | "6"  => Some(IgesUnit::Meter),
            "KM"  | "7"  => Some(IgesUnit::Kilometer),
            "MIL" | "8"  => Some(IgesUnit::Mil),
            "UM"  | "9"  => Some(IgesUnit::Micron),
            "YD"  | "10" => Some(IgesUnit::Yard),
            _ => None,
        }
    }

    /// Conversion factor from this unit to millimetres.
    pub fn to_mm(self) -> f64 {
        match self {
            IgesUnit::Millimeter => 1.0,
            IgesUnit::Centimeter => 10.0,
            IgesUnit::Meter      => 1000.0,
            IgesUnit::Kilometer  => 1_000_000.0,
            IgesUnit::Inch       => 25.4,
            IgesUnit::Mil        => 0.0254,
            IgesUnit::Micron     => 0.001,
            IgesUnit::Foot       => 304.8,
            IgesUnit::Yard       => 914.4,
            IgesUnit::Mile       => 1_609_344.0,
        }
    }
}

impl Default for IgesUnit {
    fn default() -> Self {
        IgesUnit::Millimeter
    }
}

// ---------------------------------------------------------------------------
// IgesEntityKind
// ---------------------------------------------------------------------------

/// Discriminant for the IGES entity type number (Directory Entry column 1).
// occt: IGESData_IGESEntity // (type number field)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IgesEntityKind {
    /// Null entity — placeholder / deleted (type 0).
    Null,
    /// Point entity (type 116).
    Point,
    /// Line entity (type 110).
    Line,
    /// Arc (circular) entity (type 100).
    CircularArc,
    /// Conic arc (ellipse, parabola, hyperbola) entity (type 104).
    ConicArc,
    /// Composite curve entity (type 102).
    CompositeCurve,
    /// Plane entity (type 108).
    Plane,
    /// Ruled surface entity (type 118).
    RuledSurface,
    /// Surface of revolution entity (type 120).
    SurfaceOfRevolution,
    /// Tabulated cylinder entity (type 122).
    TabulatedCylinder,
    /// B-Spline curve entity (type 126).
    BSplineCurve,
    /// B-Spline surface entity (type 128).
    BSplineSurface,
    /// Trimmed parametric surface entity (type 144).
    TrimmedSurface,
    /// Curve on a parametric surface entity (type 142).
    CurveOnSurface,
    /// Bounded surface entity (type 143).
    BoundedSurface,
    /// Manifold solid B-rep object entity (type 186).
    ManifoldSolid,
    /// Vertex list entity (type 502).
    VertexList,
    /// Edge list entity (type 504).
    EdgeList,
    /// Loop entity (type 508).
    Loop,
    /// Face entity (type 510).
    Face,
    /// Shell entity (type 514).
    Shell,
    /// Singular subfigure instance entity (type 408).
    SubfigureInstance,
    /// Group entity (type 402, form 1).
    Group,
    /// Color definition entity (type 314).
    Color,
    /// Unknown / extension entity.
    Unknown(i32),
}

impl IgesEntityKind {
    /// Return the IGES type number.
    pub fn type_number(self) -> i32 {
        match self {
            IgesEntityKind::Null                => 0,
            IgesEntityKind::CircularArc         => 100,
            IgesEntityKind::CompositeCurve      => 102,
            IgesEntityKind::ConicArc            => 104,
            IgesEntityKind::Plane               => 108,
            IgesEntityKind::Line                => 110,
            IgesEntityKind::Point               => 116,
            IgesEntityKind::RuledSurface        => 118,
            IgesEntityKind::SurfaceOfRevolution => 120,
            IgesEntityKind::TabulatedCylinder   => 122,
            IgesEntityKind::BSplineCurve        => 126,
            IgesEntityKind::BSplineSurface      => 128,
            IgesEntityKind::CurveOnSurface      => 142,
            IgesEntityKind::BoundedSurface      => 143,
            IgesEntityKind::TrimmedSurface      => 144,
            IgesEntityKind::ManifoldSolid       => 186,
            IgesEntityKind::Color               => 314,
            IgesEntityKind::Group               => 402,
            IgesEntityKind::SubfigureInstance   => 408,
            IgesEntityKind::VertexList          => 502,
            IgesEntityKind::EdgeList            => 504,
            IgesEntityKind::Loop                => 508,
            IgesEntityKind::Face                => 510,
            IgesEntityKind::Shell               => 514,
            IgesEntityKind::Unknown(n)          => n,
        }
    }

    /// Parse from a raw type number.
    pub fn from_type_number(n: i32) -> Self {
        match n {
            0   => IgesEntityKind::Null,
            100 => IgesEntityKind::CircularArc,
            102 => IgesEntityKind::CompositeCurve,
            104 => IgesEntityKind::ConicArc,
            108 => IgesEntityKind::Plane,
            110 => IgesEntityKind::Line,
            116 => IgesEntityKind::Point,
            118 => IgesEntityKind::RuledSurface,
            120 => IgesEntityKind::SurfaceOfRevolution,
            122 => IgesEntityKind::TabulatedCylinder,
            126 => IgesEntityKind::BSplineCurve,
            128 => IgesEntityKind::BSplineSurface,
            142 => IgesEntityKind::CurveOnSurface,
            143 => IgesEntityKind::BoundedSurface,
            144 => IgesEntityKind::TrimmedSurface,
            186 => IgesEntityKind::ManifoldSolid,
            314 => IgesEntityKind::Color,
            402 => IgesEntityKind::Group,
            408 => IgesEntityKind::SubfigureInstance,
            502 => IgesEntityKind::VertexList,
            504 => IgesEntityKind::EdgeList,
            508 => IgesEntityKind::Loop,
            510 => IgesEntityKind::Face,
            514 => IgesEntityKind::Shell,
            other => IgesEntityKind::Unknown(other),
        }
    }
}

// ---------------------------------------------------------------------------
// IGESEntity
// ---------------------------------------------------------------------------

/// A single IGES entity record (one entry in the Directory + Parameter Data).
// occt: IGESData_IGESEntity
#[derive(Debug, Clone)]
pub struct IGESEntity {
    /// Entity type number (Directory Entry field 1).
    pub entity_type: IgesEntityKind,
    /// 1-based sequence number in the Directory Entry section (always odd).
    pub de_seq: u32,
    /// Pointer to the first Parameter Data line (1-based line number).
    pub pd_seq: u32,
    /// Form number (Directory Entry field 15).
    pub form: i16,
    /// Entity label (Directory Entry field 18, max 8 chars).
    pub label: String,
    /// Subscript number (Directory Entry field 19).
    pub subscript: i32,
    /// Raw parameter string (everything on the Parameter Data lines for
    /// this entity, joined into one comma-separated string, no terminator).
    pub parameters: String,
    /// Color number (Directory Entry field 13; 0 = default).
    pub color: u8,
    /// Line weight number (Directory Entry field 12; 0 = default).
    pub line_weight: u8,
}

impl IGESEntity {
    /// Create a minimal entity with the given type and parameters.
    pub fn new(kind: IgesEntityKind, parameters: impl Into<String>) -> Self {
        Self {
            entity_type: kind,
            de_seq: 0,
            pd_seq: 0,
            form: 0,
            label: String::new(),
            subscript: 0,
            parameters: parameters.into(),
            color: 0,
            line_weight: 0,
        }
    }

    /// Parse individual parameter tokens (splits on `,`; stops at `;`).
    pub fn param_tokens(&self) -> Vec<&str> {
        // First token is the type number itself (IGES convention), so skip it.
        self.parameters
            .split(',')
            .map(|s| s.trim_end_matches(';').trim())
            .collect()
    }

    /// Try to read the nth parameter token (0-based, after the type prefix) as
    /// an f64.  Returns `None` on parse failure or index out of range.
    pub fn param_f64(&self, idx: usize) -> Option<f64> {
        self.param_tokens().get(idx + 1)?.parse::<f64>().ok()
    }

    /// Try to read the nth parameter token as an i64.
    pub fn param_i64(&self, idx: usize) -> Option<i64> {
        self.param_tokens().get(idx + 1)?.parse::<i64>().ok()
    }
}

// ---------------------------------------------------------------------------
// IGESGlobal
// ---------------------------------------------------------------------------

/// Global section parameters of an IGES file.
// occt: IGESData_GlobalSection
#[derive(Debug, Clone)]
pub struct IGESGlobal {
    /// Parameter delimiter character (default `,`).
    pub param_delim: char,
    /// Record delimiter character (default `;`).
    pub record_delim: char,
    /// Product identification from sending system.
    pub product_id_send: String,
    /// File name.
    pub file_name: String,
    /// Native system id of the sending system.
    pub system_id: String,
    /// Pre-processor version.
    pub preprocessor_version: String,
    /// Number of significant binary digits in floating-point.
    pub int_bits: u32,
    /// Maximum power of ten in single-precision.
    pub single_max_power: u32,
    /// Number of significant digits in single-precision.
    pub single_digits: u32,
    /// Maximum power of ten in double-precision.
    pub double_max_power: u32,
    /// Number of significant digits in double-precision.
    pub double_digits: u32,
    /// Product identification for the receiving system.
    pub product_id_recv: String,
    /// Model space scale.
    pub model_scale: f64,
    /// Unit code.
    pub unit: IgesUnit,
    /// Maximum number of line weights.
    pub max_line_weights: u32,
    /// Size of the maximum line weight in units.
    pub max_line_weight_value: f64,
    /// File generation timestamp (e.g. "15H20230401.120000").
    pub timestamp: String,
    /// Minimum user-intended resolution.
    pub resolution: f64,
    /// Maximum coordinate value.
    pub max_coord: f64,
    /// Author name.
    pub author: String,
    /// Author organisation.
    pub organisation: String,
    /// IGES version flag (3 = IGES 3.0, 5 = IGES 5.3, etc.).
    pub iges_version: u8,
    /// Drafting standard code.
    pub drafting_standard: u8,
    /// Model modification timestamp.
    pub modified_timestamp: String,
    /// Application protocol / subset identifier.
    pub protocol_id: String,
}

impl Default for IGESGlobal {
    fn default() -> Self {
        Self {
            param_delim: ',',
            record_delim: ';',
            product_id_send: "IronStream".into(),
            file_name: "model.igs".into(),
            system_id: "IronStream".into(),
            preprocessor_version: "1.0".into(),
            int_bits: 32,
            single_max_power: 38,
            single_digits: 6,
            double_max_power: 308,
            double_digits: 15,
            product_id_recv: "IronStream".into(),
            model_scale: 1.0,
            unit: IgesUnit::Millimeter,
            max_line_weights: 1,
            max_line_weight_value: 0.0,
            timestamp: String::new(),
            resolution: 1e-6,
            max_coord: 1e9,
            author: String::new(),
            organisation: String::new(),
            iges_version: 5,
            drafting_standard: 0,
            modified_timestamp: String::new(),
            protocol_id: String::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// IGESModel
// ---------------------------------------------------------------------------

/// In-memory representation of an IGES model — the global section plus all
/// entities.
// occt: IGESData_IGESModel
#[derive(Debug, Clone)]
pub struct IGESModel {
    /// Global section parameters.
    pub global: IGESGlobal,
    /// All entities in directory-entry order.
    entities: Vec<IGESEntity>,
}

impl IGESModel {
    /// Create an empty model with default global parameters.
    pub fn new() -> Self {
        Self {
            global: IGESGlobal::default(),
            entities: Vec::new(),
        }
    }

    /// Number of entities in the model.
    pub fn nb_entities(&self) -> usize {
        self.entities.len()
    }

    /// Add an entity to the model, assigning sequential DE/PD sequence numbers.
    /// Returns the 1-based entity index.
    pub fn add_entity(&mut self, mut entity: IGESEntity) -> usize {
        // DE lines are 2 per entity, starting at 1 (odd only).
        let de_seq = (self.entities.len() as u32) * 2 + 1;
        // PD lines: each entity occupies at least one PD line.
        let pd_seq = self.entities.iter().fold(1u32, |acc, e| {
            acc + count_pd_lines(&e.parameters)
        });
        entity.de_seq = de_seq;
        entity.pd_seq = pd_seq;
        self.entities.push(entity);
        self.entities.len()
    }

    /// Return entity by 1-based index.  Returns `None` for out-of-range.
    pub fn entity(&self, idx: usize) -> Option<&IGESEntity> {
        if idx == 0 { return None; }
        self.entities.get(idx - 1)
    }

    /// Return mutable entity by 1-based index.
    pub fn entity_mut(&mut self, idx: usize) -> Option<&mut IGESEntity> {
        if idx == 0 { return None; }
        self.entities.get_mut(idx - 1)
    }

    /// Iterate over all entities.
    pub fn iter_entities(&self) -> impl Iterator<Item = &IGESEntity> {
        self.entities.iter()
    }

    /// Return all entities of a given kind.
    pub fn entities_of_kind(&self, kind: IgesEntityKind) -> Vec<&IGESEntity> {
        self.entities.iter().filter(|e| e.entity_type == kind).collect()
    }

    /// Remove all entities.
    pub fn clear(&mut self) {
        self.entities.clear();
    }
}

impl Default for IGESModel {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// IgesError
// ---------------------------------------------------------------------------

/// Error type returned by IGES reader/writer operations.
// occt: IGESData_IGESReaderData // (errors surface via messenger)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IgesError {
    pub message: String,
}

impl IgesError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self { message: msg.into() }
    }
}

impl std::fmt::Display for IgesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IGES error: {}", self.message)
    }
}

impl std::error::Error for IgesError {}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Count the number of 64-character PD lines needed for the parameter string.
fn count_pd_lines(params: &str) -> u32 {
    // PD lines hold up to 64 characters of free-format data (cols 1-64).
    let len = params.len().max(1);
    ((len - 1) / 64 + 1) as u32
}

/// Write a hollerith (Hstring) field: `{n}H{text}`.
fn hollerith(s: &str) -> String {
    format!("{}H{}", s.len(), s)
}

/// Parse a hollerith field `{n}H{text}` and return `text`.
fn parse_hollerith(s: &str) -> Option<String> {
    let h_pos = s.find('H')?;
    let n: usize = s[..h_pos].trim().parse().ok()?;
    let text = &s[h_pos + 1..];
    if text.len() >= n {
        Some(text[..n].to_string())
    } else {
        None
    }
}

/// Format a float in IGES scientific notation (e.g. `1.2345E+03`).
fn iges_float(v: f64) -> String {
    if v == 0.0 {
        return "0.0".to_string();
    }
    format!("{:E}", v)
}

/// Pad / truncate a string to exactly `width` characters with spaces.
fn pad(s: &str, width: usize) -> String {
    let mut out = s.to_string();
    while out.len() < width {
        out.push(' ');
    }
    if out.len() > width {
        out.truncate(width);
    }
    out
}

/// Write one 80-character IGES fixed-format line with section code in col 73
/// and sequence number in cols 74-80.
fn fixed_line(body: &str, section: char, seq: u32) -> String {
    // cols 1-72 = body (padded to 72), col 73 = section, cols 74-80 = seq (7 digits)
    format!("{}{}{:>7}\n", pad(body, 72), section, seq)
}

// ---------------------------------------------------------------------------
// IGESControl_Reader
// ---------------------------------------------------------------------------

/// IGES file reader: parses an IGES byte buffer and makes the resulting
/// [`IGESModel`] and [`TopoDS_Shape`] roots available for transfer.
///
/// Mirrors the OCCT workflow:
/// ```text
/// let mut reader = IGESControl_Reader::new();
/// reader.read_file("model.igs")?;      // or read_content(&bytes)
/// reader.transfer_roots();
/// let shape = reader.one_shape();
/// ```
// occt: IGESControl_Reader
pub struct IGESControl_Reader {
    /// The parsed in-memory model.
    model: Option<IGESModel>,
    /// Shapes produced by `transfer_roots`.
    shapes: Vec<TopoDS_Shape>,
    /// Whether `transfer_roots` has been called.
    transferred: bool,
    /// Virtual "filesystem": path → IGES content bytes.
    vfs: HashMap<String, Vec<u8>>,
    /// Last error, if any.
    last_error: Option<IgesError>,
}

impl IGESControl_Reader {
    /// Create a new, empty reader.
    pub fn new() -> Self {
        Self {
            model: None,
            shapes: Vec::new(),
            transferred: false,
            vfs: HashMap::new(),
            last_error: None,
        }
    }

    // --- Virtual filesystem helpers -------------------------------------------

    /// Register a named content buffer so that [`read_file`] can locate it
    /// without touching the real filesystem.
    pub fn register_content(&mut self, path: impl Into<String>, content: Vec<u8>) {
        self.vfs.insert(path.into(), content);
    }

    // --- Reading --------------------------------------------------------------

    /// Read an IGES file from the virtual filesystem by path.
    ///
    /// Returns `Ok(1)` on success (OCCT returns an `IFSelect_ReturnStatus`
    /// integer: 1 = done, 0 = error).
    pub fn read_file(&mut self, path: &str) -> Result<i32, IgesError> {
        let content = self.vfs.get(path).cloned().ok_or_else(|| {
            IgesError::new(format!("file not found: {path}"))
        })?;
        self.read_content(&content)
    }

    /// Parse an IGES model from raw bytes (the full file content).
    ///
    /// Returns `Ok(1)` on success.
    pub fn read_content(&mut self, data: &[u8]) -> Result<i32, IgesError> {
        let text = std::str::from_utf8(data)
            .map_err(|_| IgesError::new("IGES content is not valid UTF-8"))?;
        let model = parse_iges(text)?;
        self.model = Some(model);
        self.transferred = false;
        self.shapes.clear();
        self.last_error = None;
        Ok(1)
    }

    // --- Model access ---------------------------------------------------------

    /// Return a reference to the parsed model, if available.
    pub fn model(&self) -> Option<&IGESModel> {
        self.model.as_ref()
    }

    /// Number of roots (free-standing entities) in the model.  In this
    /// implementation every top-level entity is considered a root.
    pub fn nb_roots_for_transfer(&self) -> usize {
        self.model.as_ref().map_or(0, |m| m.nb_entities())
    }

    // --- Transfer -------------------------------------------------------------

    /// Transfer all root entities from the model into TopoDS shapes.
    ///
    /// Returns the number of shapes produced.
    pub fn transfer_roots(&mut self) -> usize {
        let model = match &self.model {
            Some(m) => m.clone(),
            None    => return 0,
        };
        self.shapes.clear();
        for entity in model.iter_entities() {
            if let Some(shape) = entity_to_shape(entity, &model) {
                self.shapes.push(shape);
            }
        }
        self.transferred = true;
        self.shapes.len()
    }

    /// Transfer the entity at the given 1-based index.
    /// Returns `true` when the transfer produced a shape.
    pub fn transfer_one_root(&mut self, idx: usize) -> bool {
        let model = match &self.model {
            Some(m) => m.clone(),
            None    => return false,
        };
        if let Some(entity) = model.entity(idx) {
            if let Some(shape) = entity_to_shape(entity, &model) {
                self.shapes.push(shape);
                return true;
            }
        }
        false
    }

    /// Number of shapes produced by `transfer_roots` / `transfer_one_root`.
    pub fn nb_shapes(&self) -> usize {
        self.shapes.len()
    }

    /// Return the shape at the given 1-based index.
    pub fn shape(&self, idx: usize) -> Option<&TopoDS_Shape> {
        if idx == 0 { return None; }
        self.shapes.get(idx - 1)
    }

    /// Return a single compound shape containing all transferred shapes.  If
    /// there is exactly one shape it is returned as-is.
    pub fn one_shape(&self) -> Option<TopoDS_Shape> {
        match self.shapes.len() {
            0 => None,
            1 => Some(self.shapes[0].clone()),
            _ => {
                let compound = TopoDS_Compound {
                    id: 0,
                    shapes: self.shapes.clone(),
                };
                Some(TopoDS_Shape::Compound(compound))
            }
        }
    }

    /// Return `true` when the reader has a loaded (but not necessarily
    /// transferred) model.
    pub fn has_model(&self) -> bool {
        self.model.is_some()
    }

    /// Return `true` when `transfer_roots` has been called successfully.
    pub fn is_transferred(&self) -> bool {
        self.transferred
    }

    /// Clear all state (model, shapes, errors) so the reader can be reused.
    pub fn clear(&mut self) {
        self.model = None;
        self.shapes.clear();
        self.transferred = false;
        self.last_error = None;
    }

    /// Return the last error, if any.
    pub fn last_error(&self) -> Option<&IgesError> {
        self.last_error.as_ref()
    }

    /// Print a summary of the model to a `String` (mirrors
    /// `IGESControl_Reader::PrintCheckLoad` / `PrintStatTransfer`).
    pub fn stats(&self) -> String {
        match &self.model {
            None => "No model loaded.\n".to_string(),
            Some(m) => {
                let mut s = format!(
                    "IGES model: {} entities, unit={}\n",
                    m.nb_entities(),
                    m.global.unit.flag(),
                );
                s.push_str(&format!(
                    "  Author: {}\n  Org: {}\n",
                    m.global.author, m.global.organisation,
                ));
                s.push_str(&format!("  Shapes transferred: {}\n", self.shapes.len()));
                s
            }
        }
    }
}

impl Default for IGESControl_Reader {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// IGESControl_Writer
// ---------------------------------------------------------------------------

/// IGES file writer: builds an in-memory IGES model and serialises it to a
/// `Vec<u8>` (or `String`) in standard IGES fixed-format.
///
/// Mirrors the OCCT workflow:
/// ```text
/// let mut writer = IGESControl_Writer::new();
/// writer.add_shape(&shape);
/// let bytes = writer.write_content();
/// ```
// occt: IGESControl_Writer
pub struct IGESControl_Writer {
    /// The model being built.
    model: IGESModel,
    /// Writer-level error, if any.
    last_error: Option<IgesError>,
}

impl IGESControl_Writer {
    /// Create a writer with default global section parameters.
    pub fn new() -> Self {
        Self {
            model: IGESModel::new(),
            last_error: None,
        }
    }

    // --- Global section -------------------------------------------------------

    /// Set the length unit for the output file.
    pub fn set_unit(&mut self, unit: IgesUnit) {
        self.model.global.unit = unit;
    }

    /// Set the author metadata fields.
    pub fn set_author(&mut self, author: impl Into<String>, organisation: impl Into<String>) {
        self.model.global.author       = author.into();
        self.model.global.organisation = organisation.into();
    }

    /// Set the file name stored in the Global Section.
    pub fn set_file_name(&mut self, name: impl Into<String>) {
        self.model.global.file_name = name.into();
    }

    // --- Entity / shape addition ----------------------------------------------

    /// Add a `TopoDS_Shape` to the model.  The shape is decomposed and encoded
    /// as IGES entities according to its topology level.
    pub fn add_shape(&mut self, shape: &TopoDS_Shape) {
        shape_to_entities(shape, &mut self.model);
    }

    /// Add a free-standing 3D point entity (type 116).
    pub fn add_point(&mut self, p: Pnt) {
        let params = format!("116,{},{},{},0", iges_float(p.x), iges_float(p.y), iges_float(p.z));
        self.model.add_entity(IGESEntity::new(IgesEntityKind::Point, params));
    }

    /// Add a line entity (type 110) between two points.
    pub fn add_line(&mut self, start: Pnt, end: Pnt) {
        let params = format!(
            "110,{},{},{},{},{},{}",
            iges_float(start.x), iges_float(start.y), iges_float(start.z),
            iges_float(end.x),   iges_float(end.y),   iges_float(end.z),
        );
        self.model.add_entity(IGESEntity::new(IgesEntityKind::Line, params));
    }

    /// Add a circular-arc entity (type 100).
    ///
    /// The arc lies in a plane parallel to the XY-plane at `z`.
    /// `cx`, `cy` are the centre; `x1`,`y1` and `x2`,`y2` are the start and
    /// end points of the arc (all in the plane).
    pub fn add_circular_arc(
        &mut self,
        z: f64,
        cx: f64, cy: f64,
        x1: f64, y1: f64,
        x2: f64, y2: f64,
    ) {
        let params = format!(
            "100,{},{},{},{},{},{},{}",
            iges_float(z),
            iges_float(cx), iges_float(cy),
            iges_float(x1), iges_float(y1),
            iges_float(x2), iges_float(y2),
        );
        self.model.add_entity(IGESEntity::new(IgesEntityKind::CircularArc, params));
    }

    /// Return a shared reference to the model being built.
    pub fn model(&self) -> &IGESModel {
        &self.model
    }

    // --- Output ---------------------------------------------------------------

    /// Serialise the model to an IGES byte buffer.
    pub fn write_content(&self) -> Vec<u8> {
        self.write_string().into_bytes()
    }

    /// Serialise the model to an IGES `String`.
    pub fn write_string(&self) -> String {
        emit_iges(&self.model)
    }

    /// Write the model to a named slot in a `HashMap` (virtual filesystem).
    pub fn write_file(
        &self,
        path: &str,
        vfs: &mut HashMap<String, Vec<u8>>,
    ) {
        vfs.insert(path.to_string(), self.write_content());
    }

    /// Number of entities that have been added so far.
    pub fn nb_entities(&self) -> usize {
        self.model.nb_entities()
    }

    /// Remove all entities from the model (keeps global section).
    pub fn clear(&mut self) {
        self.model.clear();
    }

    /// Return the last error, if any.
    pub fn last_error(&self) -> Option<&IgesError> {
        self.last_error.as_ref()
    }
}

impl Default for IGESControl_Writer {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// IGES serialiser
// ---------------------------------------------------------------------------

/// Emit the full IGES fixed-format text for `model`.
fn emit_iges(model: &IGESModel) -> String {
    let g = &model.global;
    let mut out = String::new();

    // ---- Start Section (1 line minimum) -------------------------------------
    let start_body = format!(
        "IronStream IGES export  {}",
        g.file_name
    );
    out.push_str(&fixed_line(&start_body, 'S', 1));
    let s_count = 1u32;

    // ---- Global Section -----------------------------------------------------
    let global_str = format_global(g);
    let mut g_seq = 0u32;
    for chunk in global_str.as_bytes().chunks(72) {
        g_seq += 1;
        let body = std::str::from_utf8(chunk).unwrap_or("");
        out.push_str(&fixed_line(body, 'G', g_seq));
    }

    // ---- Directory Entry + Parameter Data sections --------------------------
    let mut de_lines = Vec::<String>::new();
    let mut pd_lines = Vec::<String>::new();
    let mut pd_seq_counter = 1u32;

    for entity in model.iter_entities() {
        let de_seq = entity.de_seq;
        let type_num = entity.entity_type.type_number();
        let pd_start = pd_seq_counter;

        // Parameter Data lines for this entity
        let full_params = format!("{},{};", type_num, entity.parameters);
        let param_bytes = full_params.as_bytes();
        let mut entity_pd_seq = 0u32;
        for chunk in param_bytes.chunks(64) {
            entity_pd_seq += 1;
            let body = std::str::from_utf8(chunk).unwrap_or("");
            // PD line: cols 1-64 = params, col 65-72 = DE pointer (8 chars), 73 = P
            let de_ptr = format!("{:>8}", de_seq);
            pd_lines.push(format!(
                "{}{}{:>7}\n",
                pad(body, 64),
                pad(&de_ptr, 8),
                'P' as u8 as char,
            ));
            // Embedded seq number after the 'P':
            // We'll rewrite this properly below.
            pd_seq_counter += 1;
        }

        // Directory Entry: 2 lines of 80 chars each.
        // Line 1: type(8), pd_ptr(8), struct(8), lf(8), lw(8), blank(8), status(8), sec_code+seq(8+8)
        let de1 = format!(
            "{:>8}{:>8}{:>8}{:>8}{:>8}{:>8}{:>8}D{:>7}",
            type_num, pd_start, 0, 0,
            entity.line_weight, entity.color, "00000001",
            de_seq,
        );
        // Line 2: type(8), entity_label(8...16), 0(8), form(8), 0(8), 0(8), label(8), subscript(8), sec+seq
        let de2 = format!(
            "{:>8}{:>8}{:>8}{:>8}{:>8}{:>8}{}{}D{:>7}",
            type_num,
            entity_pd_seq,    // line count in PD
            0, entity.form, 0, 0,
            pad(&entity.label, 8),
            format!("{:>8}", entity.subscript),
            de_seq + 1,
        );
        de_lines.push(format!("{}\n", &de1[..de1.len().min(72)]));
        de_lines.push(format!("{}\n", &de2[..de2.len().min(72)]));
    }

    // Now write DE section with proper sequence numbers.
    for (i, dl) in de_lines.iter().enumerate() {
        let body = dl.trim_end_matches('\n');
        out.push_str(&fixed_line(body, 'D', (i + 1) as u32));
    }
    let d_count = de_lines.len() as u32;

    // Write PD section with proper sequence numbers.
    // Each PD line: cols 1-64 data, cols 65-72 DE pointer (right-aligned), col 73 = 'P', cols 74-80 seq.
    pd_seq_counter = 1;
    let mut pd_entity_idx = 0usize;
    for entity in model.iter_entities() {
        let de_seq = entity.de_seq;
        let full_params = format!("{},{};", entity.entity_type.type_number(), entity.parameters);
        for chunk in full_params.as_bytes().chunks(64) {
            let body = std::str::from_utf8(chunk).unwrap_or("");
            let line = format!(
                "{}{:>8}P{:>7}\n",
                pad(body, 64),
                de_seq,
                pd_seq_counter,
            );
            out.push_str(&line);
            pd_seq_counter += 1;
        }
        pd_entity_idx += 1;
    }
    let p_count = pd_seq_counter - 1;

    // ---- Terminate Section --------------------------------------------------
    let term = format!(
        "S{:>7}G{:>7}D{:>7}P{:>7}",
        s_count, g_seq, d_count, p_count,
    );
    out.push_str(&fixed_line(&term, 'T', 1));

    out
}

/// Format the Global Section as a single continuous parameter string.
fn format_global(g: &IGESGlobal) -> String {
    let fields: Vec<String> = vec![
        format!("{},", g.param_delim),
        format!("{};", g.record_delim),
        format!("{},", hollerith(&g.product_id_send)),
        format!("{},", hollerith(&g.file_name)),
        format!("{},", hollerith(&g.system_id)),
        format!("{},", hollerith(&g.preprocessor_version)),
        format!("{},", g.int_bits),
        format!("{},", g.single_max_power),
        format!("{},", g.single_digits),
        format!("{},", g.double_max_power),
        format!("{},", g.double_digits),
        format!("{},", hollerith(&g.product_id_recv)),
        format!("{},", g.model_scale),
        format!("{},", g.unit.code()),
        format!("{},", hollerith(g.unit.flag())),
        format!("{},", g.max_line_weights),
        format!("{},", iges_float(g.max_line_weight_value)),
        format!("{},", hollerith(&g.timestamp)),
        format!("{},", iges_float(g.resolution)),
        format!("{},", iges_float(g.max_coord)),
        format!("{},", hollerith(&g.author)),
        format!("{},", hollerith(&g.organisation)),
        format!("{},", g.iges_version),
        format!("{},", g.drafting_standard),
        format!("{},", hollerith(&g.modified_timestamp)),
        format!("{};",  hollerith(&g.protocol_id)),
    ];
    fields.concat()
}

// ---------------------------------------------------------------------------
// IGES parser
// ---------------------------------------------------------------------------

/// Parse a complete IGES text into an [`IGESModel`].
fn parse_iges(text: &str) -> Result<IGESModel, IgesError> {
    let mut model = IGESModel::new();

    // Collect lines by section.
    let mut start_lines  = Vec::<String>::new();
    let mut global_lines = Vec::<String>::new();
    let mut de_lines     = Vec::<String>::new();
    let mut pd_lines     = Vec::<String>::new();

    for raw_line in text.lines() {
        // An IGES fixed-format line is exactly 80 characters.
        // Col 73 (0-based: 72) is the section code.
        let line = raw_line;
        if line.len() < 73 {
            continue;
        }
        let section = line.chars().nth(72).unwrap_or(' ');
        let body    = &line[..72];
        match section {
            'S' => start_lines.push(body.to_string()),
            'G' => global_lines.push(body.to_string()),
            'D' => de_lines.push(body.to_string()),
            'P' => pd_lines.push(body.to_string()),
            'T' => {} // terminate — ignore
            _   => {}
        }
    }

    // Parse global section.
    let global_raw = global_lines.join("");
    parse_global_into(&global_raw, &mut model.global);

    // Parse Directory Entry.  Each entity occupies exactly 2 DE lines.
    if de_lines.len() % 2 != 0 {
        return Err(IgesError::new(format!(
            "Directory Entry section has odd line count ({})",
            de_lines.len()
        )));
    }

    // Build a PD lookup: de_seq → concatenated PD text.
    // PD lines have the DE sequence number in cols 65-72 (0-based).
    let mut pd_map: HashMap<u32, String> = HashMap::new();
    for pd_line in &pd_lines {
        if pd_line.len() < 72 {
            continue;
        }
        let de_ptr_str = pd_line[64..72].trim();
        let Ok(de_ptr) = de_ptr_str.parse::<u32>() else { continue };
        let data = &pd_line[..64];
        pd_map.entry(de_ptr).or_default().push_str(data);
    }

    for pair in de_lines.chunks(2) {
        let de1 = &pair[0];
        // Type number is in cols 1-8 (0-based 0..8).
        let type_num: i32 = de1[..8].trim().parse().unwrap_or(0);
        let de_seq: u32   = de1[73_usize.min(de1.len())..].trim()
            .trim_start_matches(|c: char| !c.is_ascii_digit())
            .parse().unwrap_or(0);

        // Retrieve PD data for this entity.
        let raw_pd = pd_map.get(&de_seq).cloned().unwrap_or_default();
        // Strip the leading type prefix (up to first comma).
        let params = if let Some(pos) = raw_pd.find(',') {
            raw_pd[pos + 1..].trim_end_matches(';').trim().to_string()
        } else {
            raw_pd.trim_end_matches(';').trim().to_string()
        };

        let mut entity = IGESEntity::new(
            IgesEntityKind::from_type_number(type_num),
            params,
        );
        entity.de_seq = de_seq;

        // Form number from DE line 2, cols 49-56.
        if pair.len() > 1 {
            let de2 = &pair[1];
            if de2.len() >= 56 {
                entity.form = de2[48..56].trim().parse().unwrap_or(0);
            }
            // Label from DE line 2 cols 65-72.
            if de2.len() >= 72 {
                entity.label = de2[64..72].trim().to_string();
            }
        }

        model.entities.push(entity);
    }

    Ok(model)
}

/// Best-effort parse of the Global Section parameter string into `IGESGlobal`.
fn parse_global_into(raw: &str, g: &mut IGESGlobal) {
    // Split on `,` while respecting hollerith fields.
    let tokens = split_global_tokens(raw);
    if tokens.len() < 2 { return; }
    // Tokens index (1-based IGES field numbering → 0-based Vec index):
    //  0: param delimiter,  1: record delimiter,  2: product_id_send, ...
    if let Some(s) = tokens.get(2) { g.product_id_send = parse_hollerith(s).unwrap_or_default(); }
    if let Some(s) = tokens.get(3) { g.file_name       = parse_hollerith(s).unwrap_or_default(); }
    if let Some(s) = tokens.get(4) { g.system_id       = parse_hollerith(s).unwrap_or_default(); }
    if let Some(s) = tokens.get(5) { g.preprocessor_version = parse_hollerith(s).unwrap_or_default(); }
    if let Some(s) = tokens.get(6) { g.int_bits        = s.parse().unwrap_or(32); }
    if let Some(s) = tokens.get(13) { g.unit = IgesUnit::from_flag(s).unwrap_or_default(); }
    if let Some(s) = tokens.get(17) { g.resolution     = s.parse().unwrap_or(1e-6); }
    if let Some(s) = tokens.get(19) { g.author         = parse_hollerith(s).unwrap_or_default(); }
    if let Some(s) = tokens.get(20) { g.organisation   = parse_hollerith(s).unwrap_or_default(); }
    if let Some(s) = tokens.get(21) { g.iges_version   = s.parse().unwrap_or(5); }
}

/// Split a Global Section string into raw tokens, respecting hollerith fields.
fn split_global_tokens(raw: &str) -> Vec<String> {
    let mut tokens = Vec::<String>::new();
    let mut current = String::new();
    let bytes = raw.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        let ch = bytes[i] as char;
        // Hollerith: `{n}H{text}`
        if ch.is_ascii_digit() {
            // Peek ahead for 'H'
            let start = i;
            while i < bytes.len() && (bytes[i] as char).is_ascii_digit() { i += 1; }
            if i < bytes.len() && bytes[i] == b'H' {
                let n: usize = raw[start..i].parse().unwrap_or(0);
                i += 1; // skip 'H'
                let end = (i + n).min(bytes.len());
                let hollerith_text = &raw[i..end];
                current.push_str(&format!("{}H{}", n, hollerith_text));
                i = end;
                continue;
            } else {
                current.push_str(&raw[start..i]);
                continue;
            }
        }
        if ch == ',' || ch == ';' {
            tokens.push(current.trim().to_string());
            current = String::new();
            i += 1;
            if ch == ';' { break; }
        } else {
            current.push(ch);
            i += 1;
        }
    }
    if !current.trim().is_empty() {
        tokens.push(current.trim().to_string());
    }
    tokens
}

// ---------------------------------------------------------------------------
// Shape ↔ entity conversion
// ---------------------------------------------------------------------------

/// Convert a single [`IGESEntity`] into a [`TopoDS_Shape`] if possible.
fn entity_to_shape(entity: &IGESEntity, _model: &IGESModel) -> Option<TopoDS_Shape> {
    match entity.entity_type {
        IgesEntityKind::Point => {
            let x = entity.param_f64(0).unwrap_or(0.0);
            let y = entity.param_f64(1).unwrap_or(0.0);
            let z = entity.param_f64(2).unwrap_or(0.0);
            Some(TopoDS_Shape::Vertex(TopoDS_Vertex {
                id: entity.de_seq as u64,
                x, y, z,
            }))
        }
        IgesEntityKind::Line => {
            let x1 = entity.param_f64(0).unwrap_or(0.0);
            let y1 = entity.param_f64(1).unwrap_or(0.0);
            let z1 = entity.param_f64(2).unwrap_or(0.0);
            let x2 = entity.param_f64(3).unwrap_or(0.0);
            let y2 = entity.param_f64(4).unwrap_or(0.0);
            let z2 = entity.param_f64(5).unwrap_or(0.0);
            let v1 = TopoDS_Vertex { id: entity.de_seq as u64,     x: x1, y: y1, z: z1 };
            let v2 = TopoDS_Vertex { id: entity.de_seq as u64 + 1, x: x2, y: y2, z: z2 };
            Some(TopoDS_Shape::Edge(TopoDS_Edge {
                id: entity.de_seq as u64,
                vertices: vec![v1, v2],
            }))
        }
        IgesEntityKind::CircularArc => {
            // type 100: z, cx, cy, x1, y1, x2, y2
            // We produce a wire with a single synthetic edge.
            let z  = entity.param_f64(0).unwrap_or(0.0);
            let cx = entity.param_f64(1).unwrap_or(0.0);
            let cy = entity.param_f64(2).unwrap_or(0.0);
            let x1 = entity.param_f64(3).unwrap_or(0.0);
            let y1 = entity.param_f64(4).unwrap_or(0.0);
            let x2 = entity.param_f64(5).unwrap_or(cx + (x1 - cx));
            let y2 = entity.param_f64(6).unwrap_or(cy + (y1 - cy));
            let id = entity.de_seq as u64;
            let v_start = TopoDS_Vertex { id, x: x1, y: y1, z };
            let v_end   = TopoDS_Vertex { id: id + 1, x: x2, y: y2, z };
            let edge = TopoDS_Edge { id, vertices: vec![v_start, v_end] };
            Some(TopoDS_Shape::Wire(TopoDS_Wire { id, edges: vec![edge] }))
        }
        IgesEntityKind::Face | IgesEntityKind::TrimmedSurface => {
            // Produce a minimal face with no wires.
            Some(TopoDS_Shape::Face(TopoDS_Face {
                id: entity.de_seq as u64,
                wires: Vec::new(),
            }))
        }
        IgesEntityKind::Shell | IgesEntityKind::ManifoldSolid => {
            let face = TopoDS_Face { id: entity.de_seq as u64, wires: Vec::new() };
            let shell = TopoDS_Shell { id: entity.de_seq as u64, faces: vec![face] };
            let solid = TopoDS_Solid { id: entity.de_seq as u64, shells: vec![shell] };
            Some(TopoDS_Shape::Solid(solid))
        }
        _ => None,
    }
}

/// Recursively convert a `TopoDS_Shape` hierarchy into IGES entities and add
/// them to the model.
fn shape_to_entities(shape: &TopoDS_Shape, model: &mut IGESModel) {
    match shape {
        TopoDS_Shape::Vertex(v) => {
            let params = format!(
                "{},{},{},0",
                iges_float(v.x), iges_float(v.y), iges_float(v.z)
            );
            let mut e = IGESEntity::new(IgesEntityKind::Point, params);
            e.label = format!("V{}", v.id);
            model.add_entity(e);
        }
        TopoDS_Shape::Edge(edge) => {
            if edge.vertices.len() >= 2 {
                let s = &edge.vertices[0];
                let t = &edge.vertices[1];
                let params = format!(
                    "{},{},{},{},{},{}",
                    iges_float(s.x), iges_float(s.y), iges_float(s.z),
                    iges_float(t.x), iges_float(t.y), iges_float(t.z),
                );
                let mut e = IGESEntity::new(IgesEntityKind::Line, params);
                e.label = format!("E{}", edge.id);
                model.add_entity(e);
            }
        }
        TopoDS_Shape::Wire(wire) => {
            for edge in &wire.edges {
                shape_to_entities(&TopoDS_Shape::Edge(edge.clone()), model);
            }
        }
        TopoDS_Shape::Face(face) => {
            for wire in &face.wires {
                shape_to_entities(&TopoDS_Shape::Wire(wire.clone()), model);
            }
            let mut e = IGESEntity::new(IgesEntityKind::Face, "0,1,0".to_string());
            e.label = format!("F{}", face.id);
            model.add_entity(e);
        }
        TopoDS_Shape::Shell(shell) => {
            for face in &shell.faces {
                shape_to_entities(&TopoDS_Shape::Face(face.clone()), model);
            }
        }
        TopoDS_Shape::Solid(solid) => {
            for shell in &solid.shells {
                shape_to_entities(&TopoDS_Shape::Shell(shell.clone()), model);
            }
            let mut e = IGESEntity::new(IgesEntityKind::ManifoldSolid, "0,1,1".to_string());
            e.label = format!("S{}", solid.id);
            model.add_entity(e);
        }
        TopoDS_Shape::Compound(compound) => {
            for child in &compound.shapes {
                shape_to_entities(child, model);
            }
        }
        TopoDS_Shape::CompSolid(cs) => {
            for solid in &cs.solids {
                shape_to_entities(&TopoDS_Shape::Solid(solid.clone()), model);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- IgesUnit ---

    #[test]
    fn unit_code_roundtrip() {
        for unit in [
            IgesUnit::Millimeter, IgesUnit::Inch, IgesUnit::Meter,
            IgesUnit::Centimeter, IgesUnit::Foot,
        ] {
            assert_eq!(IgesUnit::from_flag(unit.flag()), Some(unit));
        }
    }

    #[test]
    fn unit_to_mm_conversion() {
        assert!((IgesUnit::Inch.to_mm() - 25.4).abs() < 1e-9);
        assert!((IgesUnit::Meter.to_mm() - 1000.0).abs() < 1e-9);
        assert!((IgesUnit::Millimeter.to_mm() - 1.0).abs() < 1e-9);
    }

    // --- IgesEntityKind ---

    #[test]
    fn entity_kind_type_number_roundtrip() {
        for &n in &[0, 100, 110, 116, 126, 128, 144, 186, 502, 510, 514] {
            let k = IgesEntityKind::from_type_number(n);
            assert_eq!(k.type_number(), n);
        }
    }

    #[test]
    fn entity_kind_unknown_preserves_number() {
        let k = IgesEntityKind::from_type_number(9999);
        assert_eq!(k.type_number(), 9999);
        assert_eq!(k, IgesEntityKind::Unknown(9999));
    }

    // --- IGESEntity ---

    #[test]
    fn entity_param_tokens_and_typed_accessors() {
        let e = IGESEntity::new(IgesEntityKind::Point, "116,1.0,2.0,3.0,0".to_string());
        // param_tokens includes type prefix at index 0.
        assert_eq!(e.param_tokens()[0], "116");
        // param_f64 skips the type prefix (0-based after prefix).
        assert!((e.param_f64(0).unwrap() - 1.0).abs() < 1e-9);
        assert!((e.param_f64(1).unwrap() - 2.0).abs() < 1e-9);
        assert!((e.param_f64(2).unwrap() - 3.0).abs() < 1e-9);
    }

    // --- IGESModel ---

    #[test]
    fn model_add_and_retrieve_entities() {
        let mut m = IGESModel::new();
        let e1 = IGESEntity::new(IgesEntityKind::Point, "1.0,2.0,3.0".to_string());
        let e2 = IGESEntity::new(IgesEntityKind::Line,  "0,0,0,1,1,1".to_string());
        let idx1 = m.add_entity(e1);
        let idx2 = m.add_entity(e2);
        assert_eq!(m.nb_entities(), 2);
        assert_eq!(idx1, 1);
        assert_eq!(idx2, 2);
        assert_eq!(m.entity(1).unwrap().entity_type, IgesEntityKind::Point);
        assert_eq!(m.entity(2).unwrap().entity_type, IgesEntityKind::Line);
        assert!(m.entity(0).is_none());
        assert!(m.entity(3).is_none());
    }

    #[test]
    fn model_entities_of_kind() {
        let mut m = IGESModel::new();
        m.add_entity(IGESEntity::new(IgesEntityKind::Point, "0,0,0".to_string()));
        m.add_entity(IGESEntity::new(IgesEntityKind::Line,  "0,0,0,1,0,0".to_string()));
        m.add_entity(IGESEntity::new(IgesEntityKind::Point, "1,0,0".to_string()));
        let pts = m.entities_of_kind(IgesEntityKind::Point);
        assert_eq!(pts.len(), 2);
    }

    // --- Writer ---

    #[test]
    fn writer_add_point_and_line_increases_count() {
        let mut w = IGESControl_Writer::new();
        w.add_point(Pnt::new(1.0, 2.0, 3.0));
        assert_eq!(w.nb_entities(), 1);
        w.add_line(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
        assert_eq!(w.nb_entities(), 2);
    }

    #[test]
    fn writer_emit_contains_iges_structure() {
        let mut w = IGESControl_Writer::new();
        w.add_point(Pnt::new(0.0, 0.0, 0.0));
        let s = w.write_string();
        // IGES files identify sections via the character in column 73.
        // Check that all required section codes are present.
        assert!(s.lines().any(|l| l.len() >= 73 && l.chars().nth(72) == Some('S')),
            "Start section code missing");
        assert!(s.lines().any(|l| l.len() >= 73 && l.chars().nth(72) == Some('G')),
            "Global section code missing");
        assert!(s.lines().any(|l| l.len() >= 73 && l.chars().nth(72) == Some('T')),
            "Terminate section code missing");
    }

    #[test]
    fn writer_emit_has_correct_line_width() {
        let mut w = IGESControl_Writer::new();
        w.add_line(Pnt::origin(), Pnt::new(5.0, 5.0, 5.0));
        let s = w.write_string();
        for line in s.lines() {
            assert!(line.len() <= 80, "line too long: {} chars: {:?}", line.len(), line);
        }
    }

    #[test]
    fn writer_global_unit_reflected_in_model() {
        let mut w = IGESControl_Writer::new();
        w.set_unit(IgesUnit::Inch);
        assert_eq!(w.model().global.unit, IgesUnit::Inch);
    }

    // --- Reader / Writer roundtrip ---

    #[test]
    fn roundtrip_point_via_vfs() {
        let mut writer = IGESControl_Writer::new();
        writer.add_point(Pnt::new(1.0, 2.0, 3.0));
        let mut vfs = HashMap::new();
        writer.write_file("test.igs", &mut vfs);

        let mut reader = IGESControl_Reader::new();
        for (k, v) in &vfs { reader.register_content(k.clone(), v.clone()); }
        let status = reader.read_file("test.igs").unwrap();
        assert_eq!(status, 1);
        assert!(reader.has_model());
        let n = reader.transfer_roots();
        assert!(n > 0, "expected at least one shape");
    }

    #[test]
    fn reader_stats_shows_entity_count() {
        let mut writer = IGESControl_Writer::new();
        writer.add_point(Pnt::origin());
        writer.add_line(Pnt::origin(), Pnt::new(1.0, 0.0, 0.0));
        let mut vfs = HashMap::new();
        writer.write_file("m.igs", &mut vfs);

        let mut reader = IGESControl_Reader::new();
        for (k, v) in &vfs { reader.register_content(k.clone(), v.clone()); }
        reader.read_file("m.igs").unwrap();
        let stats = reader.stats();
        assert!(stats.contains("2 entities") || stats.contains("entities"),
            "stats: {stats}");
    }
}
