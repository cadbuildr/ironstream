// FILE: rust/ironstream/crates/ironstream/src/prs3d_text.rs

// occt: Graphic3d_HorizontalTextAlignment
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prs3dTextHAlign {
    Left,
    Center,
    Right,
}

// occt: Graphic3d_VerticalTextAlignment
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prs3dTextVAlign {
    Bottom,
    Center,
    Top,
}

// occt: Prs3d_TextAspect
pub struct Prs3dTextAspect {
    color: [f32; 4],
    font: String,
    height: f64,
    h_align: Prs3dTextHAlign,
    v_align: Prs3dTextVAlign,
}

impl Prs3dTextAspect {
    pub fn new() -> Self {
        Self {
            color: [0.0, 0.0, 0.0, 1.0],
            font: String::from("Courier"),
            height: 12.0,
            h_align: Prs3dTextHAlign::Left,
            v_align: Prs3dTextVAlign::Bottom,
        }
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    pub fn color(&self) -> [f32; 4] {
        self.color
    }

    pub fn set_font(&mut self, font: &str) {
        self.font = font.to_string();
    }

    pub fn font(&self) -> &str {
        &self.font
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn set_h_align(&mut self, h_align: Prs3dTextHAlign) {
        self.h_align = h_align;
    }

    pub fn h_align(&self) -> Prs3dTextHAlign {
        self.h_align
    }

    pub fn set_v_align(&mut self, v_align: Prs3dTextVAlign) {
        self.v_align = v_align;
    }

    pub fn v_align(&self) -> Prs3dTextVAlign {
        self.v_align
    }
}

impl Default for Prs3dTextAspect {
    fn default() -> Self {
        Self::new()
    }
}

// occt: Prs3d_Text
pub struct Prs3dText {
    text: String,
    position: [f64; 3],
    aspect: Prs3dTextAspect,
}

impl Prs3dText {
    pub fn new(text: &str, position: [f64; 3]) -> Self {
        Self {
            text: text.to_string(),
            position,
            aspect: Prs3dTextAspect::new(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn position(&self) -> [f64; 3] {
        self.position
    }

    pub fn set_position(&mut self, position: [f64; 3]) {
        self.position = position;
    }

    pub fn aspect(&self) -> &Prs3dTextAspect {
        &self.aspect
    }

    pub fn aspect_mut(&mut self) -> &mut Prs3dTextAspect {
        &mut self.aspect
    }
}
