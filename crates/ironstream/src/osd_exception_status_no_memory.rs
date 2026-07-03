// FILE: osd_exception_status_no_memory.rs
// occt: OSD_Exception_STATUS_NO_MEMORY

/// Out of memory exception.
#[derive(Debug, Clone)]
pub struct StatusNoMemoryException;

impl std::fmt::Display for StatusNoMemoryException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Status No Memory")
    }
}

impl std::error::Error for StatusNoMemoryException {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_no_memory() {
        let exc = StatusNoMemoryException;
        let s = format!("{}", exc);
        assert_eq!(s, "Status No Memory");
    }
}
