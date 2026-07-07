// FILE: iges_defs_attribute_def.rs
// occt: IGESDefs_AttributeDef

//! Attribute definition entity for IGES.

#[derive(Clone, Debug)]
pub struct AttributeDef {
    name: String,
    attr_type: i32,
}

impl AttributeDef {
    pub fn new(name: &str, attr_type: i32) -> Self {
        AttributeDef {
            name: name.to_string(),
            attr_type,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn attr_type(&self) -> i32 {
        self.attr_type
    }
}

impl Default for AttributeDef {
    fn default() -> Self {
        Self::new("", 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let def = AttributeDef::new("attr", 2);
        assert_eq!(def.name(), "attr");
        assert_eq!(def.attr_type(), 2);
    }

    #[test]
    fn test_default() {
        let def = AttributeDef::default();
        assert_eq!(def.name(), "");
        assert_eq!(def.attr_type(), 0);
    }
}
