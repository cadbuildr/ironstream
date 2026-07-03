// FILE: step_data_ext.rs
// occt: StepData_StepModel, StepData_Protocol, StepData_SelectMember

use std::collections::HashMap;

/// STEP entity category.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepEntityCategory {
    Geometry,
    Topology,
    Product,
    Representation,
    Application,
    Unknown,
}

impl Default for StepEntityCategory {
    fn default() -> Self { Self::Unknown }
}

/// A STEP entity record.
#[derive(Clone, Debug)]
pub struct StepEntity {
    pub id: u32,
    pub type_name: String,
    pub category: StepEntityCategory,
    pub attributes: HashMap<String, String>,
}

impl StepEntity {
    pub fn new(id: u32, type_name: &str) -> Self {
        Self { id, type_name: type_name.to_owned(), category: StepEntityCategory::Unknown, attributes: HashMap::new() }
    }

    pub fn set_attr(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_owned(), value.to_owned());
    }

    pub fn get_attr(&self, key: &str) -> Option<&str> { self.attributes.get(key).map(|s| s.as_str()) }
}

// occt: StepData_StepModel — the data model for a STEP file
#[derive(Clone, Debug, Default)]
pub struct StepDataStepModel {
    pub entities: Vec<StepEntity>,
    pub header_entities: Vec<StepEntity>,
    pub schema: String,
    pub entity_map: HashMap<u32, usize>,
}

impl StepDataStepModel {
    pub fn new() -> Self { Self { schema: "AP214".to_owned(), ..Default::default() } }

    pub fn set_schema(&mut self, s: &str) { self.schema = s.to_owned(); }

    pub fn add_entity(&mut self, entity: StepEntity) {
        let idx = self.entities.len();
        self.entity_map.insert(entity.id, idx);
        self.entities.push(entity);
    }

    pub fn nb_entities(&self) -> usize { self.entities.len() }

    pub fn entity(&self, id: u32) -> Option<&StepEntity> {
        self.entity_map.get(&id).and_then(|&i| self.entities.get(i))
    }

    pub fn entity_by_index(&self, i: usize) -> Option<&StepEntity> { self.entities.get(i) }

    pub fn clear(&mut self) {
        self.entities.clear();
        self.header_entities.clear();
        self.entity_map.clear();
    }

    pub fn has_entity(&self, id: u32) -> bool { self.entity_map.contains_key(&id) }

    pub fn entities_of_type(&self, type_name: &str) -> Vec<&StepEntity> {
        self.entities.iter().filter(|e| e.type_name == type_name).collect()
    }
}

// occt: StepData_Protocol — describes which STEP entities are recognized
#[derive(Clone, Debug, Default)]
pub struct StepDataProtocol {
    pub name: String,
    pub recognized_types: Vec<String>,
    pub version: String,
}

impl StepDataProtocol {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_owned(), version: "1".to_owned(), ..Default::default() }
    }

    pub fn add_type(&mut self, type_name: &str) { self.recognized_types.push(type_name.to_owned()); }
    pub fn nb_types(&self) -> usize { self.recognized_types.len() }
    pub fn recognizes(&self, type_name: &str) -> bool { self.recognized_types.iter().any(|t| t == type_name) }
    pub fn schema_name(&self) -> &str { &self.name }
}

/// A discriminated-union member for STEP SELECT types.
#[derive(Clone, Debug)]
pub enum SelectValue {
    Integer(i64),
    Real(f64),
    String(String),
    Entity(u32),
    Bool(bool),
    Enum(String),
}

// occt: StepData_SelectMember — a typed member of a STEP SELECT value
#[derive(Clone, Debug)]
pub struct StepDataSelectMember {
    pub kind: String,
    pub value: SelectValue,
}

impl StepDataSelectMember {
    pub fn integer(kind: &str, v: i64) -> Self {
        Self { kind: kind.to_owned(), value: SelectValue::Integer(v) }
    }
    pub fn real(kind: &str, v: f64) -> Self {
        Self { kind: kind.to_owned(), value: SelectValue::Real(v) }
    }
    pub fn string(kind: &str, v: &str) -> Self {
        Self { kind: kind.to_owned(), value: SelectValue::String(v.to_owned()) }
    }
    pub fn entity(kind: &str, id: u32) -> Self {
        Self { kind: kind.to_owned(), value: SelectValue::Entity(id) }
    }

    pub fn kind(&self) -> &str { &self.kind }

    pub fn as_integer(&self) -> Option<i64> {
        if let SelectValue::Integer(v) = &self.value { Some(*v) } else { None }
    }
    pub fn as_real(&self) -> Option<f64> {
        if let SelectValue::Real(v) = &self.value { Some(*v) } else { None }
    }
    pub fn as_string(&self) -> Option<&str> {
        if let SelectValue::String(s) = &self.value { Some(s.as_str()) } else { None }
    }
    pub fn as_entity(&self) -> Option<u32> {
        if let SelectValue::Entity(id) = &self.value { Some(*id) } else { None }
    }
}

// occt: StepData_EDescr — entity descriptor (type metadata)
#[derive(Clone, Debug)]
pub struct StepDataEDescr {
    pub type_name: String,
    pub fields: Vec<String>,
    pub is_complex: bool,
    pub super_types: Vec<String>,
}

impl StepDataEDescr {
    pub fn new(type_name: &str) -> Self {
        Self { type_name: type_name.to_owned(), fields: Vec::new(), is_complex: false, super_types: Vec::new() }
    }

    pub fn add_field(&mut self, name: &str) { self.fields.push(name.to_owned()); }
    pub fn add_super_type(&mut self, name: &str) { self.super_types.push(name.to_owned()); }
    pub fn nb_fields(&self) -> usize { self.fields.len() }
    pub fn has_field(&self, name: &str) -> bool { self.fields.iter().any(|f| f == name) }
    pub fn type_name(&self) -> &str { &self.type_name }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_model_add_entity() {
        let mut m = StepDataStepModel::new();
        let e = StepEntity::new(1, "PRODUCT");
        m.add_entity(e);
        assert_eq!(m.nb_entities(), 1);
        assert!(m.has_entity(1));
        assert_eq!(m.entity(1).unwrap().type_name, "PRODUCT");
    }

    #[test]
    fn step_model_filter_by_type() {
        let mut m = StepDataStepModel::new();
        m.add_entity(StepEntity::new(1, "PRODUCT"));
        m.add_entity(StepEntity::new(2, "PRODUCT"));
        m.add_entity(StepEntity::new(3, "PLANE"));
        assert_eq!(m.entities_of_type("PRODUCT").len(), 2);
        assert_eq!(m.entities_of_type("PLANE").len(), 1);
    }

    #[test]
    fn step_protocol_recognizes() {
        let mut proto = StepDataProtocol::new("AP214");
        proto.add_type("PRODUCT");
        proto.add_type("PLANE");
        assert!(proto.recognizes("PRODUCT"));
        assert!(!proto.recognizes("SPHERE"));
    }

    #[test]
    fn select_member_integer() {
        let sm = StepDataSelectMember::integer("LENGTH_MEASURE", 42);
        assert_eq!(sm.as_integer(), Some(42));
        assert_eq!(sm.kind(), "LENGTH_MEASURE");
        assert!(sm.as_real().is_none());
    }

    #[test]
    fn select_member_entity() {
        let sm = StepDataSelectMember::entity("PRODUCT_REF", 100);
        assert_eq!(sm.as_entity(), Some(100));
    }

    #[test]
    fn edescr_fields() {
        let mut d = StepDataEDescr::new("PRODUCT");
        d.add_field("id");
        d.add_field("name");
        d.add_super_type("PRODUCT_DEFINITION");
        assert_eq!(d.nb_fields(), 2);
        assert!(d.has_field("name"));
        assert!(!d.has_field("missing"));
    }
}
