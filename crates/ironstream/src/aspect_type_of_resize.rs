// FILE: aspect_type_of_resize.rs
// occt: Aspect_TypeOfResize

/// Window resize edge/corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectTypeOfResize {
    /// No resize.
    None = 0,
    /// Left edge.
    Left = 1,
    /// Right edge.
    Right = 2,
    /// Top edge.
    Top = 3,
    /// Bottom edge.
    Bottom = 4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values() {
        assert_eq!(AspectTypeOfResize::None as i32, 0);
        assert_eq!(AspectTypeOfResize::Left as i32, 1);
        assert_eq!(AspectTypeOfResize::Bottom as i32, 4);
    }
}
