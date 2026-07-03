// FILE: contap_i_type.rs
// occt: Contap_IType

/// Enumeration of contour line types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContapIType {
    Lin,
    Circle,
    Walking,
    Restriction,
}

impl ContapIType {
    /// Convert to integer code
    pub fn to_code(&self) -> i32 {
        match self {
            ContapIType::Lin => 0,
            ContapIType::Circle => 1,
            ContapIType::Walking => 2,
            ContapIType::Restriction => 3,
        }
    }

    /// Convert from integer code
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            0 => Some(ContapIType::Lin),
            1 => Some(ContapIType::Circle),
            2 => Some(ContapIType::Walking),
            3 => Some(ContapIType::Restriction),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContapIType;

    #[test]
    fn test_to_code() {
        assert_eq!(ContapIType::Lin.to_code(), 0);
        assert_eq!(ContapIType::Circle.to_code(), 1);
        assert_eq!(ContapIType::Walking.to_code(), 2);
        assert_eq!(ContapIType::Restriction.to_code(), 3);
    }

    #[test]
    fn test_from_code() {
        assert_eq!(ContapIType::from_code(0), Some(ContapIType::Lin));
        assert_eq!(ContapIType::from_code(1), Some(ContapIType::Circle));
        assert_eq!(ContapIType::from_code(4), None);
    }
}
