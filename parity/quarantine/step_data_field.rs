// FILE: step_data_field.rs
// occt: StepData_Field

use crate::step_data_logical::StepDataLogical;

const KIND_INTEGER: i32 = 1;
const KIND_BOOLEAN: i32 = 2;
const KIND_LOGICAL: i32 = 3;
const KIND_ENUM: i32 = 4;
const KIND_REAL: i32 = 5;
const KIND_STRING: i32 = 6;
const KIND_ENTITY: i32 = 7;
const KIND_ANY: i32 = 8;
const KIND_DERIVED: i32 = 9;
const KIND_SELECT: i32 = 16;
const KIND_LIST: i32 = 64;
const KIND_LIST2: i32 = 128;

//! Defines a generally defined Field for STEP data
pub struct StepDataField {
    kind: i32,
    int_val: i32,
    real_val: f64,
    any_val: Option<String>,
}

impl StepDataField {
    //! Creates a Field, empty
    pub fn new() -> Self {
        StepDataField {
            kind: 0,
            int_val: 0,
            real_val: 0.0,
            any_val: None,
        }
    }

    //! Clear the field
    pub fn clear(&mut self, kind: i32) {
        self.kind = kind;
        self.int_val = 0;
        self.real_val = 0.0;
        self.any_val = None;
    }

    //! Sets the field as derived
    pub fn set_derived(&mut self) {
        self.kind = KIND_DERIVED;
    }

    //! Set an integer value
    pub fn set_integer(&mut self, val: i32) {
        self.kind = KIND_INTEGER;
        self.int_val = val;
    }

    //! Set a boolean value
    pub fn set_boolean(&mut self, val: bool) {
        self.kind = KIND_BOOLEAN;
        self.int_val = if val { 1 } else { 0 };
    }

    //! Set a logical value
    pub fn set_logical(&mut self, val: StepDataLogical) {
        self.kind = KIND_LOGICAL;
        self.int_val = match val {
            StepDataLogical::False => 0,
            StepDataLogical::True => 1,
            StepDataLogical::Unknown => 2,
        };
    }

    //! Set a real value
    pub fn set_real(&mut self, val: f64) {
        self.kind = KIND_REAL;
        self.real_val = val;
    }

    //! Set a string value
    pub fn set_string(&mut self, val: &str) {
        if self.kind != KIND_STRING && self.kind != KIND_ENUM {
            self.kind = KIND_STRING;
        }
        self.any_val = Some(val.to_string());
    }

    //! Set an enum value
    pub fn set_enum(&mut self, val: i32, text: Option<&str>) {
        self.kind = KIND_ENUM;
        self.int_val = val;
        if let Some(t) = text {
            self.any_val = Some(t.to_string());
        }
    }

    //! Returns the kind of the field
    pub fn kind(&self, type_only: bool) -> i32 {
        if type_only {
            self.kind & 15
        } else {
            self.kind
        }
    }

    //! Get integer value
    pub fn integer(&self) -> i32 {
        self.int_val
    }

    //! Get boolean value
    pub fn boolean(&self) -> bool {
        self.int_val != 0
    }

    //! Get logical value
    pub fn logical(&self) -> StepDataLogical {
        match self.int_val {
            0 => StepDataLogical::False,
            1 => StepDataLogical::True,
            _ => StepDataLogical::Unknown,
        }
    }

    //! Get real value
    pub fn real(&self) -> f64 {
        self.real_val
    }

    //! Get string value
    pub fn string(&self) -> &str {
        self.any_val.as_deref().unwrap_or("")
    }

    //! Get enum value
    pub fn enum_val(&self) -> i32 {
        self.int_val
    }

    //! Get enum text
    pub fn enum_text(&self) -> &str {
        self.any_val.as_deref().unwrap_or("")
    }

    //! Check if the field is set
    pub fn is_set(&self) -> bool {
        self.kind != 0
    }
}

impl Default for StepDataField {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for StepDataField {
    fn clone(&self) -> Self {
        StepDataField {
            kind: self.kind,
            int_val: self.int_val,
            real_val: self.real_val,
            any_val: self.any_val.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_new() {
        let field = StepDataField::new();
        assert!(!field.is_set());
    }

    #[test]
    fn test_set_integer() {
        let mut field = StepDataField::new();
        field.set_integer(42);
        assert_eq!(field.integer(), 42);
    }

    #[test]
    fn test_set_real() {
        let mut field = StepDataField::new();
        field.set_real(3.14);
        assert!((field.real() - 3.14).abs() < 1e-10);
    }

    #[test]
    fn test_set_string() {
        let mut field = StepDataField::new();
        field.set_string("hello");
        assert_eq!(field.string(), "hello");
    }

    #[test]
    fn test_set_boolean() {
        let mut field = StepDataField::new();
        field.set_boolean(true);
        assert!(field.boolean());
    }

    #[test]
    fn test_set_logical() {
        let mut field = StepDataField::new();
        field.set_logical(StepDataLogical::True);
        assert_eq!(field.logical(), StepDataLogical::True);
    }

    #[test]
    fn test_set_derived() {
        let mut field = StepDataField::new();
        field.set_derived();
        assert_eq!(field.kind(true), KIND_DERIVED);
    }
}
