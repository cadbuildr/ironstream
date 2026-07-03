// FILE: osd_exception_int_overflow.rs
// occt: OSD_Exception_INT_OVERFLOW

/// Integer overflow exception.
#[derive(Debug, Clone)]
pub struct IntOverflowException;

impl std::fmt::Display for IntOverflowException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Integer Overflow")
    }
}

impl std::error::Error for IntOverflowException {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_overflow() {
        let exc = IntOverflowException;
        let s = format!("{}", exc);
        assert_eq!(s, "Integer Overflow");
    }
}
