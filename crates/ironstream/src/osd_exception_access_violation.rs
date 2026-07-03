// FILE: osd_exception_access_violation.rs
// occt: OSD_Exception_ACCESS_VIOLATION

/// Access violation exception.
#[derive(Debug, Clone)]
pub struct AccessViolationException;

impl std::fmt::Display for AccessViolationException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Access Violation")
    }
}

impl std::error::Error for AccessViolationException {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_violation() {
        let exc = AccessViolationException;
        let s = format!("{}", exc);
        assert_eq!(s, "Access Violation");
    }
}
