// FILE: open_gl_gl_core21.rs
// occt: OpenGl_GlCore21

/// OpenGL 2.1 core based on 2.0 version.
/// Extends OpenGL 2.0 with support for non-square matrix uniforms.
pub struct OpenGlGlCore21;

impl OpenGlGlCore21 {
    /// OpenGL 2.1 (not in ES 2.0): glUniformMatrix2x3fv
    /// Sets a 2x3 float matrix uniform variable.
    pub fn gl_uniform_matrix_2x3fv() -> &'static str {
        "glUniformMatrix2x3fv"
    }

    /// OpenGL 2.1 (not in ES 2.0): glUniformMatrix2x4fv
    /// Sets a 2x4 float matrix uniform variable.
    pub fn gl_uniform_matrix_2x4fv() -> &'static str {
        "glUniformMatrix2x4fv"
    }

    /// OpenGL 2.1 (not in ES 2.0): glUniformMatrix3x2fv
    /// Sets a 3x2 float matrix uniform variable.
    pub fn gl_uniform_matrix_3x2fv() -> &'static str {
        "glUniformMatrix3x2fv"
    }

    /// OpenGL 2.1 (not in ES 2.0): glUniformMatrix3x4fv
    /// Sets a 3x4 float matrix uniform variable.
    pub fn gl_uniform_matrix_3x4fv() -> &'static str {
        "glUniformMatrix3x4fv"
    }

    /// OpenGL 2.1 (not in ES 2.0): glUniformMatrix4x2fv
    /// Sets a 4x2 float matrix uniform variable.
    pub fn gl_uniform_matrix_4x2fv() -> &'static str {
        "glUniformMatrix4x2fv"
    }

    /// OpenGL 2.1 (not in ES 2.0): glUniformMatrix4x3fv
    /// Sets a 4x3 float matrix uniform variable.
    pub fn gl_uniform_matrix_4x3fv() -> &'static str {
        "glUniformMatrix4x3fv"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core21_non_square_matrix_functions() {
        // Verify non-square matrix uniform functions (not in ES 2.0)
        let funcs = vec![
            OpenGlGlCore21::gl_uniform_matrix_2x3fv(),
            OpenGlGlCore21::gl_uniform_matrix_2x4fv(),
            OpenGlGlCore21::gl_uniform_matrix_3x2fv(),
            OpenGlGlCore21::gl_uniform_matrix_3x4fv(),
            OpenGlGlCore21::gl_uniform_matrix_4x2fv(),
            OpenGlGlCore21::gl_uniform_matrix_4x3fv(),
        ];

        for func in funcs {
            assert!(func.contains("UniformMatrix"));
            assert!(!func.is_empty());
        }
    }

    #[test]
    fn test_gl_core21_function_names() {
        // Verify specific function names
        assert_eq!(OpenGlGlCore21::gl_uniform_matrix_2x3fv(), "glUniformMatrix2x3fv");
        assert_eq!(OpenGlGlCore21::gl_uniform_matrix_3x4fv(), "glUniformMatrix3x4fv");
        assert_eq!(OpenGlGlCore21::gl_uniform_matrix_4x3fv(), "glUniformMatrix4x3fv");
    }

    #[test]
    fn test_gl_core21_rectangular_matrices() {
        // Test that rectangular matrices have correct format
        let matrix_2x3 = OpenGlGlCore21::gl_uniform_matrix_2x3fv();
        let matrix_3x2 = OpenGlGlCore21::gl_uniform_matrix_3x2fv();
        let matrix_4x3 = OpenGlGlCore21::gl_uniform_matrix_4x3fv();

        assert!(matrix_2x3.contains("2x3"));
        assert!(matrix_3x2.contains("3x2"));
        assert!(matrix_4x3.contains("4x3"));
    }
}
