// FILE: loc_ope_operation.rs
// occt: LocOpe_Operation

/// Enumeration of local operation types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocOpeOperation {
    /// Fusion operation
    Fuse = 0,
    /// Cut operation
    Cut = 1,
    /// Invalid operation
    Invalid = 2,
}

impl LocOpeOperation {
    /// Converts an integer to an operation type.
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(LocOpeOperation::Fuse),
            1 => Some(LocOpeOperation::Cut),
            2 => Some(LocOpeOperation::Invalid),
            _ => None,
        }
    }

    /// Converts an operation type to an integer.
    pub fn to_int(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_enum() {
        assert_eq!(LocOpeOperation::Fuse.to_int(), 0);
        assert_eq!(LocOpeOperation::Cut.to_int(), 1);
        assert_eq!(LocOpeOperation::Invalid.to_int(), 2);
    }

    #[test]
    fn test_operation_from_int() {
        assert_eq!(
            LocOpeOperation::from_int(0),
            Some(LocOpeOperation::Fuse)
        );
        assert_eq!(
            LocOpeOperation::from_int(3),
            None
        );
    }
}
