// FILE: osd_exception_noncontinuable_exception.rs
// occt: OSD_Exception_NONCONTINUABLE_EXCEPTION

/// Non-continuable exception.
#[derive(Debug, Clone)]
pub struct NoncontinuableException;

impl std::fmt::Display for NoncontinuableException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Non-Continuable Exception")
    }
}

impl std::error::Error for NoncontinuableException {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noncontinuable() {
        let exc = NoncontinuableException;
        let s = format!("{}", exc);
        assert_eq!(s, "Non-Continuable Exception");
    }
}
