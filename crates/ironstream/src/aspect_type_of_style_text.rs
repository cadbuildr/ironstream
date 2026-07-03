// FILE: aspect_type_of_style_text.rs
// occt: Aspect_TypeOfStyleText

/// Defines the style of text rendering in a view.
///
/// This enum specifies how text should be displayed:
/// - `Normal`: Default text displayed like any other graphic object,
///   can be hidden by closer objects
/// - `Annotation`: Text always visible, displayed over other objects
///   according to priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AspectTypeOfStyleText {
    /// Default text. Can be hidden by closer objects.
    Normal,
    /// Annotation text. Always visible, displayed over other objects.
    Annotation,
}

impl AspectTypeOfStyleText {
    /// Returns the OCCT integer representation for this style.
    pub fn to_occt_int(self) -> i32 {
        match self {
            AspectTypeOfStyleText::Normal => 0,
            AspectTypeOfStyleText::Annotation => 1,
        }
    }

    /// Creates an instance from an OCCT integer representation.
    pub fn from_occt_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(AspectTypeOfStyleText::Normal),
            1 => Some(AspectTypeOfStyleText::Annotation),
            _ => None,
        }
    }
}

impl Default for AspectTypeOfStyleText {
    fn default() -> Self {
        AspectTypeOfStyleText::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_variants() {
        let normal = AspectTypeOfStyleText::Normal;
        let annotation = AspectTypeOfStyleText::Annotation;
        assert_ne!(normal, annotation);
    }

    #[test]
    fn test_to_occt_int() {
        assert_eq!(AspectTypeOfStyleText::Normal.to_occt_int(), 0);
        assert_eq!(AspectTypeOfStyleText::Annotation.to_occt_int(), 1);
    }

    #[test]
    fn test_from_occt_int_normal() {
        let result = AspectTypeOfStyleText::from_occt_int(0);
        assert_eq!(result, Some(AspectTypeOfStyleText::Normal));
    }

    #[test]
    fn test_from_occt_int_annotation() {
        let result = AspectTypeOfStyleText::from_occt_int(1);
        assert_eq!(result, Some(AspectTypeOfStyleText::Annotation));
    }

    #[test]
    fn test_from_occt_int_invalid() {
        let result = AspectTypeOfStyleText::from_occt_int(2);
        assert_eq!(result, None);
        let result = AspectTypeOfStyleText::from_occt_int(-1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_default() {
        assert_eq!(AspectTypeOfStyleText::default(), AspectTypeOfStyleText::Normal);
    }

    #[test]
    fn test_round_trip_conversion() {
        for original in &[AspectTypeOfStyleText::Normal, AspectTypeOfStyleText::Annotation] {
            let int_val = original.to_occt_int();
            let restored = AspectTypeOfStyleText::from_occt_int(int_val);
            assert_eq!(restored, Some(*original));
        }
    }

    #[test]
    fn test_clone_and_copy() {
        let original = AspectTypeOfStyleText::Annotation;
        let cloned = original.clone();
        let copied = original;
        assert_eq!(cloned, original);
        assert_eq!(copied, original);
    }
}
