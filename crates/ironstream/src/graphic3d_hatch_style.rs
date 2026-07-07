// FILE: graphic3d_hatch_style.rs
// occt: Graphic3d_HatchStyle

/// Aspect hatch style enumeration (predefined styles)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AspectHatchStyle {
    Solid = 0,
    HorizontalWide = 1,
    VerticalWide = 2,
    DiagWide = 3,
}

impl Default for AspectHatchStyle {
    fn default() -> Self {
        AspectHatchStyle::Solid
    }
}

/// Hatch style for fill patterns
#[derive(Debug)]
pub struct HatchStyle {
    hatch_type: i32,
    pattern: Option<Vec<u8>>,
}

impl HatchStyle {
    /// Creates a new predefined hatch style
    pub fn new_predefined(style_type: AspectHatchStyle) -> Self {
        HatchStyle {
            hatch_type: style_type as i32,
            pattern: None,
        }
    }

    /// Creates a new custom hatch style with the given pattern
    /// Pattern must be a 32x32 bitmap (1024 bytes)
    pub fn new_custom(pattern: Vec<u8>, style_id: i32) -> Result<Self, &'static str> {
        if pattern.len() != 1024 {
            return Err("Pattern must be exactly 1024 bytes (32x32 bitmap)");
        }
        Ok(HatchStyle {
            hatch_type: style_id,
            pattern: Some(pattern),
        })
    }

    /// Returns the pattern of custom hatch style
    pub fn pattern(&self) -> Option<&[u8]> {
        self.pattern.as_deref()
    }

    /// Returns the hatch type (either predefined enum value or custom style ID)
    pub fn hatch_type(&self) -> i32 {
        self.hatch_type
    }

    /// Returns true if this is a custom hatch style
    pub fn is_custom(&self) -> bool {
        self.pattern.is_some()
    }
}

impl Default for HatchStyle {
    fn default() -> Self {
        HatchStyle::new_predefined(AspectHatchStyle::Solid)
    }
}

impl Clone for HatchStyle {
    fn clone(&self) -> Self {
        HatchStyle {
            hatch_type: self.hatch_type,
            pattern: self.pattern.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predefined_hatch_style() {
        let style = HatchStyle::new_predefined(AspectHatchStyle::Solid);
        assert_eq!(style.hatch_type(), AspectHatchStyle::Solid as i32);
        assert!(style.pattern().is_none());
        assert!(!style.is_custom());
    }

    #[test]
    fn test_custom_hatch_style() {
        let pattern = vec![0u8; 1024];
        let style = HatchStyle::new_custom(pattern.clone(), 100).unwrap();
        assert_eq!(style.hatch_type(), 100);
        assert!(style.pattern().is_some());
        assert!(style.is_custom());
        assert_eq!(style.pattern().unwrap().len(), 1024);
    }

    #[test]
    fn test_custom_hatch_style_invalid_size() {
        let pattern = vec![0u8; 512];
        let result = HatchStyle::new_custom(pattern, 100);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Pattern must be exactly 1024 bytes (32x32 bitmap)");
    }

    #[test]
    fn test_hatch_style_default() {
        let style = HatchStyle::default();
        assert_eq!(style.hatch_type(), AspectHatchStyle::Solid as i32);
        assert!(!style.is_custom());
    }

    #[test]
    fn test_hatch_style_clone() {
        let pattern = vec![1u8; 1024];
        let style = HatchStyle::new_custom(pattern, 50).unwrap();
        let cloned = style.clone();
        assert_eq!(style.hatch_type(), cloned.hatch_type());
        assert_eq!(style.is_custom(), cloned.is_custom());
        assert_eq!(style.pattern().unwrap(), cloned.pattern().unwrap());
    }

    #[test]
    fn test_aspect_hatch_style_enum() {
        assert_eq!(AspectHatchStyle::Solid as i32, 0);
        assert_eq!(AspectHatchStyle::HorizontalWide as i32, 1);
        assert_eq!(AspectHatchStyle::VerticalWide as i32, 2);
        assert_eq!(AspectHatchStyle::DiagWide as i32, 3);
    }

    #[test]
    fn test_multiple_predefined_styles() {
        let style1 = HatchStyle::new_predefined(AspectHatchStyle::HorizontalWide);
        let style2 = HatchStyle::new_predefined(AspectHatchStyle::VerticalWide);
        assert_ne!(style1.hatch_type(), style2.hatch_type());
    }
}
