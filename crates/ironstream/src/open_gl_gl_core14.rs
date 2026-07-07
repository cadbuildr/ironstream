// FILE: open_gl_gl_core14.rs
// occt: OpenGl_GlCore14

/// OpenGL 1.4 core based on 1.3 version.
/// Extends OpenGL 1.3 with separate blending functions and multi-draw operations.
pub struct OpenGlGlCore14;

impl OpenGlGlCore14 {
    /// OpenGL 1.4: glBlendFuncSeparate
    /// Specifies separate blending functions for RGB and alpha channels.
    pub fn gl_blend_func_separate() -> &'static str {
        "glBlendFuncSeparate"
    }

    /// OpenGL 1.4: glMultiDrawElements
    /// Renders multiple sets of indexed primitives from vertex arrays.
    pub fn gl_multi_draw_elements() -> &'static str {
        "glMultiDrawElements"
    }

    /// OpenGL 1.4 (not in ES 2.0): glMultiDrawArrays
    /// Renders multiple sets of primitives from vertex arrays.
    pub fn gl_multi_draw_arrays() -> &'static str {
        "glMultiDrawArrays"
    }

    /// OpenGL 1.4 (not in ES 2.0): glPointParameterf
    /// Sets a point parameter to a float value.
    pub fn gl_point_parameter_f() -> &'static str {
        "glPointParameterf"
    }

    /// OpenGL 1.4 (not in ES 2.0): glPointParameterfv
    /// Sets point parameters from a float array.
    pub fn gl_point_parameter_fv() -> &'static str {
        "glPointParameterfv"
    }

    /// OpenGL 1.4 (not in ES 2.0): glPointParameteri
    /// Sets a point parameter to an integer value.
    pub fn gl_point_parameter_i() -> &'static str {
        "glPointParameteri"
    }

    /// OpenGL 1.4 (not in ES 2.0): glPointParameteriv
    /// Sets point parameters from an integer array.
    pub fn gl_point_parameter_iv() -> &'static str {
        "glPointParameteriv"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core14_function_names() {
        // Verify OpenGL 1.4 core function names
        assert_eq!(OpenGlGlCore14::gl_blend_func_separate(), "glBlendFuncSeparate");
        assert_eq!(OpenGlGlCore14::gl_multi_draw_elements(), "glMultiDrawElements");
    }

    #[test]
    fn test_gl_core14_non_es20_functions() {
        // Verify functions not in ES 2.0
        let funcs = vec![
            OpenGlGlCore14::gl_multi_draw_arrays(),
            OpenGlGlCore14::gl_point_parameter_f(),
            OpenGlGlCore14::gl_point_parameter_fv(),
            OpenGlGlCore14::gl_point_parameter_i(),
            OpenGlGlCore14::gl_point_parameter_iv(),
        ];

        for func in funcs {
            assert!(!func.is_empty());
        }
    }

    #[test]
    fn test_gl_core14_point_parameters() {
        // Verify point parameter function names
        let funcs = vec![
            OpenGlGlCore14::gl_point_parameter_f(),
            OpenGlGlCore14::gl_point_parameter_fv(),
            OpenGlGlCore14::gl_point_parameter_i(),
            OpenGlGlCore14::gl_point_parameter_iv(),
        ];

        for func in funcs {
            assert!(func.contains("PointParameter"), "Function {} should contain 'PointParameter'", func);
        }
    }

    #[test]
    fn test_gl_core14_multi_draw_functions() {
        // Verify multi-draw functions
        let arrays = OpenGlGlCore14::gl_multi_draw_arrays();
        let elements = OpenGlGlCore14::gl_multi_draw_elements();

        assert!(arrays.contains("MultiDraw"));
        assert!(elements.contains("MultiDraw"));
    }
}
