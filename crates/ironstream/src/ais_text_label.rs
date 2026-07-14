// FILE: ais_text_label.rs
// occt: AIS_TextLabel, Graphic3d_Text
// occt-ref: AIS_TextAttributes
//       Font_TextFormatter

/// Text horizontal justification.
// occt-ref: Graphic3d_HorizontalTextAlignment
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HTextAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// Text vertical justification.
// occt-ref: Graphic3d_VerticalTextAlignment
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum VTextAlign {
    #[default]
    Bottom,
    Center,
    Top,
}

/// Text rendering style.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FontSlant {
    #[default]
    Regular,
    Italic,
    Bold,
    BoldItalic,
}

impl FontSlant {
    pub fn is_bold(&self) -> bool { matches!(self, Self::Bold | Self::BoldItalic) }
    pub fn is_italic(&self) -> bool { matches!(self, Self::Italic | Self::BoldItalic) }
}

/// Attributes for text rendering.
/// occt-note: AIS_TextAttributes / Graphic3d_Text
#[derive(Clone, Debug, PartialEq)]
pub struct AisTextAttributes {
    pub font_name: String,
    pub height: f32,
    pub color: [f32; 3],
    pub horizontal_align: HTextAlign,
    pub vertical_align: VTextAlign,
    pub slant: FontSlant,
    pub has_own_color: bool,
    pub display_type: u8,    // 0=normal, 1=subtitle, 2=decal, 3=blend
    pub color_subtitle: [f32; 3],
    pub angle: f32,          // rotation in radians (around Z)
    pub zoomable: bool,
}

impl Default for AisTextAttributes {
    fn default() -> Self {
        Self {
            font_name: "Courier".into(),
            height: 12.0,
            color: [1.0, 1.0, 1.0],
            horizontal_align: HTextAlign::Left,
            vertical_align: VTextAlign::Bottom,
            slant: FontSlant::Regular,
            has_own_color: false,
            display_type: 0,
            color_subtitle: [0.0, 0.0, 0.0],
            angle: 0.0,
            zoomable: true,
        }
    }
}

impl AisTextAttributes {
    pub fn new() -> Self { Self::default() }
    pub fn set_font(&mut self, name: impl Into<String>) { self.font_name = name.into(); }
    pub fn set_height(&mut self, h: f32) { self.height = h.max(0.0); }
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) { self.color = [r, g, b]; self.has_own_color = true; }
    pub fn set_align_h(&mut self, a: HTextAlign) { self.horizontal_align = a; }
    pub fn set_align_v(&mut self, a: VTextAlign) { self.vertical_align = a; }
    pub fn set_slant(&mut self, s: FontSlant) { self.slant = s; }
    pub fn set_angle(&mut self, a: f32) { self.angle = a; }
    pub fn set_zoomable(&mut self, z: bool) { self.zoomable = z; }
}

/// A text label interactive object.
/// occt: AIS_TextLabel
#[derive(Clone, Debug)]
pub struct AisTextLabel {
    pub object_id: u32,
    pub text: String,
    pub position: [f64; 3],
    pub attributes: AisTextAttributes,
    pub is_displayed: bool,
    pub z_layer: i32,
    pub selection_priority: i32,
}

impl AisTextLabel {
    pub fn new(object_id: u32, text: impl Into<String>, position: [f64; 3]) -> Self {
        Self {
            object_id,
            text: text.into(),
            position,
            attributes: AisTextAttributes::default(),
            is_displayed: false,
            z_layer: 0,
            selection_priority: 0,
        }
    }

    pub fn set_text(&mut self, t: impl Into<String>) { self.text = t.into(); }
    pub fn set_position(&mut self, p: [f64; 3]) { self.position = p; }
    pub fn set_font(&mut self, f: impl Into<String>) { self.attributes.set_font(f); }
    pub fn set_height(&mut self, h: f32) { self.attributes.set_height(h); }
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) { self.attributes.set_color(r, g, b); }
    pub fn set_z_layer(&mut self, z: i32) { self.z_layer = z; }

    pub fn display(&mut self) { self.is_displayed = true; }
    pub fn erase(&mut self) { self.is_displayed = false; }

    pub fn text_width_estimate(&self) -> f32 {
        self.text.len() as f32 * self.attributes.height * 0.6
    }
}

/// Collection of text labels.
#[derive(Clone, Debug, Default)]
pub struct AisTextLabelPool {
    pub labels: Vec<AisTextLabel>,
}

impl AisTextLabelPool {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, label: AisTextLabel) { self.labels.push(label); }

    pub fn find_mut(&mut self, id: u32) -> Option<&mut AisTextLabel> {
        self.labels.iter_mut().find(|l| l.object_id == id)
    }

    pub fn displayed(&self) -> Vec<&AisTextLabel> {
        self.labels.iter().filter(|l| l.is_displayed).collect()
    }

    pub fn nb_labels(&self) -> usize { self.labels.len() }

    pub fn remove(&mut self, id: u32) { self.labels.retain(|l| l.object_id != id); }
}

/// Simple text formatter (word-wrap stub).
/// occt: Font_TextFormatter
pub struct FontTextFormatter {
    pub max_width: f32,
    pub tab_size: usize,
}

impl FontTextFormatter {
    pub fn new(max_width: f32) -> Self { Self { max_width, tab_size: 4 } }

    /// Split text into lines by '\n' and simple word-wrap.
    pub fn format(&self, text: &str, char_width: f32) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        for raw_line in text.split('\n') {
            let expanded = raw_line.replace('\t', &" ".repeat(self.tab_size));
            if char_width <= 0.0 || self.max_width <= 0.0 {
                lines.push(expanded);
                continue;
            }
            let max_chars = (self.max_width / char_width).floor() as usize;
            if max_chars == 0 || expanded.len() <= max_chars {
                lines.push(expanded);
            } else {
                let mut start = 0;
                while start < expanded.len() {
                    let end = (start + max_chars).min(expanded.len());
                    lines.push(expanded[start..end].to_owned());
                    start = end;
                }
            }
        }
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_label_display() {
        let mut lbl = AisTextLabel::new(1, "Hello World", [0.0, 0.0, 0.0]);
        assert!(!lbl.is_displayed);
        lbl.display();
        assert!(lbl.is_displayed);
        lbl.erase();
        assert!(!lbl.is_displayed);
    }

    #[test]
    fn text_attributes() {
        let mut a = AisTextAttributes::new();
        a.set_font("Arial");
        a.set_height(18.0);
        a.set_color(1.0, 0.0, 0.0);
        assert_eq!(a.font_name, "Arial");
        assert!(a.has_own_color);
        assert!((a.height - 18.0).abs() < 1e-6);
    }

    #[test]
    fn font_slant_flags() {
        assert!(FontSlant::Bold.is_bold());
        assert!(!FontSlant::Bold.is_italic());
        assert!(FontSlant::BoldItalic.is_italic());
    }

    #[test]
    fn text_label_pool_add_find() {
        let mut pool = AisTextLabelPool::new();
        pool.add(AisTextLabel::new(1, "A", [0.0; 3]));
        pool.add(AisTextLabel::new(2, "B", [1.0, 0.0, 0.0]));
        pool.find_mut(1).unwrap().display();
        assert_eq!(pool.displayed().len(), 1);
        pool.remove(2);
        assert_eq!(pool.nb_labels(), 1);
    }

    #[test]
    fn text_formatter_wrap() {
        let fmt = FontTextFormatter::new(50.0);
        let lines = fmt.format("ABCDEFGHIJ", 10.0); // 5 chars per line
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn text_formatter_newline() {
        let fmt = FontTextFormatter::new(1000.0);
        let lines = fmt.format("line1\nline2\nline3", 10.0);
        assert_eq!(lines.len(), 3);
    }
}
