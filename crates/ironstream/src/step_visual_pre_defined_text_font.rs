// FILE: step_visual_pre_defined_text_font.rs
// occt: StepVisual_PreDefinedTextFont

/// A pre-defined text font in STEP representation.
///
/// This represents a standard predefined text font.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PreDefinedTextFont {
    Courier,
    Arial,
    TimesNewRoman,
    Helvetica,
}

impl PreDefinedTextFont {
    /// Returns the font name as a string.
    pub fn name(&self) -> &str {
        match self {
            PreDefinedTextFont::Courier => "Courier",
            PreDefinedTextFont::Arial => "Arial",
            PreDefinedTextFont::TimesNewRoman => "TimesNewRoman",
            PreDefinedTextFont::Helvetica => "Helvetica",
        }
    }

    /// Creates a font from a name string.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "Courier" => Some(PreDefinedTextFont::Courier),
            "Arial" => Some(PreDefinedTextFont::Arial),
            "TimesNewRoman" => Some(PreDefinedTextFont::TimesNewRoman),
            "Helvetica" => Some(PreDefinedTextFont::Helvetica),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_defined_text_font_name() {
        assert_eq!(PreDefinedTextFont::Courier.name(), "Courier");
        assert_eq!(PreDefinedTextFont::Arial.name(), "Arial");
    }

    #[test]
    fn test_from_name() {
        assert_eq!(
            PreDefinedTextFont::from_name("Courier"),
            Some(PreDefinedTextFont::Courier)
        );
        assert_eq!(PreDefinedTextFont::from_name("Unknown"), None);
    }
}
