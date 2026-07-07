// FILE: step_ap242_id_attribute.rs
// occt: StepAP242_IdAttribute

/// Representation of STEP AP242 IdAttribute entity.
#[derive(Clone, Debug)]
pub struct IdAttribute {
    attribute_value: String,
}

impl IdAttribute {
    pub fn new() -> Self {
        IdAttribute {
            attribute_value: String::new(),
        }
    }

    pub fn set_attribute_value(&mut self, value: String) {
        self.attribute_value = value;
    }

    pub fn attribute_value(&self) -> &str {
        &self.attribute_value
    }
}

impl Default for IdAttribute {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let attr = IdAttribute::new();
        assert_eq!(attr.attribute_value(), "");
    }

    #[test]
    fn test_set_attribute_value() {
        let mut attr = IdAttribute::new();
        attr.set_attribute_value("test_value".to_string());
        assert_eq!(attr.attribute_value(), "test_value");
    }
}
