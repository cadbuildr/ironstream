// FILE: step_data_field_list.rs
// occt: StepData_FieldList

use crate::step_data_field::StepDataField;

//! Describes a list of fields, in a general way
pub struct StepDataFieldList {
    fields: Vec<StepDataField>,
}

impl StepDataFieldList {
    //! Creates a FieldList of 0 fields
    pub fn new() -> Self {
        StepDataFieldList {
            fields: Vec::new(),
        }
    }

    //! Returns the count of fields
    pub fn nb_fields(&self) -> usize {
        self.fields.len()
    }

    //! Returns the field n0 <num> (1-based, read only)
    pub fn field(&self, num: usize) -> Option<&StepDataField> {
        if num < 1 || num > self.fields.len() {
            return None;
        }
        Some(&self.fields[num - 1])
    }

    //! Returns the field n0 <num> (1-based, mutable)
    pub fn field_mut(&mut self, num: usize) -> Option<&mut StepDataField> {
        if num < 1 || num > self.fields.len() {
            return None;
        }
        Some(&mut self.fields[num - 1])
    }

    //! Resize the field list
    pub fn resize(&mut self, size: usize) {
        self.fields.resize(size, StepDataField::new());
    }
}

impl Default for StepDataFieldList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_list_new() {
        let list = StepDataFieldList::new();
        assert_eq!(list.nb_fields(), 0);
    }

    #[test]
    fn test_field_list_resize() {
        let mut list = StepDataFieldList::new();
        list.resize(3);
        assert_eq!(list.nb_fields(), 3);
    }

    #[test]
    fn test_field_access() {
        let mut list = StepDataFieldList::new();
        list.resize(2);
        if let Some(field) = list.field_mut(1) {
            field.set_integer(42);
        }
        assert_eq!(list.field(1).map(|f| f.integer()), Some(42));
    }
}
