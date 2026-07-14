// FILE: rust/ironstream/crates/ironstream/src/iges_entity.rs

//! IGES entity model — types for representing entities read from or written to
//! an IGES file, mirroring OpenCascade's `IGESData` package
//! (`src/DataExchange/TKIGES/IGESData/`).
//!
//! Uses only `std`; zero third-party dependencies.

// occt-note: IGES entity type
/// Discriminant for the IGES entity type code stored in a Directory Entry section.
/// Only the entity types relevant to IronStream's current scope are named;
/// everything else falls back to [`IgesEntityType::Unknown`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IgesEntityType {
    /// Entity type 100 — circular arc.
    // occt-ref: IGESData // entity type CircularArc (100)
    CircularArc,
    /// Entity type 102 — composite curve.
    // occt-ref: IGESData // entity type CompositeCurve (102)
    CompositeCurve,
    /// Entity type 104 — conic arc.
    // occt-ref: IGESData // entity type ConicArc (104)
    ConicArc,
    /// Entity type 110 — line.
    // occt-ref: IGESData // entity type Line (110)
    Line,
    /// Entity type 116 — point.
    // occt-ref: IGESData // entity type Point (116)
    Point,
    /// Entity type 126 — rational B-spline curve.
    // occt-ref: IGESData // entity type RationalBSplineCurve (126)
    RationalBSplineCurve,
    /// Entity type 128 — rational B-spline surface.
    // occt-ref: IGESData // entity type RationalBSplineSurface (128)
    RationalBSplineSurface,
    /// Entity type 108 — plane.
    // occt-ref: IGESData // entity type Plane (108)
    Plane,
    /// Entity type 144 — trimmed (parametric) surface.
    // occt-ref: IGESData // entity type TrimmedSurface (144)
    TrimmedSurface,
    /// Entity type 162 — solid of revolution.
    // occt-ref: IGESData // entity type SolidOfRevolution (162)
    SolidOfRevolution,
    /// Any entity type not explicitly mapped above.
    // occt-ref: IGESData // entity type Unknown
    Unknown,
}

// occt-ref: IGESData // status
/// Transfer / read status of an [`IgesEntity`], mirroring OCCT's
/// `IGESData_DefStatus` / `IGESData_ReadStage` concepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IgesEntityStatus {
    /// Entity has not been read yet.
    // occt-note: status NotRead
    NotRead,
    /// Entity has been successfully read.
    // occt-note: status Read
    Read,
    /// Entity data is structurally invalid.
    // occt-note: status Invalid
    Invalid,
    /// Entity has been transferred to a shape.
    // occt-note: status Transferred
    Transferred,
}

// occt-ref: IGESData_IGESEntity
/// A single entity from an IGES file's Directory Entry and Parameter Data
/// sections, mirroring `IGESData_IGESEntity`.
///
/// An entity is identified by its Directory Entry (DE) sequence number
/// [`de_number`](IgesEntity::de_number), classified by
/// [`entity_type`](IgesEntity::entity_type), and carries a flat list of
/// numeric [`params`](IgesEntity::nb_params) plus optional hollerith
/// [`string_params`](IgesEntity::add_string_param).
pub struct IgesEntity {
    // occt-ref: IGESData_IGESEntity // ::DENumber
    de_number: usize,
    // occt-ref: IGESData_IGESEntity // ::TypeNumber
    entity_type: IgesEntityType,
    // occt-ref: IGESData_IGESEntity // ::FormNumber
    form_number: i32,
    // occt-ref: IGESData_IGESEntity // parameter list (numeric)
    params: Vec<f64>,
    // occt-ref: IGESData_IGESEntity // hollerith / string parameters
    string_params: Vec<String>,
    // occt-ref: IGESData_IGESEntity // ::DefStatus
    status: IgesEntityStatus,
}

impl IgesEntity {
    // ----------------------------- constructors ----------------------------

    /// Creates a new entity with the given Directory Entry sequence number and
    /// entity type. All other fields start at their zero / default values.
    pub fn new(de_number: usize, entity_type: IgesEntityType) -> Self {
        Self {
            de_number,
            entity_type,
            form_number: 0,
            params: Vec::new(),
            string_params: Vec::new(),
            status: IgesEntityStatus::NotRead,
        }
    }

    // ------------------------------- getters -------------------------------

    /// Returns the Directory Entry sequence number (1-based odd integer in the
    /// IGES spec).
    // occt-ref: IGESData_IGESEntity // ::DENumber
    pub fn de_number(&self) -> usize {
        self.de_number
    }

    /// Returns the entity type discriminant.
    // occt-ref: IGESData_IGESEntity // ::TypeNumber → IgesEntityType
    pub fn entity_type(&self) -> IgesEntityType {
        self.entity_type
    }

    /// Returns the form number (secondary classification within an entity type).
    // occt-ref: IGESData_IGESEntity // ::FormNumber
    pub fn form_number(&self) -> i32 {
        self.form_number
    }

    /// Returns the current read/transfer status.
    // occt-ref: IGESData_IGESEntity // ::DefStatus
    pub fn status(&self) -> IgesEntityStatus {
        self.status
    }

    // ------------------------------- setters -------------------------------

    /// Sets the form number.
    // occt-ref: IGESData_IGESEntity // ::SetFormNumber
    pub fn set_form_number(&mut self, n: i32) {
        self.form_number = n;
    }

    /// Sets the read/transfer status.
    // occt-ref: IGESData_IGESEntity // ::SetStatus
    pub fn set_status(&mut self, s: IgesEntityStatus) {
        self.status = s;
    }

    // -------------------------- parameter access ---------------------------

    /// Appends a numeric parameter value to the parameter list.
    // occt-ref: IGESData_IGESEntity // parameter append
    pub fn add_param(&mut self, v: f64) {
        self.params.push(v);
    }

    /// Appends a string (hollerith) parameter to the string parameter list.
    // occt-ref: IGESData_IGESEntity // hollerith parameter append
    pub fn add_string_param(&mut self, s: &str) {
        self.string_params.push(s.to_string());
    }

    /// Returns the numeric parameter at index `i` (0-based).
    ///
    /// # Panics
    /// Panics if `i >= nb_params()`.
    // occt-ref: IGESData_IGESEntity // ::Param
    pub fn param(&self, i: usize) -> f64 {
        self.params[i]
    }

    /// Returns the total number of numeric parameters.
    // occt-ref: IGESData_IGESEntity // ::NbParams
    pub fn nb_params(&self) -> usize {
        self.params.len()
    }
}

// occt-ref: IGESData_IGESModel
/// An in-memory collection of [`IgesEntity`] values read from (or to be
/// written to) an IGES file, mirroring `IGESData_IGESModel`.
pub struct IgesModel {
    // occt-ref: IGESData_IGESModel // entity sequence
    entities: Vec<IgesEntity>,
    // occt-ref: IGESData_IGESModel // file name / header
    file_name: String,
}

impl IgesModel {
    // ----------------------------- constructors ----------------------------

    /// Creates an empty model associated with the given file name.
    // occt-ref: IGESData_IGESModel // constructor
    pub fn new(file_name: &str) -> Self {
        Self {
            entities: Vec::new(),
            file_name: file_name.to_string(),
        }
    }

    // ------------------------------- mutators ------------------------------

    /// Appends an entity to the model and returns its 0-based index.
    // occt-ref: IGESData_IGESModel // ::AddEntity
    pub fn add_entity(&mut self, e: IgesEntity) -> usize {
        let idx = self.entities.len();
        self.entities.push(e);
        idx
    }

    // ------------------------------- getters -------------------------------

    /// Returns a reference to the entity at 0-based index `i`.
    ///
    /// # Panics
    /// Panics if `i >= nb_entities()`.
    // occt-ref: IGESData_IGESModel // ::Entity
    pub fn entity(&self, i: usize) -> &IgesEntity {
        &self.entities[i]
    }

    /// Returns the total number of entities in the model.
    // occt-ref: IGESData_IGESModel // ::NbEntities
    pub fn nb_entities(&self) -> usize {
        self.entities.len()
    }

    /// Returns the file name string supplied at construction time.
    // occt-ref: IGESData_IGESModel // file name
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Returns the 0-based indices of all entities whose type matches `t`.
    // occt-ref: IGESData_IGESModel // ::EntitiesOfType
    pub fn entities_of_type(&self, t: IgesEntityType) -> Vec<usize> {
        self.entities
            .iter()
            .enumerate()
            .filter_map(|(i, e)| if e.entity_type == t { Some(i) } else { None })
            .collect()
    }
}
