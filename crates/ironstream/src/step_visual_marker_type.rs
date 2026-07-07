// FILE: step_visual_marker_type.rs
// occt: StepVisual_MarkerType

/// A marker type enumeration in STEP representation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum MarkerType {
    Dot,
    X,
    Plus,
    Asterisk,
    Ring,
    Square,
    Triangle,
}

impl MarkerType {
    /// Returns the marker type as a string.
    pub fn as_str(&self) -> &str {
        match self {
            MarkerType::Dot => "Dot",
            MarkerType::X => "X",
            MarkerType::Plus => "Plus",
            MarkerType::Asterisk => "Asterisk",
            MarkerType::Ring => "Ring",
            MarkerType::Square => "Square",
            MarkerType::Triangle => "Triangle",
        }
    }

    /// Creates a marker type from a string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Dot" => Some(MarkerType::Dot),
            "X" => Some(MarkerType::X),
            "Plus" => Some(MarkerType::Plus),
            "Asterisk" => Some(MarkerType::Asterisk),
            "Ring" => Some(MarkerType::Ring),
            "Square" => Some(MarkerType::Square),
            "Triangle" => Some(MarkerType::Triangle),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_type_as_str() {
        assert_eq!(MarkerType::Dot.as_str(), "Dot");
        assert_eq!(MarkerType::X.as_str(), "X");
        assert_eq!(MarkerType::Plus.as_str(), "Plus");
    }

    #[test]
    fn test_marker_type_from_str() {
        assert_eq!(MarkerType::from_str("Dot"), Some(MarkerType::Dot));
        assert_eq!(MarkerType::from_str("X"), Some(MarkerType::X));
        assert_eq!(MarkerType::from_str("Unknown"), None);
    }
}
