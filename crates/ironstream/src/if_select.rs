// FILE: if_select.rs
// occt: IFSelect_WorkSession, IFSelect_Signature, IFSelect_SelectBase,
//       IFSelect_SelectAnyList, IFSelect_Dispatch

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkSessionStatus {
    Done,
    Fail1,   // no model
    Fail2,   // invalid file
    Fail3,   // transfer failed
}

/// Entity filter result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatchResult {
    Yes,
    No,
    Undefined,
}

// occt: IFSelect_Signature — textual attribute extractor from entity
pub trait Signature {
    fn value(&self, entity_id: u64) -> String;
    fn name(&self) -> &str;
    fn matches(&self, entity_id: u64, pattern: &str, exact: bool) -> bool {
        let v = self.value(entity_id);
        if exact { v == pattern } else { v.contains(pattern) }
    }
}

/// Type-name signature: returns entity type string.
#[derive(Clone, Debug)]
pub struct TypeNameSignature;

impl Signature for TypeNameSignature {
    fn value(&self, _id: u64) -> String { "ENTITY".into() }
    fn name(&self) -> &str { "TypeName" }
}

// occt: IFSelect_SelectBase — base selector for entities
pub trait Selector {
    fn is_selected(&self, entity_id: u64, model: &WorkModel) -> bool;
    fn label(&self) -> &str;
}

/// Select all entities.
#[derive(Clone, Debug)]
pub struct SelectAll;
impl Selector for SelectAll {
    fn is_selected(&self, _id: u64, _m: &WorkModel) -> bool { true }
    fn label(&self) -> &str { "All" }
}

/// Select entities by type name.
#[derive(Clone, Debug)]
pub struct SelectByType { pub type_name: String }
impl Selector for SelectByType {
    fn is_selected(&self, id: u64, m: &WorkModel) -> bool {
        m.entity_type(id).map(|t| t == self.type_name).unwrap_or(false)
    }
    fn label(&self) -> &str { "ByType" }
}

/// Minimal entity record in a work model.
#[derive(Clone, Debug)]
pub struct ModelEntity {
    pub id: u64,
    pub entity_type: String,
    pub refs: Vec<u64>,
}

// occt: IFSelect_WorkSession model
#[derive(Clone, Debug, Default)]
pub struct WorkModel {
    pub entities: Vec<ModelEntity>,
    pub source_file: String,
    pub schema: String,
}

impl WorkModel {
    pub fn new() -> Self { Self::default() }

    pub fn add_entity(&mut self, id: u64, entity_type: impl Into<String>) {
        self.entities.push(ModelEntity { id, entity_type: entity_type.into(), refs: Vec::new() });
    }

    pub fn add_ref(&mut self, from: u64, to: u64) {
        if let Some(e) = self.entities.iter_mut().find(|e| e.id == from) { e.refs.push(to); }
    }

    pub fn entity_type(&self, id: u64) -> Option<String> {
        self.entities.iter().find(|e| e.id == id).map(|e| e.entity_type.clone())
    }

    pub fn nb_entities(&self) -> usize { self.entities.len() }
    pub fn is_empty(&self) -> bool { self.entities.is_empty() }
    pub fn find(&self, id: u64) -> Option<&ModelEntity> { self.entities.iter().find(|e| e.id == id) }

    pub fn referencing(&self, id: u64) -> Vec<u64> {
        self.entities.iter().filter(|e| e.refs.contains(&id)).map(|e| e.id).collect()
    }

    pub fn entities_of_type(&self, t: &str) -> Vec<&ModelEntity> {
        self.entities.iter().filter(|e| e.entity_type == t).collect()
    }
}

// occt: IFSelect_WorkSession
#[derive(Clone, Debug, Default)]
pub struct WorkSession {
    pub model: WorkModel,
    pub status: Option<WorkSessionStatus>,
    pub nb_fails: usize,
    pub nb_warnings: usize,
}

impl WorkSession {
    pub fn new() -> Self { Self::default() }

    pub fn load_file(&mut self, path: &str) -> WorkSessionStatus {
        self.model.source_file = path.to_string();
        let ok = !path.is_empty() && path.ends_with(".stp") || path.ends_with(".igs") || path.ends_with(".step");
        let s = if ok { WorkSessionStatus::Done } else { WorkSessionStatus::Fail2 };
        self.status = Some(s);
        s
    }

    pub fn is_done(&self) -> bool { self.status == Some(WorkSessionStatus::Done) }
    pub fn nb_entities(&self) -> usize { self.model.nb_entities() }

    pub fn apply_selector(&self, sel: &dyn Selector) -> Vec<u64> {
        self.model.entities.iter()
            .filter(|e| sel.is_selected(e.id, &self.model))
            .map(|e| e.id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_model_entities() {
        let mut m = WorkModel::new();
        m.add_entity(1, "CARTESIAN_POINT");
        m.add_entity(2, "DIRECTION");
        m.add_ref(2, 1);
        assert_eq!(m.nb_entities(), 2);
        assert_eq!(m.entities_of_type("CARTESIAN_POINT").len(), 1);
        assert_eq!(m.referencing(1), vec![2]);
    }

    #[test]
    fn work_session_load() {
        let mut s = WorkSession::new();
        let r = s.load_file("model.stp");
        assert_eq!(r, WorkSessionStatus::Done);
        assert!(s.is_done());
    }

    #[test]
    fn work_session_selector() {
        let mut s = WorkSession::new();
        s.model.add_entity(1, "CARTESIAN_POINT");
        s.model.add_entity(2, "DIRECTION");
        s.model.add_entity(3, "CARTESIAN_POINT");
        let sel = SelectByType { type_name: "CARTESIAN_POINT".into() };
        let ids = s.apply_selector(&sel);
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn type_name_signature() {
        let sig = TypeNameSignature;
        assert!(sig.matches(1, "ENTITY", true));
        assert!(!sig.matches(1, "OTHER", true));
    }
}
