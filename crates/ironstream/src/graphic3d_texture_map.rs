// FILE: graphic3d_texture_map.rs
// occt: Graphic3d_TextureMap

/// Level of anisotropy filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelOfTextureAnisotropy {
    Off,
    Fast,
    Middle,
    Quality,
}

/// This is an abstract class for managing texture applicable on polygons.
pub struct TextureMap {
    // File path or description of the texture
    file_path: String,
    // Texture smoothing flag
    is_smoothed: bool,
    // Texture modulate mode flag (modulate vs decal)
    is_modulate: bool,
    // Texture repetition flag
    is_repeat: bool,
    // Anisotropy filter level
    aniso_filter: LevelOfTextureAnisotropy,
}

impl TextureMap {
    /// Creates a new texture map.
    pub fn new(file_path: String) -> Self {
        TextureMap {
            file_path,
            is_smoothed: false,
            is_modulate: true,
            is_repeat: true,
            aniso_filter: LevelOfTextureAnisotropy::Off,
        }
    }

    /// Returns the file path of the texture.
    pub fn file_path(&self) -> &str {
        &self.file_path
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

    /// Set level of anisotropy texture filter.
    pub fn set_aniso_filter(&mut self, level: LevelOfTextureAnisotropy) {
        self.aniso_filter = level;
    }
}

impl Default for TextureMap {
    fn default() -> Self {
        TextureMap {
            file_path: String::new(),
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
    fn test_texture_map_new() {
        let tex = TextureMap::new("texture.jpg".to_string());
        assert_eq!(tex.file_path(), "texture.jpg");
        assert!(!tex.is_smoothed());
        assert!(tex.is_modulate());
        assert!(tex.is_repeat());
        assert_eq!(tex.aniso_filter(), LevelOfTextureAnisotropy::Off);
    }

    #[test]
    fn test_enable_disable_smooth() {
        let mut tex = TextureMap::new("texture.jpg".to_string());
        assert!(!tex.is_smoothed());
        tex.enable_smooth();
        assert!(tex.is_smoothed());
        tex.disable_smooth();
        assert!(!tex.is_smoothed());
    }

    #[test]
    fn test_enable_disable_modulate() {
        let mut tex = TextureMap::new("texture.jpg".to_string());
        assert!(tex.is_modulate());
        tex.disable_modulate();
        assert!(!tex.is_modulate());
        tex.enable_modulate();
        assert!(tex.is_modulate());
    }

    #[test]
    fn test_enable_disable_repeat() {
        let mut tex = TextureMap::new("texture.jpg".to_string());
        assert!(tex.is_repeat());
        tex.disable_repeat();
        assert!(!tex.is_repeat());
        tex.enable_repeat();
        assert!(tex.is_repeat());
    }

    #[test]
    fn test_aniso_filter() {
        let mut tex = TextureMap::new("texture.jpg".to_string());
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
        let tex = TextureMap::default();
        assert_eq!(tex.file_path(), "");
        assert!(!tex.is_smoothed());
        assert!(tex.is_modulate());
        assert!(tex.is_repeat());
        assert_eq!(tex.aniso_filter(), LevelOfTextureAnisotropy::Off);
    }

    #[test]
    fn test_combined_transformations() {
        let mut tex = TextureMap::new("myfile.jpg".to_string());
        tex.enable_smooth();
        tex.disable_modulate();
        tex.disable_repeat();
        tex.set_aniso_filter(LevelOfTextureAnisotropy::Quality);

        assert_eq!(tex.file_path(), "myfile.jpg");
        assert!(tex.is_smoothed());
        assert!(!tex.is_modulate());
        assert!(!tex.is_repeat());
        assert_eq!(tex.aniso_filter(), LevelOfTextureAnisotropy::Quality);
    }
}
