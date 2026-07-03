// FILE: graphic3d_buffer_type.rs
// occt: Graphic3d_BufferType

//! Define buffers available for dump/export in 3D graphics.

/// Enumeration of available buffer types for rendering and export operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BufferType {
    /// Color buffer without alpha component (RGB).
    Rgb,
    /// Color buffer with alpha component (RGBA).
    Rgba,
    /// Depth buffer.
    Depth,
    /// Left view HDR color buffer for Ray-Tracing.
    RgbRayTraceHdrLeft,
    /// Color buffer, red channel only.
    Red,
    /// Buffer with shadow map data.
    ShadowMap,
}

impl BufferType {
    /// Returns the name of the buffer type.
    pub fn name(&self) -> &'static str {
        match self {
            BufferType::Rgb => "RGB",
            BufferType::Rgba => "RGBA",
            BufferType::Depth => "Depth",
            BufferType::RgbRayTraceHdrLeft => "RGB_RayTraceHdrLeft",
            BufferType::Red => "Red",
            BufferType::ShadowMap => "ShadowMap",
        }
    }

    /// Returns true if this is a color buffer type.
    pub fn is_color_buffer(&self) -> bool {
        matches!(
            self,
            BufferType::Rgb | BufferType::Rgba | BufferType::Red | BufferType::RgbRayTraceHdrLeft
        )
    }

    /// Returns true if this buffer type includes alpha channel.
    pub fn has_alpha(&self) -> bool {
        matches!(self, BufferType::Rgba)
    }
}

impl std::fmt::Display for BufferType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_type_names() {
        assert_eq!(BufferType::Rgb.name(), "RGB");
        assert_eq!(BufferType::Rgba.name(), "RGBA");
        assert_eq!(BufferType::Depth.name(), "Depth");
        assert_eq!(BufferType::RgbRayTraceHdrLeft.name(), "RGB_RayTraceHdrLeft");
        assert_eq!(BufferType::Red.name(), "Red");
        assert_eq!(BufferType::ShadowMap.name(), "ShadowMap");
    }

    #[test]
    fn test_buffer_type_is_color_buffer() {
        assert!(BufferType::Rgb.is_color_buffer());
        assert!(BufferType::Rgba.is_color_buffer());
        assert!(BufferType::Red.is_color_buffer());
        assert!(BufferType::RgbRayTraceHdrLeft.is_color_buffer());
        assert!(!BufferType::Depth.is_color_buffer());
        assert!(!BufferType::ShadowMap.is_color_buffer());
    }

    #[test]
    fn test_buffer_type_has_alpha() {
        assert!(!BufferType::Rgb.has_alpha());
        assert!(BufferType::Rgba.has_alpha());
        assert!(!BufferType::Depth.has_alpha());
        assert!(!BufferType::Red.has_alpha());
        assert!(!BufferType::RgbRayTraceHdrLeft.has_alpha());
        assert!(!BufferType::ShadowMap.has_alpha());
    }

    #[test]
    fn test_buffer_type_display() {
        assert_eq!(format!("{}", BufferType::Rgb), "RGB");
        assert_eq!(format!("{}", BufferType::Rgba), "RGBA");
        assert_eq!(format!("{}", BufferType::Depth), "Depth");
    }

    #[test]
    fn test_buffer_type_clone() {
        let bt = BufferType::Rgba;
        let bt_cloned = bt.clone();
        assert_eq!(bt, bt_cloned);
    }

    #[test]
    fn test_buffer_type_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(BufferType::Rgb);
        set.insert(BufferType::Rgba);
        set.insert(BufferType::Depth);
        assert_eq!(set.len(), 3);
        assert!(set.contains(&BufferType::Rgb));
        assert!(!set.contains(&BufferType::ShadowMap));
    }
}
