// FILE: step_visual_text_or_character.rs
// occt: StepVisual_TextOrCharacter

/// Represents a STEP TextOrCharacter select type.
/// Can be one of: AnnotationText, CompositeText, or TextLiteral.
pub enum TextOrCharacter {
    AnnotationText(AnnotationText),
    CompositeText(CompositeText),
    TextLiteral(TextLiteral),
}

/// Placeholder for AnnotationText
pub struct AnnotationText;

/// Placeholder for CompositeText
pub struct CompositeText;

/// Placeholder for TextLiteral
pub struct TextLiteral;

impl TextOrCharacter {
    /// Returns the case number (1=AnnotationText, 2=CompositeText, 3=TextLiteral, 0=none).
    pub fn case_num(&self) -> i32 {
        match self {
            TextOrCharacter::AnnotationText(_) => 1,
            TextOrCharacter::CompositeText(_) => 2,
            TextOrCharacter::TextLiteral(_) => 3,
        }
    }

    /// Returns the AnnotationText if this variant holds one.
    pub fn annotation_text(&self) -> Option<&AnnotationText> {
        match self {
            TextOrCharacter::AnnotationText(at) => Some(at),
            _ => None,
        }
    }

    /// Returns the CompositeText if this variant holds one.
    pub fn composite_text(&self) -> Option<&CompositeText> {
        match self {
            TextOrCharacter::CompositeText(ct) => Some(ct),
            _ => None,
        }
    }

    /// Returns the TextLiteral if this variant holds one.
    pub fn text_literal(&self) -> Option<&TextLiteral> {
        match self {
            TextOrCharacter::TextLiteral(tl) => Some(tl),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_annotation_text() {
        let toc = TextOrCharacter::AnnotationText(AnnotationText);
        assert_eq!(toc.case_num(), 1);
        assert!(toc.annotation_text().is_some());
        assert!(toc.composite_text().is_none());
        assert!(toc.text_literal().is_none());
    }

    #[test]
    fn test_composite_text() {
        let toc = TextOrCharacter::CompositeText(CompositeText);
        assert_eq!(toc.case_num(), 2);
        assert!(toc.annotation_text().is_none());
        assert!(toc.composite_text().is_some());
        assert!(toc.text_literal().is_none());
    }

    #[test]
    fn test_text_literal() {
        let toc = TextOrCharacter::TextLiteral(TextLiteral);
        assert_eq!(toc.case_num(), 3);
        assert!(toc.annotation_text().is_none());
        assert!(toc.composite_text().is_none());
        assert!(toc.text_literal().is_some());
    }
}
