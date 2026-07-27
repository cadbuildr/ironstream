// FILE: opengl_shader.rs
// occt: OpenGl_ShaderProgram, OpenGl_ShaderObject, OpenGl_ShaderManager

use std::collections::HashMap;

/// OpenGL shader stage type.
/// occt: Graphic3d_TypeOfShaderObject
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShaderStageType {
    Vertex,
    Fragment,
    Geometry,
    TessControl,
    TessEval,
    Compute,
}

/// A single shader stage object with source code.
/// occt: OpenGl_ShaderObject // / Graphic3d_ShaderObject
#[derive(Clone, Debug)]
pub struct OpenGlShaderObject {
    pub shader_id: u32,
    pub stage: ShaderStageType,
    pub source: String,
    pub is_compiled: bool,
    pub error_log: String,
}

impl OpenGlShaderObject {
    pub fn new(shader_id: u32, stage: ShaderStageType, source: &str) -> Self {
        let compiled = !source.is_empty();
        Self {
            shader_id, stage,
            source: source.to_string(),
            is_compiled: compiled,
            error_log: String::new(),
        }
    }

    pub fn is_compiled(&self) -> bool { self.is_compiled }
    pub fn stage(&self) -> ShaderStageType { self.stage }
    pub fn source_len(&self) -> usize { self.source.len() }
    pub fn error_log(&self) -> &str { &self.error_log }
}

/// Uniform variable type.
#[derive(Clone, Debug, PartialEq)]
pub enum UniformValue {
    Float(f32),
    Int(i32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Mat4([f32; 16]),
    Bool(bool),
}

/// A linked shader program.
/// occt: OpenGl_ShaderProgram // / Graphic3d_ShaderProgram
#[derive(Clone, Debug)]
pub struct OpenGlShaderProgram {
    pub program_id: u32,
    pub shaders: Vec<OpenGlShaderObject>,
    pub uniforms: HashMap<String, UniformValue>,
    pub is_linked: bool,
    pub error_log: String,
    pub header: String,  // GLSL version header e.g. "#version 330 core\n"
}

impl OpenGlShaderProgram {
    pub fn new(program_id: u32) -> Self {
        Self {
            program_id,
            shaders: Vec::new(),
            uniforms: HashMap::new(),
            is_linked: false,
            error_log: String::new(),
            header: String::from("#version 330 core\n"),
        }
    }

    pub fn attach_shader(&mut self, s: OpenGlShaderObject) {
        self.shaders.push(s);
        self.is_linked = false;
    }

    /// Simulate linking — succeeds if we have at least vertex + fragment.
    pub fn link(&mut self) -> bool {
        let has_vert = self.shaders.iter().any(|s| s.stage == ShaderStageType::Vertex);
        let has_frag = self.shaders.iter().any(|s| s.stage == ShaderStageType::Fragment);
        self.is_linked = has_vert && has_frag;
        if !self.is_linked {
            self.error_log = "Missing vertex or fragment shader".to_string();
        }
        self.is_linked
    }

    pub fn set_uniform_float(&mut self, name: &str, v: f32) {
        self.uniforms.insert(name.to_string(), UniformValue::Float(v));
    }

    pub fn set_uniform_int(&mut self, name: &str, v: i32) {
        self.uniforms.insert(name.to_string(), UniformValue::Int(v));
    }

    pub fn set_uniform_vec3(&mut self, name: &str, v: [f32; 3]) {
        self.uniforms.insert(name.to_string(), UniformValue::Vec3(v));
    }

    pub fn set_uniform_mat4(&mut self, name: &str, v: [f32; 16]) {
        self.uniforms.insert(name.to_string(), UniformValue::Mat4(v));
    }

    pub fn get_uniform(&self, name: &str) -> Option<&UniformValue> {
        self.uniforms.get(name)
    }

    pub fn has_uniform(&self, name: &str) -> bool { self.uniforms.contains_key(name) }

    pub fn is_linked(&self) -> bool { self.is_linked }
    pub fn nb_shaders(&self) -> usize { self.shaders.len() }
    pub fn set_header(&mut self, h: &str) { self.header = h.to_string(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shader_object_compiled() {
        let s = OpenGlShaderObject::new(1, ShaderStageType::Vertex, "void main() {}");
        assert!(s.is_compiled());
        assert_eq!(s.stage(), ShaderStageType::Vertex);
    }

    #[test]
    fn shader_object_empty_not_compiled() {
        let s = OpenGlShaderObject::new(1, ShaderStageType::Fragment, "");
        assert!(!s.is_compiled());
    }

    #[test]
    fn program_link_success() {
        let mut p = OpenGlShaderProgram::new(1);
        p.attach_shader(OpenGlShaderObject::new(1, ShaderStageType::Vertex, "void main(){}"));
        p.attach_shader(OpenGlShaderObject::new(2, ShaderStageType::Fragment, "void main(){}"));
        assert!(p.link());
        assert!(p.is_linked());
    }

    #[test]
    fn program_link_fail_no_fragment() {
        let mut p = OpenGlShaderProgram::new(1);
        p.attach_shader(OpenGlShaderObject::new(1, ShaderStageType::Vertex, "void main(){}"));
        assert!(!p.link());
        assert!(!p.is_linked());
        assert!(!p.error_log.is_empty());
    }

    #[test]
    fn program_uniforms() {
        let mut p = OpenGlShaderProgram::new(1);
        p.set_uniform_float("uTime", 1.5);
        p.set_uniform_vec3("uColor", [1.0, 0.5, 0.0]);
        assert!(p.has_uniform("uTime"));
        assert!(p.has_uniform("uColor"));
        assert!(!p.has_uniform("uMissing"));
        match p.get_uniform("uTime") {
            Some(UniformValue::Float(v)) => assert!((v - 1.5).abs() < 1e-6),
            _ => panic!("wrong type"),
        }
    }
}
