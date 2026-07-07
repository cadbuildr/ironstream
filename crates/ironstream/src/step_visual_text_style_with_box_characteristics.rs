// FILE: step_visual_text_style_with_box_characteristics.rs
// occt: StepVisual_TextStyleWithBoxCharacteristics

/// Represents a STEP TextStyleWithBoxCharacteristics entity.
pub struct TextStyleWithBoxCharacteristics {
    name: String,
    character_appearance: Option<TextStyleForDefinedFont>,
    characteristics: Vec<BoxCharacteristicSelect>,
}

/// Placeholder for TextStyleForDefinedFont
pub struct TextStyleForDefinedFont;

/// Placeholder for BoxCharacteristicSelect
pub struct BoxCharacteristicSelect;

impl TextStyleWithBoxCharacteristics {
    /// Creates a new text style with box characteristics.
    pub fn new() -> Self {
        TextStyleWithBoxCharacteristics {
            name: String::new(),
            character_appearance: None,
            characteristics: Vec::new(),
        }
    }

    /// Initializes all fields.
    pub fn init(
        &mut self,
        name: String,
        character_appearance: Option<TextStyleForDefinedFont>,
        characteristics: Vec<BoxCharacteristicSelect>,
    ) {
        self.name = name;
        self.character_appearance = character_appearance;
        self.characteristics = characteristics;
    }

    /// Returns the characteristics.
    pub fn characteristics(&self) -> &[BoxCharacteristicSelect] {
        &self.characteristics
    }

    /// Sets the characteristics.
    pub fn set_characteristics(&mut self, characteristics: Vec<BoxCharacteristicSelect>) {
        self.characteristics = characteristics;
    }

    /// Returns the characteristic at the given index.
    pub fn characteristics_value(&self, idx: usize) -> Option<&BoxCharacteristicSelect> {
        self.characteristics.get(idx)
    }

    /// Returns the number of characteristics.
    pub fn nb_characteristics(&self) -> usize {
        self.characteristics.len()
    }

    /// Returns the character appearance.
    pub fn character_appearance(&self) -> Option<&TextStyleForDefinedFont> {
        self.character_appearance.as_ref()
    }
}

impl Default for TextStyleWithBoxCharacteristics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tswbc = TextStyleWithBoxCharacteristics::new();
        assert_eq!(tswbc.nb_characteristics(), 0);
    }

    #[test]
    fn test_characteristics() {
        let mut tswbc = TextStyleWithBoxCharacteristics::new();
        let chars = vec![];
        tswbc.set_characteristics(chars);
        assert_eq!(tswbc.nb_characteristics(), 0);
    }
}
