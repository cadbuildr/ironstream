// FILE: vrml_texture2.rs
// occt: Vrml_Texture2
//
// Faithful port of OCCT Vrml_Texture2 (DataExchange/TKDEVRML/Vrml/
// Vrml_Texture2.hxx/.cxx): the VRML 1.0 `Texture2` node.
// Contains filename, wrap modes, and repeat settings.

/// Port of Vrml_Texture2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VrmlTexture2 {
    filename: String,
wrap_s: String,
    wrap_t: String,
    repeat_s: bool,
    repeat_t: bool,
}

impl VrmlTexture2 {
    /// Vrml_Texture2 with defaults (empty filename, REPEAT wrap modes).
    pub fn new() -> Self {
        VrmlTexture2 {
            filename: String::new(),
            wrap_s: "REPEAT".to_string(),
            wrap_t: "REPEAT".to_string(),
            repeat_s: true,
            repeat_t: true,
        }
    }

    /// Vrml_Texture2(aFilename).
    pub fn with_filename(a_filename: &str) -> Self {
        VrmlTexture2 {
            filename: a_filename.to_string(),
            wrap_s: "REPEAT".to_string(),
            wrap_t: "REPEAT".to_string(),
            repeat_s: true,
            repeat_t: true,
        }
    }

    pub fn set_filename(&mut self, a_filename: &str) {
        self.filename = a_filename.to_string();
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn set_wrap_s(&mut self, a_wrap: &str) {
        self.wrap_s = a_wrap.to_string();
    }

    pub fn wrap_s(&self) -> &str {
        &self.wrap_s
    }

    pub fn set_wrap_t(&mut self, a_wrap: &str) {
        self.wrap_t = a_wrap.to_string();
    }

    pub fn wrap_t(&self) -> &str {
        &self.wrap_t
    }

    pub fn set_repeat_s(&mut self, a_repeat: bool) {
        self.repeat_s = a_repeat;
    }

    pub fn repeat_s(&self) -> bool {
        self.repeat_s
    }

    pub fn set_repeat_t(&mut self, a_repeat: bool) {
        self.repeat_t = a_repeat;
    }

    pub fn repeat_t(&self) -> bool {
        self.repeat_t
    }
}

impl Default for VrmlTexture2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_texture() {
        let tex = VrmlTexture2::new();
        assert_eq!(tex.filename(), "");
        assert_eq!(tex.wrap_s(), "REPEAT");
        assert_eq!(tex.wrap_t(), "REPEAT");
        assert!(tex.repeat_s());
        assert!(tex.repeat_t());
    }

    #[test]
    fn with_filename() {
        let tex = VrmlTexture2::with_filename("image.png");
        assert_eq!(tex.filename(), "image.png");
        assert_eq!(tex.wrap_s(), "REPEAT");
        assert_eq!(tex.wrap_t(), "REPEAT");
    }

    #[test]
    fn setters() {
        let mut tex = VrmlTexture2::new();
        tex.set_filename("texture.jpg");
        tex.set_wrap_s("CLAMP");
        tex.set_wrap_t("CLAMP");
        tex.set_repeat_s(false);
        tex.set_repeat_t(false);
        assert_eq!(tex.filename(), "texture.jpg");
        assert_eq!(tex.wrap_s(), "CLAMP");
        assert_eq!(tex.wrap_t(), "CLAMP");
        assert!(!tex.repeat_s());
        assert!(!tex.repeat_t());
    }

    #[test]
    fn equality() {
        let tex1 = VrmlTexture2::with_filename("test.png");
        let tex2 = VrmlTexture2::with_filename("test.png");
        let tex3 = VrmlTexture2::with_filename("other.png");
        assert_eq!(tex1, tex2);
        assert_ne!(tex1, tex3);
    }
}
