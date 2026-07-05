// FILE: open_gl_gl_core45.rs
// occt: OpenGl_GlCore45

/// OpenGL 4.5 core.
/// Introduces direct state access functions (DSA) removing need for current binding state.
pub struct OpenGlGlCore45;

impl OpenGlGlCore45 {
    // OpenGL 4.5 core additives to 4.4

    /// glBindTextureUnit: Binds an existing texture object to a texture unit.
    pub fn gl_bind_texture_unit() -> &'static str {
        "glBindTextureUnit"
    }

    /// glBlitNamedFramebuffer: Copies pixels from one framebuffer to another.
    pub fn gl_blit_named_framebuffer() -> &'static str {
        "glBlitNamedFramebuffer"
    }

    /// glCheckNamedFramebufferStatus: Checks framebuffer status.
    pub fn gl_check_named_framebuffer_status() -> &'static str {
        "glCheckNamedFramebufferStatus"
    }

    /// glClearNamedBufferData: Clears all data in a named buffer.
    pub fn gl_clear_named_buffer_data() -> &'static str {
        "glClearNamedBufferData"
    }

    /// glClearNamedBufferSubData: Clears a subset of a named buffer's data.
    pub fn gl_clear_named_buffer_sub_data() -> &'static str {
        "glClearNamedBufferSubData"
    }

    /// glClearNamedFramebufferfi: Clears a named framebuffer to depth and stencil.
    pub fn gl_clear_named_framebufferfi() -> &'static str {
        "glClearNamedFramebufferfi"
    }

    /// glClearNamedFramebufferfv: Clears a named framebuffer to float values.
    pub fn gl_clear_named_framebufferfv() -> &'static str {
        "glClearNamedFramebufferfv"
    }

    /// glClearNamedFramebufferiv: Clears a named framebuffer to integer values.
    pub fn gl_clear_named_framebufferiv() -> &'static str {
        "glClearNamedFramebufferiv"
    }

    /// glClearNamedFramebufferuiv: Clears a named framebuffer to unsigned integer values.
    pub fn gl_clear_named_framebufferuiv() -> &'static str {
        "glClearNamedFramebufferuiv"
    }

    /// glClipControl: Controls clipping behavior.
    pub fn gl_clip_control() -> &'static str {
        "glClipControl"
    }

    /// glCompressedTextureSubImage1D: Updates a subset of a 1D compressed texture.
    pub fn gl_compressed_texture_sub_image_1d() -> &'static str {
        "glCompressedTextureSubImage1D"
    }

    /// glCompressedTextureSubImage2D: Updates a subset of a 2D compressed texture.
    pub fn gl_compressed_texture_sub_image_2d() -> &'static str {
        "glCompressedTextureSubImage2D"
    }

    /// glCompressedTextureSubImage3D: Updates a subset of a 3D compressed texture.
    pub fn gl_compressed_texture_sub_image_3d() -> &'static str {
        "glCompressedTextureSubImage3D"
    }

    /// glCopyNamedBufferSubData: Copies data between named buffers.
    pub fn gl_copy_named_buffer_sub_data() -> &'static str {
        "glCopyNamedBufferSubData"
    }

    /// glCopyTextureSubImage1D: Copies pixels to a 1D texture.
    pub fn gl_copy_texture_sub_image_1d() -> &'static str {
        "glCopyTextureSubImage1D"
    }

    /// glCopyTextureSubImage2D: Copies pixels to a 2D texture.
    pub fn gl_copy_texture_sub_image_2d() -> &'static str {
        "glCopyTextureSubImage2D"
    }

    /// glCopyTextureSubImage3D: Copies pixels to a 3D texture.
    pub fn gl_copy_texture_sub_image_3d() -> &'static str {
        "glCopyTextureSubImage3D"
    }

    /// glCreateBuffers: Creates buffer objects.
    pub fn gl_create_buffers() -> &'static str {
        "glCreateBuffers"
    }

    /// glCreateFramebuffers: Creates framebuffer objects.
    pub fn gl_create_framebuffers() -> &'static str {
        "glCreateFramebuffers"
    }

    /// glCreateProgramPipelines: Creates program pipelines.
    pub fn gl_create_program_pipelines() -> &'static str {
        "glCreateProgramPipelines"
    }

    /// glCreateQueries: Creates query objects.
    pub fn gl_create_queries() -> &'static str {
        "glCreateQueries"
    }

    /// glCreateRenderbuffers: Creates renderbuffer objects.
    pub fn gl_create_renderbuffers() -> &'static str {
        "glCreateRenderbuffers"
    }

    /// glCreateSamplers: Creates sampler objects.
    pub fn gl_create_samplers() -> &'static str {
        "glCreateSamplers"
    }

    /// glCreateTextures: Creates texture objects.
    pub fn gl_create_textures() -> &'static str {
        "glCreateTextures"
    }

    /// glCreateTransformFeedbacks: Creates transform feedback objects.
    pub fn gl_create_transform_feedbacks() -> &'static str {
        "glCreateTransformFeedbacks"
    }

    /// glCreateVertexArrays: Creates vertex array objects.
    pub fn gl_create_vertex_arrays() -> &'static str {
        "glCreateVertexArrays"
    }

    /// glDisableVertexArrayAttrib: Disables a vertex attribute in a vertex array.
    pub fn gl_disable_vertex_array_attrib() -> &'static str {
        "glDisableVertexArrayAttrib"
    }

    /// glEnableVertexArrayAttrib: Enables a vertex attribute in a vertex array.
    pub fn gl_enable_vertex_array_attrib() -> &'static str {
        "glEnableVertexArrayAttrib"
    }

    /// glFlushMappedNamedBufferRange: Flushes mapped named buffer range.
    pub fn gl_flush_mapped_named_buffer_range() -> &'static str {
        "glFlushMappedNamedBufferRange"
    }

    /// glGenerateTextureMipmap: Generates mipmaps for a named texture.
    pub fn gl_generate_texture_mipmap() -> &'static str {
        "glGenerateTextureMipmap"
    }

    /// glGetCompressedTextureImage: Retrieves a compressed texture image.
    pub fn gl_get_compressed_texture_image() -> &'static str {
        "glGetCompressedTextureImage"
    }

    /// glGetCompressedTextureSubImage: Retrieves a compressed texture subimage.
    pub fn gl_get_compressed_texture_sub_image() -> &'static str {
        "glGetCompressedTextureSubImage"
    }

    /// glGetGraphicsResetStatus: Gets the graphics reset status.
    pub fn gl_get_graphics_reset_status() -> &'static str {
        "glGetGraphicsResetStatus"
    }

    /// glGetNamedBufferParameteri64v: Gets named buffer parameters (64-bit integer).
    pub fn gl_get_named_buffer_parameteri_64v() -> &'static str {
        "glGetNamedBufferParameteri64v"
    }

    /// glGetNamedBufferParameteriv: Gets named buffer parameters (integer).
    pub fn gl_get_named_buffer_parameteriv() -> &'static str {
        "glGetNamedBufferParameteriv"
    }

    /// glGetNamedBufferPointerv: Gets the address of named buffer data.
    pub fn gl_get_named_buffer_pointerv() -> &'static str {
        "glGetNamedBufferPointerv"
    }

    /// glGetNamedBufferSubData: Retrieves a subset of named buffer data.
    pub fn gl_get_named_buffer_sub_data() -> &'static str {
        "glGetNamedBufferSubData"
    }

    /// glGetNamedFramebufferAttachmentParameteriv: Gets named framebuffer attachment parameters.
    pub fn gl_get_named_framebuffer_attachment_parameteriv() -> &'static str {
        "glGetNamedFramebufferAttachmentParameteriv"
    }

    /// glGetNamedFramebufferParameteriv: Gets named framebuffer parameters.
    pub fn gl_get_named_framebuffer_parameteriv() -> &'static str {
        "glGetNamedFramebufferParameteriv"
    }

    /// glGetNamedRenderbufferParameteriv: Gets named renderbuffer parameters.
    pub fn gl_get_named_renderbuffer_parameteriv() -> &'static str {
        "glGetNamedRenderbufferParameteriv"
    }

    /// glGetnCompressedTexImage: Gets compressed texture with bound check.
    pub fn gl_getn_compressed_tex_image() -> &'static str {
        "glGetnCompressedTexImage"
    }

    /// glGetnTexImage: Gets texture with bound check.
    pub fn gl_getn_tex_image() -> &'static str {
        "glGetnTexImage"
    }

    /// glGetnUniformdv: Gets double uniform with bound check.
    pub fn gl_getn_uniformdv() -> &'static str {
        "glGetnUniformdv"
    }

    /// glGetnUniformfv: Gets float uniform with bound check.
    pub fn gl_getn_uniformfv() -> &'static str {
        "glGetnUniformfv"
    }

    /// glGetnUniformiv: Gets integer uniform with bound check.
    pub fn gl_getn_uniformiv() -> &'static str {
        "glGetnUniformiv"
    }

    /// glGetnUniformuiv: Gets unsigned integer uniform with bound check.
    pub fn gl_getn_uniformuiv() -> &'static str {
        "glGetnUniformuiv"
    }

    /// glGetQueryBufferObjecti64v: Gets query results to buffer (64-bit int).
    pub fn gl_get_query_buffer_objecti_64v() -> &'static str {
        "glGetQueryBufferObjecti64v"
    }

    /// glGetQueryBufferObjectiv: Gets query results to buffer (int).
    pub fn gl_get_query_buffer_objectiv() -> &'static str {
        "glGetQueryBufferObjectiv"
    }

    /// glGetQueryBufferObjectui64v: Gets query results to buffer (64-bit uint).
    pub fn gl_get_query_buffer_objectui_64v() -> &'static str {
        "glGetQueryBufferObjectui64v"
    }

    /// glGetQueryBufferObjectuiv: Gets query results to buffer (uint).
    pub fn gl_get_query_buffer_objectuiv() -> &'static str {
        "glGetQueryBufferObjectuiv"
    }

    /// glGetTextureImage: Retrieves a texture image.
    pub fn gl_get_texture_image() -> &'static str {
        "glGetTextureImage"
    }

    /// glGetTextureLevelParameterfv: Gets texture level parameters (float).
    pub fn gl_get_texture_level_parameterfv() -> &'static str {
        "glGetTextureLevelParameterfv"
    }

    /// glGetTextureLevelParameteriv: Gets texture level parameters (int).
    pub fn gl_get_texture_level_parameteriv() -> &'static str {
        "glGetTextureLevelParameteriv"
    }

    /// glGetTextureParameterfv: Gets texture parameters (float).
    pub fn gl_get_texture_parameterfv() -> &'static str {
        "glGetTextureParameterfv"
    }

    /// glGetTextureParameterIiv: Gets texture parameters (signed int).
    pub fn gl_get_texture_parameter_iiv() -> &'static str {
        "glGetTextureParameterIiv"
    }

    /// glGetTextureParameterIuiv: Gets texture parameters (unsigned int).
    pub fn gl_get_texture_parameter_iuiv() -> &'static str {
        "glGetTextureParameterIuiv"
    }

    /// glGetTextureParameteriv: Gets texture parameters (int).
    pub fn gl_get_texture_parameteriv() -> &'static str {
        "glGetTextureParameteriv"
    }

    /// glGetTextureSubImage: Retrieves a texture subimage.
    pub fn gl_get_texture_sub_image() -> &'static str {
        "glGetTextureSubImage"
    }

    /// glGetTransformFeedbacki64_v: Gets transform feedback indexed 64-bit state.
    pub fn gl_get_transform_feedbacki_64_v() -> &'static str {
        "glGetTransformFeedbacki64_v"
    }

    /// glGetTransformFeedbacki_v: Gets transform feedback indexed state.
    pub fn gl_get_transform_feedbacki_v() -> &'static str {
        "glGetTransformFeedbacki_v"
    }

    /// glGetTransformFeedbackiv: Gets transform feedback state.
    pub fn gl_get_transform_feedbackiv() -> &'static str {
        "glGetTransformFeedbackiv"
    }

    /// glGetVertexArrayIndexed64iv: Gets vertex array indexed 64-bit state.
    pub fn gl_get_vertex_array_indexed_64iv() -> &'static str {
        "glGetVertexArrayIndexed64iv"
    }

    /// glGetVertexArrayIndexediv: Gets vertex array indexed state.
    pub fn gl_get_vertex_array_indexediv() -> &'static str {
        "glGetVertexArrayIndexediv"
    }

    /// glGetVertexArrayiv: Gets vertex array state.
    pub fn gl_get_vertex_arrayiv() -> &'static str {
        "glGetVertexArrayiv"
    }

    /// glInvalidateNamedFramebufferData: Invalidates named framebuffer data.
    pub fn gl_invalidate_named_framebuffer_data() -> &'static str {
        "glInvalidateNamedFramebufferData"
    }

    /// glInvalidateNamedFramebufferSubData: Invalidates named framebuffer subdata.
    pub fn gl_invalidate_named_framebuffer_sub_data() -> &'static str {
        "glInvalidateNamedFramebufferSubData"
    }

    /// glMapNamedBuffer: Maps a named buffer to client memory.
    pub fn gl_map_named_buffer() -> &'static str {
        "glMapNamedBuffer"
    }

    /// glMapNamedBufferRange: Maps a range of a named buffer.
    pub fn gl_map_named_buffer_range() -> &'static str {
        "glMapNamedBufferRange"
    }

    /// glMemoryBarrierByRegion: Synchronizes memory coherency by region.
    pub fn gl_memory_barrier_by_region() -> &'static str {
        "glMemoryBarrierByRegion"
    }

    /// glNamedBufferData: Creates and initializes named buffer data store.
    pub fn gl_named_buffer_data() -> &'static str {
        "glNamedBufferData"
    }

    /// glNamedBufferStorage: Creates immutable named buffer data store.
    pub fn gl_named_buffer_storage() -> &'static str {
        "glNamedBufferStorage"
    }

    /// glNamedBufferSubData: Updates a subset of named buffer data.
    pub fn gl_named_buffer_sub_data() -> &'static str {
        "glNamedBufferSubData"
    }

    /// glNamedFramebufferDrawBuffer: Sets draw buffer for named framebuffer.
    pub fn gl_named_framebuffer_draw_buffer() -> &'static str {
        "glNamedFramebufferDrawBuffer"
    }

    /// glNamedFramebufferDrawBuffers: Sets draw buffers for named framebuffer.
    pub fn gl_named_framebuffer_draw_buffers() -> &'static str {
        "glNamedFramebufferDrawBuffers"
    }

    /// glNamedFramebufferParameteri: Sets named framebuffer parameter.
    pub fn gl_named_framebuffer_parameteri() -> &'static str {
        "glNamedFramebufferParameteri"
    }

    /// glNamedFramebufferReadBuffer: Sets read buffer for named framebuffer.
    pub fn gl_named_framebuffer_read_buffer() -> &'static str {
        "glNamedFramebufferReadBuffer"
    }

    /// glNamedFramebufferRenderbuffer: Attaches renderbuffer to named framebuffer.
    pub fn gl_named_framebuffer_renderbuffer() -> &'static str {
        "glNamedFramebufferRenderbuffer"
    }

    /// glNamedFramebufferTexture: Attaches texture to named framebuffer.
    pub fn gl_named_framebuffer_texture() -> &'static str {
        "glNamedFramebufferTexture"
    }

    /// glNamedFramebufferTextureLayer: Attaches texture layer to named framebuffer.
    pub fn gl_named_framebuffer_texture_layer() -> &'static str {
        "glNamedFramebufferTextureLayer"
    }

    /// glNamedRenderbufferStorage: Creates named renderbuffer storage.
    pub fn gl_named_renderbuffer_storage() -> &'static str {
        "glNamedRenderbufferStorage"
    }

    /// glNamedRenderbufferStorageMultisample: Creates multisampled named renderbuffer storage.
    pub fn gl_named_renderbuffer_storage_multisample() -> &'static str {
        "glNamedRenderbufferStorageMultisample"
    }

    /// glReadnPixels: Reads pixels with bound check.
    pub fn gl_readn_pixels() -> &'static str {
        "glReadnPixels"
    }

    /// glTextureBarrier: Ensures texture and framebuffer coherency.
    pub fn gl_texture_barrier() -> &'static str {
        "glTextureBarrier"
    }

    /// glTextureBuffer: Attaches buffer to named buffer texture.
    pub fn gl_texture_buffer() -> &'static str {
        "glTextureBuffer"
    }

    /// glTextureBufferRange: Attaches buffer range to named buffer texture.
    pub fn gl_texture_buffer_range() -> &'static str {
        "glTextureBufferRange"
    }

    /// glTextureParameterf: Sets named texture parameter (float).
    pub fn gl_texture_parameterf() -> &'static str {
        "glTextureParameterf"
    }

    /// glTextureParameterfv: Sets named texture parameters (float).
    pub fn gl_texture_parameterfv() -> &'static str {
        "glTextureParameterfv"
    }

    /// glTextureParameteri: Sets named texture parameter (int).
    pub fn gl_texture_parameteri() -> &'static str {
        "glTextureParameteri"
    }

    /// glTextureParameterIiv: Sets named texture parameters (signed int).
    pub fn gl_texture_parameter_iiv() -> &'static str {
        "glTextureParameterIiv"
    }

    /// glTextureParameterIuiv: Sets named texture parameters (unsigned int).
    pub fn gl_texture_parameter_iuiv() -> &'static str {
        "glTextureParameterIuiv"
    }

    /// glTextureParameteriv: Sets named texture parameters (int).
    pub fn gl_texture_parameteriv() -> &'static str {
        "glTextureParameteriv"
    }

    /// glTextureStorage1D: Creates immutable named 1D texture.
    pub fn gl_texture_storage_1d() -> &'static str {
        "glTextureStorage1D"
    }

    /// glTextureStorage2D: Creates immutable named 2D texture.
    pub fn gl_texture_storage_2d() -> &'static str {
        "glTextureStorage2D"
    }

    /// glTextureStorage2DMultisample: Creates immutable multisampled named 2D texture.
    pub fn gl_texture_storage_2d_multisample() -> &'static str {
        "glTextureStorage2DMultisample"
    }

    /// glTextureStorage3D: Creates immutable named 3D texture.
    pub fn gl_texture_storage_3d() -> &'static str {
        "glTextureStorage3D"
    }

    /// glTextureStorage3DMultisample: Creates immutable multisampled named 3D texture.
    pub fn gl_texture_storage_3d_multisample() -> &'static str {
        "glTextureStorage3DMultisample"
    }

    /// glTextureSubImage1D: Updates a subset of named 1D texture.
    pub fn gl_texture_sub_image_1d() -> &'static str {
        "glTextureSubImage1D"
    }

    /// glTextureSubImage2D: Updates a subset of named 2D texture.
    pub fn gl_texture_sub_image_2d() -> &'static str {
        "glTextureSubImage2D"
    }

    /// glTextureSubImage3D: Updates a subset of named 3D texture.
    pub fn gl_texture_sub_image_3d() -> &'static str {
        "glTextureSubImage3D"
    }

    /// glTransformFeedbackBufferBase: Binds transform feedback buffer to indexed target.
    pub fn gl_transform_feedback_buffer_base() -> &'static str {
        "glTransformFeedbackBufferBase"
    }

    /// glTransformFeedbackBufferRange: Binds transform feedback buffer range.
    pub fn gl_transform_feedback_buffer_range() -> &'static str {
        "glTransformFeedbackBufferRange"
    }

    /// glUnmapNamedBuffer: Unmaps a named buffer.
    pub fn gl_unmap_named_buffer() -> &'static str {
        "glUnmapNamedBuffer"
    }

    /// glVertexArrayAttribBinding: Associates vertex attribute with binding.
    pub fn gl_vertex_array_attrib_binding() -> &'static str {
        "glVertexArrayAttribBinding"
    }

    /// glVertexArrayAttribFormat: Specifies vertex attribute format in array.
    pub fn gl_vertex_array_attrib_format() -> &'static str {
        "glVertexArrayAttribFormat"
    }

    /// glVertexArrayAttribIFormat: Specifies integer vertex attribute format in array.
    pub fn gl_vertex_array_attrib_i_format() -> &'static str {
        "glVertexArrayAttribIFormat"
    }

    /// glVertexArrayAttribLFormat: Specifies 64-bit vertex attribute format in array.
    pub fn gl_vertex_array_attrib_l_format() -> &'static str {
        "glVertexArrayAttribLFormat"
    }

    /// glVertexArrayBindingDivisor: Sets instance rate of vertex binding in array.
    pub fn gl_vertex_array_binding_divisor() -> &'static str {
        "glVertexArrayBindingDivisor"
    }

    /// glVertexArrayElementBuffer: Sets element buffer for vertex array.
    pub fn gl_vertex_array_element_buffer() -> &'static str {
        "glVertexArrayElementBuffer"
    }

    /// glVertexArrayVertexBuffer: Binds buffer to vertex array binding.
    pub fn gl_vertex_array_vertex_buffer() -> &'static str {
        "glVertexArrayVertexBuffer"
    }

    /// glVertexArrayVertexBuffers: Binds multiple buffers to vertex array bindings.
    pub fn gl_vertex_array_vertex_buffers() -> &'static str {
        "glVertexArrayVertexBuffers"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core45_dsa_create_functions() {
        // Verify direct state access create functions
        assert_eq!(OpenGlGlCore45::gl_create_buffers(), "glCreateBuffers");
        assert_eq!(OpenGlGlCore45::gl_create_textures(), "glCreateTextures");
        assert_eq!(OpenGlGlCore45::gl_create_framebuffers(), "glCreateFramebuffers");
    }

    #[test]
    fn test_gl_core45_named_buffer_functions() {
        // Verify named buffer functions
        let funcs = vec![
            OpenGlGlCore45::gl_named_buffer_data(),
            OpenGlGlCore45::gl_named_buffer_sub_data(),
            OpenGlGlCore45::gl_get_named_buffer_sub_data(),
        ];

        for func in funcs {
            assert!(func.contains("NamedBuffer"));
        }
    }

    #[test]
    fn test_gl_core45_dsa_basic() {
        // Verify basic DSA availability
        assert!(!OpenGlGlCore45::gl_create_buffers().is_empty());
        assert!(!OpenGlGlCore45::gl_create_textures().is_empty());
    }
}
