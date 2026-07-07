// FILE: iges_data_free_format_entity.rs
// occt: IGESData_FreeFormatEntity

//! Allows creating IGES Entities in a literal form without specific class recognition.
//! Used to define test files without creating specific entity classes.

#[derive(Clone, Debug)]
pub enum ParamType {
    Integer,
    Real,
    String,
    Reference,
}

/// FreeFormatEntity represents an undefined IGES entity with flexible parameters
#[derive(Clone, Debug)]
pub struct FreeFormatEntity {
    type_number: i32,
    form_number: i32,
    params: Vec<(ParamType, String)>,
    negative_pointers: Vec<usize>,
}

impl FreeFormatEntity {
    /// Creates a completely empty FreeFormatEntity
    pub fn new() -> Self {
        FreeFormatEntity {
            type_number: 0,
            form_number: 0,
            params: Vec::new(),
            negative_pointers: Vec::new(),
        }
    }

    /// Sets Type Number to a new Value, and Form Number to Zero
    pub fn set_type_number(&mut self, typenum: i32) {
        self.type_number = typenum;
        self.form_number = 0;
    }

    /// Sets Form Number to a new Value (to be called after SetTypeNumber)
    pub fn set_form_number(&mut self, formnum: i32) {
        self.form_number = formnum;
    }

    /// Gives count of recorded parameters
    pub fn nb_params(&self) -> usize {
        self.params.len()
    }

    /// Returns the ParamType of a Param, given its rank
    pub fn param_type(&self, num: usize) -> Option<ParamType> {
        if num > 0 && num <= self.params.len() {
            Some(self.params[num - 1].0.clone())
        } else {
            None
        }
    }

    /// Returns True if a Parameter is recorded as an entity
    pub fn is_param_entity(&self, num: usize) -> bool {
        if num > 0 && num <= self.params.len() {
            matches!(self.params[num - 1].0, ParamType::Reference)
        } else {
            false
        }
    }

    /// Returns True if <num> is noted as for a "Negative Pointer"
    pub fn is_negative_pointer(&self, num: usize) -> bool {
        self.negative_pointers.contains(&num)
    }

    /// Returns literal value of a Parameter, given its rank
    pub fn param_value(&self, num: usize) -> Option<String> {
        if num > 0 && num <= self.params.len() {
            Some(self.params[num - 1].1.clone())
        } else {
            None
        }
    }

    /// Returns the list of ranks of Parameters which have been noted as Negative Pointers
    pub fn negative_pointers(&self) -> Vec<usize> {
        self.negative_pointers.clone()
    }

    /// Adds a literal Parameter to the list
    pub fn add_literal(&mut self, ptype: ParamType, value: &str) {
        self.params.push((ptype, value.to_string()));
    }

    /// Adds an entity Parameter to the list
    pub fn add_entity(&mut self, entity_rank: usize) {
        self.params.push((ParamType::Reference, entity_rank.to_string()));
    }

    /// Adds an entity Parameter noted as Negative Pointer
    pub fn add_negative_pointer(&mut self, entity_rank: usize) {
        let rank = self.params.len() + 1;
        self.params.push((ParamType::Reference, entity_rank.to_string()));
        self.negative_pointers.push(rank);
    }

    /// Returns type and form numbers
    pub fn type_form(&self) -> (i32, i32) {
        (self.type_number, self.form_number)
    }
}

impl Default for FreeFormatEntity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ffe = FreeFormatEntity::new();
        assert_eq!(ffe.type_number, 0);
        assert_eq!(ffe.form_number, 0);
        assert_eq!(ffe.nb_params(), 0);
    }

    #[test]
    fn test_set_type_number() {
        let mut ffe = FreeFormatEntity::new();
        ffe.set_type_number(102);
        assert_eq!(ffe.type_number, 102);
        assert_eq!(ffe.form_number, 0);
    }

    #[test]
    fn test_set_form_number() {
        let mut ffe = FreeFormatEntity::new();
        ffe.set_type_number(102);
        ffe.set_form_number(5);
        assert_eq!(ffe.type_number, 102);
        assert_eq!(ffe.form_number, 5);
    }

    #[test]
    fn test_add_literal() {
        let mut ffe = FreeFormatEntity::new();
        ffe.add_literal(ParamType::String, "test");
        assert_eq!(ffe.nb_params(), 1);
        assert_eq!(ffe.param_value(1), Some("test".to_string()));
    }

    #[test]
    fn test_add_entity() {
        let mut ffe = FreeFormatEntity::new();
        ffe.add_entity(42);
        assert_eq!(ffe.nb_params(), 1);
        assert!(ffe.is_param_entity(1));
    }

    #[test]
    fn test_is_param_entity() {
        let mut ffe = FreeFormatEntity::new();
        ffe.add_literal(ParamType::String, "text");
        ffe.add_entity(100);

        assert!(!ffe.is_param_entity(1));
        assert!(ffe.is_param_entity(2));
        assert!(!ffe.is_param_entity(3));
    }

    #[test]
    fn test_negative_pointers() {
        let mut ffe = FreeFormatEntity::new();
        ffe.add_entity(10);
        ffe.add_negative_pointer(20);
        ffe.add_literal(ParamType::Real, "3.14");

        assert!(!ffe.is_negative_pointer(1));
        assert!(ffe.is_negative_pointer(2));
        assert!(!ffe.is_negative_pointer(3));

        let neg_ptrs = ffe.negative_pointers();
        assert_eq!(neg_ptrs.len(), 1);
        assert_eq!(neg_ptrs[0], 2);
    }

    #[test]
    fn test_type_form() {
        let mut ffe = FreeFormatEntity::new();
        ffe.set_type_number(128);
        ffe.set_form_number(7);
        let (t, f) = ffe.type_form();
        assert_eq!(t, 128);
        assert_eq!(f, 7);
    }
}
