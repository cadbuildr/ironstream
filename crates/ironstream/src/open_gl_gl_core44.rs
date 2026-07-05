// FILE: open_gl_gl_core44.rs
// occt: OpenGl_GlCore44

/// OpenGL 4.4 core.
/// Introduces bulk buffer/texture binding and immutable buffer storage.
pub struct OpenGlGlCore44;

impl OpenGlGlCore44 {
    // OpenGL 4.4 core additives to 4.3

    /// glBindBuffersBase: Binds multiple buffers to indexed binding points.
    pub fn gl_bind_buffers_base() -> &'static str {
        "glBindBuffersBase"
    }

    /// glBindBuffersRange: Binds ranges of multiple buffers to indexed binding points.
    pub fn gl_bind_buffers_range() -> &'static str {
        "glBindBuffersRange"
    }

    /// glBindImageTextures: Binds multiple textures to image units.
    pub fn gl_bind_image_textures() -> &'static str {
        "glBindImageTextures"
    }

    /// glBindSamplers: Binds multiple samplers to texture units.
    pub fn gl_bind_samplers() -> &'static str {
        "glBindSamplers"
    }

    /// glBindTextures: Binds multiple textures to texture units.
    pub fn gl_bind_textures() -> &'static str {
        "glBindTextures"
    }

    /// glBindVertexBuffers: Binds multiple buffers to vertex buffer binding points.
    pub fn gl_bind_vertex_buffers() -> &'static str {
        "glBindVertexBuffers"
    }

    /// glBufferStorage: Creates and initializes an immutable buffer object's data store.
    pub fn gl_buffer_storage() -> &'static str {
        "glBufferStorage"
    }

    /// glClearTexImage: Clears all of a texture image.
    pub fn gl_clear_tex_image() -> &'static str {
        "glClearTexImage"
    }

    /// glClearTexSubImage: Clears a subset of a texture image.
    pub fn gl_clear_tex_sub_image() -> &'static str {
        "glClearTexSubImage"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core44_multi_bind_functions() {
        // Verify multi-bind functions
        assert_eq!(OpenGlGlCore44::gl_bind_buffers_base(), "glBindBuffersBase");
        assert_eq!(OpenGlGlCore44::gl_bind_buffers_range(), "glBindBuffersRange");
        assert_eq!(OpenGlGlCore44::gl_bind_image_textures(), "glBindImageTextures");
        assert_eq!(OpenGlGlCore44::gl_bind_samplers(), "glBindSamplers");
        assert_eq!(OpenGlGlCore44::gl_bind_textures(), "glBindTextures");
    }

    #[test]
    fn test_gl_core44_buffer_storage() {
        // Verify immutable buffer storage
        assert_eq!(OpenGlGlCore44::gl_buffer_storage(), "glBufferStorage");
    }

    #[test]
    fn test_gl_core44_clear_tex_functions() {
        // Verify clear texture functions
        assert_eq!(OpenGlGlCore44::gl_clear_tex_image(), "glClearTexImage");
        assert_eq!(OpenGlGlCore44::gl_clear_tex_sub_image(), "glClearTexSubImage");
    }
}
