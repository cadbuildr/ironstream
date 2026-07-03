// FILE: aspect_color_space.rs
// occt: Aspect_ColorSpace

/// Texture color spaces accepted by XR composer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AspectColorSpace {
    /// Non-linear sRGB color space.
    SRGB = 0,
    /// Linear RGB color space.
    Linear = 1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values() {
        assert_eq!(AspectColorSpace::SRGB as i32, 0);
        assert_eq!(AspectColorSpace::Linear as i32, 1);
    }
}
