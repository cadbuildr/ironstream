// FILE: step_visual_null_style.rs
// occt: StepVisual_NullStyle

/// A null style in STEP representation.
///
/// This represents the absence of a style.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NullStyle;

impl NullStyle {
    /// Creates a new null style.
    pub fn new() -> Self {
        NullStyle
    }
}

impl Default for NullStyle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_style_new() {
        let _style = NullStyle::new();
    }

    #[test]
    fn test_null_style_default() {
        let _style = NullStyle::default();
    }

    #[test]
    fn test_null_style_equality() {
        let s1 = NullStyle::new();
        let s2 = NullStyle::new();
        assert_eq!(s1, s2);
    }
}
