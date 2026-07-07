// FILE: open_gl_gl_core46.rs
// occt: OpenGl_GlCore46

/// OpenGL 4.6 core.
/// Final core version introducing indirect count and shader specialization.
pub struct OpenGlGlCore46;

impl OpenGlGlCore46 {
    // OpenGL 4.6 core additives to 4.5

    /// glMultiDrawArraysIndirectCount: Renders multiple primitive sets with indirect count.
    pub fn gl_multi_draw_arrays_indirect_count() -> &'static str {
        "glMultiDrawArraysIndirectCount"
    }

    /// glMultiDrawElementsIndirectCount: Renders multiple indexed primitives with indirect count.
    pub fn gl_multi_draw_elements_indirect_count() -> &'static str {
        "glMultiDrawElementsIndirectCount"
    }

    /// glPolygonOffsetClamp: Sets polygon offset with clamping.
    pub fn gl_polygon_offset_clamp() -> &'static str {
        "glPolygonOffsetClamp"
    }

    /// glSpecializeShader: Specializes a shader.
    pub fn gl_specialize_shader() -> &'static str {
        "glSpecializeShader"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core46_indirect_count_functions() {
        // Verify indirect count drawing functions
        assert_eq!(
            OpenGlGlCore46::gl_multi_draw_arrays_indirect_count(),
            "glMultiDrawArraysIndirectCount"
        );
        assert_eq!(
            OpenGlGlCore46::gl_multi_draw_elements_indirect_count(),
            "glMultiDrawElementsIndirectCount"
        );
    }

    #[test]
    fn test_gl_core46_polygon_offset_clamp() {
        // Verify polygon offset clamp
        assert_eq!(
            OpenGlGlCore46::gl_polygon_offset_clamp(),
            "glPolygonOffsetClamp"
        );
    }

    #[test]
    fn test_gl_core46_shader_specialization() {
        // Verify shader specialization
        assert_eq!(OpenGlGlCore46::gl_specialize_shader(), "glSpecializeShader");
    }
}
