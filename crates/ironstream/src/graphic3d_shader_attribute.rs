// FILE: graphic3d_shader_attribute.rs
// occt: Graphic3d_ShaderAttribute

/// Describes a custom vertex shader attribute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShaderAttribute {
    /// Attribute name
    name: String,
    /// Attribute location to be bound on GLSL program linkage stage
    location: i32,
}

impl ShaderAttribute {
    /// Creates a new shader attribute with the given name and location.
    ///
    /// # Arguments
    /// * `name` - The name of the shader variable
    /// * `location` - The attribute location (should be >= 0)
    pub fn new(name: impl Into<String>, location: i32) -> Self {
        ShaderAttribute {
            name: name.into(),
            location,
        }
    }

    /// Returns the name of the shader variable.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the attribute location to be bound on GLSL program linkage stage.
    pub fn location(&self) -> i32 {
        self.location
    }

    /// Sets the attribute location.
    pub fn set_location(&mut self, location: i32) {
        self.location = location;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_attribute_creation() {
        let attr = ShaderAttribute::new("vertexPosition", 0);
        assert_eq!(attr.name(), "vertexPosition");
        assert_eq!(attr.location(), 0);
    }

    #[test]
    fn test_shader_attribute_from_string() {
        let name = "vertexColor".to_string();
        let attr = ShaderAttribute::new(name, 1);
        assert_eq!(attr.name(), "vertexColor");
        assert_eq!(attr.location(), 1);
    }

    #[test]
    fn test_shader_attribute_from_str() {
        let attr = ShaderAttribute::new("vertexNormal", 2);
        assert_eq!(attr.name(), "vertexNormal");
        assert_eq!(attr.location(), 2);
    }

    #[test]
    fn test_shader_attribute_set_location() {
        let mut attr = ShaderAttribute::new("texCoord", 0);
        assert_eq!(attr.location(), 0);
        attr.set_location(5);
        assert_eq!(attr.location(), 5);
    }

    #[test]
    fn test_shader_attribute_equality() {
        let attr1 = ShaderAttribute::new("position", 0);
        let attr2 = ShaderAttribute::new("position", 0);
        let attr3 = ShaderAttribute::new("position", 1);
        let attr4 = ShaderAttribute::new("normal", 0);

        assert_eq!(attr1, attr2);
        assert_ne!(attr1, attr3); // different location
        assert_ne!(attr1, attr4); // different name
    }

    #[test]
    fn test_shader_attribute_clone() {
        let attr1 = ShaderAttribute::new("vertexData", 3);
        let attr2 = attr1.clone();
        assert_eq!(attr1, attr2);
        assert_eq!(attr2.name(), "vertexData");
        assert_eq!(attr2.location(), 3);
    }

    #[test]
    fn test_shader_attribute_debug() {
        let attr = ShaderAttribute::new("test", 42);
        let debug_str = format!("{:?}", attr);
        assert!(debug_str.contains("test"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn test_shader_attribute_typical_usage() {
        // Simulate typical GLSL attribute declarations
        let position = ShaderAttribute::new("aVertexPosition", 0);
        let color = ShaderAttribute::new("aVertexColor", 1);
        let texcoord = ShaderAttribute::new("aTexCoord", 2);
        let normal = ShaderAttribute::new("aNormal", 3);

        assert_eq!(position.location(), 0);
        assert_eq!(color.location(), 1);
        assert_eq!(texcoord.location(), 2);
        assert_eq!(normal.location(), 3);
    }

    #[test]
    fn test_shader_attribute_negative_location() {
        // Location can be negative (though unusual in practice)
        let attr = ShaderAttribute::new("attr", -1);
        assert_eq!(attr.location(), -1);
    }
}
