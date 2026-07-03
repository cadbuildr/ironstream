// FILE: osd_exception_in_page_error.rs
// occt: OSD_Exception_IN_PAGE_ERROR

/// In-page error exception.
#[derive(Debug, Clone)]
pub struct InPageErrorException;

impl std::fmt::Display for InPageErrorException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "In-Page Error")
    }
}

impl std::error::Error for InPageErrorException {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_page_error() {
        let exc = InPageErrorException;
        let s = format!("{}", exc);
        assert_eq!(s, "In-Page Error");
    }
}
