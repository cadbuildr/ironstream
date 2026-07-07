// FILE: step_visual_surface_style_control_grid.rs
// occt: StepVisual_SurfaceStyleControlGrid

use std::sync::Arc;

pub struct CurveStyle;

pub struct SurfaceStyleControlGrid {
    style_of_control_grid: Option<Arc<CurveStyle>>,
}

impl SurfaceStyleControlGrid {
    pub fn new() -> Self {
        SurfaceStyleControlGrid {
            style_of_control_grid: None,
        }
    }

    pub fn init(&mut self, style: Option<Arc<CurveStyle>>) {
        self.style_of_control_grid = style;
    }

    pub fn set_style_of_control_grid(&mut self, style: Option<Arc<CurveStyle>>) {
        self.style_of_control_grid = style;
    }

    pub fn style_of_control_grid(&self) -> Option<&Arc<CurveStyle>> {
        self.style_of_control_grid.as_ref()
    }
}

impl Default for SurfaceStyleControlGrid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sscg = SurfaceStyleControlGrid::new();
        assert!(sscg.style_of_control_grid().is_none());
    }

    #[test]
    fn test_set_and_get_style() {
        let mut sscg = SurfaceStyleControlGrid::new();
        let style = Arc::new(CurveStyle);
        sscg.set_style_of_control_grid(Some(style.clone()));
        assert!(sscg.style_of_control_grid().is_some());
    }

    #[test]
    fn test_init() {
        let mut sscg = SurfaceStyleControlGrid::new();
        let style = Arc::new(CurveStyle);
        sscg.init(Some(style));
        assert!(sscg.style_of_control_grid().is_some());
    }
}
