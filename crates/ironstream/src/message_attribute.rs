// FILE: message_attribute.rs
// occt: Message_Attribute

/// Attribute for message metadata.
pub struct MessageAttribute {
    name: String,
    value: String,
}

impl MessageAttribute {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let attr = MessageAttribute::new("key", "val");
        assert_eq!(attr.name(), "key");
        assert_eq!(attr.value(), "val");
    }
}
