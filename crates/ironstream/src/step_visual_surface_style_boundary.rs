// FILE: step_visual_surface_style_boundary.rs
// occt: StepVisual_SurfaceStyleBoundary

use std::sync::Arc;

pub struct CurveStyle;

pub struct SurfaceStyleBoundary {
    style_of_boundary: Option<Arc<CurveStyle>>,
}

impl SurfaceStyleBoundary {
    pub fn new() -> Self {
        SurfaceStyleBoundary {
            style_of_boundary: None,
        }
    }

    pub fn init(&mut self, style: Option<Arc<CurveStyle>>) {
        self.style_of_boundary = style;
    }

    pub fn set_style_of_boundary(&mut self, style: Option<Arc<CurveStyle>>) {
        self.style_of_boundary = style;
    }

    pub fn style_of_boundary(&self) -> Option<&Arc<CurveStyle>> {
        self.style_of_boundary.as_ref()
    }
}

impl Default for SurfaceStyleBoundary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ssb = SurfaceStyleBoundary::new();
        assert!(ssb.style_of_boundary().is_none());
    }

    #[test]
    fn test_set_and_get_style() {
        let mut ssb = SurfaceStyleBoundary::new();
        let style = Arc::new(CurveStyle);
        ssb.set_style_of_boundary(Some(style.clone()));
        assert!(ssb.style_of_boundary().is_some());
    }

    #[test]
    fn test_init() {
        let mut ssb = SurfaceStyleBoundary::new();
        let style = Arc::new(CurveStyle);
        ssb.init(Some(style));
        assert!(ssb.style_of_boundary().is_some());
    }
}
