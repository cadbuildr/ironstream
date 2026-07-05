// FILE: open_gl_gl_core43.rs
// occt: OpenGl_GlCore43

/// OpenGL 4.3 core.
/// Introduces compute shaders, debugging support, and enhanced vertex attributes.
pub struct OpenGlGlCore43;

impl OpenGlGlCore43 {
    // OpenGL 4.3 core additives to 4.2

    /// glBindVertexBuffer: Binds a buffer to a generic vertex attribute binding.
    pub fn gl_bind_vertex_buffer() -> &'static str {
        "glBindVertexBuffer"
    }

    /// glClearBufferData: Clears all of a buffer object's data store.
    pub fn gl_clear_buffer_data() -> &'static str {
        "glClearBufferData"
    }

    /// glClearBufferSubData: Clears a subset of a buffer object's data store.
    pub fn gl_clear_buffer_sub_data() -> &'static str {
        "glClearBufferSubData"
    }

    /// glCopyImageSubData: Copies pixels between images.
    pub fn gl_copy_image_sub_data() -> &'static str {
        "glCopyImageSubData"
    }

    /// glDebugMessageCallback: Sets the callback for debug messages.
    pub fn gl_debug_message_callback() -> &'static str {
        "glDebugMessageCallback"
    }

    /// glDebugMessageControl: Specifies which debug messages are produced.
    pub fn gl_debug_message_control() -> &'static str {
        "glDebugMessageControl"
    }

    /// glDebugMessageInsert: Injects a debug message.
    pub fn gl_debug_message_insert() -> &'static str {
        "glDebugMessageInsert"
    }

    /// glDispatchCompute: Launches compute work groups.
    pub fn gl_dispatch_compute() -> &'static str {
        "glDispatchCompute"
    }

    /// glDispatchComputeIndirect: Launches compute work groups with indirect parameters.
    pub fn gl_dispatch_compute_indirect() -> &'static str {
        "glDispatchComputeIndirect"
    }

    /// glFramebufferParameteri: Sets framebuffer parameters.
    pub fn gl_framebuffer_parameteri() -> &'static str {
        "glFramebufferParameteri"
    }

    /// glGetDebugMessageLog: Retrieves debug messages.
    pub fn gl_get_debug_message_log() -> &'static str {
        "glGetDebugMessageLog"
    }

    /// glGetFramebufferParameteriv: Gets framebuffer parameters.
    pub fn gl_get_framebuffer_parameteriv() -> &'static str {
        "glGetFramebufferParameteriv"
    }

    /// glGetInternalformati64v: Queries internal format properties (64-bit).
    pub fn gl_get_internalformati_64v() -> &'static str {
        "glGetInternalformati64v"
    }

    /// glGetObjectLabel: Retrieves the label of an object.
    pub fn gl_get_object_label() -> &'static str {
        "glGetObjectLabel"
    }

    /// glGetObjectPtrLabel: Retrieves the label of an object by pointer.
    pub fn gl_get_object_ptr_label() -> &'static str {
        "glGetObjectPtrLabel"
    }

    /// glGetProgramInterfaceiv: Gets properties of a program interface.
    pub fn gl_get_program_interfaceiv() -> &'static str {
        "glGetProgramInterfaceiv"
    }

    /// glGetProgramResourceIndex: Gets the index of a named resource.
    pub fn gl_get_program_resource_index() -> &'static str {
        "glGetProgramResourceIndex"
    }

    /// glGetProgramResourceiv: Gets properties of a named resource.
    pub fn gl_get_program_resourceiv() -> &'static str {
        "glGetProgramResourceiv"
    }

    /// glGetProgramResourceLocation: Gets the location of a named resource.
    pub fn gl_get_program_resource_location() -> &'static str {
        "glGetProgramResourceLocation"
    }

    /// glGetProgramResourceLocationIndex: Gets the location index of a named resource.
    pub fn gl_get_program_resource_location_index() -> &'static str {
        "glGetProgramResourceLocationIndex"
    }

    /// glGetProgramResourceName: Gets the name of a named resource.
    pub fn gl_get_program_resource_name() -> &'static str {
        "glGetProgramResourceName"
    }

    /// glInvalidateBufferData: Invalidates all buffer data.
    pub fn gl_invalidate_buffer_data() -> &'static str {
        "glInvalidateBufferData"
    }

    /// glInvalidateBufferSubData: Invalidates a portion of buffer data.
    pub fn gl_invalidate_buffer_sub_data() -> &'static str {
        "glInvalidateBufferSubData"
    }

    /// glInvalidateFramebuffer: Invalidates framebuffer attachments.
    pub fn gl_invalidate_framebuffer() -> &'static str {
        "glInvalidateFramebuffer"
    }

    /// glInvalidateSubFramebuffer: Invalidates a subset of framebuffer attachments.
    pub fn gl_invalidate_sub_framebuffer() -> &'static str {
        "glInvalidateSubFramebuffer"
    }

    /// glInvalidateTexImage: Invalidates a texture image.
    pub fn gl_invalidate_tex_image() -> &'static str {
        "glInvalidateTexImage"
    }

    /// glInvalidateTexSubImage: Invalidates a subset of texture image data.
    pub fn gl_invalidate_tex_sub_image() -> &'static str {
        "glInvalidateTexSubImage"
    }

    /// glMultiDrawArraysIndirect: Renders multiple primitive sets with indirect parameters.
    pub fn gl_multi_draw_arrays_indirect() -> &'static str {
        "glMultiDrawArraysIndirect"
    }

    /// glMultiDrawElementsIndirect: Renders multiple indexed primitive sets with indirect parameters.
    pub fn gl_multi_draw_elements_indirect() -> &'static str {
        "glMultiDrawElementsIndirect"
    }

    /// glObjectLabel: Sets the label of an object.
    pub fn gl_object_label() -> &'static str {
        "glObjectLabel"
    }

    /// glObjectPtrLabel: Sets the label of an object by pointer.
    pub fn gl_object_ptr_label() -> &'static str {
        "glObjectPtrLabel"
    }

    /// glPopDebugGroup: Pops a debug group.
    pub fn gl_pop_debug_group() -> &'static str {
        "glPopDebugGroup"
    }

    /// glPushDebugGroup: Pushes a debug group.
    pub fn gl_push_debug_group() -> &'static str {
        "glPushDebugGroup"
    }

    /// glShaderStorageBlockBinding: Sets shader storage block binding.
    pub fn gl_shader_storage_block_binding() -> &'static str {
        "glShaderStorageBlockBinding"
    }

    /// glTexBufferRange: Attaches a range of a buffer to a buffer texture.
    pub fn gl_tex_buffer_range() -> &'static str {
        "glTexBufferRange"
    }

    /// glTexStorage2DMultisample: Creates an immutable multisampled 2D texture.
    pub fn gl_tex_storage_2d_multisample() -> &'static str {
        "glTexStorage2DMultisample"
    }

    /// glTexStorage3DMultisample: Creates an immutable multisampled 3D texture.
    pub fn gl_tex_storage_3d_multisample() -> &'static str {
        "glTexStorage3DMultisample"
    }

    /// glTextureView: Creates a view of a texture.
    pub fn gl_texture_view() -> &'static str {
        "glTextureView"
    }

    /// glVertexAttribBinding: Associates a vertex attribute with a vertex buffer binding.
    pub fn gl_vertex_attrib_binding() -> &'static str {
        "glVertexAttribBinding"
    }

    /// glVertexAttribFormat: Specifies the format of vertex attributes.
    pub fn gl_vertex_attrib_format() -> &'static str {
        "glVertexAttribFormat"
    }

    /// glVertexAttribIFormat: Specifies the integer format of vertex attributes.
    pub fn gl_vertex_attrib_i_format() -> &'static str {
        "glVertexAttribIFormat"
    }

    /// glVertexAttribLFormat: Specifies the 64-bit format of vertex attributes.
    pub fn gl_vertex_attrib_l_format() -> &'static str {
        "glVertexAttribLFormat"
    }

    /// glVertexBindingDivisor: Sets the instance rate of a vertex buffer binding.
    pub fn gl_vertex_binding_divisor() -> &'static str {
        "glVertexBindingDivisor"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core43_compute_functions() {
        // Verify compute shader functions
        assert_eq!(OpenGlGlCore43::gl_dispatch_compute(), "glDispatchCompute");
        assert_eq!(
            OpenGlGlCore43::gl_dispatch_compute_indirect(),
            "glDispatchComputeIndirect"
        );
    }

    #[test]
    fn test_gl_core43_debug_functions() {
        // Verify debug functions
        let funcs = vec![
            OpenGlGlCore43::gl_debug_message_callback(),
            OpenGlGlCore43::gl_debug_message_control(),
            OpenGlGlCore43::gl_debug_message_insert(),
        ];

        for func in funcs {
            assert!(func.contains("Debug") || func.contains("Message"));
        }
    }

    #[test]
    fn test_gl_core43_vertex_attribute_functions() {
        // Verify vertex attribute format functions
        assert_eq!(OpenGlGlCore43::gl_vertex_attrib_format(), "glVertexAttribFormat");
        assert_eq!(
            OpenGlGlCore43::gl_vertex_attrib_i_format(),
            "glVertexAttribIFormat"
        );
        assert_eq!(
            OpenGlGlCore43::gl_vertex_attrib_l_format(),
            "glVertexAttribLFormat"
        );
    }
}
