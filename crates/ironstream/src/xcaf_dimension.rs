// FILE: rust/ironstream/crates/ironstream/src/xcaf_dimension.rs

// occt: XCAFDimTolObjects_DimensionType // — classifies a dimension or tolerance
// annotation stored in an XDE document label.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DimTolType {
    /// A length or distance measurement between two geometry elements.
    Linear,
    /// An angular measurement between two directions or planes.
    Angular,
    /// The radius of a circular or spherical feature.
    Radius,
    /// The diameter of a circular or cylindrical feature.
    Diameter,
    /// A form tolerance controlling the deviation of a surface from a true cylinder.
    Cylindricity,
    /// A form tolerance controlling the deviation of a surface from a true plane.
    Flatness,
    /// A form tolerance controlling the deviation of a cross-section from a true circle.
    Roundness,
    /// A form tolerance controlling the deviation of an axis or edge from a straight line.
    Straightness,
    /// An orientation tolerance specifying that two features must be equidistant along their lengths.
    Parallelism,
    /// An orientation tolerance specifying that a feature is exactly 90 degrees to a datum.
    Perpendicularity,
    /// An orientation tolerance specifying the angular relationship of a feature to a datum.
    Angularity,
    /// A location tolerance specifying that two features share the same axis.
    Coaxiality,
    /// A location tolerance specifying symmetric relationship about a datum plane.
    Symmetry,
    /// A location tolerance specifying that a feature lies within a tolerance zone.
    PositionTolerance,
}

// occt-note: dimension/tolerance entry — a single annotated dimension or tolerance
// record attached to a shape label, mirroring XCAFDoc_DimTolTool's per-entry
// data model (label + value + limits + optional description).
#[derive(Clone, Debug)]
pub struct DimTolEntry {
    id: usize,
    dim_type: DimTolType,
    value: f64,
    lower: f64,
    upper: f64,
    description: String,
}

impl DimTolEntry {
    /// Create a new entry with the given id, type, and nominal value.
    /// Limits default to the nominal value (symmetric zero-width zone).
    pub fn new(id: usize, dim_type: DimTolType, value: f64) -> Self {
        Self {
            id,
            dim_type,
            value,
            lower: value,
            upper: value,
            description: String::new(),
        }
    }

    /// Return the unique id of this entry.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Return the dimension/tolerance type.
    pub fn dim_type(&self) -> DimTolType {
        self.dim_type
    }

    /// Return the nominal value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Return the lower limit of the tolerance zone.
    pub fn lower(&self) -> f64 {
        self.lower
    }

    /// Return the upper limit of the tolerance zone.
    pub fn upper(&self) -> f64 {
        self.upper
    }

    /// Set both lower and upper limits of the tolerance zone.
    pub fn set_limits(&mut self, lower: f64, upper: f64) {
        self.lower = lower;
        self.upper = upper;
    }

    /// Return the human-readable description string.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Replace the description string.
    pub fn set_description(&mut self, desc: &str) {
        self.description = desc.to_owned();
    }
}

// occt: XCAFDoc_DimTolTool // — manages a collection of dimension and tolerance
// entries stored in an XDE document, providing creation, lookup, and
// type-filtered queries.
pub struct DimTolTool {
    entries: Vec<DimTolEntry>,
    next_id: usize,
}

impl DimTolTool {
    /// Create an empty tool with no entries.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 0,
        }
    }

    /// Add a plain dimension (lower and upper both equal to value). Returns the
    /// new entry's id.
    pub fn add_dimension(&mut self, dim_type: DimTolType, value: f64) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.entries.push(DimTolEntry::new(id, dim_type, value));
        id
    }

    /// Add a tolerance entry with explicit lower/upper limits. Returns the new
    /// entry's id.
    pub fn add_tolerance(
        &mut self,
        dim_type: DimTolType,
        value: f64,
        lower: f64,
        upper: f64,
    ) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let mut entry = DimTolEntry::new(id, dim_type, value);
        entry.set_limits(lower, upper);
        self.entries.push(entry);
        id
    }

    /// Look up an entry by its id. Returns `None` if no entry with that id
    /// exists.
    pub fn find(&self, id: usize) -> Option<&DimTolEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    /// Return the total number of entries in the tool.
    pub fn nb_entries(&self) -> usize {
        self.entries.len()
    }

    /// Return the ids of all entries whose type matches `t`.
    pub fn entries_of_type(&self, t: DimTolType) -> Vec<usize> {
        self.entries
            .iter()
            .filter(|e| e.dim_type == t)
            .map(|e| e.id)
            .collect()
    }
}

impl Default for DimTolTool {
    fn default() -> Self {
        Self::new()
    }
}
