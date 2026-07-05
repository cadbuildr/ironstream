// FILE: open_gl_gl_core31.rs
// occt: OpenGl_GlCore31

/// OpenGL 3.1 core.
/// Introduces uniform buffer objects, copy buffer operations, and instanced rendering.
pub struct OpenGlGlCore31;

impl OpenGlGlCore31 {
    // GL_ARB_uniform_buffer_object functions (added to OpenGL 3.1 core)

    /// glGetActiveUniformBlockiv: Retrieves properties of an active uniform block.
    pub fn gl_get_active_uniform_blockiv() -> &'static str {
        "glGetActiveUniformBlockiv"
    }

    /// glGetActiveUniformBlockName: Retrieves the name of an active uniform block.
    pub fn gl_get_active_uniform_block_name() -> &'static str {
        "glGetActiveUniformBlockName"
    }

    /// glGetActiveUniformsiv: Retrieves properties of active uniforms in a block.
    pub fn gl_get_active_uniformsiv() -> &'static str {
        "glGetActiveUniformsiv"
    }

    /// glGetUniformBlockIndex: Returns the index of a uniform block.
    pub fn gl_get_uniform_block_index() -> &'static str {
        "glGetUniformBlockIndex"
    }

    /// glGetUniformIndices: Returns indices of uniforms within a block.
    pub fn gl_get_uniform_indices() -> &'static str {
        "glGetUniformIndices"
    }

    /// glUniformBlockBinding: Assigns a binding point to a uniform block.
    pub fn gl_uniform_block_binding() -> &'static str {
        "glUniformBlockBinding"
    }

    /// glGetActiveUniformName: Retrieves the name of an active uniform (not in ES 2.0).
    pub fn gl_get_active_uniform_name() -> &'static str {
        "glGetActiveUniformName"
    }

    // GL_ARB_copy_buffer functions (added to OpenGL 3.1 core)

    /// glCopyBufferSubData: Copies a range of buffer data from one buffer to another.
    pub fn gl_copy_buffer_sub_data() -> &'static str {
        "glCopyBufferSubData"
    }

    // OpenGL 3.1 core additives to 3.0

    /// glDrawArraysInstanced: Renders multiple instances of primitive arrays.
    pub fn gl_draw_arrays_instanced() -> &'static str {
        "glDrawArraysInstanced"
    }

    /// glDrawElementsInstanced: Renders multiple instances of indexed primitives.
    pub fn gl_draw_elements_instanced() -> &'static str {
        "glDrawElementsInstanced"
    }

    /// glPrimitiveRestartIndex: Sets the primitive restart index (not in ES 2.0).
    pub fn gl_primitive_restart_index() -> &'static str {
        "glPrimitiveRestartIndex"
    }

    /// glTexBuffer: Attaches a buffer object to a buffer texture (not in ES 2.0, ES 3.2+).
    pub fn gl_tex_buffer() -> &'static str {
        "glTexBuffer"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core31_uniform_buffer_functions() {
        // Verify uniform buffer object functions
        assert_eq!(OpenGlGlCore31::gl_get_uniform_block_index(), "glGetUniformBlockIndex");
        assert_eq!(OpenGlGlCore31::gl_uniform_block_binding(), "glUniformBlockBinding");
        assert_eq!(
            OpenGlGlCore31::gl_get_active_uniform_blockiv(),
            "glGetActiveUniformBlockiv"
        );
    }

    #[test]
    fn test_gl_core31_instanced_rendering() {
        // Verify instanced rendering functions
        assert_eq!(
            OpenGlGlCore31::gl_draw_arrays_instanced(),
            "glDrawArraysInstanced"
        );
        assert_eq!(
            OpenGlGlCore31::gl_draw_elements_instanced(),
            "glDrawElementsInstanced"
        );
    }

    #[test]
    fn test_gl_core31_copy_buffer() {
        // Verify buffer copy function
        assert_eq!(OpenGlGlCore31::gl_copy_buffer_sub_data(), "glCopyBufferSubData");
    }

    #[test]
    fn test_gl_core31_non_es20_functions() {
        // Verify non-ES 2.0 functions
        let funcs = vec![
            OpenGlGlCore31::gl_get_active_uniform_name(),
            OpenGlGlCore31::gl_primitive_restart_index(),
            OpenGlGlCore31::gl_tex_buffer(),
        ];

        for func in funcs {
            assert!(!func.is_empty());
        }
    }
}
