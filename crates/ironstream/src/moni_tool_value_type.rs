// FILE: moni_tool_value_type.rs
// occt: MoniTool_ValueType

/// Enumeration of value types supported in MoniTool data model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MoniToolValueType {
    /// Miscellaneous/unknown type
    Misc = 0,
    /// Integer value
    Integer = 1,
    /// Real (floating-point) value
    Real = 2,
    /// Identifier (symbol/name)
    Ident = 3,
    /// Void/empty value
    Void = 4,
    /// Text/string value
    Text = 5,
    /// Enumeration value
    Enum = 6,
    /// Logical (boolean) value
    Logical = 7,
    /// Sub-entity/structure
    Sub = 8,
    /// Hexadecimal value
    Hexa = 9,
    /// Binary value
    Binary = 10,
}

impl Default for MoniToolValueType {
    fn default() -> Self {
        Self::Misc
    }
}

impl MoniToolValueType {
    /// Returns the name of the value type as a string.
    pub fn name(self) -> &'static str {
        match self {
            Self::Misc => "Misc",
            Self::Integer => "Integer",
            Self::Real => "Real",
            Self::Ident => "Ident",
            Self::Void => "Void",
            Self::Text => "Text",
            Self::Enum => "Enum",
            Self::Logical => "Logical",
            Self::Sub => "Sub",
            Self::Hexa => "Hexa",
            Self::Binary => "Binary",
        }
    }

    /// Parses a string to a value type.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Misc" => Some(Self::Misc),
            "Integer" => Some(Self::Integer),
            "Real" => Some(Self::Real),
            "Ident" => Some(Self::Ident),
            "Void" => Some(Self::Void),
            "Text" => Some(Self::Text),
            "Enum" => Some(Self::Enum),
            "Logical" => Some(Self::Logical),
            "Sub" => Some(Self::Sub),
            "Hexa" => Some(Self::Hexa),
            "Binary" => Some(Self::Binary),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_type_default() {
        assert_eq!(MoniToolValueType::default(), MoniToolValueType::Misc);
    }

    #[test]
    fn test_value_type_names() {
        assert_eq!(MoniToolValueType::Misc.name(), "Misc");
        assert_eq!(MoniToolValueType::Integer.name(), "Integer");
        assert_eq!(MoniToolValueType::Real.name(), "Real");
        assert_eq!(MoniToolValueType::Text.name(), "Text");
        assert_eq!(MoniToolValueType::Logical.name(), "Logical");
        assert_eq!(MoniToolValueType::Binary.name(), "Binary");
    }

    #[test]
    fn test_value_type_from_str() {
        assert_eq!(MoniToolValueType::from_str("Integer"), Some(MoniToolValueType::Integer));
        assert_eq!(MoniToolValueType::from_str("Real"), Some(MoniToolValueType::Real));
        assert_eq!(MoniToolValueType::from_str("Text"), Some(MoniToolValueType::Text));
        assert_eq!(MoniToolValueType::from_str("Unknown"), None);
    }

    #[test]
    fn test_value_type_discriminants() {
        assert_eq!(MoniToolValueType::Misc as i32, 0);
        assert_eq!(MoniToolValueType::Integer as i32, 1);
        assert_eq!(MoniToolValueType::Real as i32, 2);
        assert_eq!(MoniToolValueType::Binary as i32, 10);
    }

    #[test]
    fn test_value_type_hash_and_eq() {
        let mut set = std::collections::HashSet::new();
        set.insert(MoniToolValueType::Integer);
        set.insert(MoniToolValueType::Integer); // duplicate
        assert_eq!(set.len(), 1);
        assert!(set.contains(&MoniToolValueType::Integer));
    }
}
