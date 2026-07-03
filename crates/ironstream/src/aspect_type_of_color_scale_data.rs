// FILE: aspect_type_of_color_scale_data.rs
// occt: Aspect_TypeOfColorScaleData

/// Type of color scale data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectTypeOfColorScaleData {
    /// Automatically computed from object geometry.
    Automatic = 0,
    /// User-defined range.
    UserDefined = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values() {
        assert_eq!(AspectTypeOfColorScaleData::Automatic as i32, 0);
        assert_eq!(AspectTypeOfColorScaleData::UserDefined as i32, 1);
    }
}
