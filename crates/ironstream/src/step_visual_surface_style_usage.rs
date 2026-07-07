// FILE: step_visual_surface_style_usage.rs
// occt: StepVisual_SurfaceStyleUsage

use std::sync::Arc;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceSide {
    Negative = 0,
    Positive = 1,
    Both = 2,
}

pub struct SurfaceSideStyle;

pub struct SurfaceStyleUsage {
    side: SurfaceSide,
    style: Option<Arc<SurfaceSideStyle>>,
}

impl SurfaceStyleUsage {
    pub fn new() -> Self {
        SurfaceStyleUsage {
            side: SurfaceSide::Negative,
            style: None,
        }
    }

    pub fn init(&mut self, side: SurfaceSide, style: Option<Arc<SurfaceSideStyle>>) {
        self.side = side;
        self.style = style;
    }

    pub fn set_side(&mut self, side: SurfaceSide) {
        self.side = side;
    }

    pub fn side(&self) -> SurfaceSide {
        self.side
    }

    pub fn set_style(&mut self, style: Option<Arc<SurfaceSideStyle>>) {
        self.style = style;
    }

    pub fn style(&self) -> Option<&Arc<SurfaceSideStyle>> {
        self.style.as_ref()
    }
}

impl Default for SurfaceStyleUsage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ssu = SurfaceStyleUsage::new();
        assert_eq!(ssu.side(), SurfaceSide::Negative);
        assert!(ssu.style().is_none());
    }

    #[test]
    fn test_set_and_get_side() {
        let mut ssu = SurfaceStyleUsage::new();
        ssu.set_side(SurfaceSide::Positive);
        assert_eq!(ssu.side(), SurfaceSide::Positive);
    }

    #[test]
    fn test_set_and_get_style() {
        let mut ssu = SurfaceStyleUsage::new();
        let style = Arc::new(SurfaceSideStyle);
        ssu.set_style(Some(style.clone()));
        assert!(ssu.style().is_some());
    }

    #[test]
    fn test_init() {
        let mut ssu = SurfaceStyleUsage::new();
        let style = Arc::new(SurfaceSideStyle);
        ssu.init(SurfaceSide::Both, Some(style));
        assert_eq!(ssu.side(), SurfaceSide::Both);
        assert!(ssu.style().is_some());
    }
}
