// FILE: step_data_field_list_n.rs
// occt: StepData_FieldListN

use crate::step_data_field::StepDataField;

//! Describes a fixed list of fields
pub struct StepDataFieldListN {
    fields: Vec<StepDataField>,
}

impl StepDataFieldListN {
    //! Creates a FieldListN with a number of fields
    pub fn new(nb: usize) -> Self {
        let mut fields = Vec::with_capacity(nb);
        for _ in 0..nb {
            fields.push(StepDataField::new());
        }
        StepDataFieldListN { fields }
    }

    //! Returns the count of fields
    pub fn nb_fields(&self) -> usize {
        self.fields.len()
    }

    //! Returns the field n0 (1-based, read only)
    pub fn field(&self, num: usize) -> Option<&StepDataField> {
        if num < 1 || num > self.fields.len() {
            return None;
        }
        Some(&self.fields[num - 1])
    }

    //! Returns the field n0 (1-based, mutable)
    pub fn field_mut(&mut self, num: usize) -> Option<&mut StepDataField> {
        if num < 1 || num > self.fields.len() {
            return None;
        }
        Some(&mut self.fields[num - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_list_n_new() {
        let list = StepDataFieldListN::new(5);
        assert_eq!(list.nb_fields(), 5);
    }

    #[test]
    fn test_field_access() {
        let mut list = StepDataFieldListN::new(3);
        if let Some(field) = list.field_mut(2) {
            field.set_integer(99);
        }
        assert_eq!(list.field(2).map(|f| f.integer()), Some(99));
    }

    #[test]
    fn test_out_of_bounds() {
        let list = StepDataFieldListN::new(3);
        assert!(list.field(0).is_none());
        assert!(list.field(4).is_none());
    }
}
