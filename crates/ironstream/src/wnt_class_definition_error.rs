// FILE: wnt_class_definition_error.rs
// occt: WNT_ClassDefinitionError

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassDefinitionError {
    Success = 0,
    ClassNameTooLong = 1,
    InvalidClassName = 2,
    ClassAlreadyRegistered = 3,
    RegistrationFailed = 4,
}

impl ClassDefinitionError {
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            0 => Some(ClassDefinitionError::Success),
            1 => Some(ClassDefinitionError::ClassNameTooLong),
            2 => Some(ClassDefinitionError::InvalidClassName),
            3 => Some(ClassDefinitionError::ClassAlreadyRegistered),
            4 => Some(ClassDefinitionError::RegistrationFailed),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        assert_eq!(ClassDefinitionError::Success as i32, 0);
    }
}
