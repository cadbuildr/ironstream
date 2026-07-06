// FILE: step_data_simple.rs
// occt: StepData_Simple

use std::rc::Rc;

// Local helper mirroring StepData_Field (external plumbing, subset)
#[derive(Clone, Default)]
pub struct StepDataField {
    kind: i32,
    int_val: i32,
    real_val: f64,
    text: Option<String>,
}

impl StepDataField {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_integer(&mut self, val: i32) {
        self.kind = 1;
        self.int_val = val;
    }
    pub fn integer(&self) -> i32 {
        self.int_val
    }
    pub fn set_real(&mut self, val: f64) {
        self.kind = 5;
        self.real_val = val;
    }
    pub fn real(&self) -> f64 {
        self.real_val
    }
    pub fn set_string(&mut self, val: &str) {
        self.kind = 6;
        self.text = Some(val.to_string());
    }
    pub fn string(&self) -> &str {
        self.text.as_deref().unwrap_or("")
    }
    pub fn is_set(&self) -> bool {
        self.kind != 0
    }
}

// Local helper mirroring StepData_ESDescr (external plumbing, subset):
// describes a simple entity: its type name and named fields
pub struct StepDataESDescr {
    type_name: String,
    field_names: Vec<String>,
}

impl StepDataESDescr {
    pub fn new(type_name: &str) -> Self {
        StepDataESDescr {
            type_name: type_name.to_string(),
            field_names: Vec::new(),
        }
    }

    pub fn set_nb_fields(&mut self, nb: usize) {
        self.field_names.resize(nb, String::new());
    }

    // SetField: names the field of rank num (1-based)
    pub fn set_field(&mut self, num: usize, name: &str) {
        if num >= 1 && num <= self.field_names.len() {
            self.field_names[num - 1] = name.to_string();
        }
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    pub fn nb_fields(&self) -> usize {
        self.field_names.len()
    }

    // Rank: 1-based rank of a field name, 0 if unknown
    pub fn rank(&self, name: &str) -> usize {
        for (i, n) in self.field_names.iter().enumerate() {
            if n == name {
                return i + 1;
            }
        }
        0
    }

    // Matches: exact type name match (no sub-type here)
    pub fn matches(&self, step_type: &str) -> bool {
        self.type_name == step_type
    }
}

// A Simple Entity is defined by a type (described by an ESDescr)
// and a list of parameters (fields), sized from the description
pub struct StepDataSimple {
    descr: Rc<StepDataESDescr>,
    fields: Vec<StepDataField>,
}

impl StepDataSimple {
    // Creates a Simple Entity from its description
    pub fn new(descr: Rc<StepDataESDescr>) -> Self {
        let nb = descr.nb_fields();
        StepDataSimple {
            descr,
            fields: vec![StepDataField::new(); nb],
        }
    }

    // Returns description, as for simple
    pub fn es_descr(&self) -> &Rc<StepDataESDescr> {
        &self.descr
    }

    // Returns the recorded StepType (TypeName of its ESDescr)
    pub fn step_type(&self) -> &str {
        self.descr.type_name()
    }

    // Returns False
    pub fn is_complex(&self) -> bool {
        false
    }

    // Tells if a step type is matched: own type
    pub fn matches(&self, step_type: &str) -> bool {
        self.descr.matches(step_type)
    }

    // Returns self if it matches, else None
    pub fn as_type(&self, step_type: &str) -> Option<&Self> {
        if self.matches(step_type) {
            Some(self)
        } else {
            None
        }
    }

    // Tells if a Field brings a given name
    pub fn has_field(&self, name: &str) -> bool {
        self.descr.rank(name) > 0
    }

    // Returns a Field from its name; read-only. None if unknown name
    pub fn field(&self, name: &str) -> Option<&StepDataField> {
        let num = self.descr.rank(name);
        if num == 0 {
            return None;
        }
        self.field_num(num)
    }

    // Returns a Field from its name; read or write. None if unknown name
    pub fn c_field(&mut self, name: &str) -> Option<&mut StepDataField> {
        let num = self.descr.rank(name);
        if num == 0 {
            return None;
        }
        self.c_field_num(num)
    }

    // Returns the count of fields
    pub fn nb_fields(&self) -> usize {
        self.fields.len()
    }

    // Returns a field from its rank (1-based), for read-only use
    pub fn field_num(&self, num: usize) -> Option<&StepDataField> {
        if num < 1 || num > self.fields.len() {
            return None;
        }
        Some(&self.fields[num - 1])
    }

    // Returns a field from its rank (1-based), in order to modify it
    pub fn c_field_num(&mut self, num: usize) -> Option<&mut StepDataField> {
        if num < 1 || num > self.fields.len() {
            return None;
        }
        Some(&mut self.fields[num - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point_descr() -> Rc<StepDataESDescr> {
        let mut d = StepDataESDescr::new("CARTESIAN_POINT");
        d.set_nb_fields(2);
        d.set_field(1, "name");
        d.set_field(2, "coordinates");
        Rc::new(d)
    }

    #[test]
    fn test_simple_new() {
        let simple = StepDataSimple::new(point_descr());
        assert_eq!(simple.step_type(), "CARTESIAN_POINT");
        assert_eq!(simple.nb_fields(), 2);
        assert!(!simple.is_complex());
    }

    #[test]
    fn test_matches_and_as_type() {
        let simple = StepDataSimple::new(point_descr());
        assert!(simple.matches("CARTESIAN_POINT"));
        assert!(!simple.matches("DIRECTION"));
        assert!(simple.as_type("CARTESIAN_POINT").is_some());
        assert!(simple.as_type("DIRECTION").is_none());
    }

    #[test]
    fn test_fields_by_name_and_rank() {
        let mut simple = StepDataSimple::new(point_descr());
        assert!(simple.has_field("name"));
        assert!(simple.has_field("coordinates"));
        assert!(!simple.has_field("radius"));

        simple.c_field("name").unwrap().set_string("P1");
        simple.c_field_num(2).unwrap().set_real(4.5);

        assert_eq!(simple.field("name").unwrap().string(), "P1");
        assert_eq!(simple.field_num(1).unwrap().string(), "P1");
        assert!((simple.field("coordinates").unwrap().real() - 4.5).abs() < 1e-12);
        assert!(simple.field("radius").is_none());
        assert!(simple.field_num(3).is_none());
        assert!(simple.field_num(0).is_none());
    }
}
