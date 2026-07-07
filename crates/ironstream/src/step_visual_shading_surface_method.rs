// FILE: step_visual_shading_surface_method.rs
// occt: StepVisual_ShadingSurfaceMethod

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadingSurfaceMethod {
    ConstantShading = 0,
    ColourShading = 1,
    DotShading = 2,
    NormalShading = 3,
}

impl ShadingSurfaceMethod {
    pub fn to_i32(self) -> i32 {
        self as i32
    }

    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(ShadingSurfaceMethod::ConstantShading),
            1 => Some(ShadingSurfaceMethod::ColourShading),
            2 => Some(ShadingSurfaceMethod::DotShading),
            3 => Some(ShadingSurfaceMethod::NormalShading),
            _ => None,
        }
    }
}

impl Default for ShadingSurfaceMethod {
    fn default() -> Self {
        ShadingSurfaceMethod::ConstantShading
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_values() {
        assert_eq!(ShadingSurfaceMethod::ConstantShading as i32, 0);
        assert_eq!(ShadingSurfaceMethod::ColourShading as i32, 1);
        assert_eq!(ShadingSurfaceMethod::DotShading as i32, 2);
        assert_eq!(ShadingSurfaceMethod::NormalShading as i32, 3);
    }

    #[test]
    fn test_to_i32() {
        assert_eq!(ShadingSurfaceMethod::ConstantShading.to_i32(), 0);
        assert_eq!(ShadingSurfaceMethod::ColourShading.to_i32(), 1);
        assert_eq!(ShadingSurfaceMethod::DotShading.to_i32(), 2);
        assert_eq!(ShadingSurfaceMethod::NormalShading.to_i32(), 3);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(
            ShadingSurfaceMethod::from_i32(0),
            Some(ShadingSurfaceMethod::ConstantShading)
        );
        assert_eq!(
            ShadingSurfaceMethod::from_i32(1),
            Some(ShadingSurfaceMethod::ColourShading)
        );
        assert_eq!(
            ShadingSurfaceMethod::from_i32(2),
            Some(ShadingSurfaceMethod::DotShading)
        );
        assert_eq!(
            ShadingSurfaceMethod::from_i32(3),
            Some(ShadingSurfaceMethod::NormalShading)
        );
        assert_eq!(ShadingSurfaceMethod::from_i32(99), None);
    }

    #[test]
    fn test_default() {
        assert_eq!(ShadingSurfaceMethod::default(), ShadingSurfaceMethod::ConstantShading);
    }

    #[test]
    fn test_copy() {
        let method = ShadingSurfaceMethod::ColourShading;
        let method2 = method;
        assert_eq!(method, method2);
    }
}
