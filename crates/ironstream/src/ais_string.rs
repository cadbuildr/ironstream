// FILE: ais_string.rs
// occt-ref: AIS_TextLabel // (string-specific), AIS_DisplayMode,
//       AIS_TextRenderMode, Prs3d_Text

/// Text rendering mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum AisTextRenderMode {
    #[default]
    Bitmap,
    Outline,
    SmoothPixmap,
    TriangleMesh,
}

/// Font style specification.
#[derive(Clone, Debug, Default)]
pub struct AisFontSpec {
    pub family: String,
    pub height: f32,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl AisFontSpec {
    pub fn new(family: impl Into<String>, height: f32) -> Self {
        Self { family: family.into(), height, ..Self::default() }
    }

    pub fn with_bold(mut self) -> Self { self.bold = true; self }
    pub fn with_italic(mut self) -> Self { self.italic = true; self }
}

/// String (text) object in AIS context.
// occt-ref: AIS_TextLabel // with string value
#[derive(Clone, Debug)]
pub struct AisString {
    pub object_id: u32,
    pub text: String,
    pub position: [f64; 3],
    pub font_spec: AisFontSpec,
    pub color: [f32; 3],
    pub alignment: u8,  // 0=left 1=center 2=right
    pub is_visible: bool,
    pub width_hint: Option<f32>,
}

impl AisString {
    pub fn new(obj_id: u32, text: impl Into<String>, pos: [f64; 3]) -> Self {
        Self {
            object_id: obj_id,
            text: text.into(),
            position: pos,
            font_spec: AisFontSpec::default(),
            color: [1.0, 1.0, 1.0],
            alignment: 0,
            is_visible: true,
            width_hint: None,
        }
    }

    pub fn set_font(&mut self, spec: AisFontSpec) { self.font_spec = spec; }
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) { self.color = [r, g, b]; }
    pub fn set_alignment(&mut self, align: u8) { self.alignment = align.min(2); }
    pub fn set_width_hint(&mut self, w: f32) { self.width_hint = Some(w.max(1.0)); }

    pub fn text_length(&self) -> usize { self.text.len() }
    pub fn display(&mut self) { self.is_visible = true; }
    pub fn erase(&mut self) { self.is_visible = false; }

    pub fn is_left_aligned(&self) -> bool { self.alignment == 0 }
    pub fn is_center_aligned(&self) -> bool { self.alignment == 1 }
    pub fn is_right_aligned(&self) -> bool { self.alignment == 2 }
}

/// String formatter: wrapping + escaping.
/// occt-note: Text formatter utilities
#[derive(Clone, Debug, Default)]
pub struct AisStringFormatter;

impl AisStringFormatter {
    /// Escape special characters in string for display.
    pub fn escape_for_display(s: &str) -> String {
        s.replace('\\', "\\\\")
         .replace('\n', "\\n")
         .replace('\r', "\\r")
         .replace('\t', "\\t")
    }

    /// Wrap text to maximum width (character count approximation).
    pub fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
        if text.is_empty() { return vec![]; }
        let mut lines = Vec::new();
        for line in text.lines() {
            if line.len() <= max_chars {
                lines.push(line.to_string());
            } else {
                let mut current = String::new();
                for word in line.split_whitespace() {
                    if current.len() + word.len() + 1 > max_chars {
                        if !current.is_empty() { lines.push(current.clone()); }
                        current = word.to_string();
                    } else {
                        if !current.is_empty() { current.push(' '); }
                        current.push_str(word);
                    }
                }
                if !current.is_empty() { lines.push(current); }
            }
        }
        lines
    }

    /// Count visible characters (skip escape sequences).
    pub fn visible_length(s: &str) -> usize {
        let mut count = 0;
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                chars.next(); // skip escaped char
            } else {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_spec() {
        let f = AisFontSpec::new("Arial", 12.0).with_bold().with_italic();
        assert_eq!(f.family, "Arial");
        assert_eq!(f.height, 12.0);
        assert!(f.bold);
        assert!(f.italic);
    }

    #[test]
    fn ais_string() {
        let mut s = AisString::new(1, "Hello World", [0.0, 0.0, 0.0]);
        assert_eq!(s.text_length(), 11);
        s.set_alignment(1);
        assert!(s.is_center_aligned());
        s.set_color(1.0, 0.0, 0.0);
        assert_eq!(s.color, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn formatter_escape() {
        let escaped = AisStringFormatter::escape_for_display("line1\nline2\ttab");
        assert!(escaped.contains("\\n"));
        assert!(escaped.contains("\\t"));
    }

    #[test]
    fn formatter_wrap() {
        let wrapped = AisStringFormatter::wrap_text("this is a long test string", 10);
        assert!(wrapped.len() > 1);
        for line in &wrapped { assert!(line.len() <= 10); }
    }

    #[test]
    fn formatter_visible_length() {
        let len = AisStringFormatter::visible_length("hello\\nworld");
        assert_eq!(len, 10); // counts the escaped \n as 1 visible char
    }
}
