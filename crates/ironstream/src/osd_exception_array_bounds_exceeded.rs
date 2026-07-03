// FILE: osd_exception_array_bounds_exceeded.rs
// occt: OSD_Exception_ARRAY_BOUNDS_EXCEEDED

/// Array bounds exceeded exception.
#[derive(Debug, Clone)]
pub struct ArrayBoundsExceededException;

impl std::fmt::Display for ArrayBoundsExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Array Bounds Exceeded")
    }
}

impl std::error::Error for ArrayBoundsExceededException {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_bounds_exceeded() {
        let exc = ArrayBoundsExceededException;
        let s = format!("{}", exc);
        assert_eq!(s, "Array Bounds Exceeded");
    }
}
