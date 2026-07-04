// FILE: step_data_es_descr.rs
// occt: StepData_ESDescr

use std::collections::HashMap;

//! This class is intended to describe the authorized form for a
//! Simple (not Plex) Entity, as a list of fields
pub struct StepDataESDescr {
    type_name: String,
    fields: Vec<Option<String>>, // field names at each index
    descr: HashMap<String, usize>, // name -> field index (1-based)
    base: Option<Box<StepDataESDescr>>,
    super_type: Option<Box<StepDataESDescr>>,
}

impl StepDataESDescr {
    //! Creates an ESDescr with a type name
    pub fn new(name: &str) -> Self {
        StepDataESDescr {
            type_name: name.to_string(),
            fields: Vec::new(),
            descr: HashMap::new(),
            base: None,
            super_type: None,
        }
    }

    //! Sets a new count of fields
    pub fn set_nb_fields(&mut self, nb: usize) {
        self.descr.clear();
        if nb == 0 {
            self.fields.clear();
            return;
        }
        let old_nb = self.fields.len();
        self.fields.resize(nb, None);

        // Rebuild name-to-index mapping
        for i in 0..old_nb.min(nb) {
            if let Some(ref name) = self.fields[i] {
                self.descr.insert(name.clone(), i + 1);
            }
        }
    }

    //! Sets a PDescr to describe a field
    pub fn set_field(&mut self, num: usize, name: &str) {
        if num < 1 || num > self.fields.len() {
            return;
        }
        self.fields[num - 1] = Some(name.to_string());
        self.descr.insert(name.to_string(), num);
    }

    //! Sets an ESDescr as based on another one
    pub fn set_base(&mut self, base: StepDataESDescr) {
        self.base = Some(Box::new(base));
    }

    //! Sets an ESDescr as "super-type"
    pub fn set_super(&mut self, super_type: StepDataESDescr) {
        let sup = if let Some(ref base) = super_type.base {
            *base.clone()
        } else {
            super_type
        };

        if let Some(ref mut base) = self.base {
            base.set_super(sup);
        } else {
            self.super_type = Some(Box::new(sup));
        }
    }

    //! Returns the type name given at creation time
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    //! Returns the basic ESDescr, None if not derived
    pub fn base(&self) -> Option<&StepDataESDescr> {
        self.base.as_ref().map(|b| b.as_ref())
    }

    //! Returns the super-type ESDescr, None if root
    pub fn super_type(&self) -> Option<&StepDataESDescr> {
        self.super_type.as_ref().map(|s| s.as_ref())
    }

    //! Tells if this is sub-type of another one
    pub fn is_sub(&self, other: &StepDataESDescr) -> bool {
        let oth = if let Some(ref base) = other.base {
            base.as_ref()
        } else {
            other
        };

        if let Some(ref base) = self.base {
            return base.is_sub(oth);
        }

        if std::ptr::eq(self, oth) {
            return true;
        }

        if let Some(ref super_type) = self.super_type {
            if std::ptr::eq(super_type.as_ref(), oth) {
                return true;
            }
            return super_type.is_sub(oth);
        }

        false
    }

    //! Returns the count of fields
    pub fn nb_fields(&self) -> usize {
        self.fields.len()
    }

    //! Returns the rank of a field from its name. 0 if unknown
    pub fn rank(&self, name: &str) -> usize {
        self.descr.get(name).copied().unwrap_or(0)
    }

    //! Returns the name of a field from its rank
    pub fn name(&self, num: usize) -> Option<&str> {
        if num < 1 || num > self.fields.len() {
            return None;
        }
        self.fields[num - 1].as_deref()
    }

    //! Tells if an ESDescr matches a step type
    pub fn matches(&self, step_type: &str) -> bool {
        if self.type_name == step_type {
            return true;
        }
        if let Some(ref super_type) = self.super_type {
            return super_type.matches(step_type);
        }
        false
    }

    //! Returns False (not a complex type)
    pub fn is_complex(&self) -> bool {
        false
    }
}

impl Clone for StepDataESDescr {
    fn clone(&self) -> Self {
        StepDataESDescr {
            type_name: self.type_name.clone(),
            fields: self.fields.clone(),
            descr: self.descr.clone(),
            base: self.base.clone(),
            super_type: self.super_type.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_es_descr() {
        let descr = StepDataESDescr::new("TestType");
        assert_eq!(descr.type_name(), "TestType");
        assert_eq!(descr.nb_fields(), 0);
        assert!(!descr.is_complex());
    }

    #[test]
    fn test_set_nb_fields() {
        let mut descr = StepDataESDescr::new("TestType");
        descr.set_nb_fields(3);
        assert_eq!(descr.nb_fields(), 3);
    }

    #[test]
    fn test_set_field() {
        let mut descr = StepDataESDescr::new("TestType");
        descr.set_nb_fields(2);
        descr.set_field(1, "field1");
        descr.set_field(2, "field2");

        assert_eq!(descr.rank("field1"), 1);
        assert_eq!(descr.rank("field2"), 2);
        assert_eq!(descr.name(1), Some("field1"));
        assert_eq!(descr.name(2), Some("field2"));
    }

    #[test]
    fn test_matches() {
        let descr = StepDataESDescr::new("MyType");
        assert!(descr.matches("MyType"));
        assert!(!descr.matches("OtherType"));
    }

    #[test]
    fn test_rank_unknown_field() {
        let descr = StepDataESDescr::new("TestType");
        assert_eq!(descr.rank("nonexistent"), 0);
    }
}
