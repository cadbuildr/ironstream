// FILE: open_gl_gl_core40.rs
// occt: OpenGl_GlCore40

/// OpenGL 4.0 core.
/// Introduces tessellation shaders, double precision uniforms, and indirect drawing.
pub struct OpenGlGlCore40;

impl OpenGlGlCore40 {
    // GL_ARB_draw_indirect functions (added to OpenGL 4.0 core)

    /// glDrawArraysIndirect: Renders primitives from array data with indirect parameters.
    pub fn gl_draw_arrays_indirect() -> &'static str {
        "glDrawArraysIndirect"
    }

    /// glDrawElementsIndirect: Renders indexed primitives with indirect parameters.
    pub fn gl_draw_elements_indirect() -> &'static str {
        "glDrawElementsIndirect"
    }

    // GL_ARB_gpu_shader_fp64 functions (added to OpenGL 4.0 core)

    /// glGetUniformdv: Returns double precision uniform variable values.
    pub fn gl_get_uniformdv() -> &'static str {
        "glGetUniformdv"
    }

    /// glUniform1d: Sets a double precision uniform variable.
    pub fn gl_uniform_1d() -> &'static str {
        "glUniform1d"
    }

    /// glUniform1dv: Sets double precision uniform variables as a vector.
    pub fn gl_uniform_1dv() -> &'static str {
        "glUniform1dv"
    }

    /// glUniform2d: Sets a 2D double precision uniform variable.
    pub fn gl_uniform_2d() -> &'static str {
        "glUniform2d"
    }

    /// glUniform2dv: Sets 2D double precision uniform variables.
    pub fn gl_uniform_2dv() -> &'static str {
        "glUniform2dv"
    }

    /// glUniform3d: Sets a 3D double precision uniform variable.
    pub fn gl_uniform_3d() -> &'static str {
        "glUniform3d"
    }

    /// glUniform3dv: Sets 3D double precision uniform variables.
    pub fn gl_uniform_3dv() -> &'static str {
        "glUniform3dv"
    }

    /// glUniform4d: Sets a 4D double precision uniform variable.
    pub fn gl_uniform_4d() -> &'static str {
        "glUniform4d"
    }

    /// glUniform4dv: Sets 4D double precision uniform variables.
    pub fn gl_uniform_4dv() -> &'static str {
        "glUniform4dv"
    }

    /// glUniformMatrix2dv: Sets a 2x2 double precision matrix uniform.
    pub fn gl_uniform_matrix_2dv() -> &'static str {
        "glUniformMatrix2dv"
    }

    /// glUniformMatrix2x3dv: Sets a 2x3 double precision matrix uniform.
    pub fn gl_uniform_matrix_2x3dv() -> &'static str {
        "glUniformMatrix2x3dv"
    }

    /// glUniformMatrix2x4dv: Sets a 2x4 double precision matrix uniform.
    pub fn gl_uniform_matrix_2x4dv() -> &'static str {
        "glUniformMatrix2x4dv"
    }

    /// glUniformMatrix3dv: Sets a 3x3 double precision matrix uniform.
    pub fn gl_uniform_matrix_3dv() -> &'static str {
        "glUniformMatrix3dv"
    }

    /// glUniformMatrix3x2dv: Sets a 3x2 double precision matrix uniform.
    pub fn gl_uniform_matrix_3x2dv() -> &'static str {
        "glUniformMatrix3x2dv"
    }

    /// glUniformMatrix3x4dv: Sets a 3x4 double precision matrix uniform.
    pub fn gl_uniform_matrix_3x4dv() -> &'static str {
        "glUniformMatrix3x4dv"
    }

    /// glUniformMatrix4dv: Sets a 4x4 double precision matrix uniform.
    pub fn gl_uniform_matrix_4dv() -> &'static str {
        "glUniformMatrix4dv"
    }

    /// glUniformMatrix4x2dv: Sets a 4x2 double precision matrix uniform.
    pub fn gl_uniform_matrix_4x2dv() -> &'static str {
        "glUniformMatrix4x2dv"
    }

    /// glUniformMatrix4x3dv: Sets a 4x3 double precision matrix uniform.
    pub fn gl_uniform_matrix_4x3dv() -> &'static str {
        "glUniformMatrix4x3dv"
    }

    // GL_ARB_shader_subroutine functions (added to OpenGL 4.0 core)

    /// glGetActiveSubroutineName: Retrieves the name of an active subroutine.
    pub fn gl_get_active_subroutine_name() -> &'static str {
        "glGetActiveSubroutineName"
    }

    /// glGetActiveSubroutineUniformiv: Retrieves active subroutine uniform properties.
    pub fn gl_get_active_subroutine_uniformiv() -> &'static str {
        "glGetActiveSubroutineUniformiv"
    }

    /// glGetActiveSubroutineUniformName: Retrieves the name of an active subroutine uniform.
    pub fn gl_get_active_subroutine_uniform_name() -> &'static str {
        "glGetActiveSubroutineUniformName"
    }

    /// glGetProgramStageiv: Retrieves program stage properties.
    pub fn gl_get_program_stageiv() -> &'static str {
        "glGetProgramStageiv"
    }

    /// glGetSubroutineIndex: Returns the index of a subroutine.
    pub fn gl_get_subroutine_index() -> &'static str {
        "glGetSubroutineIndex"
    }

    /// glGetSubroutineUniformLocation: Returns the location of a subroutine uniform.
    pub fn gl_get_subroutine_uniform_location() -> &'static str {
        "glGetSubroutineUniformLocation"
    }

    /// glGetUniformSubroutineuiv: Returns subroutine uniform values.
    pub fn gl_get_uniform_subroutineuiv() -> &'static str {
        "glGetUniformSubroutineuiv"
    }

    /// glUniformSubroutinesuiv: Sets subroutine uniform values.
    pub fn gl_uniform_subroutinesuiv() -> &'static str {
        "glUniformSubroutinesuiv"
    }

    // GL_ARB_tessellation_shader functions (added to OpenGL 4.0 core)

    /// glPatchParameterfv: Sets patch parameters (float).
    pub fn gl_patch_parameterfv() -> &'static str {
        "glPatchParameterfv"
    }

    /// glPatchParameteri: Sets patch parameters (integer).
    pub fn gl_patch_parameteri() -> &'static str {
        "glPatchParameteri"
    }

    // GL_ARB_transform_feedback2 functions (added to OpenGL 4.0 core)

    /// glBindTransformFeedback: Binds a transform feedback object.
    pub fn gl_bind_transform_feedback() -> &'static str {
        "glBindTransformFeedback"
    }

    /// glDeleteTransformFeedbacks: Deletes transform feedback objects.
    pub fn gl_delete_transform_feedbacks() -> &'static str {
        "glDeleteTransformFeedbacks"
    }

    /// glDrawTransformFeedback: Renders primitives captured in a transform feedback buffer.
    pub fn gl_draw_transform_feedback() -> &'static str {
        "glDrawTransformFeedback"
    }

    /// glGenTransformFeedbacks: Generates transform feedback object names.
    pub fn gl_gen_transform_feedbacks() -> &'static str {
        "glGenTransformFeedbacks"
    }

    /// glIsTransformFeedback: Tests if a name is a transform feedback object.
    pub fn gl_is_transform_feedback() -> &'static str {
        "glIsTransformFeedback"
    }

    /// glPauseTransformFeedback: Pauses transform feedback capture.
    pub fn gl_pause_transform_feedback() -> &'static str {
        "glPauseTransformFeedback"
    }

    /// glResumeTransformFeedback: Resumes paused transform feedback capture.
    pub fn gl_resume_transform_feedback() -> &'static str {
        "glResumeTransformFeedback"
    }

    // GL_ARB_transform_feedback3 functions (added to OpenGL 4.0 core)

    /// glBeginQueryIndexed: Begins a query with stream index.
    pub fn gl_begin_query_indexed() -> &'static str {
        "glBeginQueryIndexed"
    }

    /// glDrawTransformFeedbackStream: Renders primitives from a specific stream.
    pub fn gl_draw_transform_feedback_stream() -> &'static str {
        "glDrawTransformFeedbackStream"
    }

    /// glEndQueryIndexed: Ends a query with stream index.
    pub fn gl_end_query_indexed() -> &'static str {
        "glEndQueryIndexed"
    }

    /// glGetQueryIndexediv: Retrieves query state indexed by stream.
    pub fn gl_get_query_indexediv() -> &'static str {
        "glGetQueryIndexediv"
    }

    // OpenGL 4.0 core additives to 3.3

    /// glBlendEquationi: Sets per-buffer blend equation.
    pub fn gl_blend_equationi() -> &'static str {
        "glBlendEquationi"
    }

    /// glBlendEquationSeparatei: Sets separate per-buffer blend equations.
    pub fn gl_blend_equation_separatei() -> &'static str {
        "glBlendEquationSeparatei"
    }

    /// glBlendFunci: Sets per-buffer blend function.
    pub fn gl_blend_funci() -> &'static str {
        "glBlendFunci"
    }

    /// glBlendFuncSeparatei: Sets separate per-buffer blend functions.
    pub fn gl_blend_func_separatei() -> &'static str {
        "glBlendFuncSeparatei"
    }

    /// glMinSampleShading: Specifies minimum sample shading coverage.
    pub fn gl_min_sample_shading() -> &'static str {
        "glMinSampleShading"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core40_double_precision_uniforms() {
        // Verify double precision uniform functions
        assert_eq!(OpenGlGlCore40::gl_uniform_1d(), "glUniform1d");
        assert_eq!(OpenGlGlCore40::gl_uniform_2d(), "glUniform2d");
        assert_eq!(OpenGlGlCore40::gl_uniform_3d(), "glUniform3d");
        assert_eq!(OpenGlGlCore40::gl_uniform_4d(), "glUniform4d");
    }

    #[test]
    fn test_gl_core40_indirect_drawing() {
        // Verify indirect drawing functions
        assert_eq!(OpenGlGlCore40::gl_draw_arrays_indirect(), "glDrawArraysIndirect");
        assert_eq!(OpenGlGlCore40::gl_draw_elements_indirect(), "glDrawElementsIndirect");
    }

    #[test]
    fn test_gl_core40_transform_feedback_objects() {
        // Verify transform feedback object functions
        assert_eq!(OpenGlGlCore40::gl_gen_transform_feedbacks(), "glGenTransformFeedbacks");
        assert_eq!(OpenGlGlCore40::gl_bind_transform_feedback(), "glBindTransformFeedback");
        assert_eq!(OpenGlGlCore40::gl_delete_transform_feedbacks(), "glDeleteTransformFeedbacks");
    }

    #[test]
    fn test_gl_core40_tessellation_functions() {
        // Verify tessellation shader functions
        assert_eq!(OpenGlGlCore40::gl_patch_parameteri(), "glPatchParameteri");
        assert_eq!(OpenGlGlCore40::gl_patch_parameterfv(), "glPatchParameterfv");
    }
}
