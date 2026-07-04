// FILE: interface_param_type.rs
// occt: Interface_ParamType

/// Parameter type enumeration
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterfaceParamType {
    ParamMisc = 0,
    ParamInteger = 1,
    ParamReal = 2,
    ParamIdent = 3,
    ParamVoid = 4,
    ParamText = 5,
    ParamEnum = 6,
    ParamLogical = 7,
    ParamSub = 8,
    ParamHexa = 9,
    ParamBinary = 10,
}

impl InterfaceParamType {
    pub fn to_i32(self) -> i32 {
        self as i32
    }

    pub fn from_i32(val: i32) -> Option<Self> {
        match val {
            0 => Some(InterfaceParamType::ParamMisc),
            1 => Some(InterfaceParamType::ParamInteger),
            2 => Some(InterfaceParamType::ParamReal),
            3 => Some(InterfaceParamType::ParamIdent),
            4 => Some(InterfaceParamType::ParamVoid),
            5 => Some(InterfaceParamType::ParamText),
            6 => Some(InterfaceParamType::ParamEnum),
            7 => Some(InterfaceParamType::ParamLogical),
            8 => Some(InterfaceParamType::ParamSub),
            9 => Some(InterfaceParamType::ParamHexa),
            10 => Some(InterfaceParamType::ParamBinary),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_values() {
        assert_eq!(InterfaceParamType::ParamMisc.to_i32(), 0);
        assert_eq!(InterfaceParamType::ParamInteger.to_i32(), 1);
        assert_eq!(InterfaceParamType::ParamReal.to_i32(), 2);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(InterfaceParamType::from_i32(0), Some(InterfaceParamType::ParamMisc));
        assert_eq!(InterfaceParamType::from_i32(1), Some(InterfaceParamType::ParamInteger));
        assert_eq!(InterfaceParamType::from_i32(99), None);
    }

    #[test]
    fn test_comparison() {
        assert_eq!(InterfaceParamType::ParamInteger, InterfaceParamType::ParamInteger);
        assert_ne!(InterfaceParamType::ParamInteger, InterfaceParamType::ParamReal);
    }
}
