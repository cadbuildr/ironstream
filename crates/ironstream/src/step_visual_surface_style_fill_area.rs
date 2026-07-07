// FILE: step_visual_surface_style_fill_area.rs
// occt: StepVisual_SurfaceStyleFillArea

use std::sync::Arc;

pub struct FillAreaStyle;

pub struct SurfaceStyleFillArea {
    fill_area: Option<Arc<FillAreaStyle>>,
}

impl SurfaceStyleFillArea {
    pub fn new() -> Self {
        SurfaceStyleFillArea {
            fill_area: None,
        }
    }

    pub fn init(&mut self, fill_area: Option<Arc<FillAreaStyle>>) {
        self.fill_area = fill_area;
    }

    pub fn set_fill_area(&mut self, fill_area: Option<Arc<FillAreaStyle>>) {
        self.fill_area = fill_area;
    }

    pub fn fill_area(&self) -> Option<&Arc<FillAreaStyle>> {
        self.fill_area.as_ref()
    }
}

impl Default for SurfaceStyleFillArea {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ssfa = SurfaceStyleFillArea::new();
        assert!(ssfa.fill_area().is_none());
    }

    #[test]
    fn test_set_and_get_fill_area() {
        let mut ssfa = SurfaceStyleFillArea::new();
        let fill_area = Arc::new(FillAreaStyle);
        ssfa.set_fill_area(Some(fill_area.clone()));
        assert!(ssfa.fill_area().is_some());
    }

    #[test]
    fn test_init() {
        let mut ssfa = SurfaceStyleFillArea::new();
        let fill_area = Arc::new(FillAreaStyle);
        ssfa.init(Some(fill_area));
        assert!(ssfa.fill_area().is_some());
    }
}
