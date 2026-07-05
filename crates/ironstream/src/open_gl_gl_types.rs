// FILE: open_gl_gl_types.rs
// occt: OpenGl_GlTypes

/// OpenGL type definitions and constants.
/// Provides type aliases and buffer/primitive constants for OpenGL.

// Type aliases matching OpenGL specifications
pub type GLbyte = i8;
pub type GLubyte = u8;
pub type GLshort = i16;
pub type GLushort = u16;
pub type GLint = i32;
pub type GLuint = u32;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLfloat = f32;
pub type GLdouble = f64;
pub type GLclampf = f32;
pub type GLclampd = f64;
pub type GLboolean = u8;
pub type GLsizei = i32;
pub type GLsizeiptr = isize;
pub type GLintptr = isize;
pub type GLenum = u32;
pub type GLbitfield = u32;
pub type GLchar = u8;
pub type GLvoid = ();

/// Represents an OpenGL sync object (opaque pointer).
#[repr(transparent)]
pub struct GLsync(*mut std::ffi::c_void);

/// OpenGL constants for buffer bits.
pub mod buffer_bits {
    pub const GL_DEPTH_BUFFER_BIT: u32 = 0x00000100;
    pub const GL_STENCIL_BUFFER_BIT: u32 = 0x00000400;
    pub const GL_COLOR_BUFFER_BIT: u32 = 0x00004000;
}

/// OpenGL constants for boolean values.
pub mod boolean {
    pub const GL_FALSE: u8 = 0;
    pub const GL_TRUE: u8 = 1;
}

/// OpenGL constants for primitive types.
pub mod primitives {
    pub const GL_POINTS: u32 = 0x0000;
    pub const GL_LINES: u32 = 0x0001;
    pub const GL_LINE_LOOP: u32 = 0x0002;
    pub const GL_LINE_STRIP: u32 = 0x0003;
    pub const GL_TRIANGLES: u32 = 0x0004;
    pub const GL_TRIANGLE_STRIP: u32 = 0x0005;
    pub const GL_TRIANGLE_FAN: u32 = 0x0006;
}

/// OpenGL constants for blending factors.
pub mod blend_factors {
    pub const GL_ZERO: u32 = 0;
    pub const GL_ONE: u32 = 1;
    pub const GL_SRC_COLOR: u32 = 0x0300;
    pub const GL_ONE_MINUS_SRC_COLOR: u32 = 0x0301;
    pub const GL_SRC_ALPHA: u32 = 0x0302;
    pub const GL_ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
    pub const GL_DST_ALPHA: u32 = 0x0304;
    pub const GL_ONE_MINUS_DST_ALPHA: u32 = 0x0305;
    pub const GL_DST_COLOR: u32 = 0x0306;
    pub const GL_ONE_MINUS_DST_COLOR: u32 = 0x0307;
    pub const GL_SRC_ALPHA_SATURATE: u32 = 0x0308;
}

/// OpenGL constants for blend equations.
pub mod blend_equations {
    pub const GL_FUNC_ADD: u32 = 0x8006;
    pub const GL_MIN: u32 = 0x8007;
    pub const GL_MAX: u32 = 0x8008;
    pub const GL_FUNC_SUBTRACT: u32 = 0x800A;
    pub const GL_FUNC_REVERSE_SUBTRACT: u32 = 0x800B;
}

/// Sentinel value representing no GL constant.
pub const GL_NONE: u32 = 0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_types_sizes() {
        // Verify type sizes match OpenGL specifications
        assert_eq!(std::mem::size_of::<GLbyte>(), 1);
        assert_eq!(std::mem::size_of::<GLubyte>(), 1);
        assert_eq!(std::mem::size_of::<GLshort>(), 2);
        assert_eq!(std::mem::size_of::<GLushort>(), 2);
        assert_eq!(std::mem::size_of::<GLint>(), 4);
        assert_eq!(std::mem::size_of::<GLuint>(), 4);
        assert_eq!(std::mem::size_of::<GLfloat>(), 4);
        assert_eq!(std::mem::size_of::<GLdouble>(), 8);
        assert_eq!(std::mem::size_of::<GLint64>(), 8);
        assert_eq!(std::mem::size_of::<GLuint64>(), 8);
    }

    #[test]
    fn test_gl_types_buffer_bits() {
        // Verify buffer bit constants
        assert_ne!(buffer_bits::GL_COLOR_BUFFER_BIT, 0);
        assert_ne!(buffer_bits::GL_DEPTH_BUFFER_BIT, 0);
        assert_ne!(buffer_bits::GL_STENCIL_BUFFER_BIT, 0);
        // Verify they're distinct
        assert_ne!(buffer_bits::GL_COLOR_BUFFER_BIT, buffer_bits::GL_DEPTH_BUFFER_BIT);
    }

    #[test]
    fn test_gl_types_primitives() {
        // Verify primitive type constants
        assert_eq!(primitives::GL_POINTS, 0x0000);
        assert_eq!(primitives::GL_LINES, 0x0001);
        assert_eq!(primitives::GL_TRIANGLES, 0x0004);
        assert_ne!(primitives::GL_POINTS, primitives::GL_TRIANGLES);
    }

    #[test]
    fn test_gl_types_boolean() {
        // Verify boolean constants
        assert_eq!(boolean::GL_FALSE, 0);
        assert_eq!(boolean::GL_TRUE, 1);
    }

    #[test]
    fn test_gl_types_blend() {
        // Verify blend constants
        assert_eq!(blend_factors::GL_ZERO, 0);
        assert_eq!(blend_factors::GL_ONE, 1);
        assert_ne!(blend_factors::GL_SRC_ALPHA, blend_factors::GL_DST_ALPHA);
        assert_ne!(blend_equations::GL_FUNC_ADD, blend_equations::GL_FUNC_SUBTRACT);
    }
}
