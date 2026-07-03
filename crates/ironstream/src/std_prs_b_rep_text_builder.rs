// FILE: std_prs_b_rep_text_builder.rs
// occt: StdPrs_BRepTextBuilder

/// Builder for creating BRep text shapes from font glyphs.
#[derive(Clone, Debug)]
pub struct BRepTextBuilder {
    pub builder_id: u32,
    pub glyph_count: usize,
}

impl BRepTextBuilder {
    pub fn new(builder_id: u32) -> Self {
        Self {
            builder_id,
            glyph_count: 0,
        }
    }

    pub fn add_glyph(&mut self) {
        self.glyph_count += 1;
    }

    pub fn nb_glyphs(&self) -> usize {
        self.glyph_count
    }

    pub fn clear(&mut self) {
        self.glyph_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_builder() {
        let builder = BRepTextBuilder::new(1);
        assert_eq!(builder.builder_id, 1);
        assert_eq!(builder.nb_glyphs(), 0);
    }

    #[test]
    fn add_glyphs() {
        let mut builder = BRepTextBuilder::new(1);
        builder.add_glyph();
        builder.add_glyph();
        builder.add_glyph();
        assert_eq!(builder.nb_glyphs(), 3);
    }

    #[test]
    fn clear() {
        let mut builder = BRepTextBuilder::new(1);
        builder.add_glyph();
        builder.clear();
        assert_eq!(builder.nb_glyphs(), 0);
    }
}
