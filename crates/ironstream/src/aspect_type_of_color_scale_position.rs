// FILE: aspect_type_of_color_scale_position.rs
// occt: Aspect_TypeOfColorScalePosition

/// Position of color scale in view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectTypeOfColorScalePosition {
    /// Left side.
    Left = 0,
    /// Right side.
    Right = 1,
    /// Top side.
    Top = 2,
    /// Bottom side.
    Bottom = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values() {
        assert_eq!(AspectTypeOfColorScalePosition::Left as i32, 0);
        assert_eq!(AspectTypeOfColorScalePosition::Right as i32, 1);
        assert_eq!(AspectTypeOfColorScalePosition::Bottom as i32, 3);
    }
}
