// FILE: math_singular_matrix.rs
// occt: math_SingularMatrix

/// Exception for singular matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingularMatrix;

impl std::fmt::Display for SingularMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix is singular")
    }
}

impl std::error::Error for SingularMatrix {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singular_matrix_display() {
        let err = SingularMatrix;
        assert_eq!(format!("{}", err), "Matrix is singular");
    }
}
