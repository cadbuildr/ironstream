// FILE: step_visual_pre_defined_colour.rs
// occt: StepVisual_PreDefinedColour

/// A pre-defined colour in STEP representation.
///
/// This represents a standard predefined colour.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PreDefinedColour {
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    White,
}

impl PreDefinedColour {
    /// Returns the colour name as a string.
    pub fn name(&self) -> &str {
        match self {
            PreDefinedColour::Black => "Black",
            PreDefinedColour::Red => "Red",
            PreDefinedColour::Green => "Green",
            PreDefinedColour::Blue => "Blue",
            PreDefinedColour::Yellow => "Yellow",
            PreDefinedColour::Magenta => "Magenta",
            PreDefinedColour::Cyan => "Cyan",
            PreDefinedColour::White => "White",
        }
    }

    /// Creates a colour from a name string.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "Black" => Some(PreDefinedColour::Black),
            "Red" => Some(PreDefinedColour::Red),
            "Green" => Some(PreDefinedColour::Green),
            "Blue" => Some(PreDefinedColour::Blue),
            "Yellow" => Some(PreDefinedColour::Yellow),
            "Magenta" => Some(PreDefinedColour::Magenta),
            "Cyan" => Some(PreDefinedColour::Cyan),
            "White" => Some(PreDefinedColour::White),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pre_defined_colour_name() {
        assert_eq!(PreDefinedColour::Black.name(), "Black");
        assert_eq!(PreDefinedColour::Red.name(), "Red");
    }

    #[test]
    fn test_from_name() {
        assert_eq!(
            PreDefinedColour::from_name("Black"),
            Some(PreDefinedColour::Black)
        );
        assert_eq!(PreDefinedColour::from_name("Unknown"), None);
    }
}
