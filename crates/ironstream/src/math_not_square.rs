// FILE: math_not_square.rs
// occt: math_NotSquare

/// Exception for non-square matrix operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotSquare;

impl std::fmt::Display for NotSquare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix is not square")
    }
}

impl std::error::Error for NotSquare {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_square_display() {
        let err = NotSquare;
        assert_eq!(format!("{}", err), "Matrix is not square");
    }
}
