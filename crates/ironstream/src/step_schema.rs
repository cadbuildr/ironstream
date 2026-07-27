// FILE: rust/ironstream/crates/ironstream/src/step_schema.rs

// occt-note: STEP entity type enum — mirrors the named entities recognised by
// StepData_StepModel / RWStepShape readers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StepEntityType {
    ProductDefinition,
    ShapeRepresentation,
    AdvancedFace,
    ClosedShell,
    ManifoldSolidBrep,
    GeometricCurveSet,
    Axis2Placement3D,
    CartesianPoint,
    Direction,
    Vector,
    Line,
    Circle,
    Plane,
    Unknown,
}

// occt: StepData_SelectType // — entity reference; carries the numeric STEP
// instance id and the resolved entity type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepEntityRef {
    id: usize,
    entity_type: StepEntityType,
}

impl StepEntityRef {
    pub fn new(id: usize, entity_type: StepEntityType) -> Self {
        Self { id, entity_type }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn entity_type(&self) -> StepEntityType {
        self.entity_type
    }
}

// occt-ref: StepData_StepModel // entity — one parsed STEP instance, holding its
// literal parameter strings and resolved entity cross-references.
#[derive(Debug, Clone)]
pub struct StepEntity {
    id: usize,
    entity_type: StepEntityType,
    params: Vec<String>,
    refs: Vec<StepEntityRef>,
}

impl StepEntity {
    pub fn new(id: usize, entity_type: StepEntityType) -> Self {
        Self {
            id,
            entity_type,
            params: Vec::new(),
            refs: Vec::new(),
        }
    }

    pub fn add_param(&mut self, s: &str) {
        self.params.push(s.to_owned());
    }

    pub fn add_ref(&mut self, r: StepEntityRef) {
        self.refs.push(r);
    }

    pub fn nb_params(&self) -> usize {
        self.params.len()
    }

    pub fn param(&self, i: usize) -> &str {
        &self.params[i]
    }

    pub fn nb_refs(&self) -> usize {
        self.refs.len()
    }

    pub fn entity_ref(&self, i: usize) -> &StepEntityRef {
        &self.refs[i]
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn entity_type(&self) -> StepEntityType {
        self.entity_type
    }
}

// occt-ref: StepData_StepModel // — the in-memory representation of a complete STEP
// file: one header string plus a flat list of entity instances indexed by their
// STEP numeric id.
#[derive(Debug, Clone)]
pub struct StepModel {
    pub header: String,
    pub entities: Vec<StepEntity>,
    pub next_id: usize,
}

impl StepModel {
    pub fn new(header: &str) -> Self {
        Self {
            header: header.to_owned(),
            entities: Vec::new(),
            next_id: 1,
        }
    }

    /// Add an entity to the model and return the index at which it was stored.
    pub fn add_entity(&mut self, e: StepEntity) -> usize {
        let idx = self.entities.len();
        self.entities.push(e);
        self.next_id = self.next_id.max(self.entities[idx].id + 1);
        idx
    }

    /// Look up an entity by its STEP numeric instance id.
    pub fn find_entity(&self, id: usize) -> Option<&StepEntity> {
        self.entities.iter().find(|e| e.id() == id)
    }

    pub fn nb_entities(&self) -> usize {
        self.entities.len()
    }

    /// Return the indices (into `self.entities`) of every entity whose type
    /// matches `t`.
    pub fn entities_of_type(&self, t: StepEntityType) -> Vec<usize> {
        self.entities
            .iter()
            .enumerate()
            .filter_map(|(i, e)| if e.entity_type() == t { Some(i) } else { None })
            .collect()
    }
}
