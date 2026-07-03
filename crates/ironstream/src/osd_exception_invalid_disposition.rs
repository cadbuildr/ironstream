// FILE: osd_exception_invalid_disposition.rs
// occt: OSD_Exception_INVALID_DISPOSITION

/// Invalid disposition exception.
#[derive(Debug, Clone)]
pub struct InvalidDispositionException;

impl std::fmt::Display for InvalidDispositionException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid Disposition")
    }
}

impl std::error::Error for InvalidDispositionException {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_disposition() {
        let exc = InvalidDispositionException;
        let s = format!("{}", exc);
        assert_eq!(s, "Invalid Disposition");
    }
}
