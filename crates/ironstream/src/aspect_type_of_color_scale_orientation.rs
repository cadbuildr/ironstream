// FILE: aspect_type_of_color_scale_orientation.rs
// occt: Aspect_TypeOfColorScaleOrientation

/// Orientation of color scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectTypeOfColorScaleOrientation {
    /// Vertical scale.
    Vertical = 0,
    /// Horizontal scale.
    Horizontal = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values() {
        assert_eq!(AspectTypeOfColorScaleOrientation::Vertical as i32, 0);
        assert_eq!(AspectTypeOfColorScaleOrientation::Horizontal as i32, 1);
    }
}
