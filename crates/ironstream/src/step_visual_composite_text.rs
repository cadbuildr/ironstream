// FILE: step_visual_composite_text.rs
// occt: StepVisual_CompositeText

/// Text or character value in composite text.
#[derive(Clone, Debug, PartialEq)]
pub enum TextOrCharacter {
    Text(String),
    Marker(i32),
}

/// A composite text in STEP representation.
///
/// This represents a composite text element composed of text and character items.
pub struct CompositeText {
    name: String,
    collected_text: Vec<TextOrCharacter>,
}

impl CompositeText {
    /// Creates a new composite text.
    pub fn new(name: String) -> Self {
        CompositeText {
            name,
            collected_text: Vec::new(),
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the collected text items.
    pub fn set_collected_text(&mut self, items: Vec<TextOrCharacter>) {
        self.collected_text = items;
    }

    /// Returns the collected text items.
    pub fn collected_text(&self) -> &[TextOrCharacter] {
        &self.collected_text
    }

    /// Returns the collected text value at the given index (1-based).
    pub fn collected_text_value(&self, index: usize) -> Option<&TextOrCharacter> {
        if index > 0 && index <= self.collected_text.len() {
            Some(&self.collected_text[index - 1])
        } else {
            None
        }
    }

    /// Returns the number of collected text items.
    pub fn nb_collected_text(&self) -> usize {
        self.collected_text.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_text_new() {
        let text = CompositeText::new("MyText".to_string());
        assert_eq!(text.name(), "MyText");
        assert_eq!(text.nb_collected_text(), 0);
    }

    #[test]
    fn test_collected_text_operations() {
        let mut text = CompositeText::new("CompositeLabel".to_string());
        let items = vec![
            TextOrCharacter::Text("Part".to_string()),
            TextOrCharacter::Text(" A".to_string()),
            TextOrCharacter::Marker(1),
        ];
        text.set_collected_text(items);
        assert_eq!(text.nb_collected_text(), 3);
        assert_eq!(
            text.collected_text_value(1),
            Some(&TextOrCharacter::Text("Part".to_string()))
        );
        assert_eq!(
            text.collected_text_value(2),
            Some(&TextOrCharacter::Text(" A".to_string()))
        );
        assert_eq!(text.collected_text_value(3), Some(&TextOrCharacter::Marker(1)));
        assert_eq!(text.collected_text_value(4), None);
    }
}
