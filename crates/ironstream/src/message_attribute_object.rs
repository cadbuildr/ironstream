// FILE: message_attribute_object.rs
// occt: Message_AttributeObject

/// Object attribute for message metadata.
pub struct MessageAttributeObject {
    name: String,
    obj_id: usize,
}

impl MessageAttributeObject {
    pub fn new(name: &str, obj_id: usize) -> Self {
        Self {
            name: name.to_string(),
            obj_id,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn obj_id(&self) -> usize {
        self.obj_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let attr = MessageAttributeObject::new("obj", 42);
        assert_eq!(attr.name(), "obj");
        assert_eq!(attr.obj_id(), 42);
    }
}
