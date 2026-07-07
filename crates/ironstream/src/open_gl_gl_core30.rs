// FILE: open_gl_gl_core30.rs
// occt: OpenGl_GlCore30

/// OpenGL 3.0 core.
/// First version with deprecation model - fixed pipeline functionality marked deprecated.
/// Introduces framebuffer objects, vertex array objects, and transform feedback.
pub struct OpenGlGlCore30;

impl OpenGlGlCore30 {
    // GL_ARB_framebuffer_object functions (added to OpenGL 3.0 core)

    /// glBindFramebuffer: Binds a framebuffer object.
    pub fn gl_bind_framebuffer() -> &'static str {
        "glBindFramebuffer"
    }

    /// glBindRenderbuffer: Binds a renderbuffer object.
    pub fn gl_bind_renderbuffer() -> &'static str {
        "glBindRenderbuffer"
    }

    /// glBlitFramebuffer: Copies a block of pixels from one framebuffer to another.
    pub fn gl_blit_framebuffer() -> &'static str {
        "glBlitFramebuffer"
    }

    /// glCheckFramebufferStatus: Checks the status of a framebuffer object.
    pub fn gl_check_framebuffer_status() -> &'static str {
        "glCheckFramebufferStatus"
    }

    /// glDeleteFramebuffers: Deletes framebuffer objects.
    pub fn gl_delete_framebuffers() -> &'static str {
        "glDeleteFramebuffers"
    }

    /// glDeleteRenderbuffers: Deletes renderbuffer objects.
    pub fn gl_delete_renderbuffers() -> &'static str {
        "glDeleteRenderbuffers"
    }

    /// glFramebufferRenderbuffer: Attaches a renderbuffer object to a framebuffer.
    pub fn gl_framebuffer_renderbuffer() -> &'static str {
        "glFramebufferRenderbuffer"
    }

    /// glFramebufferTexture2D: Attaches a 2D texture to a framebuffer.
    pub fn gl_framebuffer_texture_2d() -> &'static str {
        "glFramebufferTexture2D"
    }

    /// glFramebufferTextureLayer: Attaches a layer of a 3D/array texture to a framebuffer.
    pub fn gl_framebuffer_texture_layer() -> &'static str {
        "glFramebufferTextureLayer"
    }

    /// glGenerateMipmap: Generates mipmaps for a texture.
    pub fn gl_generate_mipmap() -> &'static str {
        "glGenerateMipmap"
    }

    /// glGenFramebuffers: Generates framebuffer object names.
    pub fn gl_gen_framebuffers() -> &'static str {
        "glGenFramebuffers"
    }

    /// glGenRenderbuffers: Generates renderbuffer object names.
    pub fn gl_gen_renderbuffers() -> &'static str {
        "glGenRenderbuffers"
    }

    /// glGetFramebufferAttachmentParameteriv: Retrieves framebuffer attachment parameters.
    pub fn gl_get_framebuffer_attachment_parameteriv() -> &'static str {
        "glGetFramebufferAttachmentParameteriv"
    }

    /// glGetRenderbufferParameteriv: Retrieves renderbuffer parameters.
    pub fn gl_get_renderbuffer_parameteriv() -> &'static str {
        "glGetRenderbufferParameteriv"
    }

    /// glIsFramebuffer: Tests if a name is a framebuffer object.
    pub fn gl_is_framebuffer() -> &'static str {
        "glIsFramebuffer"
    }

    /// glIsRenderbuffer: Tests if a name is a renderbuffer object.
    pub fn gl_is_renderbuffer() -> &'static str {
        "glIsRenderbuffer"
    }

    /// glRenderbufferStorage: Creates and initializes a renderbuffer object's data store.
    pub fn gl_renderbuffer_storage() -> &'static str {
        "glRenderbufferStorage"
    }

    /// glRenderbufferStorageMultisample: Creates multisampled renderbuffer storage.
    pub fn gl_renderbuffer_storage_multisample() -> &'static str {
        "glRenderbufferStorageMultisample"
    }

    /// glFramebufferTexture1D: Attaches a 1D texture to a framebuffer (not in ES 2.0).
    pub fn gl_framebuffer_texture_1d() -> &'static str {
        "glFramebufferTexture1D"
    }

    /// glFramebufferTexture3D: Attaches a 3D texture to a framebuffer (not in ES 2.0).
    pub fn gl_framebuffer_texture_3d() -> &'static str {
        "glFramebufferTexture3D"
    }

    // GL_ARB_vertex_array_object functions (added to OpenGL 3.0 core)

    /// glBindVertexArray: Binds a vertex array object.
    pub fn gl_bind_vertex_array() -> &'static str {
        "glBindVertexArray"
    }

    /// glDeleteVertexArrays: Deletes vertex array objects.
    pub fn gl_delete_vertex_arrays() -> &'static str {
        "glDeleteVertexArrays"
    }

    /// glGenVertexArrays: Generates vertex array object names.
    pub fn gl_gen_vertex_arrays() -> &'static str {
        "glGenVertexArrays"
    }

    /// glIsVertexArray: Tests if a name is a vertex array object.
    pub fn gl_is_vertex_array() -> &'static str {
        "glIsVertexArray"
    }

    // GL_ARB_map_buffer_range functions (added to OpenGL 3.0 core)

    /// glFlushMappedBufferRange: Indicates modifications to mapped buffer range.
    pub fn gl_flush_mapped_buffer_range() -> &'static str {
        "glFlushMappedBufferRange"
    }

    /// glMapBufferRange: Maps a section of a buffer object's data store.
    pub fn gl_map_buffer_range() -> &'static str {
        "glMapBufferRange"
    }

    // OpenGL 3.0 core additives to 2.1

    /// glBeginTransformFeedback: Begins transform feedback mode.
    pub fn gl_begin_transform_feedback() -> &'static str {
        "glBeginTransformFeedback"
    }

    /// glBindBufferBase: Binds a buffer object to an indexed buffer target.
    pub fn gl_bind_buffer_base() -> &'static str {
        "glBindBufferBase"
    }

    /// glBindBufferRange: Binds a range of a buffer object to an indexed target.
    pub fn gl_bind_buffer_range() -> &'static str {
        "glBindBufferRange"
    }

    /// glClearBufferfi: Clears a buffer with floating-point and integer values.
    pub fn gl_clear_bufferfi() -> &'static str {
        "glClearBufferfi"
    }

    /// glClearBufferfv: Clears a floating-point buffer.
    pub fn gl_clear_bufferfv() -> &'static str {
        "glClearBufferfv"
    }

    /// glClearBufferiv: Clears an integer buffer.
    pub fn gl_clear_bufferiv() -> &'static str {
        "glClearBufferiv"
    }

    /// glClearBufferuiv: Clears an unsigned integer buffer.
    pub fn gl_clear_bufferuiv() -> &'static str {
        "glClearBufferuiv"
    }

    /// glEndTransformFeedback: Ends transform feedback mode.
    pub fn gl_end_transform_feedback() -> &'static str {
        "glEndTransformFeedback"
    }

    /// glGetBooleani_v: Retrieves indexed boolean state variables.
    pub fn gl_get_booleani_v() -> &'static str {
        "glGetBooleani_v"
    }

    /// glGetFragDataLocation: Returns the location of a fragment shader's output variable.
    pub fn gl_get_frag_data_location() -> &'static str {
        "glGetFragDataLocation"
    }

    /// glGetIntegeri_v: Retrieves indexed integer state variables.
    pub fn gl_get_integeri_v() -> &'static str {
        "glGetIntegeri_v"
    }

    /// glGetStringi: Returns a pointer to a string describing an OpenGL extension.
    pub fn gl_get_stringi() -> &'static str {
        "glGetStringi"
    }

    /// glGetTransformFeedbackVarying: Retrieves transform feedback varying variable info.
    pub fn gl_get_transform_feedback_varying() -> &'static str {
        "glGetTransformFeedbackVarying"
    }

    /// glGetUniformuiv: Returns unsigned integer uniform variable values.
    pub fn gl_get_uniformuiv() -> &'static str {
        "glGetUniformuiv"
    }

    /// glGetVertexAttribIiv: Returns signed integer vertex attribute values.
    pub fn gl_get_vertex_attrib_iiv() -> &'static str {
        "glGetVertexAttribIiv"
    }

    /// glGetVertexAttribIuiv: Returns unsigned integer vertex attribute values.
    pub fn gl_get_vertex_attrib_iuiv() -> &'static str {
        "glGetVertexAttribIuiv"
    }

    /// glTransformFeedbackVaryings: Specifies variables for transform feedback.
    pub fn gl_transform_feedback_varyings() -> &'static str {
        "glTransformFeedbackVaryings"
    }

    /// glUniform1ui: Sets an unsigned integer uniform variable.
    pub fn gl_uniform_1ui() -> &'static str {
        "glUniform1ui"
    }

    /// glUniform1uiv: Sets unsigned integer uniform variables as a vector.
    pub fn gl_uniform_1uiv() -> &'static str {
        "glUniform1uiv"
    }

    /// glUniform2ui: Sets a 2D unsigned integer uniform variable.
    pub fn gl_uniform_2ui() -> &'static str {
        "glUniform2ui"
    }

    /// glUniform2uiv: Sets 2D unsigned integer uniform variables.
    pub fn gl_uniform_2uiv() -> &'static str {
        "glUniform2uiv"
    }

    /// glUniform3ui: Sets a 3D unsigned integer uniform variable.
    pub fn gl_uniform_3ui() -> &'static str {
        "glUniform3ui"
    }

    /// glUniform3uiv: Sets 3D unsigned integer uniform variables.
    pub fn gl_uniform_3uiv() -> &'static str {
        "glUniform3uiv"
    }

    /// glUniform4ui: Sets a 4D unsigned integer uniform variable.
    pub fn gl_uniform_4ui() -> &'static str {
        "glUniform4ui"
    }

    /// glUniform4uiv: Sets 4D unsigned integer uniform variables.
    pub fn gl_uniform_4uiv() -> &'static str {
        "glUniform4uiv"
    }

    /// glVertexAttribI4i: Sets a 4D signed integer value for a vertex attribute.
    pub fn gl_vertex_attrib_i4i() -> &'static str {
        "glVertexAttribI4i"
    }

    /// glVertexAttribI4iv: Sets 4D signed integer values for a vertex attribute.
    pub fn gl_vertex_attrib_i4iv() -> &'static str {
        "glVertexAttribI4iv"
    }

    /// glVertexAttribI4ui: Sets a 4D unsigned integer value for a vertex attribute.
    pub fn gl_vertex_attrib_i4ui() -> &'static str {
        "glVertexAttribI4ui"
    }

    /// glVertexAttribI4uiv: Sets 4D unsigned integer values for a vertex attribute.
    pub fn gl_vertex_attrib_i4uiv() -> &'static str {
        "glVertexAttribI4uiv"
    }

    /// glVertexAttribIPointer: Specifies the location and data format for integer vertex attributes.
    pub fn gl_vertex_attrib_i_pointer() -> &'static str {
        "glVertexAttribIPointer"
    }

    // Desktop OpenGL only (not in ES 2.0) functions

    /// glColorMaski: Enables/disables writing to color buffer for individual buffers (ES 3.2+).
    pub fn gl_color_maski() -> &'static str {
        "glColorMaski"
    }

    /// glDisablei: Disables capabilities for indexed targets (ES 3.2+).
    pub fn gl_disablei() -> &'static str {
        "glDisablei"
    }

    /// glEnablei: Enables capabilities for indexed targets (ES 3.2+).
    pub fn gl_enablei() -> &'static str {
        "glEnablei"
    }

    /// glIsEnabledi: Tests if a capability is enabled for indexed targets (ES 3.2+).
    pub fn gl_is_enabledi() -> &'static str {
        "glIsEnabledi"
    }

    /// glGetTexParameterIiv: Returns signed integer texture parameter values.
    pub fn gl_get_tex_parameter_iiv() -> &'static str {
        "glGetTexParameterIiv"
    }

    /// glGetTexParameterIuiv: Returns unsigned integer texture parameter values.
    pub fn gl_get_tex_parameter_iuiv() -> &'static str {
        "glGetTexParameterIuiv"
    }

    /// glTexParameterIiv: Sets signed integer texture parameters.
    pub fn gl_tex_parameter_iiv() -> &'static str {
        "glTexParameterIiv"
    }

    /// glTexParameterIuiv: Sets unsigned integer texture parameters.
    pub fn gl_tex_parameter_iuiv() -> &'static str {
        "glTexParameterIuiv"
    }

    /// glBeginConditionalRender: Begins conditional rendering based on query results.
    pub fn gl_begin_conditional_render() -> &'static str {
        "glBeginConditionalRender"
    }

    /// glBindFragDataLocation: Binds a user-defined varying out variable to a framebuffer color.
    pub fn gl_bind_frag_data_location() -> &'static str {
        "glBindFragDataLocation"
    }

    /// glClampColor: Controls color clamping.
    pub fn gl_clamp_color() -> &'static str {
        "glClampColor"
    }

    /// glEndConditionalRender: Ends conditional rendering.
    pub fn gl_end_conditional_render() -> &'static str {
        "glEndConditionalRender"
    }

    /// glVertexAttribI1i: Sets a 1D signed integer value for a vertex attribute.
    pub fn gl_vertex_attrib_i1i() -> &'static str {
        "glVertexAttribI1i"
    }

    /// glVertexAttribI1iv: Sets 1D signed integer values for a vertex attribute.
    pub fn gl_vertex_attrib_i1iv() -> &'static str {
        "glVertexAttribI1iv"
    }

    /// glVertexAttribI1ui: Sets a 1D unsigned integer value for a vertex attribute.
    pub fn gl_vertex_attrib_i1ui() -> &'static str {
        "glVertexAttribI1ui"
    }

    /// glVertexAttribI1uiv: Sets 1D unsigned integer values for a vertex attribute.
    pub fn gl_vertex_attrib_i1uiv() -> &'static str {
        "glVertexAttribI1uiv"
    }

    /// glVertexAttribI2i: Sets a 2D signed integer value for a vertex attribute.
    pub fn gl_vertex_attrib_i2i() -> &'static str {
        "glVertexAttribI2i"
    }

    /// glVertexAttribI2iv: Sets 2D signed integer values for a vertex attribute.
    pub fn gl_vertex_attrib_i2iv() -> &'static str {
        "glVertexAttribI2iv"
    }

    /// glVertexAttribI2ui: Sets a 2D unsigned integer value for a vertex attribute.
    pub fn gl_vertex_attrib_i2ui() -> &'static str {
        "glVertexAttribI2ui"
    }

    /// glVertexAttribI2uiv: Sets 2D unsigned integer values for a vertex attribute.
    pub fn gl_vertex_attrib_i2uiv() -> &'static str {
        "glVertexAttribI2uiv"
    }

    /// glVertexAttribI3i: Sets a 3D signed integer value for a vertex attribute.
    pub fn gl_vertex_attrib_i3i() -> &'static str {
        "glVertexAttribI3i"
    }

    /// glVertexAttribI3iv: Sets 3D signed integer values for a vertex attribute.
    pub fn gl_vertex_attrib_i3iv() -> &'static str {
        "glVertexAttribI3iv"
    }

    /// glVertexAttribI3ui: Sets a 3D unsigned integer value for a vertex attribute.
    pub fn gl_vertex_attrib_i3ui() -> &'static str {
        "glVertexAttribI3ui"
    }

    /// glVertexAttribI3uiv: Sets 3D unsigned integer values for a vertex attribute.
    pub fn gl_vertex_attrib_i3uiv() -> &'static str {
        "glVertexAttribI3uiv"
    }

    /// glVertexAttribI4bv: Sets a 4D signed byte value for a vertex attribute.
    pub fn gl_vertex_attrib_i4bv() -> &'static str {
        "glVertexAttribI4bv"
    }

    /// glVertexAttribI4sv: Sets 4D signed short values for a vertex attribute.
    pub fn gl_vertex_attrib_i4sv() -> &'static str {
        "glVertexAttribI4sv"
    }

    /// glVertexAttribI4ubv: Sets a 4D unsigned byte value for a vertex attribute.
    pub fn gl_vertex_attrib_i4ubv() -> &'static str {
        "glVertexAttribI4ubv"
    }

    /// glVertexAttribI4usv: Sets 4D unsigned short values for a vertex attribute.
    pub fn gl_vertex_attrib_i4usv() -> &'static str {
        "glVertexAttribI4usv"
    }

    // OpenGL ES 3.0+ specific functions

    /// glBeginQuery: Begins recording samples for a query (ES 3.0+).
    pub fn gl_begin_query() -> &'static str {
        "glBeginQuery"
    }

    /// glDeleteQueries: Deletes query objects (ES 3.0+).
    pub fn gl_delete_queries() -> &'static str {
        "glDeleteQueries"
    }

    /// glEndQuery: Ends recording samples for a query (ES 3.0+).
    pub fn gl_end_query() -> &'static str {
        "glEndQuery"
    }

    /// glGenQueries: Generates query object names (ES 3.0+).
    pub fn gl_gen_queries() -> &'static str {
        "glGenQueries"
    }

    /// glGetQueryiv: Retrieves query state parameters (ES 3.0+).
    pub fn gl_get_queryiv() -> &'static str {
        "glGetQueryiv"
    }

    /// glGetQueryObjectuiv: Retrieves unsigned integer query results (ES 3.0+).
    pub fn gl_get_query_objectuiv() -> &'static str {
        "glGetQueryObjectuiv"
    }

    /// glIsQuery: Tests if a query object name is valid (ES 3.0+).
    pub fn gl_is_query() -> &'static str {
        "glIsQuery"
    }

    /// glUnmapBuffer: Unmaps a previously mapped buffer object (ES 3.0+, not Emscripten).
    pub fn gl_unmap_buffer() -> &'static str {
        "glUnmapBuffer"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core30_framebuffer_functions() {
        // Verify framebuffer operations
        assert_eq!(OpenGlGlCore30::gl_bind_framebuffer(), "glBindFramebuffer");
        assert_eq!(OpenGlGlCore30::gl_gen_framebuffers(), "glGenFramebuffers");
        assert_eq!(OpenGlGlCore30::gl_delete_framebuffers(), "glDeleteFramebuffers");
        assert_eq!(OpenGlGlCore30::gl_check_framebuffer_status(), "glCheckFramebufferStatus");
    }

    #[test]
    fn test_gl_core30_vertex_array_functions() {
        // Verify vertex array operations
        assert_eq!(OpenGlGlCore30::gl_bind_vertex_array(), "glBindVertexArray");
        assert_eq!(OpenGlGlCore30::gl_gen_vertex_arrays(), "glGenVertexArrays");
        assert_eq!(OpenGlGlCore30::gl_delete_vertex_arrays(), "glDeleteVertexArrays");
    }

    #[test]
    fn test_gl_core30_transform_feedback_functions() {
        // Verify transform feedback operations
        assert_eq!(OpenGlGlCore30::gl_begin_transform_feedback(), "glBeginTransformFeedback");
        assert_eq!(OpenGlGlCore30::gl_end_transform_feedback(), "glEndTransformFeedback");
    }

    #[test]
    fn test_gl_core30_unsigned_uniform_functions() {
        // Verify unsigned integer uniform functions
        let funcs = vec![
            OpenGlGlCore30::gl_uniform_1ui(),
            OpenGlGlCore30::gl_uniform_2ui(),
            OpenGlGlCore30::gl_uniform_3ui(),
            OpenGlGlCore30::gl_uniform_4ui(),
        ];

        for func in funcs {
            assert!(func.contains("Uniform"));
            assert!(func.contains("ui"));
        }
    }
}
