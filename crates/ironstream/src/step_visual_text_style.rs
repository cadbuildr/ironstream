// FILE: step_visual_text_style.rs
// occt: StepVisual_TextStyle

/// Represents a STEP TextStyle entity.
pub struct TextStyle {
    name: String,
    character_appearance: Option<TextStyleForDefinedFont>,
}

/// Placeholder for TextStyleForDefinedFont
pub struct TextStyleForDefinedFont;

impl TextStyle {
    /// Creates a new text style.
    pub fn new() -> Self {
        TextStyle {
            name: String::new(),
            character_appearance: None,
        }
    }

    /// Initializes all fields.
    pub fn init(&mut self, name: String, character_appearance: Option<TextStyleForDefinedFont>) {
        self.name = name;
        self.character_appearance = character_appearance;
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns the character appearance.
    pub fn character_appearance(&self) -> Option<&TextStyleForDefinedFont> {
        self.character_appearance.as_ref()
    }

    /// Sets the character appearance.
    pub fn set_character_appearance(&mut self, appearance: TextStyleForDefinedFont) {
        self.character_appearance = Some(appearance);
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ts = TextStyle::new();
        assert_eq!(ts.name(), "");
        assert!(ts.character_appearance().is_none());
    }

    #[test]
    fn test_init() {
        let mut ts = TextStyle::new();
        ts.init("MyStyle".to_string(), None);
        assert_eq!(ts.name(), "MyStyle");
    }

    #[test]
    fn test_character_appearance() {
        let mut ts = TextStyle::new();
        let appearance = TextStyleForDefinedFont;
        ts.set_character_appearance(appearance);
        assert!(ts.character_appearance().is_some());
    }
}
