// FILE: moni_tool_typed_value.rs
// occt: MoniTool_TypedValue

/// A typed value for monitoring tools
pub struct MoniToolTypedValue {
    name: String,
    value: String,
    value_type: i32,
}

impl MoniToolTypedValue {
    pub fn new(name: &str, value_type: i32) -> Self {
        MoniToolTypedValue {
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

    pub fn int_value(&self) -> Option<i32> {
        self.value.parse().ok()
    }

    pub fn real_value(&self) -> Option<f64> {
        self.value.parse().ok()
    }
}

impl Default for MoniToolTypedValue {
    fn default() -> Self {
        MoniToolTypedValue {
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
        let val = MoniToolTypedValue::new("test", 1);
        assert_eq!(val.name(), "test");
        assert_eq!(val.value_type(), 1);
    }

    #[test]
    fn test_set_value() {
        let mut val = MoniToolTypedValue::new("test", 1);
        val.set_value("42");
        assert_eq!(val.int_value(), Some(42));
    }
}
