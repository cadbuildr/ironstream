// FILE: open_gl_gl_core32.rs
// occt: OpenGl_GlCore32

/// OpenGL 3.2 core.
/// Introduces synchronization, multisampled textures, and base vertex drawing.
pub struct OpenGlGlCore32;

impl OpenGlGlCore32 {
    // GL_ARB_draw_elements_base_vertex functions (added to OpenGL 3.2 core)

    /// glDrawElementsBaseVertex: Renders indexed primitives with a base vertex offset.
    pub fn gl_draw_elements_base_vertex() -> &'static str {
        "glDrawElementsBaseVertex"
    }

    /// glDrawElementsInstancedBaseVertex: Renders instanced indexed primitives with base vertex.
    pub fn gl_draw_elements_instanced_base_vertex() -> &'static str {
        "glDrawElementsInstancedBaseVertex"
    }

    /// glDrawRangeElementsBaseVertex: Renders indexed primitives in a range with base vertex.
    pub fn gl_draw_range_elements_base_vertex() -> &'static str {
        "glDrawRangeElementsBaseVertex"
    }

    /// glMultiDrawElementsBaseVertex: Renders multiple sets of indexed primitives with base vertex.
    pub fn gl_multi_draw_elements_base_vertex() -> &'static str {
        "glMultiDrawElementsBaseVertex"
    }

    // GL_ARB_provoking_vertex functions (added to OpenGL 3.2 core)

    /// glProvokingVertex: Specifies which vertex is the provoking vertex in flat shading.
    pub fn gl_provoking_vertex() -> &'static str {
        "glProvokingVertex"
    }

    // GL_ARB_sync functions (added to OpenGL 3.2 core)

    /// glClientWaitSync: Blocks until a sync object is signaled or timeout expires.
    pub fn gl_client_wait_sync() -> &'static str {
        "glClientWaitSync"
    }

    /// glDeleteSync: Deletes a sync object.
    pub fn gl_delete_sync() -> &'static str {
        "glDeleteSync"
    }

    /// glFenceSync: Creates a fence sync object.
    pub fn gl_fence_sync() -> &'static str {
        "glFenceSync"
    }

    /// glGetInteger64v: Retrieves 64-bit integer state variables.
    pub fn gl_get_integer_64v() -> &'static str {
        "glGetInteger64v"
    }

    /// glGetSynciv: Retrieves sync object parameter values.
    pub fn gl_get_synciv() -> &'static str {
        "glGetSynciv"
    }

    /// glIsSync: Tests if a name is a sync object.
    pub fn gl_is_sync() -> &'static str {
        "glIsSync"
    }

    /// glWaitSync: Blocks GPU execution until a sync object is signaled.
    pub fn gl_wait_sync() -> &'static str {
        "glWaitSync"
    }

    // GL_ARB_texture_multisample functions (added to OpenGL 3.2 core)

    /// glGetMultisamplefv: Retrieves multisample sample positions.
    pub fn gl_get_multisamplefv() -> &'static str {
        "glGetMultisamplefv"
    }

    /// glSampleMaski: Sets sample coverage mask for a particular sample.
    pub fn gl_sample_maski() -> &'static str {
        "glSampleMaski"
    }

    /// glTexImage2DMultisample: Creates a multisampled 2D texture.
    pub fn gl_tex_image_2d_multisample() -> &'static str {
        "glTexImage2DMultisample"
    }

    /// glTexImage3DMultisample: Creates a multisampled 3D texture.
    pub fn gl_tex_image_3d_multisample() -> &'static str {
        "glTexImage3DMultisample"
    }

    // OpenGL 3.2 core additives to 3.1

    /// glFramebufferTexture: Attaches a texture to a framebuffer.
    pub fn gl_framebuffer_texture() -> &'static str {
        "glFramebufferTexture"
    }

    /// glGetBufferParameteri64v: Retrieves 64-bit integer buffer parameter values.
    pub fn gl_get_buffer_parameteri_64v() -> &'static str {
        "glGetBufferParameteri64v"
    }

    /// glGetInteger64i_v: Retrieves indexed 64-bit integer state variables.
    pub fn gl_get_integer_64i_v() -> &'static str {
        "glGetInteger64i_v"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core32_base_vertex_functions() {
        // Verify base vertex drawing functions
        assert_eq!(
            OpenGlGlCore32::gl_draw_elements_base_vertex(),
            "glDrawElementsBaseVertex"
        );
        assert_eq!(
            OpenGlGlCore32::gl_draw_elements_instanced_base_vertex(),
            "glDrawElementsInstancedBaseVertex"
        );
    }

    #[test]
    fn test_gl_core32_sync_functions() {
        // Verify sync object functions
        assert_eq!(OpenGlGlCore32::gl_fence_sync(), "glFenceSync");
        assert_eq!(OpenGlGlCore32::gl_client_wait_sync(), "glClientWaitSync");
        assert_eq!(OpenGlGlCore32::gl_wait_sync(), "glWaitSync");
        assert_eq!(OpenGlGlCore32::gl_delete_sync(), "glDeleteSync");
    }

    #[test]
    fn test_gl_core32_multisample_functions() {
        // Verify multisample texture functions
        assert_eq!(
            OpenGlGlCore32::gl_tex_image_2d_multisample(),
            "glTexImage2DMultisample"
        );
        assert_eq!(
            OpenGlGlCore32::gl_tex_image_3d_multisample(),
            "glTexImage3DMultisample"
        );
    }

    #[test]
    fn test_gl_core32_64bit_functions() {
        // Verify 64-bit integer functions
        let funcs = vec![
            OpenGlGlCore32::gl_get_integer_64v(),
            OpenGlGlCore32::gl_get_buffer_parameteri_64v(),
            OpenGlGlCore32::gl_get_integer_64i_v(),
        ];

        for func in funcs {
            assert!(func.contains("64"));
        }
    }
}
