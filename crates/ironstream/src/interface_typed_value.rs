// FILE: interface_typed_value.rs
// occt: Interface_TypedValue

/// Represents a typed value
pub struct InterfaceTypedValue {
    name: String,
    value: String,
    value_type: i32,
}

impl InterfaceTypedValue {
    pub fn new(name: &str, value_type: i32) -> Self {
        InterfaceTypedValue {
            name: name.to_string(),
            value: String::new(),
            value_type,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }

    pub fn value_type(&self) -> i32 {
        self.value_type
    }

    pub fn set_value_type(&mut self, vtype: i32) {
        self.value_type = vtype;
    }

    pub fn int_value(&self) -> Option<i32> {
        self.value.parse().ok()
    }

    pub fn real_value(&self) -> Option<f64> {
        self.value.parse().ok()
    }
}

impl Default for InterfaceTypedValue {
    fn default() -> Self {
        InterfaceTypedValue {
            name: String::new(),
            value: String::new(),
            value_type: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let val = InterfaceTypedValue::new("test", 1);
        assert_eq!(val.name(), "test");
        assert_eq!(val.value_type(), 1);
    }

    #[test]
    fn test_set_value() {
        let mut val = InterfaceTypedValue::new("test", 1);
        val.set_value("42");
        assert_eq!(val.value(), "42");
    }

    #[test]
    fn test_int_value() {
        let mut val = InterfaceTypedValue::new("test", 1);
        val.set_value("123");
        assert_eq!(val.int_value(), Some(123));
    }
}
