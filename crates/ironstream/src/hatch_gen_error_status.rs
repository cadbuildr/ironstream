// FILE: hatch_gen_error_status.rs
// occt: HatchGen_ErrorStatus

/// Error status enumeration for hatch generation operations
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HatchGenErrorStatus {
    /// No problem
    NoProblem = 0,
    /// Trim operation failed
    TrimFailure = 1,
    /// Transition operation failed
    TransitionFailure = 2,
    /// Incoherent parity detected
    IncoherentParity = 3,
    /// Incompatible states
    IncompatibleStates = 4,
}

impl HatchGenErrorStatus {
    /// Convert from integer representation
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(HatchGenErrorStatus::NoProblem),
            1 => Some(HatchGenErrorStatus::TrimFailure),
            2 => Some(HatchGenErrorStatus::TransitionFailure),
            3 => Some(HatchGenErrorStatus::IncoherentParity),
            4 => Some(HatchGenErrorStatus::IncompatibleStates),
            _ => None,
        }
    }

    /// Convert to integer representation
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_values() {
        assert_eq!(HatchGenErrorStatus::NoProblem as i32, 0);
        assert_eq!(HatchGenErrorStatus::TrimFailure as i32, 1);
        assert_eq!(HatchGenErrorStatus::TransitionFailure as i32, 2);
        assert_eq!(HatchGenErrorStatus::IncoherentParity as i32, 3);
        assert_eq!(HatchGenErrorStatus::IncompatibleStates as i32, 4);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(
            HatchGenErrorStatus::from_i32(0),
            Some(HatchGenErrorStatus::NoProblem)
        );
        assert_eq!(
            HatchGenErrorStatus::from_i32(1),
            Some(HatchGenErrorStatus::TrimFailure)
        );
        assert_eq!(HatchGenErrorStatus::from_i32(5), None);
    }

    #[test]
    fn test_as_i32() {
        assert_eq!(HatchGenErrorStatus::NoProblem.as_i32(), 0);
        assert_eq!(HatchGenErrorStatus::TrimFailure.as_i32(), 1);
    }

    #[test]
    fn test_clone_copy() {
        let status = HatchGenErrorStatus::TrimFailure;
        let status2 = status;
        assert_eq!(status, status2);
    }
}
