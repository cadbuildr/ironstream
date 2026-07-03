// FILE: mat_side.rs
// occt: MAT_Side

/// Definition of left and right sides on a figure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatSide {
    /// Left side
    Left = 0,
    /// Right side
    Right = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_side_values() {
        assert_eq!(MatSide::Left as i32, 0);
        assert_eq!(MatSide::Right as i32, 1);
    }

    #[test]
    fn test_side_equality() {
        assert_eq!(MatSide::Left, MatSide::Left);
        assert_ne!(MatSide::Left, MatSide::Right);
    }

    #[test]
    fn test_side_copy() {
        let left = MatSide::Left;
        let left2 = left;
        assert_eq!(left, left2);
    }
}
