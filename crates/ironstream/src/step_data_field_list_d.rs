// FILE: step_data_field_list_d.rs
// occt: StepData_FieldListD

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

// Describes a dynamic list of fields
pub struct StepDataFieldListD {
    fields: Vec<StepDataField>,
}

impl StepDataFieldListD {
    // Creates a FieldListD with a number of fields
    pub fn new(nb: usize) -> Self {
        let mut fields = Vec::with_capacity(nb);
        for _ in 0..nb {
            fields.push(StepDataField::new());
        }
        StepDataFieldListD { fields }
    }

    // Sets a new count of fields
    pub fn set_nb(&mut self, nb: usize) {
        if nb < self.fields.len() {
            self.fields.truncate(nb);
        } else {
            while self.fields.len() < nb {
                self.fields.push(StepDataField::new());
            }
        }
    }

    // Returns the count of fields
    pub fn nb_fields(&self) -> usize {
        self.fields.len()
    }

    // Returns the field n0 (1-based, read only)
    pub fn field(&self, num: usize) -> Option<&StepDataField> {
        if num < 1 || num > self.fields.len() {
            return None;
        }
        Some(&self.fields[num - 1])
    }

    // Returns the field n0 (1-based, mutable)
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
    fn test_field_list_d_new() {
        let list = StepDataFieldListD::new(5);
        assert_eq!(list.nb_fields(), 5);
    }

    #[test]
    fn test_set_nb() {
        let mut list = StepDataFieldListD::new(3);
        list.set_nb(5);
        assert_eq!(list.nb_fields(), 5);

        list.set_nb(2);
        assert_eq!(list.nb_fields(), 2);
    }

    #[test]
    fn test_field_access() {
        let mut list = StepDataFieldListD::new(3);
        if let Some(field) = list.field_mut(2) {
            field.set_integer(99);
        }
        assert_eq!(list.field(2).map(|f| f.integer()), Some(99));
    }
}
