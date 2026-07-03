// FILE: graphic3d_shader_manager.rs
// occt: Graphic3d_ShaderManager

/// Graphics API type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AspectGraphicsLibrary {
    OpenGL = 0,
    OpenGLES = 1,
    Vulkan = 2,
    Direct3D = 3,
}

/// GLSL extensions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Graphic3dGlslExtension {
    StandardDerivatives = 0,
    ShaderTextureLod = 1,
    FragDepth = 2,
    GpuShader4 = 3,
}

/// Manager for shader programs generation.
pub struct Graphic3dShaderManager {
    gapi: AspectGraphicsLibrary,
    gapi_version: (i32, i32),
    glsl_extensions: [bool; 4],
    has_flat_shading: bool,
    to_reverse_dfdx_sign: bool,
    use_red_alpha: bool,
    to_emulate_depth_clamp: bool,
}

impl Graphic3dShaderManager {
    /// Create shader manager for given GAPI.
    pub fn new(gapi: AspectGraphicsLibrary) -> Self {
        Graphic3dShaderManager {
            gapi,
            gapi_version: (1, 0),
            glsl_extensions: [false; 4],
            has_flat_shading: true,
            to_reverse_dfdx_sign: false,
            use_red_alpha: false,
            to_emulate_depth_clamp: true,
        }
    }

    /// Get GAPI.
    pub fn gapi(&self) -> AspectGraphicsLibrary {
        self.gapi
    }

    /// Get GAPI version (major, minor).
    pub fn gapi_version(&self) -> (i32, i32) {
        self.gapi_version
    }

    /// Set GAPI version.
    pub fn set_gapi_version(&mut self, major: i32, minor: i32) {
        self.gapi_version = (major, minor);
    }

    /// Check if GAPI version is >= requested.
    pub fn is_gapi_greater_equal(&self, major: i32, minor: i32) -> bool {
        self.gapi_version.0 > major
            || (self.gapi_version.0 == major && self.gapi_version.1 >= minor)
    }

    /// Check if extension is available.
    pub fn has_glsl_extension(&self, ext: Graphic3dGlslExtension) -> bool {
        self.glsl_extensions[ext as usize]
    }

    /// Enable/disable extension.
    pub fn enable_glsl_extension(&mut self, ext: Graphic3dGlslExtension, enable: bool) {
        self.glsl_extensions[ext as usize] = enable;
    }

    /// Get flat shading flag.
    pub fn has_flat_shading(&self) -> bool {
        self.has_flat_shading
    }

    /// Set flat shading.
    pub fn set_flat_shading(&mut self, use_shading: bool, reverse_sign: bool) {
        self.has_flat_shading = use_shading;
        self.to_reverse_dfdx_sign = reverse_sign;
    }

    /// Get depth clamp emulation flag.
    pub fn to_emulate_depth_clamp(&self) -> bool {
        self.to_emulate_depth_clamp
    }

    /// Set depth clamp emulation.
    pub fn set_emulate_depth_clamp(&mut self, emulate: bool) {
        self.to_emulate_depth_clamp = emulate;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let manager = Graphic3dShaderManager::new(AspectGraphicsLibrary::OpenGL);
        assert_eq!(manager.gapi(), AspectGraphicsLibrary::OpenGL);
        assert_eq!(manager.gapi_version(), (1, 0));
    }

    #[test]
    fn test_version_check() {
        let mut manager = Graphic3dShaderManager::new(AspectGraphicsLibrary::OpenGL);
        manager.set_gapi_version(2, 0);
        assert!(manager.is_gapi_greater_equal(2, 0));
        assert!(manager.is_gapi_greater_equal(1, 9));
        assert!(!manager.is_gapi_greater_equal(2, 1));
    }

    #[test]
    fn test_extensions() {
        let mut manager = Graphic3dShaderManager::new(AspectGraphicsLibrary::OpenGL);
        assert!(!manager.has_glsl_extension(Graphic3dGlslExtension::StandardDerivatives));
        manager.enable_glsl_extension(Graphic3dGlslExtension::StandardDerivatives, true);
        assert!(manager.has_glsl_extension(Graphic3dGlslExtension::StandardDerivatives));
    }

    #[test]
    fn test_flat_shading() {
        let mut manager = Graphic3dShaderManager::new(AspectGraphicsLibrary::OpenGL);
        assert!(manager.has_flat_shading());
        manager.set_flat_shading(false, false);
        assert!(!manager.has_flat_shading());
    }
}
