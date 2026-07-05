// FILE: open_gl_gl_core42.rs
// occt: OpenGl_GlCore42

/// OpenGL 4.2 core.
/// Introduces atomic counters, image load/store, and immutable texture storage.
pub struct OpenGlGlCore42;

impl OpenGlGlCore42 {
    // GL_ARB_base_instance functions (added to OpenGL 4.2 core)

    /// glDrawArraysInstancedBaseInstance: Renders instanced primitives with base instance.
    pub fn gl_draw_arrays_instanced_base_instance() -> &'static str {
        "glDrawArraysInstancedBaseInstance"
    }

    /// glDrawElementsInstancedBaseInstance: Renders instanced indexed primitives with base instance.
    pub fn gl_draw_elements_instanced_base_instance() -> &'static str {
        "glDrawElementsInstancedBaseInstance"
    }

    /// glDrawElementsInstancedBaseVertexBaseInstance: Renders instanced primitives with base vertex and instance.
    pub fn gl_draw_elements_instanced_base_vertex_base_instance() -> &'static str {
        "glDrawElementsInstancedBaseVertexBaseInstance"
    }

    // GL_ARB_transform_feedback_instanced functions (added to OpenGL 4.2 core)

    /// glDrawTransformFeedbackInstanced: Renders instanced primitives from transform feedback.
    pub fn gl_draw_transform_feedback_instanced() -> &'static str {
        "glDrawTransformFeedbackInstanced"
    }

    /// glDrawTransformFeedbackStreamInstanced: Renders instanced primitives from transform feedback stream.
    pub fn gl_draw_transform_feedback_stream_instanced() -> &'static str {
        "glDrawTransformFeedbackStreamInstanced"
    }

    // GL_ARB_internalformat_query functions (added to OpenGL 4.2 core)

    /// glGetInternalformativ: Queries internal format properties.
    pub fn gl_get_internalformativ() -> &'static str {
        "glGetInternalformativ"
    }

    // GL_ARB_shader_atomic_counters functions (added to OpenGL 4.2 core)

    /// glGetActiveAtomicCounterBufferiv: Retrieves properties of active atomic counter buffers.
    pub fn gl_get_active_atomic_counter_bufferiv() -> &'static str {
        "glGetActiveAtomicCounterBufferiv"
    }

    // GL_ARB_shader_image_load_store functions (added to OpenGL 4.2 core)

    /// glBindImageTexture: Binds a texture to an image unit for shader access.
    pub fn gl_bind_image_texture() -> &'static str {
        "glBindImageTexture"
    }

    /// glMemoryBarrier: Controls memory coherency for shader image access.
    pub fn gl_memory_barrier() -> &'static str {
        "glMemoryBarrier"
    }

    // GL_ARB_texture_storage functions (added to OpenGL 4.2 core)

    /// glTexStorage1D: Creates an immutable 1D texture.
    pub fn gl_tex_storage_1d() -> &'static str {
        "glTexStorage1D"
    }

    /// glTexStorage2D: Creates an immutable 2D texture.
    pub fn gl_tex_storage_2d() -> &'static str {
        "glTexStorage2D"
    }

    /// glTexStorage3D: Creates an immutable 3D texture.
    pub fn gl_tex_storage_3d() -> &'static str {
        "glTexStorage3D"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core42_base_instance_functions() {
        // Verify base instance drawing functions
        assert_eq!(
            OpenGlGlCore42::gl_draw_arrays_instanced_base_instance(),
            "glDrawArraysInstancedBaseInstance"
        );
        assert_eq!(
            OpenGlGlCore42::gl_draw_elements_instanced_base_instance(),
            "glDrawElementsInstancedBaseInstance"
        );
    }

    #[test]
    fn test_gl_core42_texture_storage() {
        // Verify immutable texture storage functions
        assert_eq!(OpenGlGlCore42::gl_tex_storage_1d(), "glTexStorage1D");
        assert_eq!(OpenGlGlCore42::gl_tex_storage_2d(), "glTexStorage2D");
        assert_eq!(OpenGlGlCore42::gl_tex_storage_3d(), "glTexStorage3D");
    }

    #[test]
    fn test_gl_core42_image_load_store() {
        // Verify image load/store functions
        assert_eq!(OpenGlGlCore42::gl_bind_image_texture(), "glBindImageTexture");
        assert_eq!(OpenGlGlCore42::gl_memory_barrier(), "glMemoryBarrier");
    }
}
