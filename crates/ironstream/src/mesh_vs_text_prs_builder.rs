// FILE: mesh_vs_text_prs_builder.rs
// occt: MeshVS_TextPrsBuilder

//! Builder for text presentation in mesh visualization.

#[derive(Clone, Debug)]
pub struct TextPrsBuilder {
    pub builder_id: u32,
    pub priority: i32,
    pub font_size: i32,
}

impl TextPrsBuilder {
    pub fn new(builder_id: u32, priority: i32) -> Self {
        TextPrsBuilder {
            builder_id,
            priority,
            font_size: 12,
        }
    }

    pub fn default_priority() -> i32 {
        20
    }

    pub fn with_font_size(mut self, size: i32) -> Self {
        self.font_size = size.clamp(6, 72);
        self
    }

    pub fn is_valid_font_size(size: i32) -> bool {
        size >= 6 && size <= 72
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_prs_builder_creation() {
        let builder = TextPrsBuilder::new(1, 20);
        assert_eq!(builder.builder_id, 1);
        assert_eq!(builder.priority, 20);
        assert_eq!(builder.font_size, 12);
    }

    #[test]
    fn text_prs_builder_default_priority() {
        assert_eq!(TextPrsBuilder::default_priority(), 20);
    }

    #[test]
    fn text_prs_builder_font_size() {
        let builder = TextPrsBuilder::new(1, 20)
            .with_font_size(24);

        assert_eq!(builder.font_size, 24);
        assert!(TextPrsBuilder::is_valid_font_size(24));
    }
}
