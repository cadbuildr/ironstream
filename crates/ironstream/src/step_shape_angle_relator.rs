// FILE: step_shape_angle_relator.rs
// occt: StepShape_AngleRelator

/// Enumeration for angle relator types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AngleRelator {
    /// Equal angle
    Equal,
    /// Large angle
    Large,
    /// Small angle
    Small,
}

impl AngleRelator {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            AngleRelator::Equal => "Equal",
            AngleRelator::Large => "Large",
            AngleRelator::Small => "Small",
        }
    }

    /// Try to parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Equal" => Some(AngleRelator::Equal),
            "Large" => Some(AngleRelator::Large),
            "Small" => Some(AngleRelator::Small),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_relator_equal() {
        assert_eq!(AngleRelator::Equal.as_str(), "Equal");
    }

    #[test]
    fn test_angle_relator_large() {
        assert_eq!(AngleRelator::Large.as_str(), "Large");
    }

    #[test]
    fn test_angle_relator_small() {
        assert_eq!(AngleRelator::Small.as_str(), "Small");
    }

    #[test]
    fn test_from_str_equal() {
        assert_eq!(AngleRelator::from_str("Equal"), Some(AngleRelator::Equal));
    }

    #[test]
    fn test_from_str_large() {
        assert_eq!(AngleRelator::from_str("Large"), Some(AngleRelator::Large));
    }

    #[test]
    fn test_from_str_small() {
        assert_eq!(AngleRelator::from_str("Small"), Some(AngleRelator::Small));
    }

    #[test]
    fn test_from_str_invalid() {
        assert_eq!(AngleRelator::from_str("Invalid"), None);
    }

    #[test]
    fn test_equality() {
        assert_eq!(AngleRelator::Equal, AngleRelator::Equal);
        assert_ne!(AngleRelator::Equal, AngleRelator::Large);
    }
}
