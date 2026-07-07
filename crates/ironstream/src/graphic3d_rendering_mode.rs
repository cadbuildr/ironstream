// FILE: graphic3d_rendering_mode.rs
// occt: Graphic3d_RenderingMode

/// Describes rendering modes for graphic rendering.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderingMode {
    /// Enables OpenGL rasterization mode
    Rasterization = 0,
    /// Enables GPU ray-tracing mode
    Raytracing = 1,
}

impl RenderingMode {
    /// Returns the numeric value of the rendering mode.
    pub fn as_u32(self) -> u32 {
        self as u32
    }

    /// Converts from u32 to RenderingMode, defaulting to Rasterization for unknown values.
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => RenderingMode::Rasterization,
            1 => RenderingMode::Raytracing,
            _ => RenderingMode::Rasterization,
        }
    }

    /// Returns true if this is rasterization mode.
    pub fn is_rasterization(self) -> bool {
        self == RenderingMode::Rasterization
    }

    /// Returns true if this is raytracing mode.
    pub fn is_raytracing(self) -> bool {
        self == RenderingMode::Raytracing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rendering_mode_values() {
        assert_eq!(RenderingMode::Rasterization.as_u32(), 0);
        assert_eq!(RenderingMode::Raytracing.as_u32(), 1);
    }

    #[test]
    fn test_rendering_mode_from_u32() {
        assert_eq!(RenderingMode::from_u32(0), RenderingMode::Rasterization);
        assert_eq!(RenderingMode::from_u32(1), RenderingMode::Raytracing);
        assert_eq!(RenderingMode::from_u32(99), RenderingMode::Rasterization); // unknown defaults to rasterization
    }

    #[test]
    fn test_rendering_mode_is_rasterization() {
        assert!(RenderingMode::Rasterization.is_rasterization());
        assert!(!RenderingMode::Raytracing.is_rasterization());
    }

    #[test]
    fn test_rendering_mode_is_raytracing() {
        assert!(!RenderingMode::Rasterization.is_raytracing());
        assert!(RenderingMode::Raytracing.is_raytracing());
    }

    #[test]
    fn test_rendering_mode_roundtrip() {
        let original = RenderingMode::Raytracing;
        let value = original.as_u32();
        let converted = RenderingMode::from_u32(value);
        assert_eq!(original, converted);
    }

    #[test]
    fn test_rendering_mode_clone_copy() {
        let mode1 = RenderingMode::Rasterization;
        let mode2 = mode1;
        assert_eq!(mode1, mode2);
    }

    #[test]
    fn test_rendering_mode_debug() {
        let mode = RenderingMode::Raytracing;
        let debug_str = format!("{:?}", mode);
        assert!(debug_str.contains("Raytracing"));
    }
}
