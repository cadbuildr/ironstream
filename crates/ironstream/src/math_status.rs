// FILE: math_status.rs
// occt: math_Status

/// Status of mathematical computation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    OK,
    NotConverged,
    MaxIterations,
    NumericalError,
    InvalidInput,
    SingularMatrix,
    OutOfRange,
}

impl Status {
    pub fn is_ok(&self) -> bool {
        *self == Status::OK
    }

    pub fn is_error(&self) -> bool {
        !self.is_ok()
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::OK => write!(f, "OK"),
            Status::NotConverged => write!(f, "Not converged"),
            Status::MaxIterations => write!(f, "Max iterations reached"),
            Status::NumericalError => write!(f, "Numerical error"),
            Status::InvalidInput => write!(f, "Invalid input"),
            Status::SingularMatrix => write!(f, "Singular matrix"),
            Status::OutOfRange => write!(f, "Out of range"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_ok() {
        let s = Status::OK;
        assert!(s.is_ok());
        assert!(!s.is_error());
    }

    #[test]
    fn test_status_error() {
        let s = Status::NotConverged;
        assert!(!s.is_ok());
        assert!(s.is_error());
    }

    #[test]
    fn test_status_display() {
        assert_eq!(format!("{}", Status::OK), "OK");
        assert_eq!(format!("{}", Status::NotConverged), "Not converged");
    }
}
