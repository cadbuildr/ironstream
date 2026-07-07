// FILE: step_data_select_named.rs
// occt: StepData_SelectNamed

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

// This select member can be of any kind, and be named
pub struct StepDataSelectNamed {
    name: Option<String>,
    field: StepDataField,
}

impl StepDataSelectNamed {
    // Creates a SelectNamed
    pub fn new() -> Self {
        StepDataSelectNamed {
            name: None,
            field: StepDataField::new(),
        }
    }

    // Returns True if this has a name
    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Returns the name
    pub fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }

    // Sets the name
    pub fn set_name(&mut self, name: &str) -> bool {
        self.name = Some(name.to_string());
        true
    }

    // Returns the field
    pub fn field(&self) -> &StepDataField {
        &self.field
    }

    // Returns mutable field
    pub fn field_mut(&mut self) -> &mut StepDataField {
        &mut self.field
    }

    // Returns the kind
    pub fn kind(&self) -> i32 {
        self.field.kind(true)
    }

    // Sets the kind
    pub fn set_kind(&mut self, kind: i32) {
        self.field.clear(kind);
    }

    // Returns integer value
    pub fn int(&self) -> i32 {
        self.field.integer()
    }

    // Sets integer value
    pub fn set_int(&mut self, val: i32) {
        self.field.set_integer(val);
    }

    // Returns real value
    pub fn real(&self) -> f64 {
        self.field.real()
    }

    // Sets real value
    pub fn set_real(&mut self, val: f64) {
        self.field.set_real(val);
    }

    // Returns string value
    pub fn string(&self) -> &str {
        self.field.string()
    }

    // Sets string value
    pub fn set_string(&mut self, val: &str) {
        self.field.set_string(val);
    }
}

impl Default for StepDataSelectNamed {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_named_new() {
        let sel = StepDataSelectNamed::new();
        assert!(!sel.has_name());
    }

    #[test]
    fn test_set_name() {
        let mut sel = StepDataSelectNamed::new();
        sel.set_name("test_name");
        assert!(sel.has_name());
        assert_eq!(sel.name(), "test_name");
    }

    #[test]
    fn test_int_value() {
        let mut sel = StepDataSelectNamed::new();
        sel.set_int(42);
        assert_eq!(sel.int(), 42);
    }

    #[test]
    fn test_real_value() {
        let mut sel = StepDataSelectNamed::new();
        sel.set_real(3.14);
        assert!((sel.real() - 3.14).abs() < 1e-10);
    }

    #[test]
    fn test_string_value() {
        let mut sel = StepDataSelectNamed::new();
        sel.set_string("hello");
        assert_eq!(sel.string(), "hello");
    }
}
