// FILE: osd_exception_stack_overflow.rs
// occt: OSD_Exception_STACK_OVERFLOW

/// Stack overflow exception.
#[derive(Debug, Clone)]
pub struct StackOverflowException;

impl std::fmt::Display for StackOverflowException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Stack Overflow")
    }
}

impl std::error::Error for StackOverflowException {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_overflow() {
        let exc = StackOverflowException;
        let s = format!("{}", exc);
        assert_eq!(s, "Stack Overflow");
    }
}
