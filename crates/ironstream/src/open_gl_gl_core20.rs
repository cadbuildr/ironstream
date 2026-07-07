// FILE: open_gl_gl_core20.rs
// occt: OpenGl_GlCore20

/// OpenGL 2.0 core based on 1.5 version.
/// Introduces programmable shader pipeline with vertex and fragment shaders.
pub struct OpenGlGlCore20;

impl OpenGlGlCore20 {
    /// OpenGL 2.0: glAttachShader
    /// Attaches a compiled shader object to a program object.
    pub fn gl_attach_shader() -> &'static str {
        "glAttachShader"
    }

    /// OpenGL 2.0: glBindAttribLocation
    /// Associates a generic vertex attribute index with a named attribute variable.
    pub fn gl_bind_attrib_location() -> &'static str {
        "glBindAttribLocation"
    }

    /// OpenGL 2.0: glBlendEquationSeparate
    /// Specifies separate blend equations for RGB and alpha.
    pub fn gl_blend_equation_separate() -> &'static str {
        "glBlendEquationSeparate"
    }

    /// OpenGL 2.0: glCompileShader
    /// Compiles a shader object.
    pub fn gl_compile_shader() -> &'static str {
        "glCompileShader"
    }

    /// OpenGL 2.0: glCreateProgram
    /// Creates a new program object.
    pub fn gl_create_program() -> &'static str {
        "glCreateProgram"
    }

    /// OpenGL 2.0: glCreateShader
    /// Creates a new shader object.
    pub fn gl_create_shader() -> &'static str {
        "glCreateShader"
    }

    /// OpenGL 2.0: glDeleteProgram
    /// Deletes a program object.
    pub fn gl_delete_program() -> &'static str {
        "glDeleteProgram"
    }

    /// OpenGL 2.0: glDeleteShader
    /// Deletes a shader object.
    pub fn gl_delete_shader() -> &'static str {
        "glDeleteShader"
    }

    /// OpenGL 2.0: glDetachShader
    /// Detaches a shader object from a program object.
    pub fn gl_detach_shader() -> &'static str {
        "glDetachShader"
    }

    /// OpenGL 2.0: glDisableVertexAttribArray
    /// Disables a generic vertex attribute array.
    pub fn gl_disable_vertex_attrib_array() -> &'static str {
        "glDisableVertexAttribArray"
    }

    /// OpenGL 2.0: glEnableVertexAttribArray
    /// Enables a generic vertex attribute array.
    pub fn gl_enable_vertex_attrib_array() -> &'static str {
        "glEnableVertexAttribArray"
    }

    /// OpenGL 2.0: glGetActiveAttrib
    /// Retrieves information about an active generic attribute.
    pub fn gl_get_active_attrib() -> &'static str {
        "glGetActiveAttrib"
    }

    /// OpenGL 2.0: glGetActiveUniform
    /// Retrieves information about an active uniform variable.
    pub fn gl_get_active_uniform() -> &'static str {
        "glGetActiveUniform"
    }

    /// OpenGL 2.0: glGetAttachedShaders
    /// Returns a list of attached shaders.
    pub fn gl_get_attached_shaders() -> &'static str {
        "glGetAttachedShaders"
    }

    /// OpenGL 2.0: glGetAttribLocation
    /// Returns the location of an attribute variable.
    pub fn gl_get_attrib_location() -> &'static str {
        "glGetAttribLocation"
    }

    /// OpenGL 2.0: glGetProgramInfoLog
    /// Returns the log of a program object.
    pub fn gl_get_program_info_log() -> &'static str {
        "glGetProgramInfoLog"
    }

    /// OpenGL 2.0: glGetProgramiv
    /// Returns a parameter from a program object.
    pub fn gl_get_programiv() -> &'static str {
        "glGetProgramiv"
    }

    /// OpenGL 2.0: glGetShaderInfoLog
    /// Returns the log of a shader object.
    pub fn gl_get_shader_info_log() -> &'static str {
        "glGetShaderInfoLog"
    }

    /// OpenGL 2.0: glGetShaderiv
    /// Returns a parameter from a shader object.
    pub fn gl_get_shaderiv() -> &'static str {
        "glGetShaderiv"
    }

    /// OpenGL 2.0: glGetShaderSource
    /// Returns the source code of a shader object.
    pub fn gl_get_shader_source() -> &'static str {
        "glGetShaderSource"
    }

    /// OpenGL 2.0: glGetUniformfv
    /// Returns the value of a uniform variable (float).
    pub fn gl_get_uniformfv() -> &'static str {
        "glGetUniformfv"
    }

    /// OpenGL 2.0: glGetUniformiv
    /// Returns the value of a uniform variable (integer).
    pub fn gl_get_uniformiv() -> &'static str {
        "glGetUniformiv"
    }

    /// OpenGL 2.0: glGetUniformLocation
    /// Returns the location of a uniform variable.
    pub fn gl_get_uniform_location() -> &'static str {
        "glGetUniformLocation"
    }

    /// OpenGL 2.0: glGetVertexAttribfv
    /// Returns a generic vertex attribute parameter (float).
    pub fn gl_get_vertex_attribfv() -> &'static str {
        "glGetVertexAttribfv"
    }

    /// OpenGL 2.0: glGetVertexAttribiv
    /// Returns a generic vertex attribute parameter (integer).
    pub fn gl_get_vertex_attribiv() -> &'static str {
        "glGetVertexAttribiv"
    }

    /// OpenGL 2.0: glGetVertexAttribPointerv
    /// Returns the address of a generic vertex attribute.
    pub fn gl_get_vertex_attrib_pointerv() -> &'static str {
        "glGetVertexAttribPointerv"
    }

    /// OpenGL 2.0: glIsProgram
    /// Tests if a program object name is valid.
    pub fn gl_is_program() -> &'static str {
        "glIsProgram"
    }

    /// OpenGL 2.0: glIsShader
    /// Tests if a shader object name is valid.
    pub fn gl_is_shader() -> &'static str {
        "glIsShader"
    }

    /// OpenGL 2.0: glLinkProgram
    /// Links a program object.
    pub fn gl_link_program() -> &'static str {
        "glLinkProgram"
    }

    /// OpenGL 2.0: glShaderSource
    /// Loads source code into a shader object.
    pub fn gl_shader_source() -> &'static str {
        "glShaderSource"
    }

    /// OpenGL 2.0: glStencilFuncSeparate
    /// Sets separate stencil test function and reference value for front and back.
    pub fn gl_stencil_func_separate() -> &'static str {
        "glStencilFuncSeparate"
    }

    /// OpenGL 2.0: glStencilMaskSeparate
    /// Sets separate stencil write masks for front and back.
    pub fn gl_stencil_mask_separate() -> &'static str {
        "glStencilMaskSeparate"
    }

    /// OpenGL 2.0: glStencilOpSeparate
    /// Sets separate stencil test actions for front and back.
    pub fn gl_stencil_op_separate() -> &'static str {
        "glStencilOpSeparate"
    }

    /// OpenGL 2.0: glUniform1f
    /// Sets a float uniform variable.
    pub fn gl_uniform_1f() -> &'static str {
        "glUniform1f"
    }

    /// OpenGL 2.0: glUniform1fv
    /// Sets float uniform variables as a vector.
    pub fn gl_uniform_1fv() -> &'static str {
        "glUniform1fv"
    }

    /// OpenGL 2.0: glUniform1i
    /// Sets an integer uniform variable.
    pub fn gl_uniform_1i() -> &'static str {
        "glUniform1i"
    }

    /// OpenGL 2.0: glUniform1iv
    /// Sets integer uniform variables as a vector.
    pub fn gl_uniform_1iv() -> &'static str {
        "glUniform1iv"
    }

    /// OpenGL 2.0: glUniform2f
    /// Sets a float vector (2 components) uniform variable.
    pub fn gl_uniform_2f() -> &'static str {
        "glUniform2f"
    }

    /// OpenGL 2.0: glUniform2fv
    /// Sets float vector (2 components) uniform variables.
    pub fn gl_uniform_2fv() -> &'static str {
        "glUniform2fv"
    }

    /// OpenGL 2.0: glUniform2i
    /// Sets an integer vector (2 components) uniform variable.
    pub fn gl_uniform_2i() -> &'static str {
        "glUniform2i"
    }

    /// OpenGL 2.0: glUniform2iv
    /// Sets integer vector (2 components) uniform variables.
    pub fn gl_uniform_2iv() -> &'static str {
        "glUniform2iv"
    }

    /// OpenGL 2.0: glUniform3f
    /// Sets a float vector (3 components) uniform variable.
    pub fn gl_uniform_3f() -> &'static str {
        "glUniform3f"
    }

    /// OpenGL 2.0: glUniform3fv
    /// Sets float vector (3 components) uniform variables.
    pub fn gl_uniform_3fv() -> &'static str {
        "glUniform3fv"
    }

    /// OpenGL 2.0: glUniform3i
    /// Sets an integer vector (3 components) uniform variable.
    pub fn gl_uniform_3i() -> &'static str {
        "glUniform3i"
    }

    /// OpenGL 2.0: glUniform3iv
    /// Sets integer vector (3 components) uniform variables.
    pub fn gl_uniform_3iv() -> &'static str {
        "glUniform3iv"
    }

    /// OpenGL 2.0: glUniform4f
    /// Sets a float vector (4 components) uniform variable.
    pub fn gl_uniform_4f() -> &'static str {
        "glUniform4f"
    }

    /// OpenGL 2.0: glUniform4fv
    /// Sets float vector (4 components) uniform variables.
    pub fn gl_uniform_4fv() -> &'static str {
        "glUniform4fv"
    }

    /// OpenGL 2.0: glUniform4i
    /// Sets an integer vector (4 components) uniform variable.
    pub fn gl_uniform_4i() -> &'static str {
        "glUniform4i"
    }

    /// OpenGL 2.0: glUniform4iv
    /// Sets integer vector (4 components) uniform variables.
    pub fn gl_uniform_4iv() -> &'static str {
        "glUniform4iv"
    }

    /// OpenGL 2.0: glUniformMatrix2fv
    /// Sets a 2x2 float matrix uniform variable.
    pub fn gl_uniform_matrix_2fv() -> &'static str {
        "glUniformMatrix2fv"
    }

    /// OpenGL 2.0: glUniformMatrix3fv
    /// Sets a 3x3 float matrix uniform variable.
    pub fn gl_uniform_matrix_3fv() -> &'static str {
        "glUniformMatrix3fv"
    }

    /// OpenGL 2.0: glUniformMatrix4fv
    /// Sets a 4x4 float matrix uniform variable.
    pub fn gl_uniform_matrix_4fv() -> &'static str {
        "glUniformMatrix4fv"
    }

    /// OpenGL 2.0: glUseProgram
    /// Installs a program object as the current rendering state.
    pub fn gl_use_program() -> &'static str {
        "glUseProgram"
    }

    /// OpenGL 2.0: glValidateProgram
    /// Validates a program object.
    pub fn gl_validate_program() -> &'static str {
        "glValidateProgram"
    }

    /// OpenGL 2.0: glVertexAttrib1f
    /// Sets a float value for a generic vertex attribute.
    pub fn gl_vertex_attrib_1f() -> &'static str {
        "glVertexAttrib1f"
    }

    /// OpenGL 2.0: glVertexAttrib1fv
    /// Sets float values for a generic vertex attribute.
    pub fn gl_vertex_attrib_1fv() -> &'static str {
        "glVertexAttrib1fv"
    }

    /// OpenGL 2.0: glVertexAttrib2f
    /// Sets a 2D float value for a generic vertex attribute.
    pub fn gl_vertex_attrib_2f() -> &'static str {
        "glVertexAttrib2f"
    }

    /// OpenGL 2.0: glVertexAttrib2fv
    /// Sets 2D float values for a generic vertex attribute.
    pub fn gl_vertex_attrib_2fv() -> &'static str {
        "glVertexAttrib2fv"
    }

    /// OpenGL 2.0: glVertexAttrib3f
    /// Sets a 3D float value for a generic vertex attribute.
    pub fn gl_vertex_attrib_3f() -> &'static str {
        "glVertexAttrib3f"
    }

    /// OpenGL 2.0: glVertexAttrib3fv
    /// Sets 3D float values for a generic vertex attribute.
    pub fn gl_vertex_attrib_3fv() -> &'static str {
        "glVertexAttrib3fv"
    }

    /// OpenGL 2.0: glVertexAttrib4f
    /// Sets a 4D float value for a generic vertex attribute.
    pub fn gl_vertex_attrib_4f() -> &'static str {
        "glVertexAttrib4f"
    }

    /// OpenGL 2.0: glVertexAttrib4fv
    /// Sets 4D float values for a generic vertex attribute.
    pub fn gl_vertex_attrib_4fv() -> &'static str {
        "glVertexAttrib4fv"
    }

    /// OpenGL 2.0: glVertexAttribPointer
    /// Specifies the location and data format of the generic vertex attribute.
    pub fn gl_vertex_attrib_pointer() -> &'static str {
        "glVertexAttribPointer"
    }

    /// OpenGL 2.0 (not in ES 2.0): glDrawBuffers
    /// Specifies a list of color buffers to be drawn into.
    pub fn gl_draw_buffers() -> &'static str {
        "glDrawBuffers"
    }

    /// OpenGL 2.0 (not in ES 2.0): glGetVertexAttribdv
    /// Returns a generic vertex attribute parameter (double).
    pub fn gl_get_vertex_attribdv() -> &'static str {
        "glGetVertexAttribdv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib1d
    /// Sets a double value for a generic vertex attribute.
    pub fn gl_vertex_attrib_1d() -> &'static str {
        "glVertexAttrib1d"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib1dv
    /// Sets double values for a generic vertex attribute.
    pub fn gl_vertex_attrib_1dv() -> &'static str {
        "glVertexAttrib1dv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib1s
    /// Sets a short value for a generic vertex attribute.
    pub fn gl_vertex_attrib_1s() -> &'static str {
        "glVertexAttrib1s"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib1sv
    /// Sets short values for a generic vertex attribute.
    pub fn gl_vertex_attrib_1sv() -> &'static str {
        "glVertexAttrib1sv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib2d
    /// Sets a 2D double value for a generic vertex attribute.
    pub fn gl_vertex_attrib_2d() -> &'static str {
        "glVertexAttrib2d"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib2dv
    /// Sets 2D double values for a generic vertex attribute.
    pub fn gl_vertex_attrib_2dv() -> &'static str {
        "glVertexAttrib2dv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib2s
    /// Sets a 2D short value for a generic vertex attribute.
    pub fn gl_vertex_attrib_2s() -> &'static str {
        "glVertexAttrib2s"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib2sv
    /// Sets 2D short values for a generic vertex attribute.
    pub fn gl_vertex_attrib_2sv() -> &'static str {
        "glVertexAttrib2sv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib3d
    /// Sets a 3D double value for a generic vertex attribute.
    pub fn gl_vertex_attrib_3d() -> &'static str {
        "glVertexAttrib3d"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib3dv
    /// Sets 3D double values for a generic vertex attribute.
    pub fn gl_vertex_attrib_3dv() -> &'static str {
        "glVertexAttrib3dv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib3s
    /// Sets a 3D short value for a generic vertex attribute.
    pub fn gl_vertex_attrib_3s() -> &'static str {
        "glVertexAttrib3s"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib3sv
    /// Sets 3D short values for a generic vertex attribute.
    pub fn gl_vertex_attrib_3sv() -> &'static str {
        "glVertexAttrib3sv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib4bv
    /// Sets a 4D byte value for a generic vertex attribute.
    pub fn gl_vertex_attrib_4bv() -> &'static str {
        "glVertexAttrib4bv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib4d
    /// Sets a 4D double value for a generic vertex attribute.
    pub fn gl_vertex_attrib_4d() -> &'static str {
        "glVertexAttrib4d"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib4dv
    /// Sets 4D double values for a generic vertex attribute.
    pub fn gl_vertex_attrib_4dv() -> &'static str {
        "glVertexAttrib4dv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib4iv
    /// Sets a 4D integer value for a generic vertex attribute.
    pub fn gl_vertex_attrib_4iv() -> &'static str {
        "glVertexAttrib4iv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib4Nbv
    /// Sets a normalized 4D byte value for a generic vertex attribute.
    pub fn gl_vertex_attrib_4nbv() -> &'static str {
        "glVertexAttrib4Nbv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib4Niv
    /// Sets a normalized 4D integer value for a generic vertex attribute.
    pub fn gl_vertex_attrib_4niv() -> &'static str {
        "glVertexAttrib4Niv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib4Nsv
    /// Sets a normalized 4D short value for a generic vertex attribute.
    pub fn gl_vertex_attrib_4nsv() -> &'static str {
        "glVertexAttrib4Nsv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib4Nub
    /// Sets a normalized 4D unsigned byte value for a generic vertex attribute.
    pub fn gl_vertex_attrib_4nub() -> &'static str {
        "glVertexAttrib4Nub"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib4Nubv
    /// Sets normalized 4D unsigned byte values for a generic vertex attribute.
    pub fn gl_vertex_attrib_4nubv() -> &'static str {
        "glVertexAttrib4Nubv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib4Nuiv
    /// Sets a normalized 4D unsigned integer value for a generic vertex attribute.
    pub fn gl_vertex_attrib_4nuiv() -> &'static str {
        "glVertexAttrib4Nuiv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib4Nusv
    /// Sets a normalized 4D unsigned short value for a generic vertex attribute.
    pub fn gl_vertex_attrib_4nusv() -> &'static str {
        "glVertexAttrib4Nusv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib4s
    /// Sets a 4D short value for a generic vertex attribute.
    pub fn gl_vertex_attrib_4s() -> &'static str {
        "glVertexAttrib4s"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib4sv
    /// Sets 4D short values for a generic vertex attribute.
    pub fn gl_vertex_attrib_4sv() -> &'static str {
        "glVertexAttrib4sv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib4ubv
    /// Sets a 4D unsigned byte value for a generic vertex attribute.
    pub fn gl_vertex_attrib_4ubv() -> &'static str {
        "glVertexAttrib4ubv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib4uiv
    /// Sets a 4D unsigned integer value for a generic vertex attribute.
    pub fn gl_vertex_attrib_4uiv() -> &'static str {
        "glVertexAttrib4uiv"
    }

    /// OpenGL 2.0 (not in ES 2.0): glVertexAttrib4usv
    /// Sets a 4D unsigned short value for a generic vertex attribute.
    pub fn gl_vertex_attrib_4usv() -> &'static str {
        "glVertexAttrib4usv"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core20_shader_functions() {
        // Verify shader compilation and linking
        assert_eq!(OpenGlGlCore20::gl_create_shader(), "glCreateShader");
        assert_eq!(OpenGlGlCore20::gl_shader_source(), "glShaderSource");
        assert_eq!(OpenGlGlCore20::gl_compile_shader(), "glCompileShader");
        assert_eq!(OpenGlGlCore20::gl_create_program(), "glCreateProgram");
        assert_eq!(OpenGlGlCore20::gl_link_program(), "glLinkProgram");
    }

    #[test]
    fn test_gl_core20_uniform_functions() {
        // Verify uniform setting functions
        let funcs = vec![
            OpenGlGlCore20::gl_uniform_1f(),
            OpenGlGlCore20::gl_uniform_2f(),
            OpenGlGlCore20::gl_uniform_3f(),
            OpenGlGlCore20::gl_uniform_4f(),
        ];

        for func in funcs {
            assert!(func.contains("Uniform"));
        }
    }

    #[test]
    fn test_gl_core20_vertex_attrib_functions() {
        // Verify vertex attribute functions
        let funcs = vec![
            OpenGlGlCore20::gl_enable_vertex_attrib_array(),
            OpenGlGlCore20::gl_disable_vertex_attrib_array(),
            OpenGlGlCore20::gl_vertex_attrib_pointer(),
        ];

        for func in funcs {
            assert!(func.contains("VertexAttrib"));
        }
    }

    #[test]
    fn test_gl_core20_matrix_uniforms() {
        // Verify matrix uniform functions
        assert_eq!(OpenGlGlCore20::gl_uniform_matrix_2fv(), "glUniformMatrix2fv");
        assert_eq!(OpenGlGlCore20::gl_uniform_matrix_3fv(), "glUniformMatrix3fv");
        assert_eq!(OpenGlGlCore20::gl_uniform_matrix_4fv(), "glUniformMatrix4fv");
    }
}
