// FILE: graphic3d_texture3_d.rs
// occt: Graphic3d_Texture3D
// occt: Graphic3d_LevelOfTextureAnisotropy

/// Level of anisotropy filter.
/// Notice that actual quality depends on hardware capabilities!
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelOfTextureAnisotropy {
    Off,
    Fast,
    Middle,
    Quality,
}

/// This abstract class for managing 3D textures.
pub struct Texture3D {
    // List of file paths for the texture slices
    paths: Vec<String>,
    // Flags for texture properties
    is_smoothed: bool,
    is_modulate: bool,
    is_repeat: bool,
    aniso_filter: LevelOfTextureAnisotropy,
}

impl Texture3D {
    /// Creates a texture from a file.
    pub fn new(file_path: String) -> Self {
        Texture3D {
            paths: vec![file_path],
            is_smoothed: false,
            is_modulate: true,
            is_repeat: true,
            aniso_filter: LevelOfTextureAnisotropy::Off,
        }
    }

    /// Creates a texture from multiple files (slices).
    pub fn from_files(files: Vec<String>) -> Self {
        Texture3D {
            paths: files,
            is_smoothed: false,
            is_modulate: true,
            is_repeat: true,
            aniso_filter: LevelOfTextureAnisotropy::Off,
        }
    }

    /// Returns the number of texture slices.
    pub fn num_slices(&self) -> usize {
        self.paths.len()
    }

    /// Returns the path of a texture slice by index.
    pub fn get_path(&self, index: usize) -> Option<&str> {
        self.paths.get(index).map(|s| s.as_str())
    }

    /// Enable texture smoothing.
    pub fn enable_smooth(&mut self) {
        self.is_smoothed = true;
    }

    /// Returns TRUE if the texture is smoothed.
    pub fn is_smoothed(&self) -> bool {
        self.is_smoothed
    }

    /// Disable texture smoothing.
    pub fn disable_smooth(&mut self) {
        self.is_smoothed = false;
    }

    /// Enable texture modulate mode.
    /// The image is modulated with the shading of the surface.
    pub fn enable_modulate(&mut self) {
        self.is_modulate = true;
    }

    /// Disable texture modulate mode.
    /// The image is directly decal on the surface.
    pub fn disable_modulate(&mut self) {
        self.is_modulate = false;
    }

    /// Returns TRUE if the texture is modulate.
    pub fn is_modulate(&self) -> bool {
        self.is_modulate
    }

    /// Use this method if you want to enable texture repetition on your objects.
    pub fn enable_repeat(&mut self) {
        self.is_repeat = true;
    }

    /// Use this method if you want to disable texture repetition on your objects.
    pub fn disable_repeat(&mut self) {
        self.is_repeat = false;
    }

    /// Returns TRUE if the texture repeat is enabled.
    pub fn is_repeat(&self) -> bool {
        self.is_repeat
    }

    /// Returns level of anisotropy texture filter.
    /// Default value is LevelOfTextureAnisotropy::Off.
    pub fn aniso_filter(&self) -> LevelOfTextureAnisotropy {
        self.aniso_filter
    }

    /// Set the level of anisotropy texture filter.
    pub fn set_aniso_filter(&mut self, level: LevelOfTextureAnisotropy) {
        self.aniso_filter = level;
    }
}

impl Default for Texture3D {
    fn default() -> Self {
        Texture3D {
            paths: vec![],
            is_smoothed: false,
            is_modulate: true,
            is_repeat: true,
            aniso_filter: LevelOfTextureAnisotropy::Off,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture3d_new() {
        let tex = Texture3D::new("path/to/texture.jpg".to_string());
        assert_eq!(tex.num_slices(), 1);
        assert_eq!(tex.get_path(0), Some("path/to/texture.jpg"));
        assert!(!tex.is_smoothed());
        assert!(tex.is_modulate());
        assert!(tex.is_repeat());
        assert_eq!(tex.aniso_filter(), LevelOfTextureAnisotropy::Off);
    }

    #[test]
    fn test_texture3d_from_files() {
        let files = vec![
            "slice0.jpg".to_string(),
            "slice1.jpg".to_string(),
            "slice2.jpg".to_string(),
        ];
        let tex = Texture3D::from_files(files);
        assert_eq!(tex.num_slices(), 3);
        assert_eq!(tex.get_path(0), Some("slice0.jpg"));
        assert_eq!(tex.get_path(1), Some("slice1.jpg"));
        assert_eq!(tex.get_path(2), Some("slice2.jpg"));
        assert_eq!(tex.get_path(3), None);
    }

    #[test]
    fn test_enable_disable_smooth() {
        let mut tex = Texture3D::new("texture.jpg".to_string());
        assert!(!tex.is_smoothed());
        tex.enable_smooth();
        assert!(tex.is_smoothed());
        tex.disable_smooth();
        assert!(!tex.is_smoothed());
    }

    #[test]
    fn test_enable_disable_modulate() {
        let mut tex = Texture3D::new("texture.jpg".to_string());
        assert!(tex.is_modulate());
        tex.disable_modulate();
        assert!(!tex.is_modulate());
        tex.enable_modulate();
        assert!(tex.is_modulate());
    }

    #[test]
    fn test_enable_disable_repeat() {
        let mut tex = Texture3D::new("texture.jpg".to_string());
        assert!(tex.is_repeat());
        tex.disable_repeat();
        assert!(!tex.is_repeat());
        tex.enable_repeat();
        assert!(tex.is_repeat());
    }

    #[test]
    fn test_aniso_filter() {
        let mut tex = Texture3D::new("texture.jpg".to_string());
        assert_eq!(tex.aniso_filter(), LevelOfTextureAnisotropy::Off);

        tex.set_aniso_filter(LevelOfTextureAnisotropy::Fast);
        assert_eq!(tex.aniso_filter(), LevelOfTextureAnisotropy::Fast);

        tex.set_aniso_filter(LevelOfTextureAnisotropy::Middle);
        assert_eq!(tex.aniso_filter(), LevelOfTextureAnisotropy::Middle);

        tex.set_aniso_filter(LevelOfTextureAnisotropy::Quality);
        assert_eq!(tex.aniso_filter(), LevelOfTextureAnisotropy::Quality);
    }

    #[test]
    fn test_default() {
        let tex = Texture3D::default();
        assert_eq!(tex.num_slices(), 0);
        assert!(!tex.is_smoothed());
        assert!(tex.is_modulate());
        assert!(tex.is_repeat());
        assert_eq!(tex.aniso_filter(), LevelOfTextureAnisotropy::Off);
    }

    #[test]
    fn test_combined_settings() {
        let mut tex = Texture3D::from_files(vec!["a.jpg".to_string(), "b.jpg".to_string()]);
        tex.enable_smooth();
        tex.disable_modulate();
        tex.disable_repeat();
        tex.set_aniso_filter(LevelOfTextureAnisotropy::Quality);

        assert_eq!(tex.num_slices(), 2);
        assert!(tex.is_smoothed());
        assert!(!tex.is_modulate());
        assert!(!tex.is_repeat());
        assert_eq!(tex.aniso_filter(), LevelOfTextureAnisotropy::Quality);
    }
}
