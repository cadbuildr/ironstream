// FILE: std_prs_b_rep_font.rs
// occt: StdPrs_BRepFont

/// Font renderer that converts text glyphs to BRep shapes.
#[derive(Clone, Debug)]
pub struct BRepFont {
    pub font_id: u32,
    pub font_name: String,
    pub font_size: f64,
    pub is_loaded: bool,
}

impl BRepFont {
    pub fn new(font_id: u32, font_name: String, size: f64) -> Self {
        Self {
            font_id,
            font_name,
            font_size: size.max(0.1),
            is_loaded: false,
        }
    }

    pub fn load(&mut self) -> bool {
        self.is_loaded = true;
        true
    }

    pub fn release(&mut self) {
        self.is_loaded = false;
    }

    pub fn is_loaded(&self) -> bool {
        self.is_loaded
    }

    pub fn font_size(&self) -> f64 {
        self.font_size
    }

    pub fn set_font_size(&mut self, size: f64) {
        self.font_size = size.max(0.1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_font() {
        let font = BRepFont::new(1, "Arial".to_string(), 12.0);
        assert_eq!(font.font_id, 1);
        assert_eq!(font.font_name, "Arial");
        assert!(!font.is_loaded());
    }

    #[test]
    fn load_release() {
        let mut font = BRepFont::new(1, "Arial".to_string(), 12.0);
        assert!(font.load());
        assert!(font.is_loaded());
        font.release();
        assert!(!font.is_loaded());
    }

    #[test]
    fn set_font_size() {
        let mut font = BRepFont::new(1, "Arial".to_string(), 12.0);
        font.set_font_size(24.0);
        assert_eq!(font.font_size(), 24.0);
    }
}
