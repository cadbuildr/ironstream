// FILE: step_visual_pre_defined_curve_font.rs
// occt: StepVisual_PreDefinedCurveFont

/// A pre-defined curve font in STEP representation.
///
/// This represents a standard predefined curve font.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PreDefinedCurveFont {
    Continuous,
    Dashed,
    DotDashed,
    Dotted,
    DoubleChain,
}

impl PreDefinedCurveFont {
    /// Returns the font name as a string.
    pub fn name(&self) -> &str {
        match self {
            PreDefinedCurveFont::Continuous => "Continuous",
            PreDefinedCurveFont::Dashed => "Dashed",
            PreDefinedCurveFont::DotDashed => "DotDashed",
            PreDefinedCurveFont::Dotted => "Dotted",
            PreDefinedCurveFont::DoubleChain => "DoubleChain",
        }
    }

    /// Creates a font from a name string.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "Continuous" => Some(PreDefinedCurveFont::Continuous),
            "Dashed" => Some(PreDefinedCurveFont::Dashed),
            "DotDashed" => Some(PreDefinedCurveFont::DotDashed),
            "Dotted" => Some(PreDefinedCurveFont::Dotted),
            "DoubleChain" => Some(PreDefinedCurveFont::DoubleChain),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_defined_curve_font_name() {
        assert_eq!(PreDefinedCurveFont::Continuous.name(), "Continuous");
        assert_eq!(PreDefinedCurveFont::Dashed.name(), "Dashed");
    }

    #[test]
    fn test_from_name() {
        assert_eq!(
            PreDefinedCurveFont::from_name("Continuous"),
            Some(PreDefinedCurveFont::Continuous)
        );
        assert_eq!(PreDefinedCurveFont::from_name("Unknown"), None);
    }
}
