// FILE: open_gl_gl_core33.rs
// occt: OpenGl_GlCore33

/// OpenGL 3.3 core.
/// Introduces sampler objects, timer queries, and packed vertex attributes.
pub struct OpenGlGlCore33;

impl OpenGlGlCore33 {
    // GL_ARB_blend_func_extended functions (added to OpenGL 3.3 core)

    /// glBindFragDataLocationIndexed: Binds a user-defined output variable with an index.
    pub fn gl_bind_frag_data_location_indexed() -> &'static str {
        "glBindFragDataLocationIndexed"
    }

    /// glGetFragDataIndex: Gets the index of a fragment shader output variable.
    pub fn gl_get_frag_data_index() -> &'static str {
        "glGetFragDataIndex"
    }

    // GL_ARB_sampler_objects functions (added to OpenGL 3.3 core)

    /// glBindSampler: Binds a sampler object to a texture unit.
    pub fn gl_bind_sampler() -> &'static str {
        "glBindSampler"
    }

    /// glDeleteSamplers: Deletes sampler objects.
    pub fn gl_delete_samplers() -> &'static str {
        "glDeleteSamplers"
    }

    /// glGenSamplers: Generates sampler object names.
    pub fn gl_gen_samplers() -> &'static str {
        "glGenSamplers"
    }

    /// glGetSamplerParameterfv: Returns sampler parameter values (float).
    pub fn gl_get_sampler_parameterfv() -> &'static str {
        "glGetSamplerParameterfv"
    }

    /// glGetSamplerParameterIiv: Returns sampler parameter values (signed integer).
    pub fn gl_get_sampler_parameter_iiv() -> &'static str {
        "glGetSamplerParameterIiv"
    }

    /// glGetSamplerParameterIuiv: Returns sampler parameter values (unsigned integer).
    pub fn gl_get_sampler_parameter_iuiv() -> &'static str {
        "glGetSamplerParameterIuiv"
    }

    /// glGetSamplerParameteriv: Returns sampler parameter values (integer).
    pub fn gl_get_sampler_parameteriv() -> &'static str {
        "glGetSamplerParameteriv"
    }

    /// glIsSampler: Tests if a name is a sampler object.
    pub fn gl_is_sampler() -> &'static str {
        "glIsSampler"
    }

    /// glSamplerParameterf: Sets a sampler parameter (float).
    pub fn gl_sampler_parameterf() -> &'static str {
        "glSamplerParameterf"
    }

    /// glSamplerParameterfv: Sets sampler parameters (float vector).
    pub fn gl_sampler_parameterfv() -> &'static str {
        "glSamplerParameterfv"
    }

    /// glSamplerParameteri: Sets a sampler parameter (integer).
    pub fn gl_sampler_parameteri() -> &'static str {
        "glSamplerParameteri"
    }

    /// glSamplerParameterIiv: Sets sampler parameters (signed integer).
    pub fn gl_sampler_parameter_iiv() -> &'static str {
        "glSamplerParameterIiv"
    }

    /// glSamplerParameterIuiv: Sets sampler parameters (unsigned integer).
    pub fn gl_sampler_parameter_iuiv() -> &'static str {
        "glSamplerParameterIuiv"
    }

    /// glSamplerParameteriv: Sets sampler parameters (integer vector).
    pub fn gl_sampler_parameteriv() -> &'static str {
        "glSamplerParameteriv"
    }

    // GL_ARB_timer_query functions (added to OpenGL 3.3 core)

    /// glGetQueryObjecti64v: Returns signed 64-bit query results.
    pub fn gl_get_query_objecti_64v() -> &'static str {
        "glGetQueryObjecti64v"
    }

    /// glGetQueryObjectui64v: Returns unsigned 64-bit query results.
    pub fn gl_get_query_objectui_64v() -> &'static str {
        "glGetQueryObjectui64v"
    }

    /// glQueryCounter: Records the time elapsed since a previous timer query.
    pub fn gl_query_counter() -> &'static str {
        "glQueryCounter"
    }

    // GL_ARB_vertex_type_2_10_10_10_rev functions (added to OpenGL 3.3 core)

    /// glVertexAttribP1ui: Specifies a packed 1D vertex attribute.
    pub fn gl_vertex_attrib_p1ui() -> &'static str {
        "glVertexAttribP1ui"
    }

    /// glVertexAttribP1uiv: Specifies packed 1D vertex attributes.
    pub fn gl_vertex_attrib_p1uiv() -> &'static str {
        "glVertexAttribP1uiv"
    }

    /// glVertexAttribP2ui: Specifies a packed 2D vertex attribute.
    pub fn gl_vertex_attrib_p2ui() -> &'static str {
        "glVertexAttribP2ui"
    }

    /// glVertexAttribP2uiv: Specifies packed 2D vertex attributes.
    pub fn gl_vertex_attrib_p2uiv() -> &'static str {
        "glVertexAttribP2uiv"
    }

    /// glVertexAttribP3ui: Specifies a packed 3D vertex attribute.
    pub fn gl_vertex_attrib_p3ui() -> &'static str {
        "glVertexAttribP3ui"
    }

    /// glVertexAttribP3uiv: Specifies packed 3D vertex attributes.
    pub fn gl_vertex_attrib_p3uiv() -> &'static str {
        "glVertexAttribP3uiv"
    }

    /// glVertexAttribP4ui: Specifies a packed 4D vertex attribute.
    pub fn gl_vertex_attrib_p4ui() -> &'static str {
        "glVertexAttribP4ui"
    }

    /// glVertexAttribP4uiv: Specifies packed 4D vertex attributes.
    pub fn gl_vertex_attrib_p4uiv() -> &'static str {
        "glVertexAttribP4uiv"
    }

    // OpenGL 3.3 core additives to 3.2

    /// glVertexAttribDivisor: Specifies the rate at which vertex attributes advance.
    pub fn gl_vertex_attrib_divisor() -> &'static str {
        "glVertexAttribDivisor"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core33_sampler_functions() {
        // Verify sampler object functions
        assert_eq!(OpenGlGlCore33::gl_gen_samplers(), "glGenSamplers");
        assert_eq!(OpenGlGlCore33::gl_bind_sampler(), "glBindSampler");
        assert_eq!(OpenGlGlCore33::gl_delete_samplers(), "glDeleteSamplers");
        assert_eq!(OpenGlGlCore33::gl_is_sampler(), "glIsSampler");
    }

    #[test]
    fn test_gl_core33_timer_query_functions() {
        // Verify timer query functions
        assert_eq!(OpenGlGlCore33::gl_query_counter(), "glQueryCounter");
        assert_eq!(OpenGlGlCore33::gl_get_query_objecti_64v(), "glGetQueryObjecti64v");
        assert_eq!(
            OpenGlGlCore33::gl_get_query_objectui_64v(),
            "glGetQueryObjectui64v"
        );
    }

    #[test]
    fn test_gl_core33_packed_vertex_attributes() {
        // Verify packed vertex attribute functions
        let funcs = vec![
            OpenGlGlCore33::gl_vertex_attrib_p1ui(),
            OpenGlGlCore33::gl_vertex_attrib_p2ui(),
            OpenGlGlCore33::gl_vertex_attrib_p3ui(),
            OpenGlGlCore33::gl_vertex_attrib_p4ui(),
        ];

        for func in funcs {
            assert!(func.contains("VertexAttribP"));
        }
    }

    #[test]
    fn test_gl_core33_blend_func_extended() {
        // Verify blend function extended functions
        assert_eq!(
            OpenGlGlCore33::gl_bind_frag_data_location_indexed(),
            "glBindFragDataLocationIndexed"
        );
        assert_eq!(
            OpenGlGlCore33::gl_get_frag_data_index(),
            "glGetFragDataIndex"
        );
    }
}
