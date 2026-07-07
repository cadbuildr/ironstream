// FILE: step_visual_text_literal.rs
// occt: StepVisual_TextLiteral

/// Represents a STEP TextLiteral entity.
pub struct TextLiteral {
    name: String,
    literal: String,
    placement: Axis2Placement,
    alignment: String,
    path: TextPath,
    font: FontSelect,
}

/// Placeholder for Axis2Placement
#[derive(Clone)]
pub struct Axis2Placement;

/// Placeholder for FontSelect
#[derive(Clone)]
pub struct FontSelect;

/// Text path enumeration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextPath {
    Up,
    Right,
    Down,
    Left,
}

impl TextLiteral {
    /// Creates a new text literal.
    pub fn new() -> Self {
        TextLiteral {
            name: String::new(),
            literal: String::new(),
            placement: Axis2Placement,
            alignment: String::new(),
            path: TextPath::Up,
            font: FontSelect,
        }
    }

    /// Initializes all fields.
    pub fn init(
        &mut self,
        name: String,
        literal: String,
        placement: Axis2Placement,
        alignment: String,
        path: TextPath,
        font: FontSelect,
    ) {
        self.name = name;
        self.literal = literal;
        self.placement = placement;
        self.alignment = alignment;
        self.path = path;
        self.font = font;
    }

    /// Returns the literal.
    pub fn literal(&self) -> &str {
        &self.literal
    }

    /// Sets the literal.
    pub fn set_literal(&mut self, literal: String) {
        self.literal = literal;
    }

    /// Returns the placement.
    pub fn placement(&self) -> &Axis2Placement {
        &self.placement
    }

    /// Sets the placement.
    pub fn set_placement(&mut self, placement: Axis2Placement) {
        self.placement = placement;
    }

    /// Returns the alignment.
    pub fn alignment(&self) -> &str {
        &self.alignment
    }

    /// Sets the alignment.
    pub fn set_alignment(&mut self, alignment: String) {
        self.alignment = alignment;
    }

    /// Returns the path.
    pub fn path(&self) -> TextPath {
        self.path
    }

    /// Sets the path.
    pub fn set_path(&mut self, path: TextPath) {
        self.path = path;
    }

    /// Returns the font.
    pub fn font(&self) -> &FontSelect {
        &self.font
    }

    /// Sets the font.
    pub fn set_font(&mut self, font: FontSelect) {
        self.font = font;
    }
}

impl Default for TextLiteral {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tl = TextLiteral::new();
        assert_eq!(tl.literal(), "");
        assert_eq!(tl.alignment(), "");
        assert_eq!(tl.path(), TextPath::Up);
    }

    #[test]
    fn test_init() {
        let mut tl = TextLiteral::new();
        tl.init(
            "TextItem".to_string(),
            "Hello".to_string(),
            Axis2Placement,
            "Center".to_string(),
            TextPath::Right,
            FontSelect,
        );
        assert_eq!(tl.literal(), "Hello");
        assert_eq!(tl.alignment(), "Center");
        assert_eq!(tl.path(), TextPath::Right);
    }

    #[test]
    fn test_literal() {
        let mut tl = TextLiteral::new();
        tl.set_literal("Test".to_string());
        assert_eq!(tl.literal(), "Test");
    }

    #[test]
    fn test_path() {
        let mut tl = TextLiteral::new();
        tl.set_path(TextPath::Down);
        assert_eq!(tl.path(), TextPath::Down);
    }
}
