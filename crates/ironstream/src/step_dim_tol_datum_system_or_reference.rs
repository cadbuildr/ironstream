// FILE: step_dim_tol_datum_system_or_reference.rs
// occt: StepDimTol_DatumSystemOrReference

//! A select type for DatumSystem or DatumReference in dimensional and tolerancing.

use std::rc::Rc;

/// Placeholder for DatumSystem
#[derive(Debug, Clone)]
pub struct DatumSystem {
    id: String,
}

impl DatumSystem {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Placeholder for DatumReference
#[derive(Debug, Clone)]
pub struct DatumReference {
    id: String,
}

impl DatumReference {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

/// A select type that can hold either a DatumSystem or a DatumReference
#[derive(Debug, Clone)]
pub enum DatumSystemOrReferenceType {
    DatumSystem(Rc<DatumSystem>),
    DatumReference(Rc<DatumReference>),
}

/// StepDimTol_DatumSystemOrReference - a select type entity
#[derive(Debug, Clone)]
pub struct StepDimTolDatumSystemOrReference {
    value: Option<DatumSystemOrReferenceType>,
    case_num: i32,
}

impl StepDimTolDatumSystemOrReference {
    /// Create a new DatumSystemOrReference
    pub fn new() -> Self {
        Self {
            value: None,
            case_num: 0,
        }
    }

    /// Recognize the case number of a DatumSystemOrReference entity
    /// 1 -> DatumSystem
    /// 2 -> DatumReference
    /// 0 -> Unknown
    pub fn case_num(&self) -> i32 {
        self.case_num
    }

    /// Set value as a DatumSystem
    pub fn set_datum_system(&mut self, system: Rc<DatumSystem>) {
        self.value = Some(DatumSystemOrReferenceType::DatumSystem(system));
        self.case_num = 1;
    }

    /// Get value as a DatumSystem (returns None if it's a DatumReference)
    pub fn datum_system(&self) -> Option<Rc<DatumSystem>> {
        match &self.value {
            Some(DatumSystemOrReferenceType::DatumSystem(sys)) => Some(sys.clone()),
            _ => None,
        }
    }

    /// Set value as a DatumReference
    pub fn set_datum_reference(&mut self, reference: Rc<DatumReference>) {
        self.value = Some(DatumSystemOrReferenceType::DatumReference(reference));
        self.case_num = 2;
    }

    /// Get value as a DatumReference (returns None if it's a DatumSystem)
    pub fn datum_reference(&self) -> Option<Rc<DatumReference>> {
        match &self.value {
            Some(DatumSystemOrReferenceType::DatumReference(ref_)) => Some(ref_.clone()),
            _ => None,
        }
    }

    /// Check if this select has a value
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }
}

impl Default for StepDimTolDatumSystemOrReference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sel = StepDimTolDatumSystemOrReference::new();
        assert_eq!(sel.case_num(), 0);
        assert!(!sel.has_value());
    }

    #[test]
    fn test_datum_system() {
        let mut sel = StepDimTolDatumSystemOrReference::new();
        let system = Rc::new(DatumSystem::new("SYS_A".to_string()));
        sel.set_datum_system(system.clone());
        assert_eq!(sel.case_num(), 1);
        assert!(sel.has_value());
        assert!(sel.datum_system().is_some());
        assert!(sel.datum_reference().is_none());
        assert_eq!(sel.datum_system().unwrap().id(), "SYS_A");
    }

    #[test]
    fn test_datum_reference() {
        let mut sel = StepDimTolDatumSystemOrReference::new();
        let reference = Rc::new(DatumReference::new("REF_B".to_string()));
        sel.set_datum_reference(reference.clone());
        assert_eq!(sel.case_num(), 2);
        assert!(sel.has_value());
        assert!(sel.datum_reference().is_some());
        assert!(sel.datum_system().is_none());
        assert_eq!(sel.datum_reference().unwrap().id(), "REF_B");
    }

    #[test]
    fn test_datum_system_type() {
        let system = DatumSystem::new("SYS_X".to_string());
        assert_eq!(system.id(), "SYS_X");
    }

    #[test]
    fn test_datum_reference_type() {
        let reference = DatumReference::new("REF_Y".to_string());
        assert_eq!(reference.id(), "REF_Y");
    }

    #[test]
    fn test_default() {
        let sel = StepDimTolDatumSystemOrReference::default();
        assert_eq!(sel.case_num(), 0);
    }
}
