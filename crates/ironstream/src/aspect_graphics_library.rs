// FILE: aspect_graphics_library.rs
// occt: Aspect_GraphicsLibrary

/// Graphics library/API type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectGraphicsLibrary {
    /// OpenGL API.
    OpenGL = 0,
    /// OpenGL ES API.
    OpenGLES = 1,
    /// Vulkan API.
    Vulkan = 2,
    /// Direct3D API.
    Direct3D = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values() {
        assert_eq!(AspectGraphicsLibrary::OpenGL as i32, 0);
        assert_eq!(AspectGraphicsLibrary::Vulkan as i32, 2);
    }
}
