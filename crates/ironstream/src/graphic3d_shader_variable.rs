// FILE: graphic3d_shader_variable.rs
// occt: Graphic3d_ShaderVariable

/// Interface for generic variable value.
pub trait ValueInterface: Send + Sync {
    /// Returns unique identifier of value type.
    fn type_id(&self) -> usize;
}

/// Generates unique type identifier for variable values.
pub struct UniformValueTypeId;

impl UniformValueTypeId {
    /// Type ID for int
    pub const INT_ID: usize = 1001;
    /// Type ID for f32
    pub const FLOAT_ID: usize = 1002;
    /// Type ID for Vec2<f32>
    pub const VEC2_ID: usize = 1003;
    /// Type ID for Vec3<f32>
    pub const VEC3_ID: usize = 1004;
    /// Type ID for Vec4<f32>
    pub const VEC4_ID: usize = 1005;
    /// Type ID for Vec2<i32>
    pub const VEC2I_ID: usize = 1006;
    /// Type ID for Vec3<i32>
    pub const VEC3I_ID: usize = 1007;
    /// Type ID for Vec4<i32>
    pub const VEC4I_ID: usize = 1008;
    /// Type ID for Mat3<f32>
    pub const MAT3_ID: usize = 1009;
    /// Type ID for Mat4<f32>
    pub const MAT4_ID: usize = 1010;
}

/// Describes specific value of custom uniform variable.
#[derive(Debug, Clone)]
pub struct UniformValue<T: Clone> {
    pub value: T,
}

impl<T: Clone> UniformValue<T> {
    /// Creates new variable value.
    pub fn new(value: T) -> Self {
        UniformValue { value }
    }
}

impl ValueInterface for UniformValue<i32> {
    fn type_id(&self) -> usize {
        UniformValueTypeId::INT_ID
    }
}

impl ValueInterface for UniformValue<f32> {
    fn type_id(&self) -> usize {
        UniformValueTypeId::FLOAT_ID
    }
}

/// Describes custom uniform shader variable.
pub struct Graphic3dShaderVariable {
    name: String,
    value: Box<dyn ValueInterface>,
}

impl Graphic3dShaderVariable {
    /// Creates new initialized shader variable.
    pub fn create<T: 'static + ValueInterface + Clone>(
        name: impl Into<String>,
        value: T,
    ) -> Option<Self> {
        Some(Graphic3dShaderVariable {
            name: name.into(),
            value: Box::new(value),
        })
    }

    /// Returns name of shader variable.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Checks if the shader variable is valid or not.
    pub fn is_done(&self) -> bool {
        !self.name.is_empty()
    }

    /// Returns interface of shader variable value.
    pub fn value(&self) -> &dyn ValueInterface {
        self.value.as_ref()
    }

    /// Returns the type ID of the stored value.
    pub fn type_id(&self) -> usize {
        self.value.type_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_value_int() {
        let val = UniformValue::new(42i32);
        assert_eq!(val.type_id(), UniformValueTypeId::INT_ID);
    }

    #[test]
    fn test_uniform_value_float() {
        let val = UniformValue::new(3.14f32);
        assert_eq!(val.type_id(), UniformValueTypeId::FLOAT_ID);
    }

    #[test]
    fn test_shader_variable_creation() {
        let var = Graphic3dShaderVariable::create("myVar", UniformValue::new(10i32));
        assert!(var.is_some());
        let var = var.unwrap();
        assert_eq!(var.name(), "myVar");
        assert!(var.is_done());
    }

    #[test]
    fn test_shader_variable_value() {
        let var = Graphic3dShaderVariable::create("myVar", UniformValue::new(42i32)).unwrap();
        assert_eq!(var.type_id(), UniformValueTypeId::INT_ID);
    }

    #[test]
    fn test_shader_variable_float() {
        let var = Graphic3dShaderVariable::create("color", UniformValue::new(1.0f32)).unwrap();
        assert_eq!(var.name(), "color");
        assert_eq!(var.type_id(), UniformValueTypeId::FLOAT_ID);
    }
}
