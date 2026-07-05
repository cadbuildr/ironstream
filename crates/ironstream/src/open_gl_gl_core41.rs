// FILE: open_gl_gl_core41.rs
// occt: OpenGl_GlCore41

/// OpenGL 4.1 core.
/// Introduces program pipelines, 64-bit vertex attributes, and viewport arrays.
pub struct OpenGlGlCore41;

impl OpenGlGlCore41 {
    // GL_ARB_ES2_compatibility functions (added to OpenGL 4.1 core)

    /// glClearDepthf: Clears the depth buffer (float version for ES compatibility).
    pub fn gl_clear_depthf() -> &'static str {
        "glClearDepthf"
    }

    /// glDepthRangef: Sets near and far clipping plane depth (float version).
    pub fn gl_depth_rangef() -> &'static str {
        "glDepthRangef"
    }

    /// glGetShaderPrecisionFormat: Retrieves shader precision information.
    pub fn gl_get_shader_precision_format() -> &'static str {
        "glGetShaderPrecisionFormat"
    }

    /// glReleaseShaderCompiler: Releases shader compiler resources.
    pub fn gl_release_shader_compiler() -> &'static str {
        "glReleaseShaderCompiler"
    }

    /// glShaderBinary: Loads binary shader code.
    pub fn gl_shader_binary() -> &'static str {
        "glShaderBinary"
    }

    // GL_ARB_get_program_binary functions (added to OpenGL 4.1 core)

    /// glGetProgramBinary: Retrieves the binary representation of a compiled program.
    pub fn gl_get_program_binary() -> &'static str {
        "glGetProgramBinary"
    }

    /// glProgramBinary: Loads binary program code.
    pub fn gl_program_binary() -> &'static str {
        "glProgramBinary"
    }

    /// glProgramParameteri: Sets program parameters.
    pub fn gl_program_parameteri() -> &'static str {
        "glProgramParameteri"
    }

    // GL_ARB_separate_shader_objects functions (added to OpenGL 4.1 core)

    /// glActiveShaderProgram: Activates a program object in a pipeline.
    pub fn gl_active_shader_program() -> &'static str {
        "glActiveShaderProgram"
    }

    /// glBindProgramPipeline: Binds a program pipeline object.
    pub fn gl_bind_program_pipeline() -> &'static str {
        "glBindProgramPipeline"
    }

    /// glCreateShaderProgramv: Creates a program object from shader source code.
    pub fn gl_create_shader_programv() -> &'static str {
        "glCreateShaderProgramv"
    }

    /// glDeleteProgramPipelines: Deletes program pipeline objects.
    pub fn gl_delete_program_pipelines() -> &'static str {
        "glDeleteProgramPipelines"
    }

    /// glGenProgramPipelines: Generates program pipeline object names.
    pub fn gl_gen_program_pipelines() -> &'static str {
        "glGenProgramPipelines"
    }

    /// glGetProgramPipelineInfoLog: Retrieves program pipeline link log.
    pub fn gl_get_program_pipeline_info_log() -> &'static str {
        "glGetProgramPipelineInfoLog"
    }

    /// glGetProgramPipelineiv: Retrieves program pipeline parameters.
    pub fn gl_get_program_pipelineiv() -> &'static str {
        "glGetProgramPipelineiv"
    }

    /// glIsProgramPipeline: Tests if a name is a program pipeline object.
    pub fn gl_is_program_pipeline() -> &'static str {
        "glIsProgramPipeline"
    }

    /// glProgramUniform1d: Sets a double precision uniform in a program (not via current binding).
    pub fn gl_program_uniform_1d() -> &'static str {
        "glProgramUniform1d"
    }

    /// glProgramUniform1dv: Sets double precision uniforms in a program (vector).
    pub fn gl_program_uniform_1dv() -> &'static str {
        "glProgramUniform1dv"
    }

    /// glProgramUniform1f: Sets a float uniform in a program.
    pub fn gl_program_uniform_1f() -> &'static str {
        "glProgramUniform1f"
    }

    /// glProgramUniform1fv: Sets float uniforms in a program (vector).
    pub fn gl_program_uniform_1fv() -> &'static str {
        "glProgramUniform1fv"
    }

    /// glProgramUniform1i: Sets an integer uniform in a program.
    pub fn gl_program_uniform_1i() -> &'static str {
        "glProgramUniform1i"
    }

    /// glProgramUniform1iv: Sets integer uniforms in a program (vector).
    pub fn gl_program_uniform_1iv() -> &'static str {
        "glProgramUniform1iv"
    }

    /// glProgramUniform1ui: Sets an unsigned integer uniform in a program.
    pub fn gl_program_uniform_1ui() -> &'static str {
        "glProgramUniform1ui"
    }

    /// glProgramUniform1uiv: Sets unsigned integer uniforms in a program (vector).
    pub fn gl_program_uniform_1uiv() -> &'static str {
        "glProgramUniform1uiv"
    }

    /// glProgramUniform2d: Sets a 2D double precision uniform in a program.
    pub fn gl_program_uniform_2d() -> &'static str {
        "glProgramUniform2d"
    }

    /// glProgramUniform2dv: Sets 2D double precision uniforms in a program.
    pub fn gl_program_uniform_2dv() -> &'static str {
        "glProgramUniform2dv"
    }

    /// glProgramUniform2f: Sets a 2D float uniform in a program.
    pub fn gl_program_uniform_2f() -> &'static str {
        "glProgramUniform2f"
    }

    /// glProgramUniform2fv: Sets 2D float uniforms in a program.
    pub fn gl_program_uniform_2fv() -> &'static str {
        "glProgramUniform2fv"
    }

    /// glProgramUniform2i: Sets a 2D integer uniform in a program.
    pub fn gl_program_uniform_2i() -> &'static str {
        "glProgramUniform2i"
    }

    /// glProgramUniform2iv: Sets 2D integer uniforms in a program.
    pub fn gl_program_uniform_2iv() -> &'static str {
        "glProgramUniform2iv"
    }

    /// glProgramUniform2ui: Sets a 2D unsigned integer uniform in a program.
    pub fn gl_program_uniform_2ui() -> &'static str {
        "glProgramUniform2ui"
    }

    /// glProgramUniform2uiv: Sets 2D unsigned integer uniforms in a program.
    pub fn gl_program_uniform_2uiv() -> &'static str {
        "glProgramUniform2uiv"
    }

    /// glProgramUniform3d: Sets a 3D double precision uniform in a program.
    pub fn gl_program_uniform_3d() -> &'static str {
        "glProgramUniform3d"
    }

    /// glProgramUniform3dv: Sets 3D double precision uniforms in a program.
    pub fn gl_program_uniform_3dv() -> &'static str {
        "glProgramUniform3dv"
    }

    /// glProgramUniform3f: Sets a 3D float uniform in a program.
    pub fn gl_program_uniform_3f() -> &'static str {
        "glProgramUniform3f"
    }

    /// glProgramUniform3fv: Sets 3D float uniforms in a program.
    pub fn gl_program_uniform_3fv() -> &'static str {
        "glProgramUniform3fv"
    }

    /// glProgramUniform3i: Sets a 3D integer uniform in a program.
    pub fn gl_program_uniform_3i() -> &'static str {
        "glProgramUniform3i"
    }

    /// glProgramUniform3iv: Sets 3D integer uniforms in a program.
    pub fn gl_program_uniform_3iv() -> &'static str {
        "glProgramUniform3iv"
    }

    /// glProgramUniform3ui: Sets a 3D unsigned integer uniform in a program.
    pub fn gl_program_uniform_3ui() -> &'static str {
        "glProgramUniform3ui"
    }

    /// glProgramUniform3uiv: Sets 3D unsigned integer uniforms in a program.
    pub fn gl_program_uniform_3uiv() -> &'static str {
        "glProgramUniform3uiv"
    }

    /// glProgramUniform4d: Sets a 4D double precision uniform in a program.
    pub fn gl_program_uniform_4d() -> &'static str {
        "glProgramUniform4d"
    }

    /// glProgramUniform4dv: Sets 4D double precision uniforms in a program.
    pub fn gl_program_uniform_4dv() -> &'static str {
        "glProgramUniform4dv"
    }

    /// glProgramUniform4f: Sets a 4D float uniform in a program.
    pub fn gl_program_uniform_4f() -> &'static str {
        "glProgramUniform4f"
    }

    /// glProgramUniform4fv: Sets 4D float uniforms in a program.
    pub fn gl_program_uniform_4fv() -> &'static str {
        "glProgramUniform4fv"
    }

    /// glProgramUniform4i: Sets a 4D integer uniform in a program.
    pub fn gl_program_uniform_4i() -> &'static str {
        "glProgramUniform4i"
    }

    /// glProgramUniform4iv: Sets 4D integer uniforms in a program.
    pub fn gl_program_uniform_4iv() -> &'static str {
        "glProgramUniform4iv"
    }

    /// glProgramUniform4ui: Sets a 4D unsigned integer uniform in a program.
    pub fn gl_program_uniform_4ui() -> &'static str {
        "glProgramUniform4ui"
    }

    /// glProgramUniform4uiv: Sets 4D unsigned integer uniforms in a program.
    pub fn gl_program_uniform_4uiv() -> &'static str {
        "glProgramUniform4uiv"
    }

    /// glProgramUniformMatrix2dv: Sets a 2x2 double precision matrix uniform in a program.
    pub fn gl_program_uniform_matrix_2dv() -> &'static str {
        "glProgramUniformMatrix2dv"
    }

    /// glProgramUniformMatrix2fv: Sets a 2x2 float matrix uniform in a program.
    pub fn gl_program_uniform_matrix_2fv() -> &'static str {
        "glProgramUniformMatrix2fv"
    }

    /// glProgramUniformMatrix2x3dv: Sets a 2x3 double precision matrix uniform in a program.
    pub fn gl_program_uniform_matrix_2x3dv() -> &'static str {
        "glProgramUniformMatrix2x3dv"
    }

    /// glProgramUniformMatrix2x3fv: Sets a 2x3 float matrix uniform in a program.
    pub fn gl_program_uniform_matrix_2x3fv() -> &'static str {
        "glProgramUniformMatrix2x3fv"
    }

    /// glProgramUniformMatrix2x4dv: Sets a 2x4 double precision matrix uniform in a program.
    pub fn gl_program_uniform_matrix_2x4dv() -> &'static str {
        "glProgramUniformMatrix2x4dv"
    }

    /// glProgramUniformMatrix2x4fv: Sets a 2x4 float matrix uniform in a program.
    pub fn gl_program_uniform_matrix_2x4fv() -> &'static str {
        "glProgramUniformMatrix2x4fv"
    }

    /// glProgramUniformMatrix3dv: Sets a 3x3 double precision matrix uniform in a program.
    pub fn gl_program_uniform_matrix_3dv() -> &'static str {
        "glProgramUniformMatrix3dv"
    }

    /// glProgramUniformMatrix3fv: Sets a 3x3 float matrix uniform in a program.
    pub fn gl_program_uniform_matrix_3fv() -> &'static str {
        "glProgramUniformMatrix3fv"
    }

    /// glProgramUniformMatrix3x2dv: Sets a 3x2 double precision matrix uniform in a program.
    pub fn gl_program_uniform_matrix_3x2dv() -> &'static str {
        "glProgramUniformMatrix3x2dv"
    }

    /// glProgramUniformMatrix3x2fv: Sets a 3x2 float matrix uniform in a program.
    pub fn gl_program_uniform_matrix_3x2fv() -> &'static str {
        "glProgramUniformMatrix3x2fv"
    }

    /// glProgramUniformMatrix3x4dv: Sets a 3x4 double precision matrix uniform in a program.
    pub fn gl_program_uniform_matrix_3x4dv() -> &'static str {
        "glProgramUniformMatrix3x4dv"
    }

    /// glProgramUniformMatrix3x4fv: Sets a 3x4 float matrix uniform in a program.
    pub fn gl_program_uniform_matrix_3x4fv() -> &'static str {
        "glProgramUniformMatrix3x4fv"
    }

    /// glProgramUniformMatrix4dv: Sets a 4x4 double precision matrix uniform in a program.
    pub fn gl_program_uniform_matrix_4dv() -> &'static str {
        "glProgramUniformMatrix4dv"
    }

    /// glProgramUniformMatrix4fv: Sets a 4x4 float matrix uniform in a program.
    pub fn gl_program_uniform_matrix_4fv() -> &'static str {
        "glProgramUniformMatrix4fv"
    }

    /// glProgramUniformMatrix4x2dv: Sets a 4x2 double precision matrix uniform in a program.
    pub fn gl_program_uniform_matrix_4x2dv() -> &'static str {
        "glProgramUniformMatrix4x2dv"
    }

    /// glProgramUniformMatrix4x2fv: Sets a 4x2 float matrix uniform in a program.
    pub fn gl_program_uniform_matrix_4x2fv() -> &'static str {
        "glProgramUniformMatrix4x2fv"
    }

    /// glProgramUniformMatrix4x3dv: Sets a 4x3 double precision matrix uniform in a program.
    pub fn gl_program_uniform_matrix_4x3dv() -> &'static str {
        "glProgramUniformMatrix4x3dv"
    }

    /// glProgramUniformMatrix4x3fv: Sets a 4x3 float matrix uniform in a program.
    pub fn gl_program_uniform_matrix_4x3fv() -> &'static str {
        "glProgramUniformMatrix4x3fv"
    }

    /// glUseProgramStages: Activates stages of a program pipeline.
    pub fn gl_use_program_stages() -> &'static str {
        "glUseProgramStages"
    }

    /// glValidateProgramPipeline: Validates program pipeline linkage and executability.
    pub fn gl_validate_program_pipeline() -> &'static str {
        "glValidateProgramPipeline"
    }

    // GL_ARB_vertex_attrib_64bit functions (added to OpenGL 4.1 core)

    /// glGetVertexAttribLdv: Returns 64-bit vertex attribute values.
    pub fn gl_get_vertex_attrib_ldv() -> &'static str {
        "glGetVertexAttribLdv"
    }

    /// glVertexAttribL1d: Sets a 1D 64-bit vertex attribute.
    pub fn gl_vertex_attrib_l1d() -> &'static str {
        "glVertexAttribL1d"
    }

    /// glVertexAttribL1dv: Sets 1D 64-bit vertex attributes.
    pub fn gl_vertex_attrib_l1dv() -> &'static str {
        "glVertexAttribL1dv"
    }

    /// glVertexAttribL2d: Sets a 2D 64-bit vertex attribute.
    pub fn gl_vertex_attrib_l2d() -> &'static str {
        "glVertexAttribL2d"
    }

    /// glVertexAttribL2dv: Sets 2D 64-bit vertex attributes.
    pub fn gl_vertex_attrib_l2dv() -> &'static str {
        "glVertexAttribL2dv"
    }

    /// glVertexAttribL3d: Sets a 3D 64-bit vertex attribute.
    pub fn gl_vertex_attrib_l3d() -> &'static str {
        "glVertexAttribL3d"
    }

    /// glVertexAttribL3dv: Sets 3D 64-bit vertex attributes.
    pub fn gl_vertex_attrib_l3dv() -> &'static str {
        "glVertexAttribL3dv"
    }

    /// glVertexAttribL4d: Sets a 4D 64-bit vertex attribute.
    pub fn gl_vertex_attrib_l4d() -> &'static str {
        "glVertexAttribL4d"
    }

    /// glVertexAttribL4dv: Sets 4D 64-bit vertex attributes.
    pub fn gl_vertex_attrib_l4dv() -> &'static str {
        "glVertexAttribL4dv"
    }

    /// glVertexAttribLPointer: Specifies the location and data format for 64-bit attributes.
    pub fn gl_vertex_attrib_l_pointer() -> &'static str {
        "glVertexAttribLPointer"
    }

    // GL_ARB_viewport_array functions (added to OpenGL 4.1 core)

    /// glDepthRangeArrayv: Sets depth range for multiple viewports.
    pub fn gl_depth_range_arrayv() -> &'static str {
        "glDepthRangeArrayv"
    }

    /// glDepthRangeIndexed: Sets depth range for an indexed viewport.
    pub fn gl_depth_range_indexed() -> &'static str {
        "glDepthRangeIndexed"
    }

    /// glGetDoublei_v: Retrieves indexed double state variables.
    pub fn gl_get_doublei_v() -> &'static str {
        "glGetDoublei_v"
    }

    /// glGetFloati_v: Retrieves indexed float state variables.
    pub fn gl_get_floati_v() -> &'static str {
        "glGetFloati_v"
    }

    /// glScissorArrayv: Sets scissor rectangles for multiple viewports.
    pub fn gl_scissor_arrayv() -> &'static str {
        "glScissorArrayv"
    }

    /// glScissorIndexed: Sets scissor rectangle for an indexed viewport.
    pub fn gl_scissor_indexed() -> &'static str {
        "glScissorIndexed"
    }

    /// glScissorIndexedv: Sets scissor rectangle for an indexed viewport (vector).
    pub fn gl_scissor_indexedv() -> &'static str {
        "glScissorIndexedv"
    }

    /// glViewportArrayv: Sets viewports for multiple drawing areas.
    pub fn gl_viewport_arrayv() -> &'static str {
        "glViewportArrayv"
    }

    /// glViewportIndexedf: Sets viewport for an indexed drawing area.
    pub fn gl_viewport_indexedf() -> &'static str {
        "glViewportIndexedf"
    }

    /// glViewportIndexedfv: Sets viewport for an indexed drawing area (vector).
    pub fn gl_viewport_indexedfv() -> &'static str {
        "glViewportIndexedfv"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core41_program_pipelines() {
        // Verify program pipeline functions
        assert_eq!(OpenGlGlCore41::gl_gen_program_pipelines(), "glGenProgramPipelines");
        assert_eq!(
            OpenGlGlCore41::gl_bind_program_pipeline(),
            "glBindProgramPipeline"
        );
        assert_eq!(
            OpenGlGlCore41::gl_delete_program_pipelines(),
            "glDeleteProgramPipelines"
        );
    }

    #[test]
    fn test_gl_core41_viewport_array_functions() {
        // Verify viewport array functions
        assert_eq!(OpenGlGlCore41::gl_viewport_arrayv(), "glViewportArrayv");
        assert_eq!(OpenGlGlCore41::gl_scissor_arrayv(), "glScissorArrayv");
    }

    #[test]
    fn test_gl_core41_64bit_vertex_attributes() {
        // Verify 64-bit vertex attribute functions
        let funcs = vec![
            OpenGlGlCore41::gl_vertex_attrib_l1d(),
            OpenGlGlCore41::gl_vertex_attrib_l2d(),
            OpenGlGlCore41::gl_vertex_attrib_l3d(),
            OpenGlGlCore41::gl_vertex_attrib_l4d(),
        ];

        for func in funcs {
            assert!(func.contains("VertexAttribL"));
        }
    }
}
