// FILE: open_gl_text.rs
// occt: OpenGl_Text

//! Text rendering element, port of OpenGl_Text.
//! GL plumbing (context, font, VBOs) is modelled with small local types;
//! the state management logic (Reset / SetFontSize / FontKey / Init) is real.

pub const THE_DEFAULT_FONT: &str = "Courier";

/// Local model of Font_FontAspect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontAspect {
    Undefined = -1,
    Regular = 0,
    Bold = 1,
    Italic = 2,
    BoldItalic = 3,
}

/// Local model of Font_Hinting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontHinting {
    Off = 0,
    Normal = 1,
    Light = 2,
}

/// Local model of the text-related part of OpenGl_Aspects.
#[derive(Debug, Clone)]
pub struct OpenGlAspects {
    pub text_font: Option<String>,
    pub text_font_aspect: FontAspect,
}

impl Default for OpenGlAspects {
    fn default() -> Self {
        OpenGlAspects {
            text_font: None,
            text_font_aspect: FontAspect::Undefined,
        }
    }
}

/// Local model of Graphic3d_Text parameters.
#[derive(Debug, Clone)]
pub struct Graphic3dText {
    text: String,
    height: f32,
    position: [f32; 3],
}

impl Graphic3dText {
    pub fn new(height: f32) -> Self {
        Graphic3dText {
            text: String::new(),
            height,
            position: [0.0; 3],
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }

    pub fn position(&self) -> [f32; 3] {
        self.position
    }

    pub fn set_position(&mut self, p: [f32; 3]) {
        self.position = p;
    }
}

/// Local model of a shared OpenGl_Font resource.
#[derive(Debug, Clone)]
pub struct OpenGlFont {
    pub point_size: f32,
    pub key: String,
}

/// Local model of the GL context (tracks released resources).
#[derive(Debug, Default)]
pub struct OpenGlContext {
    pub released_fonts: usize,
}

/// Local model of a vertex buffer object.
#[derive(Debug, Clone)]
pub struct Vbo {
    pub data_size: usize,
}

/// Text rendering element.
#[derive(Debug)]
pub struct OpenGlText {
    text: Graphic3dText,
    font: Option<OpenGlFont>,
    vertex_vbos: Vec<Vbo>,
    scale_height: f32,
    is_2d: bool,
}

impl OpenGlText {
    /// Empty constructor: creates Graphic3d_Text with height 10.
    pub fn new() -> Self {
        OpenGlText {
            text: Graphic3dText::new(10.0),
            font: None,
            vertex_vbos: Vec::new(),
            scale_height: 1.0,
            is_2d: false,
        }
    }

    /// Creates new text in 3D space from the given parameters.
    pub fn with_params(params: Graphic3dText) -> Self {
        OpenGlText {
            text: params,
            font: None,
            vertex_vbos: Vec::new(),
            scale_height: 1.0,
            is_2d: false,
        }
    }

    /// Returns text parameters.
    pub fn text(&self) -> &Graphic3dText {
        &self.text
    }

    /// Sets text parameters.
    pub fn set_text(&mut self, text: Graphic3dText) {
        self.text = text;
    }

    /// Return true if text is 2D.
    pub fn is_2d(&self) -> bool {
        self.is_2d
    }

    /// Set true if text is 2D.
    pub fn set_2d(&mut self, enable: bool) {
        self.is_2d = enable;
    }

    pub fn scale_height(&self) -> f32 {
        self.scale_height
    }

    /// Create key for shared font resource.
    /// Mirrors OpenGl_Text::FontKey: "<font>:<aspect>:<resolution>:<height>:<hinting>".
    pub fn font_key(
        aspect: &OpenGlAspects,
        height: i32,
        resolution: u32,
        font_hinting: FontHinting,
    ) -> String {
        let font_aspect = if aspect.text_font_aspect != FontAspect::Undefined {
            aspect.text_font_aspect
        } else {
            FontAspect::Regular
        };
        let font = aspect
            .text_font
            .as_deref()
            .unwrap_or(THE_DEFAULT_FONT);
        format!(
            "{}:{}:{}:{}:{}",
            font, font_aspect as i32, resolution, height, font_hinting as i32
        )
    }

    /// Setup new font size: releases resources if the height changed,
    /// then stores the new height (OpenGl_Text::SetFontSize).
    pub fn set_font_size(&mut self, ctx: &mut OpenGlContext, font_size: i32) {
        if self.text.height() != font_size as f32 {
            self.release(ctx);
        }
        self.text.set_height(font_size as f32);
    }

    /// Release cached VBO resources and the previous font if height changed
    /// (OpenGl_Text::Reset).
    pub fn reset(&mut self, ctx: &mut OpenGlContext) {
        let font_mismatch = match &self.font {
            Some(f) => f.point_size != self.text.height(),
            None => false,
        };
        if font_mismatch {
            self.release(ctx);
        } else {
            self.release_vbos(ctx);
        }
    }

    /// Release both font and VBO resources (OpenGl_Text::Release).
    pub fn release(&mut self, ctx: &mut OpenGlContext) {
        self.release_vbos(ctx);
        if self.font.take().is_some() {
            ctx.released_fonts += 1;
        }
    }

    /// Release cached VBO resources (OpenGl_Text::releaseVbos).
    pub fn release_vbos(&mut self, _ctx: &mut OpenGlContext) {
        self.vertex_vbos.clear();
    }

    /// Setup new string and position (deprecated OpenGl_Text::Init).
    pub fn init(&mut self, ctx: &mut OpenGlContext, text: &str, point: [f32; 3]) {
        self.reset(ctx);
        self.set_2d(false);
        self.text.set_text(text);
        self.text.set_position(point);
    }

    /// Setup new position (deprecated OpenGl_Text::SetPosition).
    pub fn set_position(&mut self, point: [f32; 3]) {
        self.text.set_position(point);
    }

    /// Attach a modelled font resource (stands in for FindFont result).
    pub fn set_font(&mut self, font: OpenGlFont) {
        self.font = Some(font);
    }

    pub fn font(&self) -> Option<&OpenGlFont> {
        self.font.as_ref()
    }

    /// Attach a modelled VBO (filled during render in OCCT).
    pub fn add_vbo(&mut self, vbo: Vbo) {
        self.vertex_vbos.push(vbo);
    }

    /// Estimated GPU memory usage: sum of VBO data sizes
    /// (OpenGl_Text::EstimatedDataSize).
    pub fn estimated_data_size(&self) -> usize {
        self.vertex_vbos.iter().map(|v| v.data_size).sum()
    }
}

impl Default for OpenGlText {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_text_has_height_10() {
        let t = OpenGlText::new();
        assert_eq!(t.text().height(), 10.0);
        assert!(!t.is_2d());
        assert_eq!(t.scale_height(), 1.0);
    }

    #[test]
    fn with_params_keeps_params() {
        let mut params = Graphic3dText::new(24.0);
        params.set_text("hello");
        let t = OpenGlText::with_params(params);
        assert_eq!(t.text().height(), 24.0);
        assert_eq!(t.text().text(), "hello");
    }

    #[test]
    fn font_key_default_font() {
        let aspect = OpenGlAspects::default();
        let key = OpenGlText::font_key(&aspect, 14, 72, FontHinting::Off);
        assert_eq!(key, "Courier:0:72:14:0");
    }

    #[test]
    fn font_key_custom_font_and_aspect() {
        let aspect = OpenGlAspects {
            text_font: Some("Arial".to_string()),
            text_font_aspect: FontAspect::BoldItalic,
        };
        let key = OpenGlText::font_key(&aspect, 18, 96, FontHinting::Normal);
        assert_eq!(key, "Arial:3:96:18:1");
    }

    #[test]
    fn set_font_size_releases_on_change() {
        let mut ctx = OpenGlContext::default();
        let mut t = OpenGlText::new();
        t.set_font(OpenGlFont {
            point_size: 10.0,
            key: "Courier:0:72:10:0".to_string(),
        });
        t.add_vbo(Vbo { data_size: 128 });

        // Same size: no release
        t.set_font_size(&mut ctx, 10);
        assert!(t.font().is_some());
        assert_eq!(ctx.released_fonts, 0);

        // Different size: font and VBOs released, new height stored
        t.set_font_size(&mut ctx, 16);
        assert!(t.font().is_none());
        assert_eq!(ctx.released_fonts, 1);
        assert_eq!(t.text().height(), 16.0);
        assert_eq!(t.estimated_data_size(), 0);
    }

    #[test]
    fn reset_releases_font_only_on_height_mismatch() {
        let mut ctx = OpenGlContext::default();
        let mut t = OpenGlText::new(); // height 10
        t.set_font(OpenGlFont {
            point_size: 10.0,
            key: "k".to_string(),
        });
        t.add_vbo(Vbo { data_size: 64 });

        // Font matches text height: only VBOs released
        t.reset(&mut ctx);
        assert!(t.font().is_some());
        assert_eq!(t.estimated_data_size(), 0);

        // Change height without release, then reset: font is released too
        t.text = {
            let mut p = Graphic3dText::new(20.0);
            p.set_text("x");
            p
        };
        t.reset(&mut ctx);
        assert!(t.font().is_none());
        assert_eq!(ctx.released_fonts, 1);
    }

    #[test]
    fn init_sets_string_and_position() {
        let mut ctx = OpenGlContext::default();
        let mut t = OpenGlText::new();
        t.set_2d(true);
        t.init(&mut ctx, "label", [1.0, 2.0, 3.0]);
        assert_eq!(t.text().text(), "label");
        assert_eq!(t.text().position(), [1.0, 2.0, 3.0]);
        assert!(!t.is_2d());
    }

    #[test]
    fn set_position_updates_text_params() {
        let mut t = OpenGlText::new();
        t.set_position([4.0, 5.0, 6.0]);
        assert_eq!(t.text().position(), [4.0, 5.0, 6.0]);
    }

    #[test]
    fn estimated_data_size_sums_vbos() {
        let mut t = OpenGlText::new();
        t.add_vbo(Vbo { data_size: 100 });
        t.add_vbo(Vbo { data_size: 28 });
        assert_eq!(t.estimated_data_size(), 128);
    }
}
