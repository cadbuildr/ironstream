// FILE: open_gl_gl_core12.rs
// occt: OpenGl_GlCore12

/// OpenGL 1.2 core based on 1.1 version.
/// This struct re-exports a subset of OpenGL 1.2 functions from the base class,
/// providing version-specific function visibility.
pub struct OpenGlGlCore12;

impl OpenGlGlCore12 {
    /// OpenGL 1.2 additive to 1.1: glBlendColor
    /// Specifies the constant color to be used in blending operations.
    pub fn gl_blend_color() -> &'static str {
        "glBlendColor"
    }

    /// OpenGL 1.2 additive to 1.1: glBlendEquation
    /// Specifies how source and destination colors are combined in blending.
    pub fn gl_blend_equation() -> &'static str {
        "glBlendEquation"
    }

    /// OpenGL 1.2 (not in ES 2.0): glCopyTexSubImage3D
    /// Copies a rectangular region of the frame buffer to a 3D texture.
    pub fn gl_copy_tex_sub_image_3d() -> &'static str {
        "glCopyTexSubImage3D"
    }

    /// OpenGL 1.2 (not in ES 2.0): glDrawRangeElements
    /// Renders primitives from an indexed range within vertex arrays.
    pub fn gl_draw_range_elements() -> &'static str {
        "glDrawRangeElements"
    }

    /// OpenGL 1.2 (not in ES 2.0): glTexImage3D
    /// Specifies a 3D texture image.
    pub fn gl_tex_image_3d() -> &'static str {
        "glTexImage3D"
    }

    /// OpenGL 1.2 (not in ES 2.0): glTexSubImage3D
    /// Updates a rectangular subregion of a 3D texture image.
    pub fn gl_tex_sub_image_3d() -> &'static str {
        "glTexSubImage3D"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core12_function_names() {
        // Verify that the function identifiers are correct
        assert_eq!(OpenGlGlCore12::gl_blend_color(), "glBlendColor");
        assert_eq!(OpenGlGlCore12::gl_blend_equation(), "glBlendEquation");
        assert_eq!(OpenGlGlCore12::gl_copy_tex_sub_image_3d(), "glCopyTexSubImage3D");
        assert_eq!(OpenGlGlCore12::gl_draw_range_elements(), "glDrawRangeElements");
        assert_eq!(OpenGlGlCore12::gl_tex_image_3d(), "glTexImage3D");
        assert_eq!(OpenGlGlCore12::gl_tex_sub_image_3d(), "glTexSubImage3D");
    }

    #[test]
    fn test_gl_core12_additive_functions() {
        // Test that 1.2-specific additives are properly identified
        let blend_color = OpenGlGlCore12::gl_blend_color();
        let blend_equation = OpenGlGlCore12::gl_blend_equation();

        assert!(!blend_color.is_empty());
        assert!(!blend_equation.is_empty());
        assert!(blend_color.starts_with("glBlend"));
        assert!(blend_equation.starts_with("glBlend"));
    }

    #[test]
    fn test_gl_core12_3d_texture_functions() {
        // Test 3D texture functions (not in ES 2.0)
        let copy_3d = OpenGlGlCore12::gl_copy_tex_sub_image_3d();
        let tex_3d = OpenGlGlCore12::gl_tex_image_3d();
        let tex_sub_3d = OpenGlGlCore12::gl_tex_sub_image_3d();

        assert!(copy_3d.contains("3D"));
        assert!(tex_3d.contains("3D"));
        assert!(tex_sub_3d.contains("3D"));
    }
}
