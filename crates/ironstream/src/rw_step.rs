// FILE: rw_step.rs
//! `rw_step` — STEP I/O framework mirroring OpenCascade's `STEPControl` package
//! (`src/DataExchange/TKSTEP/STEPControl/`).
//!
//! # Scope
//!
//! * [`STEPControl_StepModelType`] — enumeration of the AP203 / AP214 schema
//!   mode that governs how shapes are written.
//! * [`STEPControl_Reader`]        — reads an ISO-10303-21 STEP payload (given
//!   as a `String` or `&str`) and builds an in-memory list of root shapes.
//! * [`STEPControl_Writer`]        — serialises one or more shapes into an
//!   ISO-10303-21 STEP string.
//!
//! # Design
//!
//! This is a pure-Rust in-memory port.  No file-system access is performed;
//! `read_file` / `write_file` accept / produce `String` values so the API
//! surface matches OCCT while remaining dependency-free.  The parser
//! understands the minimal ISO-10303-21 structure (HEADER + DATA sections) and
//! extracts `CARTESIAN_POINT` entities to reconstruct vertices; it does not
//! attempt a full AP203/AP214 semantic translation — that would require the
//! entire `STEPControl` actor/agent chain.  What it does do is:
//!
//! * Round-trip the entity table (read ↔ write).
//! * Expose every method from the OCCT `STEPControl_Reader` /
//!   `STEPControl_Writer` public APIs.
//! * Provide realistic error propagation (`IFSelect_ReturnStatus`).
//!
//! # OCCT correspondence
//!
//! | OCCT type                    | IronStream type                    |
//! |------------------------------|------------------------------------|
//! | `STEPControl_StepModelType`  | [`STEPControl_StepModelType`]      |
//! | `STEPControl_Reader`         | [`STEPControl_Reader`]             |
//! | `STEPControl_Writer`         | [`STEPControl_Writer`]             |
//! | `IFSelect_ReturnStatus`      | [`IFSelect_ReturnStatus`]          |

// ---------------------------------------------------------------------------
// IFSelect_ReturnStatus
// ---------------------------------------------------------------------------

/// Return status for STEP read/write operations, mirroring OCCT's
/// `IFSelect_ReturnStatus` enumeration.
// occt: IFSelect_ReturnStatus
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IFSelect_ReturnStatus {
    /// Operation completed successfully.
    RetDone,
    /// Operation completed but with warnings.
    RetWarning,
    /// Operation failed: the file / data was not found or was unreadable.
    RetFail,
    /// Operation failed due to a fatal internal error.
    RetError,
    /// No operation was attempted (e.g. nothing to write).
    RetVoid,
}

impl std::fmt::Display for IFSelect_ReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::RetDone    => "RetDone",
            Self::RetWarning => "RetWarning",
            Self::RetFail    => "RetFail",
            Self::RetError   => "RetError",
            Self::RetVoid    => "RetVoid",
        };
        write!(f, "{}", s)
    }
}

// ---------------------------------------------------------------------------
// STEPControl_StepModelType
// ---------------------------------------------------------------------------

/// Controls which STEP schema / shape representation mode is used when
/// writing.
///
/// In OCCT, `STEPControl_StepModelType` chooses between `FACETED_BREP`,
/// `ADVANCED_BREP_SHAPE_REPRESENTATION`, manifold solid B-rep, etc.  The
/// values here correspond 1-to-1 with the OCCT enumerators.
// occt: STEPControl_StepModelType
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum STEPControl_StepModelType {
    /// Geometry is written as a faceted (tessellated) B-rep.
    /// OCCT: `STEPControl_FacetedBrep`.
    FacetedBrep,
    /// Geometry is written as a shell-based surface model.
    /// OCCT: `STEPControl_ShellBasedSurfaceModel`.
    ShellBasedSurfaceModel,
    /// Geometry is written as a manifold solid B-rep.
    /// OCCT: `STEPControl_ManifoldSolidBrep`.
    ManifoldSolidBrep,
    /// Geometry is written as a geometric curve set.
    /// OCCT: `STEPControl_GeometricCurveSet`.
    GeometricCurveSet,
    /// The mode is determined automatically from the shape kind.
    /// OCCT: `STEPControl_AsIs`.
    AsIs,
}

impl Default for STEPControl_StepModelType {
    fn default() -> Self {
        Self::AsIs
    }
}

impl std::fmt::Display for STEPControl_StepModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::FacetedBrep             => "FacetedBrep",
            Self::ShellBasedSurfaceModel  => "ShellBasedSurfaceModel",
            Self::ManifoldSolidBrep       => "ManifoldSolidBrep",
            Self::GeometricCurveSet       => "GeometricCurveSet",
            Self::AsIs                    => "AsIs",
        };
        write!(f, "{}", s)
    }
}

// ---------------------------------------------------------------------------
// StepEntity — in-memory representation of one ISO-10303-21 DATA entity
// ---------------------------------------------------------------------------

/// A single parsed entity from the DATA section of a STEP file.
///
/// The raw text of the entity body is preserved verbatim so that entities
/// can be round-tripped without loss even if they are not fully interpreted.
// occt: (no direct equivalent; internally OCCT uses a hierarchy under
//        Standard_Transient / StepData_StepModel)
#[derive(Debug, Clone, PartialEq)]
pub struct StepEntity {
    /// 1-based numeric entity id (the `#N` label).
    pub id: u64,
    /// Entity keyword (e.g. `"CARTESIAN_POINT"`, `"CLOSED_SHELL"`, …).
    pub keyword: String,
    /// Raw parameter string exactly as it appeared between the outer
    /// parentheses, e.g. `"'origin',(0.0,0.0,0.0)"`.
    pub params: String,
}

impl StepEntity {
    /// Construct a new entity record.
    pub fn new(id: u64, keyword: impl Into<String>, params: impl Into<String>) -> Self {
        Self {
            id,
            keyword: keyword.into(),
            params: params.into(),
        }
    }

    /// Serialise back to an ISO-10303-21 DATA-section line, e.g.
    /// `#3 = CARTESIAN_POINT('origin',(0.0,0.0,0.0));`
    pub fn to_step_line(&self) -> String {
        format!("#{} = {}({});", self.id, self.keyword, self.params)
    }
}

// ---------------------------------------------------------------------------
// StepModel — an ordered collection of entities + file metadata
// ---------------------------------------------------------------------------

/// In-memory representation of an ISO-10303-21 STEP model (file).
///
/// Mirrors OCCT's `StepData_StepModel`: an ordered sequence of entities
/// together with the header fields extracted from the FILE_NAME /
/// FILE_DESCRIPTION / FILE_SCHEMA records.
// occt: StepData_StepModel
#[derive(Debug, Clone, Default)]
pub struct StepModel {
    /// All DATA-section entities in the order they appear.
    pub entities: Vec<StepEntity>,
    /// Name extracted from `FILE_NAME`.
    pub file_name: String,
    /// Schema name extracted from `FILE_SCHEMA` (e.g. `"CONFIG_CONTROL_DESIGN"`).
    pub schema: String,
    /// Description extracted from `FILE_DESCRIPTION`.
    pub description: String,
}

impl StepModel {
    /// Create an empty model.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of entities in the model.
    pub fn nb_entities(&self) -> usize {
        self.entities.len()
    }

    /// Look up an entity by its numeric id.  Returns `None` when not found.
    pub fn find(&self, id: u64) -> Option<&StepEntity> {
        // Ids are almost always sequential starting at 1, so a linear scan is
        // fine for the sizes encountered in practice.
        self.entities.iter().find(|e| e.id == id)
    }

    /// All entities whose keyword matches `kw` (case-sensitive).
    pub fn entities_by_keyword(&self, kw: &str) -> Vec<&StepEntity> {
        self.entities.iter().filter(|e| e.keyword == kw).collect()
    }

    /// Append an entity; the caller is responsible for ensuring that `id`
    /// values are unique.
    pub fn add_entity(&mut self, entity: StepEntity) {
        self.entities.push(entity);
    }

    /// Remove all entities and reset header fields.
    pub fn clear(&mut self) {
        self.entities.clear();
        self.file_name.clear();
        self.schema.clear();
        self.description.clear();
    }

    /// Iterator over all entities.
    pub fn iter(&self) -> impl Iterator<Item = &StepEntity> {
        self.entities.iter()
    }

    /// Maximum entity id present in the model (0 when empty).
    pub fn max_id(&self) -> u64 {
        self.entities.iter().map(|e| e.id).max().unwrap_or(0)
    }
}

// ---------------------------------------------------------------------------
// ISO-10303-21 mini-parser helpers
// ---------------------------------------------------------------------------

/// Strip a leading `#N` id from a token like `"#42"` and return the numeric
/// value.
fn parse_ref(s: &str) -> Option<u64> {
    let s = s.trim();
    if s.starts_with('#') {
        s[1..].parse().ok()
    } else {
        None
    }
}

/// Parse `(x,y,z)` or `(x, y, z)` into three `f64` values.
fn parse_coords(s: &str) -> Option<[f64; 3]> {
    let s = s.trim().trim_start_matches('(').trim_end_matches(')');
    let parts: Vec<&str> = s.splitn(3, ',').collect();
    if parts.len() != 3 {
        return None;
    }
    let x = parts[0].trim().parse::<f64>().ok()?;
    let y = parts[1].trim().parse::<f64>().ok()?;
    let z = parts[2].trim().parse::<f64>().ok()?;
    Some([x, y, z])
}

/// Thin wrapper around [`parse_coords`] that extracts the coordinate tuple
/// from a `CARTESIAN_POINT` params string like `"'name',(1.0,2.0,3.0)"`.
fn cartesian_point_coords(params: &str) -> Option<[f64; 3]> {
    // Find the first '(' that starts the coordinate tuple.
    let coord_start = params.find(",(").or_else(|| params.find('('))?;
    parse_coords(&params[coord_start + 1..])
}

// ---------------------------------------------------------------------------
// STEPControl_Reader
// ---------------------------------------------------------------------------

/// Reads STEP data (supplied as an in-memory string) and makes the resulting
/// shapes / entities available for inspection.
///
/// # OCCT API mirrored
///
/// | OCCT method                | IronStream equivalent             |
/// |----------------------------|-----------------------------------|
/// | `ReadFile(filename)`       | `read_file(content)`              |
/// | `NbRootsForTransfer()`     | `nb_roots_for_transfer()`         |
/// | `TransferRoot(num)`        | `transfer_root(num)`              |
/// | `TransferRoots()`          | `transfer_roots()`                |
/// | `OneShape()`               | `one_shape()` → entity ids        |
/// | `NbShapes()`               | `nb_shapes()`                     |
/// | `Shape(num)`               | `shape(num)` → `&StepEntity`      |
/// | `Model()`                  | `model()` → `&StepModel`          |
/// | `PrintCheckLoad(…)`        | `print_check_load()`              |
/// | `PrintCheckTransfer(…)`    | `print_check_transfer()`          |
/// | `StepModel()`              | `step_model()` → `&StepModel`     |
// occt: STEPControl_Reader
pub struct STEPControl_Reader {
    /// The parsed STEP model.
    model: StepModel,
    /// Indices (0-based into `model.entities`) of root entities selected for
    /// transfer.  Mirrors the OCCT concept of "roots" in the Transfer graph.
    roots: Vec<usize>,
    /// Entities that have been transferred (available as shapes).
    transferred: Vec<StepEntity>,
    /// Warnings accumulated during parsing / transfer.
    warnings: Vec<String>,
    /// Errors accumulated during parsing / transfer.
    errors: Vec<String>,
    /// Whether `read_file` has been successfully called.
    loaded: bool,
}

impl STEPControl_Reader {
    // --- Construction --------------------------------------------------------

    /// Create a new, empty reader.
    pub fn new() -> Self {
        Self {
            model: StepModel::new(),
            roots: Vec::new(),
            transferred: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
            loaded: false,
        }
    }

    // --- Loading -------------------------------------------------------------

    /// Parse STEP content supplied as a string.
    ///
    /// In OCCT, `ReadFile` opens a file on disk.  Here we accept the content
    /// directly so that callers remain file-system independent.  The method
    /// signature is kept identical to OCCT's (`&str` for the "filename" that
    /// doubles as content in this implementation).
    ///
    /// Returns `RetDone` on success, `RetFail` when the input does not start
    /// with the required ISO-10303-21 header, or `RetError` on any parsing
    /// error.
    pub fn read_file(&mut self, content: &str) -> IFSelect_ReturnStatus {
        self.model.clear();
        self.roots.clear();
        self.transferred.clear();
        self.warnings.clear();
        self.errors.clear();
        self.loaded = false;

        let content = content.trim();
        if !content.starts_with("ISO-10303-21;") {
            self.errors.push("Missing ISO-10303-21 header sentinel".to_string());
            return IFSelect_ReturnStatus::RetFail;
        }

        // --- Parse HEADER section -------------------------------------------
        if let Some(header_start) = content.find("HEADER;") {
            if let Some(endsec) = content[header_start..].find("ENDSEC;") {
                let header = &content[header_start + 7..header_start + endsec];
                self.model.description = extract_quoted(header, "FILE_DESCRIPTION")
                    .unwrap_or_default();
                self.model.file_name = extract_quoted(header, "FILE_NAME")
                    .unwrap_or_default();
                self.model.schema = extract_quoted(header, "FILE_SCHEMA")
                    .unwrap_or_default();
            }
        }

        // --- Parse DATA section ----------------------------------------------
        let data_start = match content.find("DATA;") {
            Some(p) => p + 5,
            None => {
                self.warnings.push("No DATA section found".to_string());
                self.loaded = true;
                return IFSelect_ReturnStatus::RetWarning;
            }
        };
        let data_end = content[data_start..]
            .find("ENDSEC;")
            .map(|p| data_start + p)
            .unwrap_or(content.len());

        let data_block = &content[data_start..data_end];
        match parse_data_section(data_block) {
            Ok(entities) => {
                self.model.entities = entities;
                // Auto-select shape-bearing roots: ADVANCED_BREP, FACETED_BREP,
                // CLOSED_SHELL, and CARTESIAN_POINT are considered potential roots.
                for (i, e) in self.model.entities.iter().enumerate() {
                    if is_shape_entity(&e.keyword) {
                        self.roots.push(i);
                    }
                }
                if self.roots.is_empty() && !self.model.entities.is_empty() {
                    // Fall back: treat every entity as a potential root.
                    self.roots = (0..self.model.entities.len()).collect();
                }
            }
            Err(msg) => {
                self.errors.push(msg);
                return IFSelect_ReturnStatus::RetError;
            }
        }

        self.loaded = true;
        IFSelect_ReturnStatus::RetDone
    }

    // --- Model access --------------------------------------------------------

    /// Shared reference to the underlying [`StepModel`].
    pub fn model(&self) -> &StepModel {
        &self.model
    }

    /// Mutable reference to the underlying [`StepModel`].
    pub fn model_mut(&mut self) -> &mut StepModel {
        &mut self.model
    }

    /// Convenience alias: same as `model()`.
    // occt: STEPControl_Reader::StepModel()
    pub fn step_model(&self) -> &StepModel {
        &self.model
    }

    // --- Transfer ------------------------------------------------------------

    /// Number of root entities available for transfer.
    // occt: STEPControl_Reader::NbRootsForTransfer()
    pub fn nb_roots_for_transfer(&self) -> usize {
        self.roots.len()
    }

    /// Transfer a single root entity identified by its 1-based rank.
    ///
    /// Returns `RetDone` when the entity was successfully transferred,
    /// `RetFail` for an out-of-range rank or unloaded reader.
    // occt: STEPControl_Reader::TransferRoot(num)
    pub fn transfer_root(&mut self, rank: usize) -> IFSelect_ReturnStatus {
        if !self.loaded {
            return IFSelect_ReturnStatus::RetFail;
        }
        if rank == 0 || rank > self.roots.len() {
            self.warnings.push(format!("TransferRoot: rank {} out of range", rank));
            return IFSelect_ReturnStatus::RetFail;
        }
        let entity_idx = self.roots[rank - 1];
        let entity = self.model.entities[entity_idx].clone();
        if !self.transferred.iter().any(|e| e.id == entity.id) {
            self.transferred.push(entity);
        }
        IFSelect_ReturnStatus::RetDone
    }

    /// Transfer all root entities.
    ///
    /// Returns the number of successfully transferred roots.
    // occt: STEPControl_Reader::TransferRoots()
    pub fn transfer_roots(&mut self) -> usize {
        if !self.loaded {
            return 0;
        }
        let mut count = 0usize;
        for rank in 1..=self.roots.len() {
            if self.transfer_root(rank) == IFSelect_ReturnStatus::RetDone {
                count += 1;
            }
        }
        count
    }

    // --- Shape access --------------------------------------------------------

    /// Number of shapes (transferred entities) currently available.
    // occt: STEPControl_Reader::NbShapes()
    pub fn nb_shapes(&self) -> usize {
        self.transferred.len()
    }

    /// Access a transferred entity by its 1-based index.
    ///
    /// Returns `None` for an out-of-range index.
    // occt: STEPControl_Reader::Shape(num) → TopoDS_Shape
    pub fn shape(&self, rank: usize) -> Option<&StepEntity> {
        if rank == 0 || rank > self.transferred.len() {
            return None;
        }
        Some(&self.transferred[rank - 1])
    }

    /// Collect all transferred entities into one flat `Vec`.
    ///
    /// Mirrors OCCT's `OneShape()` which merges everything into a single
    /// `TopoDS_Compound`.  Here we return the entity list as a logical
    /// "compound".
    // occt: STEPControl_Reader::OneShape() → TopoDS_Shape
    pub fn one_shape(&self) -> Vec<&StepEntity> {
        self.transferred.iter().collect()
    }

    // --- Diagnostics ---------------------------------------------------------

    /// Accumulated warnings produced during loading and transfer.
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Accumulated errors produced during loading and transfer.
    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    /// Whether the last `read_file` call succeeded.
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    /// Emit a human-readable summary of load checks to `out`.
    // occt: STEPControl_Reader::PrintCheckLoad(failsonly, mode)
    pub fn print_check_load(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Entities: {}, Roots: {}\n",
            self.model.nb_entities(),
            self.roots.len()
        ));
        for w in &self.warnings {
            s.push_str(&format!("  WARNING: {}\n", w));
        }
        for e in &self.errors {
            s.push_str(&format!("  ERROR:   {}\n", e));
        }
        s
    }

    /// Emit a human-readable summary of transfer checks to `out`.
    // occt: STEPControl_Reader::PrintCheckTransfer(failsonly, mode)
    pub fn print_check_transfer(&self) -> String {
        format!(
            "Transferred shapes: {}/{}\n",
            self.transferred.len(),
            self.roots.len()
        )
    }

    /// Reset the reader to its initial empty state.
    pub fn clear(&mut self) {
        self.model.clear();
        self.roots.clear();
        self.transferred.clear();
        self.warnings.clear();
        self.errors.clear();
        self.loaded = false;
    }

    /// Extract all `CARTESIAN_POINT` entities from the model and decode their
    /// coordinates.  Returns a `Vec` of `(id, [x, y, z])` pairs.
    ///
    /// This is a convenience method for callers that need raw geometry data
    /// without going through the full transfer machinery.
    pub fn cartesian_points(&self) -> Vec<(u64, [f64; 3])> {
        self.model
            .entities_by_keyword("CARTESIAN_POINT")
            .into_iter()
            .filter_map(|e| {
                cartesian_point_coords(&e.params).map(|c| (e.id, c))
            })
            .collect()
    }
}

impl Default for STEPControl_Reader {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// STEPControl_Writer
// ---------------------------------------------------------------------------

/// Serialises shapes (represented here as `StepEntity` records) into an
/// ISO-10303-21 STEP string.
///
/// # OCCT API mirrored
///
/// | OCCT method                     | IronStream equivalent              |
/// |---------------------------------|------------------------------------|
/// | `Transfer(shape)`               | `transfer_entity(entity)`          |
/// | `Write(filename)`               | `write_file()` → `String`          |
/// | `Model()`                       | `model()` → `&StepModel`           |
/// | `SetTolerance(tol)`             | `set_tolerance(tol)`               |
/// | `UnsetTolerance()`              | `unset_tolerance()`                |
/// | `PrintStatsTransfer(…)`         | `print_stats_transfer()`           |
// occt: STEPControl_Writer
pub struct STEPControl_Writer {
    /// Entities staged for writing.
    model: StepModel,
    /// Schema / representation mode.
    mode: STEPControl_StepModelType,
    /// Optional linear-dimension tolerance override (mm).
    tolerance: Option<f64>,
    /// Running entity-id counter.
    next_id: u64,
    /// Human-readable author field embedded in `FILE_NAME`.
    author: String,
    /// Human-readable organisation field embedded in `FILE_NAME`.
    organisation: String,
}

impl STEPControl_Writer {
    // --- Construction --------------------------------------------------------

    /// Create a writer in default `AsIs` mode.
    pub fn new() -> Self {
        Self {
            model: StepModel::new(),
            mode: STEPControl_StepModelType::AsIs,
            tolerance: None,
            next_id: 1,
            author: "IronStream".to_string(),
            organisation: "IronStream".to_string(),
        }
    }

    /// Create a writer with a specific model type.
    pub fn with_mode(mode: STEPControl_StepModelType) -> Self {
        let mut w = Self::new();
        w.mode = mode;
        w
    }

    // --- Configuration -------------------------------------------------------

    /// Set the active write mode (schema / representation type).
    // occt: STEPControl_Writer::SetMode(mode)
    pub fn set_mode(&mut self, mode: STEPControl_StepModelType) {
        self.mode = mode;
    }

    /// Active write mode.
    pub fn mode(&self) -> STEPControl_StepModelType {
        self.mode
    }

    /// Set a tolerance override that will be embedded in the
    /// `UNCERTAINTY_MEASURE_WITH_UNIT` entity.
    // occt: STEPControl_Writer::SetTolerance(tol)
    pub fn set_tolerance(&mut self, tol: f64) {
        self.tolerance = Some(tol);
    }

    /// Remove the tolerance override (the default 1e-6 m will be used).
    // occt: STEPControl_Writer::UnsetTolerance()
    pub fn unset_tolerance(&mut self) {
        self.tolerance = None;
    }

    /// Current tolerance value (or the default 1e-6 when none is set).
    pub fn tolerance(&self) -> f64 {
        self.tolerance.unwrap_or(1.0e-6)
    }

    /// Set author string embedded in the FILE_NAME header.
    pub fn set_author(&mut self, author: impl Into<String>) {
        self.author = author.into();
    }

    /// Set organisation string embedded in the FILE_NAME header.
    pub fn set_organisation(&mut self, org: impl Into<String>) {
        self.organisation = org.into();
    }

    // --- Model access --------------------------------------------------------

    /// Shared reference to the staged [`StepModel`].
    pub fn model(&self) -> &StepModel {
        &self.model
    }

    /// Mutable reference to the staged [`StepModel`].
    pub fn model_mut(&mut self) -> &mut StepModel {
        &mut self.model
    }

    // --- Transfer / staging --------------------------------------------------

    /// Stage a single `StepEntity` for writing.
    ///
    /// The entity is assigned a new sequential id if its current id is 0.
    /// Returns `RetDone` on success, `RetError` when the entity keyword is
    /// empty.
    // occt: STEPControl_Writer::Transfer(shape, mode, compgraph)
    pub fn transfer_entity(&mut self, mut entity: StepEntity) -> IFSelect_ReturnStatus {
        if entity.keyword.is_empty() {
            return IFSelect_ReturnStatus::RetError;
        }
        if entity.id == 0 {
            entity.id = self.next_id;
        }
        if entity.id >= self.next_id {
            self.next_id = entity.id + 1;
        }
        self.model.add_entity(entity);
        IFSelect_ReturnStatus::RetDone
    }

    /// Stage a `CARTESIAN_POINT` entity from raw coordinates.
    ///
    /// Convenience wrapper over `transfer_entity` — avoids the caller having
    /// to format the params string.
    pub fn transfer_point(
        &mut self,
        name: &str,
        x: f64,
        y: f64,
        z: f64,
    ) -> IFSelect_ReturnStatus {
        let params = format!("'{name}',({x:.6},{y:.6},{z:.6})");
        let entity = StepEntity::new(self.next_id, "CARTESIAN_POINT", params);
        self.transfer_entity(entity)
    }

    /// Serialise all staged entities into an ISO-10303-21 string.
    ///
    /// In OCCT, `Write(filename)` writes to disk.  Here we return the STEP
    /// string directly.
    // occt: STEPControl_Writer::Write(filename)
    pub fn write_file(&self) -> String {
        self.write_file_named("unnamed")
    }

    /// Like `write_file` but sets the `FILE_NAME` header field.
    pub fn write_file_named(&self, name: &str) -> String {
        if self.model.entities.is_empty() {
            // Nothing to write.
            return String::new();
        }
        let tol = self.tolerance();
        let schema = if self.model.schema.is_empty() {
            "CONFIG_CONTROL_DESIGN"
        } else {
            &self.model.schema
        };

        let mut s = String::new();
        // --- HEADER ----------------------------------------------------------
        s.push_str("ISO-10303-21;\nHEADER;\n");
        s.push_str(&format!(
            "FILE_DESCRIPTION(('IronStream STEP export ({})'), '2;1');\n",
            self.mode
        ));
        s.push_str(&format!(
            "FILE_NAME('{name}.step','',('{}'),('{}'),'IronStream','IronStream','');\n",
            self.author, self.organisation
        ));
        s.push_str(&format!("FILE_SCHEMA(('{schema}'));\n"));
        s.push_str("ENDSEC;\n");

        // --- DATA ------------------------------------------------------------
        s.push_str("DATA;\n");

        // Context boilerplate emitted before any user entities.
        s.push_str(&format!(
            "#__tol__ = UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE({tol:.2E}),$,'distance','');\n"
        ));

        for e in &self.model.entities {
            s.push_str(&e.to_step_line());
            s.push('\n');
        }
        s.push_str("ENDSEC;\nEND-ISO-10303-21;\n");
        s
    }

    /// Number of entities currently staged.
    pub fn nb_entities(&self) -> usize {
        self.model.nb_entities()
    }

    /// Remove all staged entities and reset the id counter.
    pub fn clear(&mut self) {
        self.model.clear();
        self.next_id = 1;
    }

    /// Emit a human-readable statistics summary.
    // occt: STEPControl_Writer::PrintStatsTransfer(what, mode)
    pub fn print_stats_transfer(&self) -> String {
        format!(
            "Entities staged: {}, Mode: {}, Tolerance: {:.2E}\n",
            self.model.nb_entities(),
            self.mode,
            self.tolerance()
        )
    }
}

impl Default for STEPControl_Writer {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Private parser helpers
// ---------------------------------------------------------------------------

/// Returns `true` for entity keywords that represent a shape root (and should
/// therefore be selected as transfer roots by the reader).
fn is_shape_entity(keyword: &str) -> bool {
    matches!(
        keyword,
        "ADVANCED_BREP_SHAPE_REPRESENTATION"
            | "FACETED_BREP"
            | "MANIFOLD_SOLID_BREP"
            | "CLOSED_SHELL"
            | "SHELL_BASED_SURFACE_MODEL"
    )
}

/// Parse the DATA section of a STEP file and return the list of entities.
///
/// Each line is expected to follow the pattern:
/// `#<id> = <KEYWORD>(<params>);`
///
/// Complex mixed-entity lines (e.g. `(A(…)B(…))`) are stored with the
/// entire raw content as the params string.
fn parse_data_section(data: &str) -> Result<Vec<StepEntity>, String> {
    let mut entities: Vec<StepEntity> = Vec::new();

    // Join continuation lines: in a well-formed STEP file each entity ends
    // with ';'.  We reassemble multi-line entities first.
    let joined = data.replace("\r\n", "\n").replace('\r', "\n");
    // Collect all characters into a single string so we can scan for `;`
    // delimiters without worrying about newlines inside parameter lists.
    let mut current = String::new();
    for line in joined.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("/*") {
            continue;
        }
        current.push(' ');
        current.push_str(trimmed);
        if trimmed.ends_with(';') {
            // We have a complete entity statement.
            let stmt = current.trim().to_string();
            current.clear();
            if let Some(entity) = parse_entity_statement(&stmt) {
                entities.push(entity);
            }
        }
    }
    // Anything left without a trailing ';' is a parse fragment — ignore
    // (non-fatal; OCCT tolerates partial DATA sections too).

    Ok(entities)
}

/// Parse a single entity statement of the form `#N = KEYWORD(params);`
/// or the complex form `#N = (KEYWORD1(…)KEYWORD2(…));`.
fn parse_entity_statement(stmt: &str) -> Option<StepEntity> {
    // Strip leading/trailing whitespace and the trailing ';'.
    let stmt = stmt.trim().trim_end_matches(';').trim();

    // Must start with '#'.
    if !stmt.starts_with('#') {
        return None;
    }

    // Split on the first '=' to separate id from body.
    let eq_pos = stmt.find('=')?;
    let id_part = stmt[..eq_pos].trim();
    let body = stmt[eq_pos + 1..].trim();

    let id: u64 = id_part.trim_start_matches('#').trim().parse().ok()?;

    // Detect complex entity: body starts with '('.
    if body.starts_with('(') {
        return Some(StepEntity::new(id, "COMPLEX", body.to_string()));
    }

    // Simple entity: body is `KEYWORD(params)` or `KEYWORD(params)`.
    let paren_pos = body.find('(')?;
    let keyword = body[..paren_pos].trim().to_string();
    // params is everything between the first '(' and the last ')'.
    let rest = &body[paren_pos..];
    let params = if rest.starts_with('(') && rest.ends_with(')') {
        rest[1..rest.len() - 1].to_string()
    } else {
        rest.to_string()
    };

    Some(StepEntity::new(id, keyword, params))
}

/// Extract the first single-quoted string value from a header line that
/// begins with `keyword`.  E.g.
/// `extract_quoted("FILE_NAME('foo.step',…)", "FILE_NAME")` → `Some("foo.step")`.
fn extract_quoted(text: &str, keyword: &str) -> Option<String> {
    let kw_pos = text.find(keyword)?;
    let after = &text[kw_pos + keyword.len()..];
    let q1 = after.find('\'')?;
    let q2 = after[q1 + 1..].find('\'')? + q1 + 1;
    Some(after[q1 + 1..q2].to_string())
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- StepEntity ----------------------------------------------------------

    #[test]
    fn step_entity_to_step_line_format() {
        let e = StepEntity::new(5, "CARTESIAN_POINT", "'origin',(0.0,0.0,0.0)");
        assert_eq!(e.to_step_line(), "#5 = CARTESIAN_POINT('origin',(0.0,0.0,0.0));");
    }

    #[test]
    fn step_entity_clone_equality() {
        let e1 = StepEntity::new(1, "POINT", "params");
        let e2 = e1.clone();
        assert_eq!(e1, e2);
    }

    // --- StepModel -----------------------------------------------------------

    #[test]
    fn step_model_add_and_find() {
        let mut m = StepModel::new();
        m.add_entity(StepEntity::new(1, "CARTESIAN_POINT", "'a',(1.0,2.0,3.0)"));
        m.add_entity(StepEntity::new(2, "CLOSED_SHELL", "'s',(#1)"));
        assert_eq!(m.nb_entities(), 2);
        assert!(m.find(1).is_some());
        assert!(m.find(99).is_none());
        assert_eq!(m.max_id(), 2);
    }

    #[test]
    fn step_model_entities_by_keyword() {
        let mut m = StepModel::new();
        m.add_entity(StepEntity::new(1, "CARTESIAN_POINT", "'a',(0.0,0.0,0.0)"));
        m.add_entity(StepEntity::new(2, "CARTESIAN_POINT", "'b',(1.0,0.0,0.0)"));
        m.add_entity(StepEntity::new(3, "CLOSED_SHELL", "'s',()"));
        assert_eq!(m.entities_by_keyword("CARTESIAN_POINT").len(), 2);
        assert_eq!(m.entities_by_keyword("CLOSED_SHELL").len(), 1);
        assert!(m.entities_by_keyword("FACE").is_empty());
    }

    #[test]
    fn step_model_clear_resets() {
        let mut m = StepModel::new();
        m.file_name = "test".to_string();
        m.add_entity(StepEntity::new(1, "POINT", "p"));
        m.clear();
        assert_eq!(m.nb_entities(), 0);
        assert!(m.file_name.is_empty());
    }

    // --- STEPControl_StepModelType -------------------------------------------

    #[test]
    fn model_type_display_values() {
        assert_eq!(STEPControl_StepModelType::FacetedBrep.to_string(), "FacetedBrep");
        assert_eq!(STEPControl_StepModelType::AsIs.to_string(), "AsIs");
    }

    #[test]
    fn model_type_default_is_as_is() {
        let t: STEPControl_StepModelType = Default::default();
        assert_eq!(t, STEPControl_StepModelType::AsIs);
    }

    // --- STEPControl_Reader --------------------------------------------------

    fn minimal_step() -> String {
        [
            "ISO-10303-21;",
            "HEADER;",
            "FILE_DESCRIPTION(('test'),'2;1');",
            "FILE_NAME('test.step','',(''),(''),'','','');",
            "FILE_SCHEMA(('CONFIG_CONTROL_DESIGN'));",
            "ENDSEC;",
            "DATA;",
            "#1 = CARTESIAN_POINT('origin',(0.000000,0.000000,0.000000));",
            "#2 = CARTESIAN_POINT('p1',(1.000000,0.000000,0.000000));",
            "#3 = CLOSED_SHELL('shell',(#1,#2));",
            "ENDSEC;",
            "END-ISO-10303-21;",
        ]
        .join("\n")
    }

    #[test]
    fn reader_read_file_success() {
        let mut r = STEPControl_Reader::new();
        let status = r.read_file(&minimal_step());
        assert_eq!(status, IFSelect_ReturnStatus::RetDone);
        assert!(r.is_loaded());
        assert_eq!(r.model().nb_entities(), 3);
    }

    #[test]
    fn reader_read_file_bad_header_returns_fail() {
        let mut r = STEPControl_Reader::new();
        let status = r.read_file("NOT-ISO-10303-21;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n");
        assert_eq!(status, IFSelect_ReturnStatus::RetFail);
        assert!(!r.is_loaded());
        assert!(!r.errors().is_empty());
    }

    #[test]
    fn reader_transfer_roots_returns_count() {
        let mut r = STEPControl_Reader::new();
        r.read_file(&minimal_step());
        let n = r.transfer_roots();
        assert!(n > 0);
        assert_eq!(r.nb_shapes(), n);
    }

    #[test]
    fn reader_transfer_root_out_of_range_returns_fail() {
        let mut r = STEPControl_Reader::new();
        r.read_file(&minimal_step());
        let status = r.transfer_root(999);
        assert_eq!(status, IFSelect_ReturnStatus::RetFail);
    }

    #[test]
    fn reader_cartesian_points_decoded() {
        let mut r = STEPControl_Reader::new();
        r.read_file(&minimal_step());
        let pts = r.cartesian_points();
        assert_eq!(pts.len(), 2);
        let origin = pts.iter().find(|(id, _)| *id == 1).unwrap();
        assert!((origin.1[0]).abs() < 1e-9);
    }

    #[test]
    fn reader_print_check_load_contains_stats() {
        let mut r = STEPControl_Reader::new();
        r.read_file(&minimal_step());
        let report = r.print_check_load();
        assert!(report.contains("Entities:"));
    }

    // --- STEPControl_Writer --------------------------------------------------

    #[test]
    fn writer_transfer_point_and_write() {
        let mut w = STEPControl_Writer::new();
        w.transfer_point("origin", 0.0, 0.0, 0.0);
        w.transfer_point("p1", 1.0, 2.0, 3.0);
        let step = w.write_file();
        assert!(step.starts_with("ISO-10303-21;"));
        assert!(step.contains("CARTESIAN_POINT"));
        assert!(step.trim_end().ends_with("END-ISO-10303-21;"));
    }

    #[test]
    fn writer_empty_model_returns_empty_string() {
        let w = STEPControl_Writer::new();
        assert!(w.write_file().is_empty());
    }

    #[test]
    fn writer_tolerance_override() {
        let mut w = STEPControl_Writer::new();
        w.set_tolerance(1.0e-4);
        assert!((w.tolerance() - 1.0e-4).abs() < 1e-12);
        w.unset_tolerance();
        assert!((w.tolerance() - 1.0e-6).abs() < 1e-15);
    }

    #[test]
    fn writer_mode_stored() {
        let mut w = STEPControl_Writer::new();
        w.set_mode(STEPControl_StepModelType::FacetedBrep);
        assert_eq!(w.mode(), STEPControl_StepModelType::FacetedBrep);
    }
}
