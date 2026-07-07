// FILE: step_visual_draughting_pre_defined_curve_font.rs
// occt: StepVisual_DraughtingPreDefinedCurveFont

/// A draughting pre-defined curve font in STEP representation.
///
/// This represents a standard predefined curve font for draughting.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DraughtingPreDefinedCurveFont {
    Continuous,
    Dashed,
    DotDashed,
    Dotted,
    DoubleChain,
}

impl DraughtingPreDefinedCurveFont {
    /// Returns the font name as a string.
    pub fn name(&self) -> &str {
        match self {
            DraughtingPreDefinedCurveFont::Continuous => "Continuous",
            DraughtingPreDefinedCurveFont::Dashed => "Dashed",
            DraughtingPreDefinedCurveFont::DotDashed => "DotDashed",
            DraughtingPreDefinedCurveFont::Dotted => "Dotted",
            DraughtingPreDefinedCurveFont::DoubleChain => "DoubleChain",
        }
    }

    /// Creates a font from a name string.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "Continuous" => Some(DraughtingPreDefinedCurveFont::Continuous),
            "Dashed" => Some(DraughtingPreDefinedCurveFont::Dashed),
            "DotDashed" => Some(DraughtingPreDefinedCurveFont::DotDashed),
            "Dotted" => Some(DraughtingPreDefinedCurveFont::Dotted),
            "DoubleChain" => Some(DraughtingPreDefinedCurveFont::DoubleChain),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draughting_pre_defined_curve_font_name() {
        assert_eq!(
            DraughtingPreDefinedCurveFont::Continuous.name(),
            "Continuous"
        );
        assert_eq!(DraughtingPreDefinedCurveFont::Dashed.name(), "Dashed");
    }

    #[test]
    fn test_from_name() {
        assert_eq!(
            DraughtingPreDefinedCurveFont::from_name("Continuous"),
            Some(DraughtingPreDefinedCurveFont::Continuous)
        );
        assert_eq!(
            DraughtingPreDefinedCurveFont::from_name("Dashed"),
            Some(DraughtingPreDefinedCurveFont::Dashed)
        );
        assert_eq!(
            DraughtingPreDefinedCurveFont::from_name("Unknown"),
            None
        );
    }
}
