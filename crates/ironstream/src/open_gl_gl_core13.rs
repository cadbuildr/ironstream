// FILE: open_gl_gl_core13.rs
// occt: OpenGl_GlCore13

/// OpenGL 1.3 core without deprecated entry points.
/// Extends OpenGL 1.2 with compression and texture handling improvements.
pub struct OpenGlGlCore13;

impl OpenGlGlCore13 {
    /// OpenGL 1.3 (not in ES 2.0): glCompressedTexImage1D
    /// Loads a compressed 1D texture image.
    pub fn gl_compressed_tex_image_1d() -> &'static str {
        "glCompressedTexImage1D"
    }

    /// OpenGL 1.3 (not in ES 2.0): glCompressedTexImage3D
    /// Loads a compressed 3D texture image.
    pub fn gl_compressed_tex_image_3d() -> &'static str {
        "glCompressedTexImage3D"
    }

    /// OpenGL 1.3 (not in ES 2.0): glCompressedTexSubImage1D
    /// Updates a compressed 1D texture subregion.
    pub fn gl_compressed_tex_sub_image_1d() -> &'static str {
        "glCompressedTexSubImage1D"
    }

    /// OpenGL 1.3 (not in ES 2.0): glCompressedTexSubImage3D
    /// Updates a compressed 3D texture subregion.
    pub fn gl_compressed_tex_sub_image_3d() -> &'static str {
        "glCompressedTexSubImage3D"
    }

    /// OpenGL 1.3 (not in ES 2.0): glGetCompressedTexImage
    /// Retrieves a compressed texture image from GPU memory.
    pub fn gl_get_compressed_tex_image() -> &'static str {
        "glGetCompressedTexImage"
    }

    /// OpenGL 1.3: glActiveTexture
    /// Selects the active texture unit for subsequent texture operations.
    pub fn gl_active_texture() -> &'static str {
        "glActiveTexture"
    }

    /// OpenGL 1.3: glCompressedTexImage2D
    /// Loads a compressed 2D texture image.
    pub fn gl_compressed_tex_image_2d() -> &'static str {
        "glCompressedTexImage2D"
    }

    /// OpenGL 1.3: glCompressedTexSubImage2D
    /// Updates a compressed 2D texture subregion.
    pub fn gl_compressed_tex_sub_image_2d() -> &'static str {
        "glCompressedTexSubImage2D"
    }

    /// OpenGL 1.3: glSampleCoverage
    /// Controls coverage-based anti-aliasing for multisampling.
    pub fn gl_sample_coverage() -> &'static str {
        "glSampleCoverage"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_core13_function_names() {
        // Verify OpenGL 1.3 core function names
        assert_eq!(OpenGlGlCore13::gl_active_texture(), "glActiveTexture");
        assert_eq!(OpenGlGlCore13::gl_compressed_tex_image_2d(), "glCompressedTexImage2D");
        assert_eq!(OpenGlGlCore13::gl_compressed_tex_sub_image_2d(), "glCompressedTexSubImage2D");
        assert_eq!(OpenGlGlCore13::gl_sample_coverage(), "glSampleCoverage");
    }

    #[test]
    fn test_gl_core13_non_es20_functions() {
        // Verify functions not in ES 2.0
        let funcs = vec![
            OpenGlGlCore13::gl_compressed_tex_image_1d(),
            OpenGlGlCore13::gl_compressed_tex_image_3d(),
            OpenGlGlCore13::gl_compressed_tex_sub_image_1d(),
            OpenGlGlCore13::gl_compressed_tex_sub_image_3d(),
            OpenGlGlCore13::gl_get_compressed_tex_image(),
        ];

        for func in funcs {
            assert!(!func.is_empty());
        }
    }

    #[test]
    fn test_gl_core13_compression_functions() {
        // Verify compression-related functions
        let funcs = vec![
            OpenGlGlCore13::gl_compressed_tex_image_1d(),
            OpenGlGlCore13::gl_compressed_tex_image_2d(),
            OpenGlGlCore13::gl_compressed_tex_image_3d(),
            OpenGlGlCore13::gl_compressed_tex_sub_image_1d(),
            OpenGlGlCore13::gl_compressed_tex_sub_image_2d(),
            OpenGlGlCore13::gl_compressed_tex_sub_image_3d(),
            OpenGlGlCore13::gl_get_compressed_tex_image(),
        ];

        for func in funcs {
            assert!(func.contains("Compressed"), "Function {} should contain 'Compressed'", func);
        }
    }
}
