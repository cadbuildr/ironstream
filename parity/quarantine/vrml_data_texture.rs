// FILE: vrml_data_texture.rs
// occt: VrmlData_Texture
//
// Faithful port of OCCT VrmlData_Texture (DataExchange/TKDEVRML/VrmlData/
// VrmlData_Texture.hxx/.cxx): represents VRML 2.0 texture mapping with
// URL, repeat settings, and filters. Base for ImageTexture, MovieTexture, etc.

use std::cell::RefCell;
use std::rc::Rc;

/// Texture repeat mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureRepeatMode {
    Clamp,
    Repeat,
}

impl TextureRepeatMode {
    pub fn as_str(&self) -> &str {
        match self {
            TextureRepeatMode::Clamp => "CLAMP",
            TextureRepeatMode::Repeat => "REPEAT",
        }
    }
}

impl Default for TextureRepeatMode {
    fn default() -> Self {
        TextureRepeatMode::Repeat
    }
}

/// Error status for read/write operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrmlDataTextureErrorStatus {
    Ok = 0,
    EndOfFile = 1,
    NotEndOfFile = 2,
    GeneralError = 3,
}

/// Input buffer for parsing.
pub struct VrmlDataTextureInBuffer {
    pub line_num: u32,
}

impl VrmlDataTextureInBuffer {
    pub fn new() -> Self {
        VrmlDataTextureInBuffer { line_num: 1 }
    }
}

impl Default for VrmlDataTextureInBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// VRML Texture node: image-based texture mapping.
/// Stores texture URL, repeat modes, and intensity for applying images to geometry.
pub struct VrmlDataTexture {
    my_url: String,
    my_repeat_s: bool,  // repeat in S (U) direction
    my_repeat_t: bool,  // repeat in T (V) direction
    my_intensity: f64,  // modulation intensity [0,1]
    my_name: String,
}

impl VrmlDataTexture {
    /// Constructor: creates an empty texture.
    pub fn new(name: Option<&str>) -> Self {
        VrmlDataTexture {
            my_url: String::new(),
            my_repeat_s: true,
            my_repeat_t: true,
            my_intensity: 1.0,
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Full constructor with URL and repeat settings.
    pub fn with_settings(
        url: &str,
        repeat_s: bool,
        repeat_t: bool,
        intensity: f64,
        name: Option<&str>,
    ) -> Self {
        VrmlDataTexture {
            my_url: url.to_string(),
            my_repeat_s: repeat_s,
            my_repeat_t: repeat_t,
            my_intensity: intensity.clamp(0.0, 1.0),
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Query the name.
    pub fn name(&self) -> &str {
        &self.my_name
    }

    /// Set the name.
    pub fn set_name(&mut self, name: &str) {
        self.my_name = name.to_string();
    }

    /// Get the texture URL.
    pub fn url(&self) -> &str {
        &self.my_url
    }

    /// Set the texture URL.
    pub fn set_url(&mut self, url: &str) {
        self.my_url = url.to_string();
    }

    /// Get repeat-S flag.
    pub fn repeat_s(&self) -> bool {
        self.my_repeat_s
    }

    /// Set repeat-S flag.
    pub fn set_repeat_s(&mut self, repeat: bool) {
        self.my_repeat_s = repeat;
    }

    /// Get repeat-T flag.
    pub fn repeat_t(&self) -> bool {
        self.my_repeat_t
    }

    /// Set repeat-T flag.
    pub fn set_repeat_t(&mut self, repeat: bool) {
        self.my_repeat_t = repeat;
    }

    /// Get intensity modulation [0,1].
    pub fn intensity(&self) -> f64 {
        self.my_intensity
    }

    /// Set intensity modulation [0,1], clamped to valid range.
    pub fn set_intensity(&mut self, intensity: f64) {
        self.my_intensity = intensity.clamp(0.0, 1.0);
    }

    /// Check if texture is in default state (empty URL).
    pub fn is_default(&self) -> bool {
        self.my_url.is_empty()
    }

    /// Virtual read method: parse Texture node from VRML stream.
    pub fn read(&mut self, _buffer: &mut VrmlDataTextureInBuffer) -> VrmlDataTextureErrorStatus {
        // Subclass/user provides actual parsing.
        VrmlDataTextureErrorStatus::Ok
    }

    /// Virtual write method: output Texture node to VRML format.
    pub fn write(&self, _prefix: Option<&str>) -> VrmlDataTextureErrorStatus {
        // Subclass/user provides actual output.
        VrmlDataTextureErrorStatus::Ok
    }

    /// Get the repeat mode for S direction.
    pub fn repeat_mode_s(&self) -> TextureRepeatMode {
        if self.my_repeat_s {
            TextureRepeatMode::Repeat
        } else {
            TextureRepeatMode::Clamp
        }
    }

    /// Get the repeat mode for T direction.
    pub fn repeat_mode_t(&self) -> TextureRepeatMode {
        if self.my_repeat_t {
            TextureRepeatMode::Repeat
        } else {
            TextureRepeatMode::Clamp
        }
    }
}

impl Default for VrmlDataTexture {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Clone for VrmlDataTexture {
    fn clone(&self) -> Self {
        VrmlDataTexture {
            my_url: self.my_url.clone(),
            my_repeat_s: self.my_repeat_s,
            my_repeat_t: self.my_repeat_t,
            my_intensity: self.my_intensity,
            my_name: self.my_name.clone(),
        }
    }
}

impl PartialEq for VrmlDataTexture {
    fn eq(&self, other: &Self) -> bool {
        self.my_url == other.my_url
            && self.my_repeat_s == other.my_repeat_s
            && self.my_repeat_t == other.my_repeat_t
            && (self.my_intensity - other.my_intensity).abs() < 1e-10
            && self.my_name == other.my_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_texture() {
        let tex = VrmlDataTexture::new(None);
        assert_eq!(tex.url(), "");
        assert!(tex.repeat_s());
        assert!(tex.repeat_t());
        assert_eq!(tex.intensity(), 1.0);
        assert!(tex.is_default());
    }

    #[test]
    fn named_texture() {
        let tex = VrmlDataTexture::new(Some("MyTexture"));
        assert_eq!(tex.name(), "MyTexture");
    }

    #[test]
    fn with_settings() {
        let tex = VrmlDataTexture::with_settings(
            "image.png",
            false,
            true,
            0.8,
            Some("Tex"),
        );
        assert_eq!(tex.url(), "image.png");
        assert!(!tex.repeat_s());
        assert!(tex.repeat_t());
        assert_eq!(tex.intensity(), 0.8);
    }

    #[test]
    fn set_url() {
        let mut tex = VrmlDataTexture::new(None);
        tex.set_url("brick.jpg");
        assert_eq!(tex.url(), "brick.jpg");
        assert!(!tex.is_default());
    }

    #[test]
    fn intensity_clamping() {
        let tex = VrmlDataTexture::with_settings("img.png", true, true, 1.5, None);
        assert_eq!(tex.intensity(), 1.0);

        let tex2 = VrmlDataTexture::with_settings("img.png", true, true, -0.5, None);
        assert_eq!(tex2.intensity(), 0.0);
    }

    #[test]
    fn set_intensity() {
        let mut tex = VrmlDataTexture::new(None);
        tex.set_intensity(0.75);
        assert_eq!(tex.intensity(), 0.75);

        tex.set_intensity(2.0);
        assert_eq!(tex.intensity(), 1.0); // clamped

        tex.set_intensity(-1.0);
        assert_eq!(tex.intensity(), 0.0); // clamped
    }

    #[test]
    fn repeat_modes() {
        let tex = VrmlDataTexture::with_settings("img.png", true, false, 1.0, None);
        assert_eq!(tex.repeat_mode_s(), TextureRepeatMode::Repeat);
        assert_eq!(tex.repeat_mode_t(), TextureRepeatMode::Clamp);
    }

    #[test]
    fn set_repeat_flags() {
        let mut tex = VrmlDataTexture::new(None);
        assert!(tex.repeat_s());
        assert!(tex.repeat_t());

        tex.set_repeat_s(false);
        tex.set_repeat_t(false);
        assert!(!tex.repeat_s());
        assert!(!tex.repeat_t());
    }

    #[test]
    fn clone_preserves_data() {
        let tex = VrmlDataTexture::with_settings("tile.png", false, true, 0.9, Some("Orig"));
        let cloned = tex.clone();
        assert_eq!(cloned.url(), "tile.png");
        assert!(!cloned.repeat_s());
        assert!(cloned.repeat_t());
        assert_eq!(cloned.intensity(), 0.9);
        assert_eq!(cloned.name(), "Orig");
    }

    #[test]
    fn equality() {
        let tex1 = VrmlDataTexture::with_settings("img.png", true, false, 0.5, Some("T"));
        let tex2 = VrmlDataTexture::with_settings("img.png", true, false, 0.5, Some("T"));
        assert_eq!(tex1, tex2);
    }

    #[test]
    fn inequality_different_url() {
        let tex1 = VrmlDataTexture::with_settings("img1.png", true, true, 1.0, None);
        let tex2 = VrmlDataTexture::with_settings("img2.png", true, true, 1.0, None);
        assert_ne!(tex1, tex2);
    }

    #[test]
    fn set_name() {
        let mut tex = VrmlDataTexture::new(Some("Old"));
        tex.set_name("New");
        assert_eq!(tex.name(), "New");
    }
}
