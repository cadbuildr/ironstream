// FILE: step_visual_draughting_pre_defined_colour.rs
// occt: StepVisual_DraughtingPreDefinedColour

/// A draughting pre-defined colour in STEP representation.
///
/// This represents a standard predefined colour for draughting.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DraughtingPreDefinedColour {
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    White,
}

impl DraughtingPreDefinedColour {
    /// Returns the colour name as a string.
    pub fn name(&self) -> &str {
        match self {
            DraughtingPreDefinedColour::Black => "Black",
            DraughtingPreDefinedColour::Red => "Red",
            DraughtingPreDefinedColour::Green => "Green",
            DraughtingPreDefinedColour::Blue => "Blue",
            DraughtingPreDefinedColour::Yellow => "Yellow",
            DraughtingPreDefinedColour::Magenta => "Magenta",
            DraughtingPreDefinedColour::Cyan => "Cyan",
            DraughtingPreDefinedColour::White => "White",
        }
    }

    /// Creates a colour from a name string.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "Black" => Some(DraughtingPreDefinedColour::Black),
            "Red" => Some(DraughtingPreDefinedColour::Red),
            "Green" => Some(DraughtingPreDefinedColour::Green),
            "Blue" => Some(DraughtingPreDefinedColour::Blue),
            "Yellow" => Some(DraughtingPreDefinedColour::Yellow),
            "Magenta" => Some(DraughtingPreDefinedColour::Magenta),
            "Cyan" => Some(DraughtingPreDefinedColour::Cyan),
            "White" => Some(DraughtingPreDefinedColour::White),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draughting_pre_defined_colour_name() {
        assert_eq!(DraughtingPreDefinedColour::Black.name(), "Black");
        assert_eq!(DraughtingPreDefinedColour::Red.name(), "Red");
        assert_eq!(DraughtingPreDefinedColour::Green.name(), "Green");
    }

    #[test]
    fn test_from_name() {
        assert_eq!(
            DraughtingPreDefinedColour::from_name("Black"),
            Some(DraughtingPreDefinedColour::Black)
        );
        assert_eq!(
            DraughtingPreDefinedColour::from_name("Red"),
            Some(DraughtingPreDefinedColour::Red)
        );
        assert_eq!(DraughtingPreDefinedColour::from_name("Unknown"), None);
    }
}
