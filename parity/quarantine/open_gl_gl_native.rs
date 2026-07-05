// FILE: open_gl_gl_native.rs
// occt: OpenGl_GlNative

/// Native OpenGL platform bindings.
/// Provides platform-specific macros and constants for API entry point declarations.
pub struct OpenGlGlNative;

impl OpenGlGlNative {
    /// APIENTRY macro for function declarations (platform-specific).
    /// On Windows: stdcall calling convention; elsewhere: default calling convention.
    pub fn api_entry() -> &'static str {
        if cfg!(target_os = "windows") {
            "APIENTRY"
        } else {
            "extern"
        }
    }

    /// GLAPI macro for GL function visibility declarations.
    pub fn gl_api() -> &'static str {
        "extern"
    }

    /// GL_APICALL macro for GL function calling convention.
    pub fn gl_api_call() -> &'static str {
        "extern"
    }

    /// Indicates whether OpenGL ES is being used.
    pub fn is_gles() -> bool {
        cfg!(any(
            target_os = "android",
            target_os = "ios",
            target_arch = "wasm32"
        ))
    }

    /// Indicates whether standard OpenGL desktop is being used.
    pub fn is_desktop_gl() -> bool {
        !Self::is_gles()
    }

    /// Indicates whether the platform is macOS.
    pub fn is_macos() -> bool {
        cfg!(target_os = "macos")
    }

    /// Indicates whether the platform is Windows.
    pub fn is_windows() -> bool {
        cfg!(target_os = "windows")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_native_platform_detection() {
        // Verify platform detection functions work
        let is_desktop = OpenGlGlNative::is_desktop_gl();
        let is_gles = OpenGlGlNative::is_gles();

        // One must be true for valid platform
        assert!(is_desktop || is_gles);
    }

    #[test]
    fn test_gl_native_macos_windows() {
        // Verify OS detection
        let is_mac = OpenGlGlNative::is_macos();
        let is_win = OpenGlGlNative::is_windows();

        #[cfg(target_os = "macos")]
        assert!(is_mac);

        #[cfg(target_os = "windows")]
        assert!(is_win);
    }

    #[test]
    fn test_gl_native_api_strings() {
        // Verify API strings are non-empty
        assert!(!OpenGlGlNative::api_entry().is_empty());
        assert!(!OpenGlGlNative::gl_api().is_empty());
        assert!(!OpenGlGlNative::gl_api_call().is_empty());
    }
}
