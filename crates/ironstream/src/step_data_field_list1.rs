// FILE: step_data_field_list1.rs
// occt: StepData_FieldList1

// Local helper mirroring StepData_Field (external plumbing)
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
    pub fn clear(&mut self, kind: i32) {
        self.kind = kind;
        self.int_val = 0;
        self.real_val = 0.0;
        self.text = None;
    }
    pub fn kind(&self, type_only: bool) -> i32 {
        if type_only {
            self.kind & 15
        } else {
            self.kind
        }
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

// Describes a list of ONE field
pub struct StepDataFieldList1 {
    field: StepDataField,
}

impl StepDataFieldList1 {
    // Creates a FieldList of 1 field
    pub fn new() -> Self {
        StepDataFieldList1 {
            field: StepDataField::new(),
        }
    }

    // Returns the count of fields (always 1)
    pub fn nb_fields(&self) -> usize {
        1
    }

    // Returns the field (read only)
    pub fn field(&self, num: usize) -> Option<&StepDataField> {
        if num == 1 {
            Some(&self.field)
        } else {
            None
        }
    }

    // Returns the field (mutable)
    pub fn field_mut(&mut self, num: usize) -> Option<&mut StepDataField> {
        if num == 1 {
            Some(&mut self.field)
        } else {
            None
        }
    }
}

impl Default for StepDataFieldList1 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_list1_new() {
        let list = StepDataFieldList1::new();
        assert_eq!(list.nb_fields(), 1);
    }

    #[test]
    fn test_field_access() {
        let mut list = StepDataFieldList1::new();
        if let Some(field) = list.field_mut(1) {
            field.set_integer(42);
        }
        assert_eq!(list.field(1).map(|f| f.integer()), Some(42));
    }

    #[test]
    fn test_out_of_bounds() {
        let list = StepDataFieldList1::new();
        assert!(list.field(0).is_none());
        assert!(list.field(2).is_none());
    }
}
